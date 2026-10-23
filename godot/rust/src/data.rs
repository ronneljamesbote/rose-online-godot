//! Process-wide access to the ROSE data files and the parsed data tables.

use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use rose_data::{
    AnimationEventFlags, CharacterMotionDatabase, CharacterMotionDatabaseOptions, DataDecoder, EffectDatabase, ItemDatabase,
    JobClassDatabase, NpcDatabase, NpcDatabaseOptions, QuestDatabase, SkillDatabase, SkyboxDatabase, SoundDatabase,
    StatusEffectDatabase, StringDatabase, ZoneList,
};
use rose_file_readers::{ChrFile, HostFilesystemDevice, LtbFile, RoseFile, VfsFile, VfsIndex, VirtualFilesystem};

pub struct GameData {
    pub vfs: VirtualFilesystem,
    pub zone_list: ZoneList,
    pub skybox: Arc<SkyboxDatabase>,
    pub items: ItemDatabase,
    pub job_classes: JobClassDatabase,
    pub motions: CharacterMotionDatabase,
    pub npcs: NpcDatabase,
    pub skills: SkillDatabase,
    pub status_effects: StatusEffectDatabase,
    /// NPC skeletons, motions and part lists (LIST_NPC.CHR).
    pub npc_chr: ChrFile,
    pub quests: QuestDatabase,
    pub decoder: Box<dyn DataDecoder + Send + Sync>,
    /// NPC conversation text (ULNGTB_CON.LTB).
    pub ltb_event: LtbFile,
    /// Crafting recipes (LIST_PRODUCT.STB), by an item's craft_material.
    pub craft_recipes: Vec<Option<rose_game_data::CraftRecipe>>,
    pub sounds: Arc<SoundDatabase>,
    pub effects: Arc<EffectDatabase>,
    /// What each ZMO frame event id does (footstep, hit, fire, ...).
    pub event_flags: Vec<AnimationEventFlags>,
    /// The install folder, for loose files outside the VFS (the Sound folder).
    pub root: PathBuf,
    /// The data.idx this was opened from (the importer lists its files).
    pub data_idx: PathBuf,
}

static GAME_DATA: OnceLock<GameData> = OnceLock::new();

/// Our changes to the game data (overrides/game-data.toml), the same file the server's
/// upload applies: ROSE_OVERRIDES, else the overrides folder next to the game, else the
/// repository's (when running from the Godot editor).
fn apply_overrides(vfs: &mut VirtualFilesystem) -> Result<(), anyhow::Error> {
    let candidates = std::env::var_os("ROSE_OVERRIDES").map(PathBuf::from).into_iter().chain(
        [
            std::env::current_exe().ok().and_then(|exe| exe.parent().map(|d| d.join(rose_game_overrides::OVERRIDES_FILE))),
            std::env::current_dir().ok().map(|d| d.join("..").join(rose_game_overrides::OVERRIDES_FILE)),
        ]
        .into_iter()
        .flatten(),
    );
    for path in candidates {
        if path.is_file() {
            let overrides = rose_game_overrides::Overrides::load(&path)?;
            let cells = overrides.apply(vfs)?;
            godot::global::godot_print!("rose: {} game data changes from {}", cells, path.display());
            return Ok(());
        }
    }
    Ok(())
}

/// The game data exported from the Godot resources (`overrides/data/`, see
/// `data_resources.rs`), the same files the server's upload sends: ROSE_DATA, else the
/// overrides folder next to the game, else the repository's (when running from the Godot
/// editor).
fn find_export() -> Option<PathBuf> {
    let candidates = std::env::var_os("ROSE_DATA").map(PathBuf::from).into_iter().chain(
        [
            std::env::current_exe().ok().and_then(|exe| exe.parent().map(|d| d.join(rose_data_export::DATA_DIR))),
            std::env::current_dir().ok().map(|d| d.join("..").join(rose_data_export::DATA_DIR)),
        ]
        .into_iter()
        .flatten(),
    );
    candidates.into_iter().find(|dir| dir.join(rose_data_export::ITEMS_DIR).is_dir())
}

