//! Windows cursor files (.CUR), the mouse cursors in the client's RES folder, and the
//! cursor pictures the interface settings link.

use godot::{
    classes::{image::Format, Image, ResourceLoader, Texture2D},
    prelude::*,
};

/// A cursor picture by its path: a Godot texture (the imported `res://assets/rose/res/attack.res`
/// or a picture of our own), or a client cursor file (`RES/ATTACK.CUR`). As RGBA, None when
/// it isn't there.
pub fn load(path: &str) -> Option<Gd<Image>> {
    let path = path.trim();
    if path.is_empty() {
        return None;
    }
    let mut image = if path.starts_with("res://") {
        let mut loader = ResourceLoader::singleton();
        if !loader.exists(path) {
            godot_warn!("rose: there is no cursor {path}");
            return None;
        }
        loader.load(path)?.try_cast::<Texture2D>().ok()?.get_image()?
    } else {
        let (width, height, rgba) = decode(&crate::data::read_bytes(path)?)?;
        Image::create_from_data(width, height, false, Format::RGBA8, &PackedByteArray::from(rgba.as_slice()))?
    };
    if image.is_compressed() {
        image.decompress();
    }
    image.convert(Format::RGBA8);
    Some(image)
}

/// The first image of a .CUR file as (width, height, RGBA8 pixels, top row first).
/// Handles the palette (1, 4 and 8 bit) and true-colour (24 and 32 bit) bitmaps the
/// client uses; the AND mask makes pixels see-through.
pub fn decode(bytes: &[u8]) -> Option<(i32, i32, Vec<u8>)> {
    let u16_at = |at: usize| Some(u16::from_le_bytes(bytes.get(at..at + 2)?.try_into().ok()?));
    let u32_at = |at: usize| Some(u32::from_le_bytes(bytes.get(at..at + 4)?.try_into().ok()?));
    // ICONDIR: reserved 0, type 2 (cursor), count.
    if u16_at(0)? != 0 || u16_at(2)? != 2 || u16_at(4)? == 0 {
        return None;
    }
    let info = u32_at(18)? as usize; // offset of the first image's BITMAPINFOHEADER
    let width = u32_at(info + 4)? as i32;
    let height = u32_at(info + 8)? as i32 / 2; // XOR image + AND mask
    let bpp = u16_at(info + 14)? as usize;
    if width <= 0 || height <= 0 || width > 256 || height > 256 || !matches!(bpp, 1 | 4 | 8 | 24 | 32) {
        return None;
    }
    let (w, h) = (width as usize, height as usize);
    let colours = match u32_at(info + 32)? as usize {
        0 if bpp <= 8 => 1 << bpp,
        n => n,
    };
    let palette = info + u32_at(info)? as usize;
    let pixels = palette + if bpp <= 8 { colours * 4 } else { 0 };
    let row = (w * bpp).div_ceil(32) * 4;
    let mask = pixels + row * h;
    let mask_row = w.div_ceil(32) * 4;
    bytes.get(mask..mask + mask_row * h)?;

    let mut out = vec![0u8; w * h * 4];
    for y in 0..h {
        let src = pixels + (h - 1 - y) * row; // bitmaps are stored bottom row first
        let src_mask = mask + (h - 1 - y) * mask_row;
        for x in 0..w {
            let (r, g, b, a) = match bpp {
                32 => {
                    let p = src + x * 4;
                    (bytes[p + 2], bytes[p + 1], bytes[p], bytes[p + 3])
                }
                24 => {
                    let p = src + x * 3;
                    (bytes[p + 2], bytes[p + 1], bytes[p], 255)
                }
                _ => {
                    let bit = x * bpp;
                    let index = (bytes[src + bit / 8] >> (8 - bpp - bit % 8)) & ((1u16 << bpp) - 1) as u8;
                    let p = palette + index as usize * 4;
                    (*bytes.get(p + 2)?, *bytes.get(p + 1)?, *bytes.get(p)?, 255)
                }
            };
            let hidden = (bytes[src_mask + x / 8] >> (7 - x % 8)) & 1 == 1;
            let at = (y * w + x) * 4;
            // 32-bit cursors carry their own alpha; the others only have the mask.
            let a = if hidden && (bpp != 32 || a == 0) { 0 } else if bpp == 32 { a } else { 255 };
            out[at..at + 4].copy_from_slice(&[r, g, b, a]);
        }
    }
    Some((width, height, out))
}

#[cfg(test)]
mod tests {
    use super::decode;

    /// A 2x2 8-bit cursor: red, green on top, blue and a masked pixel below.
    fn tiny() -> Vec<u8> {
        let mut f = vec![0, 0, 2, 0, 1, 0];
        f.extend([2, 2, 0, 0, 0, 0, 0, 0]); // size, colours, reserved, hotspot 0,0
        let body_len = 40 + 256 * 4 + 4 * 2 + 4 * 2;
        f.extend((body_len as u32).to_le_bytes());
        f.extend(22u32.to_le_bytes());
        // BITMAPINFOHEADER
        f.extend(40u32.to_le_bytes());
        f.extend(2u32.to_le_bytes());
        f.extend(4u32.to_le_bytes()); // double height
        f.extend(1u16.to_le_bytes());
        f.extend(8u16.to_le_bytes());
        f.extend([0u8; 24]);
        let mut palette = vec![0u8; 256 * 4];
        palette[4..8].copy_from_slice(&[0, 0, 255, 0]); // 1 red (BGR)
        palette[8..12].copy_from_slice(&[0, 255, 0, 0]); // 2 green
        palette[12..16].copy_from_slice(&[255, 0, 0, 0]); // 3 blue
        f.extend(palette);
        f.extend([3, 0, 0, 0]); // bottom row: blue, (masked)
        f.extend([1, 2, 0, 0]); // top row: red, green
        f.extend([0b0100_0000, 0, 0, 0]); // bottom mask: second pixel hidden
        f.extend([0, 0, 0, 0]);
        f
    }

    #[test]
    fn decodes_palette_cursor() {
        let (w, h, rgba) = decode(&tiny()).unwrap();
        assert_eq!((w, h), (2, 2));
        assert_eq!(&rgba[0..4], &[255, 0, 0, 255]);
        assert_eq!(&rgba[4..8], &[0, 255, 0, 255]);
        assert_eq!(&rgba[8..12], &[0, 0, 255, 255]);
        assert_eq!(rgba[15], 0);
    }

    #[test]
    fn rejects_other_files() {
        assert!(decode(b"DDS ").is_none());
        let mut icon = tiny();
        icon[2] = 1; // an .ICO, not a cursor
        assert!(decode(&icon).is_none());
    }
}
