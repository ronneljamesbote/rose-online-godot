//! ShaderMaterials for ZSC materials.
//!
//! The shader logic lives in `res://shaders/rose_object.gdshaderinc`. Each render state
//! combination (alpha mode, culling, depth flags) is one small shader file in
//! `res://shaders/objects/` that sets `render_mode` and a define and then includes it, so
//! every model scene links the same few files: Godot compiles each once, and a change to the
//! look needs no re-import. The rest (specular, a fixed alpha, ground or character light)
//! are material parameters.

use std::{cell::RefCell, collections::HashMap};

use godot::{
    classes::{ResourceLoader, Shader, ShaderMaterial, Texture2D},
    prelude::*,
};
use rose_file_readers::ZscMaterial;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ShaderKey {
    pub alpha: AlphaMode,
    pub two_sided: bool,
    pub z_write: bool,
    pub z_test: bool,
}

/// Where the shader files of the render states are.
pub const OBJECT_SHADERS: &str = "res://shaders/objects/";

/// How a model is lit: by the ground light (zone models) or the character light (characters,
/// monsters, NPCs, items). See `rose_object.gdshaderinc`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Light {
    Map,
    Character,
}

thread_local! {
    static SHADERS: RefCell<HashMap<ShaderKey, Gd<Shader>>> = RefCell::new(HashMap::new());
    static SPECULAR: RefCell<Option<Option<Gd<Texture2D>>>> = const { RefCell::new(None) };
    static GLOW_SHADERS: RefCell<Option<[Gd<Shader>; 2]>> = const { RefCell::new(None) };
    static GLOWS: RefCell<HashMap<(Option<InstanceId>, bool, [u32; 3]), Gd<ShaderMaterial>>> = RefCell::new(HashMap::new());
}

impl ShaderKey {
    /// Every render state, in a fixed order.
    #[cfg(test)]
    pub fn all() -> Vec<ShaderKey> {
        let mut keys = Vec::new();
        for alpha in [AlphaMode::Opaque, AlphaMode::Mask, AlphaMode::Blend] {
            for two_sided in [false, true] {
                for z_write in [true, false] {
                    for z_test in [true, false] {
                        keys.push(ShaderKey { alpha, two_sided, z_write, z_test });
                    }
                }
            }
        }
        keys
    }

    /// The shader file's name: `rose_object_mask_two_sided.gdshader`.
    pub fn file_name(&self) -> String {
        let mut name = String::from(match self.alpha {
            AlphaMode::Opaque => "rose_object_opaque",
            AlphaMode::Mask => "rose_object_mask",
            AlphaMode::Blend => "rose_object_blend",
        });
        if self.two_sided {
            name.push_str("_two_sided");
        }
        if !self.z_write {
            name.push_str("_no_depth_write");
        }
        if !self.z_test {
            name.push_str("_no_depth_test");
        }
        name + ".gdshader"
    }

    /// What the shader file holds.
    pub fn code(&self) -> String {
        let mut modes = vec!["fog_disabled", "specular_disabled", "ambient_light_disabled"];
        modes.push(if self.two_sided { "cull_disabled" } else { "cull_back" });
        modes.push(match (self.alpha, self.z_write) {
            (_, false) => "depth_draw_never",
            (AlphaMode::Blend, true) => "depth_draw_always",
            _ => "depth_draw_opaque",
        });
        if !self.z_test {
            modes.push("depth_test_disabled");
        }
        if self.alpha == AlphaMode::Blend {
            modes.push("blend_mix");
        }
        let mut code = format!(
            "// Made by material.rs (ShaderKey::code); the shader is rose_object.gdshaderinc.\nshader_type spatial;\nrender_mode {};\n",
            modes.join(", ")
        );
        match self.alpha {
            AlphaMode::Mask => code.push_str("#define ALPHA_MASK\n"),
            AlphaMode::Blend => code.push_str("#define ALPHA_BLEND\n"),
            AlphaMode::Opaque => {}
        }
        code.push_str("#include \"res://shaders/rose_object.gdshaderinc\"\n");
        code
    }
}

