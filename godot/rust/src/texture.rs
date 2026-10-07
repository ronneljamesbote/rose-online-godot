//! DDS textures read from the VFS, cached by path.

use std::{cell::RefCell, collections::HashMap};

use godot::{
    classes::{image::Format, Image, ImageTexture, Texture2D, Texture2DArray},
    global::Error,
    prelude::*,
};

thread_local! {
    static TEXTURES: RefCell<HashMap<String, Option<Gd<Texture2D>>>> = RefCell::new(HashMap::new());
}

fn cache_key(path: &str) -> String {
    path.replace('\\', "/").to_ascii_uppercase()
}

/// Many ROSE DDS files declare a full mip chain but only store the first level or two.
/// Godot rejects those, so such files are cut down to level 0 here and their mipmaps
/// are generated after loading. Returns the patched bytes and whether mipmaps are needed.
fn fix_partial_mip_chain(mut bytes: Vec<u8>) -> (Vec<u8>, bool) {
    if bytes.len() < 128 || &bytes[0..4] != b"DDS " {
        return (bytes, false);
    }
    let u32_at = |b: &[u8], o: usize| u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
    let height = u32_at(&bytes, 12).max(1) as usize;
    let width = u32_at(&bytes, 16).max(1) as usize;
    let mip_count = u32_at(&bytes, 28) as usize;
    let pf_flags = u32_at(&bytes, 80);
    let four_cc: [u8; 4] = bytes[84..88].try_into().unwrap();
    let bit_count = u32_at(&bytes, 88) as usize;
    let level_size = |w: usize, h: usize| -> usize {
        match (pf_flags & 0x4 != 0, &four_cc) {
            (true, b"DXT1") => w.div_ceil(4).max(1) * h.div_ceil(4).max(1) * 8,
            (true, _) => w.div_ceil(4).max(1) * h.div_ceil(4).max(1) * 16,
            (false, _) => w * h * bit_count.max(8) / 8,
        }
    };
    if pf_flags & 0x4 != 0 && &four_cc == b"DX10" {
        return (bytes, false);
    }

    let full_chain = (usize::BITS - width.max(height).leading_zeros()) as usize;
    if mip_count <= 1 {
        return (bytes, true);
    }
    let mut total = 0;
    let (mut w, mut h) = (width, height);
    for _ in 0..mip_count.min(full_chain) {
        total += level_size(w, h);
        w = (w / 2).max(1);
        h = (h / 2).max(1);
    }
    if mip_count >= full_chain && bytes.len() - 128 >= total {
        return (bytes, false);
    }

    // Keep level 0 only.
    bytes[28..32].copy_from_slice(&1u32.to_le_bytes());
    let flags = u32_at(&bytes, 8) & !0x20000; // DDSD_MIPMAPCOUNT
    bytes[8..12].copy_from_slice(&flags.to_le_bytes());
    bytes.truncate(128 + level_size(width, height));
    (bytes, true)
}

pub fn load_image(path: &str) -> Option<Gd<Image>> {
    let (bytes, needs_mipmaps) = fix_partial_mip_chain(crate::data::read_bytes(path)?);
    let mut image = Image::new_gd();
    if image.load_dds_from_buffer(&PackedByteArray::from(bytes.as_slice())) != Error::OK {
        godot_warn!("rose: could not decode texture {path}");
        return None;
    }
    if needs_mipmaps && !image.has_mipmaps() {
        if image.is_compressed() {
            image.decompress();
        }
        image.generate_mipmaps();
    }
    Some(image)
}

pub fn load_texture(path: &str) -> Option<Gd<Texture2D>> {
    let key = cache_key(path);
    if let Some(cached) = TEXTURES.with_borrow(|cache| cache.get(&key).cloned()) {
        return cached;
    }

    let texture = load_image(path)
        .and_then(|image| ImageTexture::create_from_image(&image))
        .map(|texture| texture.upcast::<Texture2D>());
    TEXTURES.with_borrow_mut(|cache| cache.insert(key, texture.clone()));
    texture
}

/// Builds one texture array from images of possibly different sizes and formats, so a
/// shader can index them per vertex. Missing images become a flat grey layer.
pub fn load_texture_array(paths: &[String]) -> Gd<Texture2DArray> {
    let mut images: Vec<Gd<Image>> = paths.iter().map(|path| load_image(path)).map(|image| {
        image.unwrap_or_else(|| {
            let mut image = Image::create_empty(4, 4, false, Format::RGBA8).unwrap();
            image.fill(Color::from_rgba(0.5, 0.5, 0.5, 1.0));
            image
        })
    }).collect();

    let size = images.iter().map(|image| image.get_width().max(image.get_height())).max().unwrap_or(4);
    let mut layers: Array<Gd<Image>> = Array::new();
    for image in images.iter_mut() {
        if image.is_compressed() {
            image.decompress();
        }
        image.convert(Format::RGBA8);
        if image.get_width() != size || image.get_height() != size {
            image.resize(size, size);
        }
        image.generate_mipmaps();
        layers.push(&*image);
    }

    let mut array = Texture2DArray::new_gd();
    array.create_from_images(&layers);
    array
}
