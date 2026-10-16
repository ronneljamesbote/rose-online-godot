//! Godot-native copies of the ROSE models and textures.
//!
//! The import command (`--import-rose`, see `RoseImport`) converts the client's ZMS meshes
//! and DDS textures into Godot resources under `res://assets/rose/`, which git ignores.
//! Edited and new art goes in `res://assets/ours/` (committed) under the same path and wins
//! over the generated copy. A ROSE path maps to a resource path by lower-casing it and
//! swapping the extension: `3DDATA/AVATAR/MALE/BODY/BODY1_00.ZMS` becomes
//! `res://assets/rose/3ddata/avatar/male/body/body1_00.mesh`. The loaders in `mesh.rs` and
//! `texture.rs` ask here first and fall back to reading the ROSE file.

use std::{collections::BTreeMap, path::Path, time::Instant};

use godot::{
    classes::{
        resource_saver::SaverFlags, ArrayMesh, ConfigFile, DirAccess, ImageTexture, ProjectSettings, Resource,
        ResourceLoader, ResourceSaver, Texture2D,
    },
    global::Error,
    prelude::*,
};
use rose_file_readers::{VfsIndex, ZmsFile};

pub const OURS: &str = "res://assets/ours/";
pub const ROSE: &str = "res://assets/rose/";
/// Bumped when the converted output changes, so the next import redoes every file.
const IMPORT_VERSION: i64 = 1;
const STAMP: &str = "res://assets/rose/import.cfg";

/// Folders that stay in data.idx for now: effects and the interface.
const SKIPPED: [&str; 2] = ["3DDATA/EFFECT/", "3DDATA/CONTROL/"];

/// What a ROSE file becomes: Godot's binary mesh or a texture resource.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    Mesh,
    Texture,
}

impl Kind {
    fn of(rose_path: &str) -> Option<Kind> {
        let upper = rose_path.to_ascii_uppercase();
        if upper.ends_with(".ZMS") {
            Some(Kind::Mesh)
        } else if upper.ends_with(".DDS") {
            Some(Kind::Texture)
        } else {
            None
        }
    }

    /// Extensions tried in `ours/`, first match wins. The generated copy is the first one.
    fn extensions(self) -> &'static [&'static str] {
        match self {
            Kind::Mesh => &["mesh", "res"],
            Kind::Texture => &["res", "png", "dds"],
        }
    }
}

/// The path of a ROSE file inside an asset folder, without the extension:
/// `3DDATA\AVATAR\X.ZMS` -> `3ddata/avatar/x`.
fn stem(rose_path: &str) -> String {
    let path = rose_path.replace('\\', "/").to_ascii_lowercase();
    let path = path.trim().trim_start_matches("./").trim_start_matches('/');
    match path.rfind('.') {
        Some(dot) if !path[dot..].contains('/') => path[..dot].to_string(),
        _ => path.to_string(),
    }
}

/// The resource standing in for a ROSE file: `ours/` first, then the imported copy.
fn find(rose_path: &str, kind: Kind) -> Option<String> {
    let stem = stem(rose_path);
    let mut loader = ResourceLoader::singleton();
    for extension in kind.extensions() {
        let path = format!("{OURS}{stem}.{extension}");
        if loader.exists(&path) {
            return Some(path);
        }
    }
    let path = format!("{ROSE}{stem}.{}", kind.extensions()[0]);
    loader.exists(&path).then_some(path)
}

fn load<T: GodotClass + Inherits<Resource>>(rose_path: &str, kind: Kind) -> Option<Gd<T>> {
    let path = find(rose_path, kind)?;
    match ResourceLoader::singleton().load(&path).map(|r| r.try_cast::<T>()) {
        Some(Ok(resource)) => Some(resource),
        _ => {
            godot_warn!("rose: {path} is not a {}", T::class_id());
            None
        }
    }
}

pub fn load_mesh(rose_path: &str) -> Option<Gd<ArrayMesh>> {
    load(rose_path, Kind::Mesh)
}

pub fn load_texture(rose_path: &str) -> Option<Gd<Texture2D>> {
    load(rose_path, Kind::Texture)
}

