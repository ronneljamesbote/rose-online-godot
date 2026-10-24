//! Godot-native copies of the ROSE models, textures and sounds.
//!
//! The import command (`--import-rose`, see `RoseImport`) converts the client's ZMS meshes,
//! DDS textures and WAV and OGG sounds into Godot resources under `res://assets/rose/`,
//! which git ignores.
//! Edited and new art goes in `res://assets/ours/` (committed) under the same path and wins
//! over the generated copy. A ROSE path maps to a resource path by lower-casing it and
//! swapping the extension: `3DDATA/AVATAR/MALE/BODY/BODY1_00.ZMS` becomes
//! `res://assets/rose/3ddata/avatar/male/body/body1_00.mesh`. The loaders in `mesh.rs` and
//! `texture.rs` ask here first and fall back to reading the ROSE file.
//!
//! A texture becomes a `.rtex`: the client's DDS file as it is (cut to the levels it really
//! stores), compressed, which `scripts/rose_texture_loader.gd` opens as an `ImageTexture`
//! and gives its mipmaps the way `texture::load_rose_image` does, so it looks the same.
//!
//! A sound becomes a `.res`: Godot's own `AudioStreamWAV` or `AudioStreamOggVorbis` of the
//! file (`Sound/attack/Sword_001.wav` -> `res://assets/rose/sound/attack/sword_001.res`),
//! which the sound resources (`RoseSound`) and the zone music link.
//!
//! An effect becomes a scene: each EFT file a `.scn` (a `RoseEffect` with its parts, see
//! `effect.rs`) and each PTL file a `RoseParticleSystem` in a `.particles.res`, linking the
//! effects' imported meshes and textures, which the effect resources (`RoseEffectFile`) link.

use std::{collections::BTreeMap, path::Path, time::Instant};

use godot::{
    classes::{
        file_access::{CompressionMode, ModeFlags}, image::Format, resource_saver::SaverFlags, ArrayMesh, AudioStream, AudioStreamOggVorbis, AudioStreamWav, ConfigFile,
        DirAccess, FileAccess, Image, ProjectSettings, Resource, ResourceLoader, ResourceSaver, Texture2D,
    },
    global::Error,
    prelude::*,
};
use rose_file_readers::{VfsIndex, ZmsFile};

pub const OURS: &str = "res://assets/ours/";
pub const ROSE: &str = "res://assets/rose/";
/// Bumped when the converted output changes, so the next import redoes every file.
const IMPORT_VERSION: i64 = 4;
const STAMP: &str = "res://assets/rose/import.cfg";

/// Folders that stay in data.idx for now: the interface.
const SKIPPED: [&str; 1] = ["3DDATA/CONTROL/"];

/// Where the client keeps its sounds: loose files next to data.idx, outside the VFS.
const SOUND_FOLDER: &str = "Sound";

/// The extensions a sound may have in the asset folders: the imported `.res` first, then
/// WAV and OGG files of our own (Godot imports those in the editor).
pub const AUDIO_EXTENSIONS: &[&str] = &["res", "wav", "ogg"];

/// What a ROSE file becomes: Godot's binary mesh, a texture, an audio resource, a particle
/// system or an effect scene. Particle systems and effects come last, as they link the
/// others.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    Mesh,
    Texture,
    Sound,
    Particles,
    Effect,
}

impl Kind {
    fn of(rose_path: &str) -> Option<Kind> {
        let upper = rose_path.to_ascii_uppercase();
        if upper.ends_with(".ZMS") {
            Some(Kind::Mesh)
        } else if upper.ends_with(".DDS") {
            Some(Kind::Texture)
        } else if upper.ends_with(".WAV") || upper.ends_with(".OGG") {
            Some(Kind::Sound)
        } else if upper.ends_with(".PTL") {
            Some(Kind::Particles)
        } else if upper.ends_with(".EFT") {
            Some(Kind::Effect)
        } else {
            None
        }
    }

