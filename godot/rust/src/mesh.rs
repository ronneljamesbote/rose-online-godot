//! ZMS meshes to Godot ArrayMesh.
//!
//! ROSE is Z-up and Godot is Y-up, so positions and normals go through the same
//! (x, y, z) -> (x, z, -y) swap the Bevy client uses. Godot treats clockwise triangles
//! as front faces, the opposite of ROSE and Bevy, so each triangle's winding is flipped.

use std::{cell::RefCell, collections::HashMap};

use godot::{
    classes::{mesh::{ArrayType, PrimitiveType}, ArrayMesh},
    prelude::*,
};
use rose_file_readers::ZmsFile;

thread_local! {
    static MESHES: RefCell<HashMap<String, Option<Gd<ArrayMesh>>>> = RefCell::new(HashMap::new());
}

pub fn swap_axes(v: [f32; 3]) -> Vector3 {
    Vector3::new(v[0], v[2], -v[1])
}

pub fn flip_winding(indices: &[u16]) -> PackedInt32Array {
    let mut out = Vec::with_capacity(indices.len());
    for tri in indices.chunks_exact(3) {
        out.extend([tri[0] as i32, tri[2] as i32, tri[1] as i32]);
    }
    PackedInt32Array::from(out.as_slice())
}

fn uv_array(uvs: &[[f32; 2]]) -> PackedVector2Array {
    uvs.iter().map(|uv| Vector2::new(uv[0], uv[1])).collect()
}

/// Loads a ZMS file as a one-surface ArrayMesh. Bone weights are kept only when `skinned`.
pub fn load_mesh(path: &str, skinned: bool) -> Option<Gd<ArrayMesh>> {
    let key = format!("{}#{}", path.to_ascii_uppercase(), skinned);
    if let Some(cached) = MESHES.with_borrow(|cache| cache.get(&key).cloned()) {
        return cached;
    }
    let mesh = crate::data::read_file::<ZmsFile>(path).and_then(|zms| build_mesh(&zms, skinned));
    if mesh.is_none() {
        godot_warn!("rose: could not load mesh {path}");
    }
    MESHES.with_borrow_mut(|cache| cache.insert(key, mesh.clone()));
    mesh
}

pub fn build_mesh(zms: &ZmsFile, skinned: bool) -> Option<Gd<ArrayMesh>> {
    if zms.position.is_empty() || zms.indices.is_empty() {
        return None;
    }

    let mut arrays = VarArray::new();
    arrays.resize(ArrayType::MAX.ord() as usize, &Variant::nil());

    let positions: PackedVector3Array = zms.position.iter().map(|&p| swap_axes(p) ).collect();
    arrays.set(ArrayType::VERTEX.ord() as usize, &positions.to_variant());

    let normals: PackedVector3Array = if zms.normal.len() == zms.position.len() {
        zms.normal.iter().map(|&n| swap_axes(n)).collect()
    } else {
        std::iter::repeat(Vector3::UP).take(zms.position.len()).collect()
    };
    arrays.set(ArrayType::NORMAL.ord() as usize, &normals.to_variant());

    if zms.color.len() == zms.position.len() {
        let colors: PackedColorArray =
            zms.color.iter().map(|c| Color::from_rgba(c[0], c[1], c[2], c[3])).collect();
        arrays.set(ArrayType::COLOR.ord() as usize, &colors.to_variant());
    }
    if zms.uv1.len() == zms.position.len() {
        arrays.set(ArrayType::TEX_UV.ord() as usize, &uv_array(&zms.uv1).to_variant());
    }
    if zms.uv2.len() == zms.position.len() {
        arrays.set(ArrayType::TEX_UV2.ord() as usize, &uv_array(&zms.uv2).to_variant());
    }

    if skinned && zms.bone_indices.len() == zms.position.len() && zms.bone_weights.len() == zms.position.len() {
        let bones: PackedInt32Array =
            zms.bone_indices.iter().flat_map(|b| b.map(|i| i as i32)).collect();
        let weights: PackedFloat32Array = zms.bone_weights.iter().flat_map(|w| *w).collect();
        arrays.set(ArrayType::BONES.ord() as usize, &bones.to_variant());
        arrays.set(ArrayType::WEIGHTS.ord() as usize, &weights.to_variant());
    }

    arrays.set(ArrayType::INDEX.ord() as usize, &flip_winding(&zms.indices).to_variant());

    let mut mesh = ArrayMesh::new_gd();
    mesh.add_surface_from_arrays(PrimitiveType::TRIANGLES, &arrays);
    Some(mesh)
}
