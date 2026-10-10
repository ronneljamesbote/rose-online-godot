//! Pictures: the icon sheets and zone minimaps are DDS textures in the game data. They are
//! copied out here with a list of where each goes, and convert-images.py turns them into
//! PNG (icons) and JPEG (maps) under wiki/assets.

use std::{collections::BTreeMap, fs, path::Path};

use rose_file_readers::{TsiFile, VfsFile, VirtualFilesystem};
use serde_json::{json, Value};

use crate::Ctx;

/// Copy a DDS file out of the game data; returns its width and height.
pub fn export_dds(vfs: &VirtualFilesystem, path: &str, out: &Path) -> Option<(u32, u32)> {
    let file = vfs.open_file(path).ok()?;
    let bytes: &[u8] = match &file {
        VfsFile::Buffer(b) => b,
        VfsFile::View(v) => v,
    };
    if bytes.len() < 20 || &bytes[0..4] != b"DDS " {
        return None;
    }
    let height = u32::from_le_bytes(bytes[12..16].try_into().ok()?);
    let width = u32::from_le_bytes(bytes[16..20].try_into().ok()?);
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent).ok()?;
    }
    fs::write(out, bytes).ok()?;
    Some((width, height))
}

/// Add a conversion to the list convert-images.py reads.
pub fn queue(dds_dir: &Path, dds: &str, target: &str, format: &str) {
    let list = dds_dir.join("convert.json");
    let mut entries: Vec<Value> = fs::read(&list).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
    entries.push(json!({ "dds": dds, "target": target, "format": format }));
    let _ = fs::write(list, serde_json::to_vec_pretty(&entries).unwrap_or_default());
}

pub fn write_icons(cx: &Ctx, wiki: &Path, dds_dir: &Path) -> anyhow::Result<()> {
    let mut all: BTreeMap<&str, BTreeMap<String, Value>> = BTreeMap::new();
    for (group, tsi_path) in [
        ("item", "3DDATA/CONTROL/RES/ITEM1.TSI"),
        ("skill", "3DDATA/CONTROL/RES/SKILLICON.TSI"),
        ("state", "3DDATA/CONTROL/RES/STATEICON.TSI"),
    ] {
        let Ok(tsi) = cx.vfs.read_file::<TsiFile, _>(tsi_path) else { continue };
        let mut exported = BTreeMap::new();
        for (i, texture) in tsi.textures.iter().enumerate() {
            let sheet = format!("{group}-{i}.png");
            let dds = format!("icons/{group}-{i}.dds");
            if export_dds(cx.vfs, &format!("3DDATA/CONTROL/RES/{}", texture.filename), &dds_dir.join(&dds)).is_some() {
                queue(dds_dir, &dds, &format!("icons/{sheet}"), "png");
                exported.insert(i, sheet);
            }
        }
        let spots = all.entry(group).or_default();
        for (n, sprite) in tsi.sprites.iter().enumerate() {
            let Some(sheet) = exported.get(&(sprite.texture_id as usize)) else { continue };
            spots.insert(
                n.to_string(),
                json!([sheet, sprite.left, sprite.top, sprite.right - sprite.left, sprite.bottom - sprite.top]),
            );
        }
    }
    let dir = wiki.join("assets").join("icons");
    fs::create_dir_all(&dir)?;
    fs::write(dir.join("icons.json"), serde_json::to_vec(&all)?)?;
    Ok(())
}