    /// Extensions tried in `ours/`, first match wins. The generated copy is the first one.
    fn extensions(self) -> &'static [&'static str] {
        match self {
            Kind::Mesh => &["mesh", "res"],
            Kind::Texture => &["rtex", "res", "png", "dds"],
            Kind::Sound => AUDIO_EXTENSIONS,
            Kind::Particles => crate::effect::PARTICLE_EXTENSIONS,
            Kind::Effect => crate::effect::SCENE_EXTENSIONS,
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

/// An audio file by the path the data gives: a client file (`Sound\BGM\Prologue.ogg`) or a
/// Godot path. A path in the asset folders (`res://assets/rose/sound/bgm/prologue.res`)
/// takes the file at the same place in `ours/` first, then the imported one; any other
/// `res://` path is loaded as it is. None when there is no such Godot file (the caller
/// reads the client's file then).
pub fn load_audio(path: &str) -> Option<Gd<AudioStream>> {
    let inner = path.strip_prefix(ROSE).or_else(|| path.strip_prefix(OURS));
    match inner {
        Some(inner) => load_stem(&stem(inner), AUDIO_EXTENSIONS),
        None if path.starts_with("res://") => {
            let mut loader = ResourceLoader::singleton();
            loader.exists(path).then(|| loader.load(path)).flatten().and_then(|r| r.try_cast::<AudioStream>().ok())
        }
        None => load(path, Kind::Sound),
    }
}

/// Every sound file in the client's sound folder, as a path from the client folder
/// (`Sound/attack/Sword_001.wav`).
fn client_sound_files(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let Some(folder) = std::fs::read_dir(root).ok().into_iter().flatten().flatten().find(|e| e.file_name().to_string_lossy().eq_ignore_ascii_case(SOUND_FOLDER)) else {
        return out;
    };
    let mut todo = vec![folder.path()];
    while let Some(dir) = todo.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                todo.push(path);
            } else if let Ok(relative) = path.strip_prefix(root) {
                let relative = relative.to_string_lossy().replace('\\', "/");
                if Kind::of(&relative) == Some(Kind::Sound) {
                    out.push(relative);
                }
            }
        }
    }
    out
}

/// Writes the Godot copy of a client sound file (WAV or OGG) as a `.res`.
fn save_sound(rose_path: &str, target: &str) -> bool {
    let Some(bytes) = crate::data::read_bytes(rose_path) else { return false };
    let buffer = PackedByteArray::from(bytes.as_slice());
    let stream = if rose_path.to_ascii_uppercase().ends_with(".OGG") {
        AudioStreamOggVorbis::load_from_buffer(&buffer).map(|s| s.upcast::<Resource>())
    } else {
        AudioStreamWav::load_from_buffer(&buffer).map(|s| s.upcast::<Resource>())
    };
    match stream {
        Some(stream) => save(stream, target),
        None => {
            godot_warn!("rose: could not decode sound {rose_path}");
            false
        }
    }
}

/// The pack with the converted assets that the Windows build writes next to the game.
pub const ASSETS_PACK: &str = "assets.pck";

/// Opens `ASSETS_PACK` from beside the executable, in an exported game only.
pub fn load_assets_pack() {
    let os = godot::classes::Os::singleton();
    if !os.has_feature("template") || godot::classes::Engine::singleton().is_editor_hint() {
        return;
    }
    let path = os.get_executable_path().get_base_dir().path_join(ASSETS_PACK);
    if !FileAccess::file_exists(&path) {
        godot_warn!("rose: {path} is missing; models, textures and zone scenes load from the client data");
        return;
    }
    if !ProjectSettings::singleton().load_resource_pack_ex(&path).replace_files(false).done() {
        godot_error!("rose: could not open {path}");
    }
}

/// The file that keeps the editor out of `res://assets/rose/`. In its tree view the editor's
/// FileSystem dock makes a thumbnail of every file in the project each time it opens, even
/// in folders nobody opens, which means loading the 16,500 converted files one by one (six
/// minutes and more, until the editor hangs). Everything links them by path, so scenes and
/// resources load them all the same; the build takes the file away while it packs them.
const GDIGNORE: &str = ".gdignore";

fn hide_from_editor(root: &str) {
    let _ = std::fs::create_dir_all(root);
    if let Err(error) = std::fs::write(Path::new(root).join(GDIGNORE), "") {
        godot_warn!("rose: could not write {root}{GDIGNORE}: {error}");
    }
}

/// The import command: converts every ZMS mesh, DDS texture, PTL particle system and EFT
/// effect in data.idx (outside the skipped folders) and every sound in the client's sound
/// folder into `res://assets/rose/`.
#[derive(GodotClass)]
#[class(base=RefCounted, init)]
struct RoseImport {
    base: Base<RefCounted>,
}

