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
        image::CompressMode, resource_saver::SaverFlags, ArrayMesh, Image, ConfigFile, DirAccess, ImageTexture, ProjectSettings, Resource,
        ResourceLoader, ResourceSaver, Texture2D,
    },
    global::Error,
    prelude::*,
};
use rose_file_readers::{VfsIndex, ZmsFile};

pub const OURS: &str = "res://assets/ours/";
pub const ROSE: &str = "res://assets/rose/";
/// Bumped when the converted output changes, so the next import redoes every file.
const IMPORT_VERSION: i64 = 2;
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
pub fn stem(rose_path: &str) -> String {
    let path = rose_path.replace('\\', "/").to_ascii_lowercase();
    let path = path.trim().trim_start_matches("./").trim_start_matches('/');
    match path.rfind('.') {
        Some(dot) if !path[dot..].contains('/') => path[..dot].to_string(),
        _ => path.to_string(),
    }
}

/// The resource at `stem` (a path inside the asset folders, without extension): `ours/`
/// with any of `extensions` first, then the imported copy with the first extension.
pub fn find_stem(stem: &str, extensions: &[&str]) -> Option<String> {
    let mut loader = ResourceLoader::singleton();
    for extension in extensions {
        let path = format!("{OURS}{stem}.{extension}");
        if loader.exists(&path) {
            return Some(path);
        }
    }
    let path = format!("{ROSE}{stem}.{}", extensions[0]);
    loader.exists(&path).then_some(path)
}

/// Loads what `find_stem` finds, as a `T`.
pub fn load_stem<T: GodotClass + Inherits<Resource>>(stem: &str, extensions: &[&str]) -> Option<Gd<T>> {
    let path = find_stem(stem, extensions)?;
    match ResourceLoader::singleton().load(&path).map(|r| r.try_cast::<T>()) {
        Some(Ok(resource)) => Some(resource),
        _ => {
            godot_warn!("rose: {path} is not a {}", T::class_id());
            None
        }
    }
}