/// The items from the export, or without one (an old client folder) from the iROSE tables.
fn load_items(vfs: &VirtualFilesystem, strings: Arc<StringDatabase>) -> Result<ItemDatabase, anyhow::Error> {
    if let Some(dir) = find_export() {
        let export = rose_data_export::ItemsExport::from_files(|name| {
            let path = dir.join(rose_data_export::items_file(name));
            match std::fs::read_to_string(&path) {
                Ok(text) => Ok(Some(text)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(anyhow::Error::new(e).context(path.display().to_string())),
            }
        })?;
        if let Some(export) = export {
            godot::global::godot_print!("rose: {} items from {}", export.count(), dir.display());
            return export.to_database(strings);
        }
    }
    godot::global::godot_warn!("rose: no item export (overrides/data/items/), the items come from the iROSE tables");
    rose_data_irose::get_item_database(vfs, strings)
}

/// The monsters, NPCs and shop tabs from the export, or without one from the iROSE tables.
/// Their models and motions still come from LIST_NPC.CHR.
fn load_npcs(vfs: &VirtualFilesystem, strings: Arc<StringDatabase>) -> Result<NpcDatabase, anyhow::Error> {
    let options = NpcDatabaseOptions { load_frame_data: false };
    if let Some(dir) = find_export() {
        let toml = rose_data_export::Format::Toml;
        let export = rose_data_export::NpcsExport::from_files_as(toml, |name| {
            let path = dir.join(rose_data_export::data_file_as(name, toml));
            match std::fs::read_to_string(&path) {
                Ok(text) => Ok(Some(text)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(anyhow::Error::new(e).context(path.display().to_string())),
            }
        })?;
        if let Some(export) = export {
            godot::global::godot_print!(
                "rose: {} monsters and {} NPCs from {}",
                export.monsters.len(),
                export.npcs.len(),
                dir.display()
            );
            let models = rose_data_export::NpcModels {
                motions: rose_data_irose::get_npc_motions(vfs, &options)?,
                conversations: rose_data_irose::get_npc_conversations(vfs)?,
                action_map: rose_data_irose::get_npc_action_map(),
            };
            return export.to_database(strings, models);
        }
    }
    godot::global::godot_warn!("rose: no monster export (overrides/data/monsters.toml), the monsters and NPCs come from the iROSE tables");
    rose_data_irose::get_npc_database(vfs, strings, &options)
}

/// The skills, status effects and job classes from the export, or without one from the
/// iROSE tables.
fn load_skills(
    vfs: &VirtualFilesystem,
    strings: Arc<StringDatabase>,
) -> Result<(SkillDatabase, StatusEffectDatabase, JobClassDatabase), anyhow::Error> {
    if let Some(dir) = find_export() {
        let toml = rose_data_export::Format::Toml;
        let export = rose_data_export::SkillsExport::from_files_as(toml, |name| {
            let path = dir.join(rose_data_export::data_file_as(name, toml));
            match std::fs::read_to_string(&path) {
                Ok(text) => Ok(Some(text)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(anyhow::Error::new(e).context(path.display().to_string())),
            }
        })?;
        if let Some(export) = export {
            godot::global::godot_print!(
                "rose: {} skills, {} status effects and {} job classes from {}",
                export.skills.len(),
                export.status_effects.len(),
                export.job_classes.len(),
                dir.display()
            );
            return export.to_databases(strings);
        }
    }
    godot::global::godot_warn!("rose: no skill export (overrides/data/skills.toml), the skills come from the iROSE tables");
    Ok((
        rose_data_irose::get_skill_database(vfs, strings.clone())?,
        rose_data_irose::get_status_effect_database(vfs, strings.clone())?,
        rose_data_irose::get_job_class_database(vfs, strings)?,
    ))
}

pub fn open(data_idx: &Path) -> Result<(), anyhow::Error> {
    if GAME_DATA.get().is_some() {
        return Ok(());
    }

    let root = data_idx.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let mut vfs = VirtualFilesystem::new(vec![
        Box::new(VfsIndex::load(data_idx)?),
        Box::new(HostFilesystemDevice::new(root.clone())),
    ]);
    apply_overrides(&mut vfs)?;
    let strings = rose_data_irose::get_string_database(&vfs, 1)?;
    let zone_list = rose_data_irose::get_zone_list(&vfs, strings.clone())?;
    let skybox = rose_data_irose::get_skybox_database(&vfs)?;
    let items = load_items(&vfs, strings.clone())?;
    let (skills, status_effects, job_classes) = load_skills(&vfs, strings.clone())?;
    let quests = rose_data_irose::get_quest_database(&vfs, strings.clone())?;
    let npcs = load_npcs(&vfs, strings)?;
    let ltb_event = vfs.read_file::<LtbFile, _>("3DDATA/EVENT/ULNGTB_CON.LTB")?;
    let npc_chr = vfs.read_file::<ChrFile, _>("3DDATA/NPC/LIST_NPC.CHR")?;
    let motions = rose_data_irose::get_character_motion_database(
        &vfs,
        &CharacterMotionDatabaseOptions { load_frame_data: false },
    )?;

    let craft_recipes = rose_game_data::load_craft_recipes(&vfs)?;
    let decoder = rose_data_irose::get_data_decoder();
    let sounds = rose_data_irose::get_sound_database(&vfs)?;
    let effects = rose_data_irose::get_effect_database(&vfs)?;
    let _ = GAME_DATA.set(GameData {
        vfs,
        zone_list,
        skybox,
        items,
        job_classes,
        motions,
        npcs,
        skills,
        status_effects,
        npc_chr,
        quests,
        decoder,
        ltb_event,
        craft_recipes,
        sounds,
        effects,
        event_flags: rose_data_irose::get_animation_event_flags(),
        root,
        data_idx: data_idx.to_path_buf(),
    });
    Ok(())
}

pub fn get() -> Option<&'static GameData> {
    GAME_DATA.get()
}

pub fn read_bytes(path: &str) -> Option<Vec<u8>> {
    let game_data = get()?;
    match game_data.vfs.open_file(path) {
        Ok(VfsFile::Buffer(buffer)) => Some(buffer),
        Ok(VfsFile::View(view)) => Some(view.into()),
        // The VFS upper-cases paths; loose files on a case-sensitive disk need a search.
        Err(_) => std::fs::read(find_ignoring_case(&game_data.root, path)?).ok(),
    }
}

/// A file under `root` whose path matches `path` ignoring case and slash direction.
fn find_ignoring_case(root: &Path, path: &str) -> Option<PathBuf> {
    let mut at = root.to_path_buf();
    for part in path.replace('\\', "/").split('/').filter(|p| !p.is_empty()) {
        let exact = at.join(part);
        if exact.exists() {
            at = exact;
            continue;
        }
        let entry = std::fs::read_dir(&at).ok()?.flatten().find(|e| e.file_name().to_string_lossy().eq_ignore_ascii_case(part))?;
        at = entry.path();
    }
    at.is_file().then_some(at)
}

pub fn read_file<T: RoseFile>(path: &str) -> Option<T>
where
    T::ReadOptions: Default,
{
    get()?.vfs.read_file::<T, _>(path).ok()
}
