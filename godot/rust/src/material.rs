//! ShaderMaterials for ZSC materials.
//!
//! The shader logic lives in `res://shaders/rose_object.gdshaderinc`. Each render state
//! combination (alpha mode, culling, depth flags, lighting path) gets its own small
//! Shader that sets `render_mode` and a few defines and then includes that file.

use std::{cell::RefCell, collections::HashMap};

use godot::{
    classes::{Shader, ShaderMaterial, Texture2D},
    prelude::*,
};
use rose_file_readers::ZscMaterial;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderKey {
    pub alpha: AlphaMode,
    pub two_sided: bool,
    pub z_write: bool,
    pub z_test: bool,
    pub lightmap: bool,
    pub specular: bool,
    pub alpha_value: bool,
}

thread_local! {
    static SHADERS: RefCell<HashMap<ShaderKey, Gd<Shader>>> = RefCell::new(HashMap::new());
    static SPECULAR: RefCell<Option<Option<Gd<Texture2D>>>> = const { RefCell::new(None) };
}

fn shader_code(key: &ShaderKey) -> String {
    let mut modes = vec!["unshaded", "fog_disabled"];
    modes.push(if key.two_sided { "cull_disabled" } else { "cull_back" });
    modes.push(match (key.alpha, key.z_write) {
        (_, false) => "depth_draw_never",
        (AlphaMode::Blend, true) => "depth_draw_always",
        _ => "depth_draw_opaque",
    });
    if !key.z_test {
        modes.push("depth_test_disabled");
    }
    if key.alpha == AlphaMode::Blend {
        modes.push("blend_mix");
    }

    let mut code = format!("shader_type spatial;\nrender_mode {};\n", modes.join(", "));
    let mut define = |name: &str, on: bool| {
        if on {
            code.push_str(&format!("#define {name}\n"));
        }
    };
    define("ALPHA_MASK", key.alpha == AlphaMode::Mask);
    define("ALPHA_BLEND", key.alpha == AlphaMode::Blend);
    define("HAS_LIGHTMAP", key.lightmap);
    define("SPECULAR", key.specular);
    define("HAS_ALPHA_VALUE", key.alpha_value);
    code.push_str("#include \"res://shaders/rose_object.gdshaderinc\"\n");
    code
}

pub fn shader(key: ShaderKey) -> Gd<Shader> {
    SHADERS.with_borrow_mut(|cache| {
        cache
            .entry(key)
            .or_insert_with(|| {
                let mut shader = Shader::new_gd();
                shader.set_code(&shader_code(&key));
                shader
            })
            .clone()
    })
}

fn specular_texture() -> Option<Gd<Texture2D>> {
    SPECULAR.with_borrow_mut(|cached| {
        cached
            .get_or_insert_with(|| crate::texture::load_texture("ETC/SPECULAR_SPHEREMAP.DDS"))
            .clone()
    })
}

/// Mirrors ObjectMaterial's alpha_mode() and uniform setup in the Bevy client.
pub fn object_material(zsc_material: &ZscMaterial, lightmap: Option<Gd<Texture2D>>) -> Gd<ShaderMaterial> {
    let specular = zsc_material.specular_enabled;
    let alpha_value = if specular || zsc_material.alpha == 1.0 { None } else { Some(zsc_material.alpha) };
    let alpha = if specular {
        AlphaMode::Opaque
    } else if alpha_value.is_some() {
        AlphaMode::Blend
    } else if zsc_material.alpha_enabled {
        if zsc_material.alpha_test.is_some() { AlphaMode::Mask } else { AlphaMode::Blend }
    } else {
        AlphaMode::Opaque
    };

    let key = ShaderKey {
        alpha,
        two_sided: zsc_material.two_sided,
        z_write: zsc_material.z_write_enabled,
        z_test: zsc_material.z_test_enabled,
        lightmap: lightmap.is_some(),
        specular,
        alpha_value: alpha_value.is_some(),
    };

    let mut material = ShaderMaterial::new_gd();
    material.set_shader(&shader(key));
    if let Some(texture) = crate::texture::load_texture(&zsc_material.path.path().to_string_lossy()) {
        material.set_shader_parameter("base_texture", &texture.to_variant());
    }
    if let Some(lightmap) = lightmap {
        material.set_shader_parameter("lightmap_texture", &lightmap.to_variant());
    }
    if specular {
        if let Some(texture) = specular_texture() {
            material.set_shader_parameter("specular_texture", &texture.to_variant());
        }
    }
    material.set_shader_parameter("alpha_cutoff", &zsc_material.alpha_test.unwrap_or(0.5).to_variant());
    material.set_shader_parameter("alpha_value", &alpha_value.unwrap_or(1.0).to_variant());
    material
}

/// Drops the cached shaders while the engine is still running.
pub fn clear_cache() {
    SHADERS.with_borrow_mut(|cache| cache.clear());
    SPECULAR.with_borrow_mut(|cache| *cache = None);
}