/// The shader of a render state: its file, or (when the file is missing) the same code.
pub fn shader(key: ShaderKey) -> Gd<Shader> {
    SHADERS.with_borrow_mut(|cache| {
        cache
            .entry(key)
            .or_insert_with(|| {
                let path = format!("{OBJECT_SHADERS}{}", key.file_name());
                if let Some(Ok(shader)) = ResourceLoader::singleton().exists(&path).then(|| godot::tools::try_load::<Shader>(&path)) {
                    return shader;
                }
                godot_warn!("rose: {path} is missing; using a built-in copy");
                let mut shader = Shader::new_gd();
                shader.set_code(&key.code());
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
pub fn object_material(zsc_material: &ZscMaterial, light: Light) -> Gd<ShaderMaterial> {
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
    };

    let mut material = ShaderMaterial::new_gd();
    material.set_shader(&shader(key));
    if let Some(texture) = crate::texture::load_texture(&zsc_material.path.path().to_string_lossy()) {
        material.set_shader_parameter("base_texture", &texture.to_variant());
    }
    if specular {
        if let Some(texture) = specular_texture() {
            material.set_shader_parameter("specular_texture", &texture.to_variant());
            material.set_shader_parameter("specular", &true.to_variant());
        }
    }
    if let Some(alpha_value) = alpha_value {
        material.set_shader_parameter("use_alpha_value", &true.to_variant());
        material.set_shader_parameter("alpha_value", &alpha_value.to_variant());
    }
    if light == Light::Map {
        material.set_shader_parameter("map_light", &true.to_variant());
    }
    material.set_shader_parameter("alpha_cutoff", &zsc_material.alpha_test.unwrap_or(0.5).to_variant());
    material
}

/// Whether a model part drawn with `material` casts a shadow: see-through parts (glass,
/// water, glows) don't, solid and cut-out parts (leaves) do.
pub fn casts_shadow(material: &Gd<ShaderMaterial>) -> bool {
    material.get_shader().is_none_or(|shader| !shader.get_code().to_string().contains("#define ALPHA_BLEND"))
}

fn glow_shaders() -> [Gd<Shader>; 2] {
    GLOW_SHADERS.with_borrow_mut(|cached| {
        cached
            .get_or_insert_with(|| {
                [false, true].map(|shell| {
                    let mut shader = Shader::new_gd();
                    shader.set_code(&format!(
                        "shader_type spatial;\nrender_mode unshaded, fog_disabled, blend_add, depth_draw_never, {}, shadows_disabled;\n{}#include \"res://shaders/rose_item_glow.gdshaderinc\"\n",
                        if shell { "cull_back" } else { "cull_disabled" },
                        if shell { "#define GLOW_SHELL\n" } else { "" },
                    ));
                    shader
                })
            })
            .clone()
    })
}

/// The refine glow of an equipment part drawn with `material` (an `object_material`), in
/// `colour` (LIST_GRADE's glow colour): an overlay material that brightens the part, with
/// the halo around it as its next pass. One per texture and colour.
pub fn grade_glow(material: Option<&Gd<ShaderMaterial>>, colour: (f32, f32, f32)) -> Gd<ShaderMaterial> {
    let texture = material.map(|m| m.get_shader_parameter("base_texture")).filter(|t| !t.is_nil());
    let alpha_tested = material.and_then(|m| m.get_shader()).is_some_and(|shader| {
        let code = shader.get_code().to_string();
        code.contains("#define ALPHA_MASK") || code.contains("#define ALPHA_BLEND")
    });
    let texture_id = texture.as_ref().and_then(|t| t.try_to::<Gd<Texture2D>>().ok()).map(|t| t.instance_id());
    let key = (texture_id, alpha_tested, [colour.0.to_bits(), colour.1.to_bits(), colour.2.to_bits()]);
    if let Some(cached) = GLOWS.with_borrow(|cache| cache.get(&key).cloned()) {
        return cached;
    }
    let [overlay_shader, shell_shader] = glow_shaders();
    let make = |shader: &Gd<Shader>| {
        let mut glow = ShaderMaterial::new_gd();
        glow.set_shader(shader);
        if let Some(texture) = &texture {
            glow.set_shader_parameter("base_texture", texture);
        }
        glow.set_shader_parameter("glow_colour", &Vector3::new(colour.0, colour.1, colour.2).to_variant());
        glow.set_shader_parameter("use_texture_alpha", &alpha_tested.to_variant());
        if let Some(material) = material {
            glow.set_shader_parameter("alpha_cutoff", &material.get_shader_parameter("alpha_cutoff"));
        }
        glow
    };
    let mut overlay = make(&overlay_shader);
    overlay.set_next_pass(&make(&shell_shader));
    GLOWS.with_borrow_mut(|cache| cache.insert(key, overlay.clone()));
    overlay
}

/// Drops the cached shaders while the engine is still running.
pub fn clear_cache() {
    SHADERS.with_borrow_mut(|cache| cache.clear());
    SPECULAR.with_borrow_mut(|cache| *cache = None);
    GLOW_SHADERS.with_borrow_mut(|cache| *cache = None);
    GLOWS.with_borrow_mut(|cache| cache.clear());
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The files in `shaders/objects/` are what `ShaderKey::code` makes. Run with
    /// `ROSE_WRITE_SHADERS=1` to write them after changing it.
    #[test]
    fn object_shader_files_are_current() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../shaders/objects");
        let write = std::env::var_os("ROSE_WRITE_SHADERS").is_some();
        let mut wanted = std::collections::BTreeSet::new();
        for key in ShaderKey::all() {
            let path = dir.join(key.file_name());
            wanted.insert(key.file_name());
            if write {
                std::fs::create_dir_all(&dir).unwrap();
                std::fs::write(&path, key.code()).unwrap();
            }
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            assert_eq!(text, key.code(), "{} is out of date; run ROSE_WRITE_SHADERS=1 cargo test -p rose-gd", path.display());
        }
        for entry in std::fs::read_dir(&dir).unwrap() {
            let name = entry.unwrap().file_name().to_string_lossy().to_string();
            if name.ends_with(".gdshader") {
                assert!(wanted.contains(&name), "{name} in shaders/objects is not a render state any more");
            }
        }
    }
}