fn load<T: GodotClass + Inherits<Resource>>(rose_path: &str, kind: Kind) -> Option<Gd<T>> {
    load_stem(&stem(rose_path), kind.extensions())
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
        let same_version = stamp.load(STAMP) == Error::OK && stamp.get_value_ex("import", "version").default(&Variant::nil()).done().try_to::<i64>().ok() == Some(IMPORT_VERSION);
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
                        .map(compress_texture)
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

        // Player characters: skeletons, part scenes, motions and motion libraries.
        let same_avatars = same_version && stamp.get_value_ex("import", "avatars").default(&Variant::nil()).done().try_to::<i64>().ok() == Some(crate::character::AVATAR_IMPORT_VERSION);
        let avatars = crate::character::import_avatars(force || !same_avatars);
        stamp.set_value("import", "avatars", &crate::character::AVATAR_IMPORT_VERSION.to_variant());

        // Zones with scenes in res://zones/: their model scenes and texture arrays.
        let same_zones = same_version && stamp.get_value_ex("import", "zones").default(&Variant::nil()).done().try_to::<i64>().ok() == Some(crate::zone_scene::ZONE_IMPORT_VERSION);
        let mut zones = crate::zone_scene::ZoneImport::default();
        for (zone_id, _) in crate::zone_scene::zone_folders() {
            crate::zone_scene::import_zone_assets(zone_id, force || !same_zones, &mut zones);
        }
        stamp.set_value("import", "zones", &crate::zone_scene::ZONE_IMPORT_VERSION.to_variant());
        stamp.save(STAMP);

        let mut by_folder = VarDictionary::new();
        for (folder, bytes) in folders {
            by_folder.set(folder.as_str(), bytes as i64);
        }
        result.set("meshes", meshes);
        result.set("textures", textures);
        result.set("failed", failed + avatars.failed + zones.failed);
        result.set("zone_files", zones.saved);
        result.set("zone_kept", zones.kept);
        result.set("zone_bytes", zones.bytes as i64);
        result.set("avatar_files", avatars.saved);
        result.set("avatar_kept", avatars.kept);
        result.set("avatar_bytes", avatars.bytes as i64);
        result.set("kept", kept);
        result.set("bytes", total_bytes as i64);
        result.set("seconds", started.elapsed().as_secs_f64());
        result.set("folders", &by_folder);
        result
    }

    /// Writes the scenes of a zone into `res://zones/<id>_<name>/` from the ROSE map files,
    /// with the model scenes and textures they use. Refuses to replace existing scenes
    /// unless `force`. Needs RoseData.open and the import. Returns {folder, blocks, objects,
    /// markers} or {error}.
    #[func]
    fn import_zone(zone_id: i32, force: bool) -> VarDictionary {
        let mut result = VarDictionary::new();
        match crate::zone_scene::write_zone_scenes(zone_id.clamp(0, u16::MAX as i32) as u16, force) {
            Ok((folder, blocks, objects, markers)) => {
                result.set("folder", folder.as_str());
                result.set("blocks", blocks as i64);
                result.set("objects", objects as i64);
                result.set("markers", markers as i64);
            }
            Err(error) => result.set("error", error.as_str()),
        }
        result
    }

    /// Writes `overrides/zones/<id>-<name>.toml` (in the repository) for every zone that has
    /// scenes in `res://zones/`, or with `check` only compares them. Needs RoseData.open.
    /// Returns {files: [{path, zone, spawns, npcs, events}], changed: [paths that differ]},
    /// empty on error.
    #[func]
    fn export_zones(check: bool) -> VarDictionary {
        let mut result = VarDictionary::new();
        if crate::data::get().is_none() {
            godot_error!("rose: open the data first");
            return result;
        }
        let project = ProjectSettings::singleton().globalize_path("res://").to_string();
        let dir = Path::new(&project).join("..").join(rose_game_overrides::ZONES_DIR);
        let (mut files, mut changed) = (VarArray::new(), VarArray::new());
        for (zone_id, folder) in crate::zone_scene::zone_folders() {
            let layout = match crate::zone_scene::zone_layout(zone_id) {
                Ok(layout) => layout,
                Err(error) => {
                    godot_error!("rose: {error}");
                    return VarDictionary::new();
                }
            };
            let path = dir.join(crate::zone_scene::layout_file_name(&folder));
            let text = crate::zone_scene::layout_text(&layout);
            if std::fs::read_to_string(&path).ok().as_deref() != Some(text.as_str()) {
                changed.push(&path.to_string_lossy().to_string().to_variant());
                if !check {
                    let written = std::fs::create_dir_all(&dir).and_then(|_| std::fs::write(&path, &text));
                    if let Err(error) = written {
                        godot_error!("rose: could not write {}: {error}", path.display());
                        return VarDictionary::new();
                    }
                }
            }
            let mut file = VarDictionary::new();
            file.set("path", path.canonicalize().unwrap_or(path).to_string_lossy().as_ref());
            file.set("zone", zone_id as i64);
            file.set("spawns", layout.spawn.len() as i64);
            file.set("npcs", layout.npc.len() as i64);
            file.set("events", layout.event.len() as i64);
            files.push(&file.to_variant());
        }
        result.set("files", &files);
        result.set("changed", &changed);
        result
    }

    /// Writes `overrides/motions.toml` (in the repository, next to the project folder) from
    /// the player, monster and NPC animations, or with `check` only compares it. Needs RoseData.open and the
    /// import. Returns {path, motions, hits, changed: [motion files that differ]}, empty on
    /// error.
    #[func]
    fn export_motions(check: bool) -> VarDictionary {
        let mut result = VarDictionary::new();
        let Some(timings) = crate::character::motion_timings() else {
            godot_error!("rose: open the data first");
            return result;
        };
        if timings.motion.is_empty() {
            godot_error!("rose: no imported animations in {ROSE} or {OURS}; run --import-rose first");
            return result;
        }
        let project = ProjectSettings::singleton().globalize_path("res://").to_string();
        let path = Path::new(&project).join("..").join(rose_game_overrides::MOTIONS_FILE);
        let current = match rose_game_overrides::MotionTimings::load(&path) {
            Ok(current) => current,
            Err(error) => {
                godot_error!("rose: {error:#}");
                return result;
            }
        };
        let mut changed: Vec<String> = timings
            .motion
            .iter()
            .filter(|m| !current.motion.contains(m))
            .map(|m| m.file.clone())
            .chain(current.motion.iter().filter(|m| !timings.motion.iter().any(|t| t.file == m.file)).map(|m| m.file.clone()))
            .collect();
        changed.sort();
        changed.dedup();
        let text = timings.to_toml();
        if !check && std::fs::read_to_string(&path).ok().as_deref() != Some(text.as_str()) {
            if let Err(error) = std::fs::write(&path, text) {
                godot_error!("rose: could not write {}: {error}", path.display());
                return result;
            }
        }
        let path = path.canonicalize().unwrap_or(path);
        result.set("path", path.to_string_lossy().as_ref());
        result.set("motions", timings.motion.len() as i64);
        result.set("hits", timings.motion.iter().filter(|m| !m.hit_frames.is_empty()).count() as i64);
        result.set("changed", &changed.iter().map(|f| f.to_variant()).collect::<VarArray>());
        result
    }
}

/// Textures the import decompressed (ROSE DDS files with a partial mip chain) go back to
/// S3TC (DXT1, or DXT5 with alpha), so the converted copies stay as small as the originals
/// and fit in the Windows .pck.
fn compress_texture(mut image: Gd<Image>) -> Gd<Image> {
    if !image.is_compressed() && image.get_width() % 4 == 0 && image.get_height() % 4 == 0 {
        image.compress(CompressMode::S3TC);
    }
    image
}

pub fn save(resource: Gd<Resource>, path: &str) -> bool {
    let folder = &path[..path.rfind('/').unwrap_or(0)];
    DirAccess::make_dir_recursive_absolute(folder);
    let error = ResourceSaver::singleton().save_ex(&resource).path(path).flags(SaverFlags::COMPRESS).done();
    if error != Error::OK {
        godot_warn!("rose: could not save {path}: {error:?}");
    }
    error == Error::OK
}