/// The import command: converts every ZMS mesh and DDS texture in data.idx (outside the
/// skipped folders) into `res://assets/rose/`.
#[derive(GodotClass)]
#[class(base=RefCounted, init)]
struct RoseImport {
    base: Base<RefCounted>,
}

#[godot_api]
impl RoseImport {
    /// Needs RoseData.open first. Files already converted by this importer version are
    /// kept unless `force`. Returns {meshes, textures, failed, skipped, bytes, seconds,
    /// folders: {top folder: bytes}}.
    #[func]
    fn run(force: bool) -> VarDictionary {
        let mut result = VarDictionary::new();
        let Some(game_data) = crate::data::get() else {
            godot_error!("rose: open the data first");
            return result;
        };
        let index = match VfsIndex::load(&game_data.data_idx) {
            Ok(index) => index,
            Err(error) => {
                godot_error!("rose: could not list {}: {error:#}", game_data.data_idx.display());
                return result;
            }
        };

        let started = Instant::now();
        let mut stamp = ConfigFile::new_gd();
        let same_version = stamp.load(STAMP) == Error::OK && stamp.get_value("import", "version").to::<i64>() == IMPORT_VERSION;
        let redo = force || !same_version;

        let mut files: Vec<(Kind, String)> = index
            .file_paths()
            .map(|p| p.to_string_lossy().to_string())
            .filter(|p| !SKIPPED.iter().any(|skip| p.starts_with(skip)))
            .filter_map(|p| Kind::of(&p).map(|kind| (kind, p)))
            .collect();
        files.sort();

        let root = ProjectSettings::singleton().globalize_path(ROSE).to_string();
        let (mut meshes, mut textures, mut failed, mut kept) = (0, 0, 0, 0);
        let mut folders: BTreeMap<String, u64> = BTreeMap::new();
        let mut total_bytes = 0u64;
        for (i, (kind, rose_path)) in files.iter().enumerate() {
            let target = format!("{ROSE}{}.{}", stem(rose_path), kind.extensions()[0]);
            let disk = Path::new(&root).join(target.trim_start_matches(ROSE));
            if redo || !disk.is_file() {
                let saved = match kind {
                    Kind::Mesh => crate::data::read_file::<ZmsFile>(rose_path)
                        .and_then(|zms| crate::mesh::build_mesh(&zms, true))
                        .map(|mesh| save(mesh.upcast(), &target)),
                    Kind::Texture => crate::texture::load_rose_image(rose_path)
                        .and_then(|image| ImageTexture::create_from_image(&image))
                        .map(|texture| save(texture.upcast(), &target)),
                };
                match saved {
                    Some(true) => match kind {
                        Kind::Mesh => meshes += 1,
                        Kind::Texture => textures += 1,
                    },
                    _ => {
                        failed += 1;
                        continue;
                    }
                }
            } else {
                kept += 1;
            }
            let bytes = std::fs::metadata(&disk).map_or(0, |m| m.len());
            total_bytes += bytes;
            let folder = stem(rose_path).split('/').take(2).collect::<Vec<_>>().join("/");
            *folders.entry(folder).or_default() += bytes;
            if (i + 1) % 2000 == 0 {
                godot_print!("rose: import {}/{}", i + 1, files.len());
            }
        }

        stamp.set_value("import", "version", &IMPORT_VERSION.to_variant());
        stamp.save(STAMP);

        let mut by_folder = VarDictionary::new();
        for (folder, bytes) in folders {
            by_folder.set(folder.as_str(), bytes as i64);
        }
        result.set("meshes", meshes);
        result.set("textures", textures);
        result.set("failed", failed);
        result.set("kept", kept);
        result.set("bytes", total_bytes as i64);
        result.set("seconds", started.elapsed().as_secs_f64());
        result.set("folders", &by_folder);
        result
    }
}

fn save(resource: Gd<Resource>, path: &str) -> bool {
    let folder = &path[..path.rfind('/').unwrap_or(0)];
    DirAccess::make_dir_recursive_absolute(folder);
    let error = ResourceSaver::singleton().save_ex(&resource).path(path).flags(SaverFlags::COMPRESS).done();
    if error != Error::OK {
        godot_warn!("rose: could not save {path}: {error:?}");
    }
    error == Error::OK
}