#[godot_api]
impl RoseImport {
    /// Needs RoseData.open first. Files already converted by this importer version are
    /// kept unless `force`. Returns {meshes, textures, sounds, effects (particle systems and
    /// effect scenes), empty_effects (EFT files with nothing to show, left out), failed,
    /// skipped, bytes, seconds, folders: {top folder: bytes}}.
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
        let same_effects = same_version && stamp.get_value_ex("import", "effects").default(&0.to_variant()).done().try_to::<i64>().ok() == Some(crate::effect::EFFECT_IMPORT_VERSION);

        let mut files: Vec<(Kind, String)> = index
            .file_paths()
            .map(|p| p.to_string_lossy().to_string())
            .filter(|p| !SKIPPED.iter().any(|skip| p.starts_with(skip)))
            .filter_map(|p| Kind::of(&p).map(|kind| (kind, p)))
            .collect();
        files.extend(client_sound_files(&game_data.root).into_iter().map(|p| (Kind::Sound, p)));
        files.sort();
        files.dedup_by(|a, b| stem(&a.1) == stem(&b.1));

        let root = ProjectSettings::singleton().globalize_path(ROSE).to_string();
        if !same_version {
            // Files an older import made under other names (the textures were .res before)
            // would be left over and shipped, so start from an empty folder.
            let _ = std::fs::remove_dir_all(&root);
        }
        hide_from_editor(&root);
        let (mut meshes, mut textures, mut sounds, mut effects, mut failed, mut kept) = (0, 0, 0, 0, 0, 0);
        let mut empty_effects = 0;
        let mut folders: BTreeMap<String, u64> = BTreeMap::new();
        let mut total_bytes = 0u64;
        for (i, (kind, rose_path)) in files.iter().enumerate() {
            let target = format!("{ROSE}{}.{}", stem(rose_path), kind.extensions()[0]);
            let disk = Path::new(&root).join(target.trim_start_matches(ROSE));
            let redo = redo || (matches!(kind, Kind::Particles | Kind::Effect) && !same_effects);
            if redo || !disk.is_file() {
                let saved = match kind {
                    Kind::Mesh => crate::data::read_file::<ZmsFile>(rose_path)
                        .and_then(|zms| crate::mesh::build_mesh(&zms, true))
                        .map(|mesh| save(mesh.upcast(), &target)),
                    Kind::Texture => Some(save_texture(rose_path, &target)),
                    Kind::Sound => Some(save_sound(rose_path, &target)),
                    Kind::Particles => Some(crate::effect::import_particle_system(rose_path, &target)),
                    Kind::Effect => match crate::effect::import_effect(rose_path, &target) {
                        Some(saved) => Some(saved),
                        None => {
                            // Unreadable or empty in the client data: the game shows nothing for it either.
                            empty_effects += 1;
                            continue;
                        }
                    },
                };
                match saved {
                    Some(true) => match kind {
                        Kind::Mesh => meshes += 1,
                        Kind::Texture => textures += 1,
                        Kind::Sound => sounds += 1,
                        Kind::Particles | Kind::Effect => effects += 1,
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
        stamp.set_value("import", "effects", &crate::effect::EFFECT_IMPORT_VERSION.to_variant());

        // Player characters: skeletons, part scenes, motions and motion libraries.
        let same_avatars = same_version && stamp.get_value_ex("import", "avatars").default(&Variant::nil()).done().try_to::<i64>().ok() == Some(crate::character::AVATAR_IMPORT_VERSION);
        let avatars = crate::character::import_avatars(force || !same_avatars);
        stamp.set_value("import", "avatars", &crate::character::AVATAR_IMPORT_VERSION.to_variant());

        // Icons and ground item models, which the data resources link.
        let same_art = same_version && stamp.get_value_ex("import", "art").default(&Variant::nil()).done().try_to::<i64>().ok() == Some(crate::items::ART_IMPORT_VERSION);
        let art = crate::items::import_art(force || !same_art);
        stamp.set_value("import", "art", &crate::items::ART_IMPORT_VERSION.to_variant());

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
        result.set("sounds", sounds);
        result.set("effects", effects);
        result.set("empty_effects", empty_effects);
        result.set("failed", failed + avatars.failed + zones.failed + art.failed);
        result.set("art_files", art.saved);
        result.set("art_kept", art.kept);
        result.set("art_bytes", art.bytes as i64);
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

    /// Moves the blocks of zone `zone_id` (0: every zone in `res://zones/`) from their own
    /// `block_X_Y.tscn` files into `zone.tscn` and deletes the block files. Needs the import
    /// (the blocks' models must load). Returns {zones: [{folder, blocks}]} or {error}.
    #[func]
    fn merge_zone_blocks(zone_id: i32) -> VarDictionary {
        let mut result = VarDictionary::new();
        let ids: Vec<u16> = if zone_id <= 0 {
            crate::zone_scene::zone_folders().into_iter().map(|(id, _)| id).collect()
        } else {
            vec![zone_id.clamp(0, u16::MAX as i32) as u16]
        };
        let mut zones = VarArray::new();
        for id in ids {
            match crate::zone_scene::merge_zone_blocks(id) {
                Ok((folder, blocks)) => {
                    let mut zone = VarDictionary::new();
                    zone.set("folder", folder.as_str());
                    zone.set("blocks", blocks as i64);
                    zones.push(&zone.to_variant());
                }
                Err(error) => {
                    result.set("error", error.as_str());
                    return result;
                }
            }
        }
        result.set("zones", &zones);
        result
    }
}

/// The first bytes of a `.rtex`, then the kind of payload, whether the loader makes the
/// mipmaps, and a spare byte. The rest is the payload: a DDS or a lossless WebP file.
const RTEX_MAGIC: &[u8; 4] = b"RTEX";
const RTEX_DDS: u8 = 0;
const RTEX_WEBP: u8 = 1;

/// Writes the converted copy of a ROSE texture: its DDS file as it is, in a compressed file.
/// Most ROSE files declare mipmaps they don't store; those are cut to level 0 and the
/// loader makes the mipmaps, as the game does with the client files (decompressed to RGBA,
/// never re-compressed: re-compressing to S3TC smeared the colour next to UV seams and
/// made the sky blocky). An uncompressed file is kept as lossless WebP instead when that is
/// smaller and decodes to the same pixels.
pub(crate) fn save_texture(rose_path: &str, target: &str) -> bool {
    let Some(bytes) = crate::data::read_bytes(rose_path) else { return false };
    let (bytes, needs_mipmaps) = crate::texture::fix_partial_mip_chain(bytes);
    let dds = PackedByteArray::from(bytes.as_slice());
    let mut image = Image::new_gd();
    let mut payload = (RTEX_DDS, dds);
    if image.load_dds_from_buffer(&payload.1) != Error::OK {
        // A paletted DDS, which Godot can't read: kept as the decoded image instead.
        let Some(decoded) = crate::texture::decode_paletted_dds(&bytes) else {
            godot_warn!("rose: could not decode texture {rose_path}");
            return false;
        };
        payload = (RTEX_WEBP, decoded.save_webp_to_buffer_ex().lossy(false).done());
        image = decoded;
    }
    if payload.0 == RTEX_DDS && needs_mipmaps && !image.is_compressed() && !image.has_mipmaps() && matches!(image.get_format(), Format::RGB8 | Format::RGBA8) {
        let webp = image.save_webp_to_buffer_ex().lossy(false).done();
        let mut decoded = Image::new_gd();
        let same = decoded.load_webp_from_buffer(&webp) == Error::OK && decoded.get_format() == image.get_format() && decoded.get_data() == image.get_data();
        if same && webp.len() < payload.1.compress(CompressionMode::ZSTD).map_or(usize::MAX, |c| c.len()) {
            payload = (RTEX_WEBP, webp);
        }
    }
    let folder = &target[..target.rfind('/').unwrap_or(0)];
    DirAccess::make_dir_recursive_absolute(folder);
    let Some(mut file) = FileAccess::open_compressed_ex(target, ModeFlags::WRITE).compression_mode(CompressionMode::ZSTD).done() else {
        godot_warn!("rose: could not save {target}");
        return false;
    };
    let mut header = RTEX_MAGIC.to_vec();
    header.extend([payload.0, needs_mipmaps as u8, 0]);
    file.store_buffer(&PackedByteArray::from(header.as_slice()));
    file.store_buffer(&payload.1);
    file.close();
    true
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
