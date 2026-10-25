//! Process-wide access to the ROSE data files and the parsed data tables.

use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use rose_data::{
    AnimationEventFlags, CharacterMotionDatabase, CharacterMotionDatabaseOptions, DataDecoder, DialogDatabase, EffectDatabase, ItemDatabase,
    JobClassDatabase, NpcDatabase, NpcDatabaseOptions, QuestDatabase, SkillDatabase, SkyboxDatabase, SoundDatabase,
    StatusEffectDatabase, StringDatabase, ZoneList,
};
use rose_file_readers::{ChrFile, HostFilesystemDevice, RoseFile, VfsFile, VfsIndex, VirtualFilesystem};

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
    /// What town NPCs say when you talk to them.
    pub dialogs: DialogDatabase,
    /// Crafting recipes, by an item's craft_material.
    pub craft_recipes: Vec<Option<rose_game_data::CraftRecipe>>,
    /// What a new character starts with (the look on the character creation screen).
    pub character_creator: rose_game_data::CharacterCreator,
    pub sounds: Arc<SoundDatabase>,
    pub effects: Arc<EffectDatabase>,
    /// The loading and character creation pictures and the mouse cursors.
    pub interface: rose_data_export::interface::InterfaceSettingsExport,
    /// What each ZMO frame event id does (footstep, hit, fire, ...).
    pub event_flags: Vec<AnimationEventFlags>,
    /// The install folder, for loose files outside the VFS (the Sound folder).
    pub root: PathBuf,
    /// The data.idx this was opened from (the importer lists its files).
    pub data_idx: PathBuf,
}

static GAME_DATA: OnceLock<GameData> = OnceLock::new();

/// The game data exported from the Godot resources (`export/data/`, see
/// `data_resources.rs`), the same files the server's upload sends: ROSE_DATA, else the
/// export folder next to the game, else the repository's (when running from the Godot
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
    godot::global::godot_warn!("rose: no item export (export/data/items/), the items come from the iROSE tables");
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
    godot::global::godot_warn!("rose: no monster export (export/data/monsters.toml), the monsters and NPCs come from the iROSE tables");
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
    godot::global::godot_warn!("rose: no skill export (export/data/skills.toml), the skills come from the iROSE tables");
    Ok((
        rose_data_irose::get_skill_database(vfs, strings.clone())?,
        rose_data_irose::get_status_effect_database(vfs, strings.clone())?,
        rose_data_irose::get_job_class_database(vfs, strings)?,
    ))
}

/// The zone settings, warp gates, crafting recipes and new characters from the export, or
/// without one from the iROSE tables.
fn load_world(vfs: &VirtualFilesystem, strings: &StringDatabase) -> Result<rose_game_data::World, anyhow::Error> {
    if let Some(dir) = find_export() {
        let toml = rose_data_export::Format::Toml;
        let export = rose_data_export::WorldExport::from_files_as(toml, |name| {
            let path = dir.join(rose_data_export::data_file_as(name, toml));
            match std::fs::read_to_string(&path) {
                Ok(text) => Ok(Some(text)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(anyhow::Error::new(e).context(path.display().to_string())),
            }
        })?;
        if let Some(export) = export {
            godot::global::godot_print!(
                "rose: {} zones, {} warp gates and {} recipes from {}",
                export.zones.len(),
                export.warp_gates.len(),
                export.craft_recipes.len(),
                dir.display()
            );
            return rose_game_data::world_from_export(&export);
        }
    }
    godot::global::godot_warn!("rose: no world export (export/data/zones.toml), the zones come from the iROSE tables");
    rose_game_data::load_world_irose(vfs, strings)
}

/// The quests and quest triggers from the export, or without one from the iROSE files (the
/// game checks a trigger's conditions before asking the server to run it).
fn load_quests(vfs: &VirtualFilesystem, strings: Arc<StringDatabase>) -> Result<QuestDatabase, anyhow::Error> {
    if let Some(dir) = find_export() {
        let toml = rose_data_export::Format::Toml;
        let export = rose_data_export::QuestsExport::from_files_as(toml, |name| {
            let path = dir.join(rose_data_export::data_file_as(name, toml));
            match std::fs::read_to_string(&path) {
                Ok(text) => Ok(Some(text)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(anyhow::Error::new(e).context(path.display().to_string())),
            }
        })?;
        if let Some(export) = export {
            godot::global::godot_print!(
                "rose: {} quests and {} quest triggers from {}",
                export.quests.len(),
                export.triggers.len(),
                dir.display()
            );
            return export.to_database(strings, &rose_game_data::irose_codes());
        }
    }
    godot::global::godot_warn!("rose: no quest export (export/data/quests.toml), the quests come from the iROSE files");
    rose_data_irose::get_quest_database(vfs, strings)
}

/// The NPC dialogs from the export, or without one from the iROSE CON files. Only the game
/// reads them (the server's upload doesn't send them).
fn load_dialogs(vfs: &VirtualFilesystem) -> Result<DialogDatabase, anyhow::Error> {
    if let Some(dir) = find_export() {
        let toml = rose_data_export::Format::Toml;
        let export = rose_data_export::DialogsExport::from_files_as(toml, |name| {
            let path = dir.join(rose_data_export::data_file_as(name, toml));
            match std::fs::read_to_string(&path) {
                Ok(text) => Ok(Some(text)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(anyhow::Error::new(e).context(path.display().to_string())),
            }
        })?;
        if let Some(export) = export {
            godot::global::godot_print!("rose: {} NPC dialogs from {}", export.dialogs.len(), dir.display());
            return export.to_database();
        }
    }
    godot::global::godot_warn!("rose: no dialog export (export/data/dialogs.toml), the NPC dialogs come from the iROSE files");
    let (dialogs, notes) = rose_data_irose::get_dialog_database(vfs)?;
    for note in notes {
        godot::global::godot_warn!("rose: {note}");
    }
    Ok(dialogs)
}

/// The sounds, hit sounds and footsteps from the export, or without one from the iROSE
/// tables. Only the game reads them (the server's upload doesn't send them).
fn load_sounds(vfs: &VirtualFilesystem) -> Result<Arc<SoundDatabase>, anyhow::Error> {
    if let Some(dir) = find_export() {
        let toml = rose_data_export::Format::Toml;
        let export = rose_data_export::SoundsExport::from_files_as(toml, |name| {
            let path = dir.join(rose_data_export::data_file_as(name, toml));
            match std::fs::read_to_string(&path) {
                Ok(text) => Ok(Some(text)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(anyhow::Error::new(e).context(path.display().to_string())),
            }
        })?;
        if let Some(export) = export {
            godot::global::godot_print!(
                "rose: {} sounds, {} hit sound sets and {} footstep sets from {}",
                export.sounds.len(),
                export.hit_sounds.len(),
                export.footsteps.len(),
                dir.display()
            );
            return Ok(Arc::new(export.to_database()?));
        }
    }
    godot::global::godot_warn!("rose: no sound export (export/data/sounds.toml), the sounds come from the iROSE tables");
    rose_data_irose::get_sound_database(vfs)
}

/// The effect files and attack effects from the export, or without one from the iROSE
/// tables. Only the game reads them (the server's upload doesn't send them).
fn load_effects(vfs: &VirtualFilesystem) -> Result<Arc<EffectDatabase>, anyhow::Error> {
    if let Some(dir) = find_export() {
        let toml = rose_data_export::Format::Toml;
        let export = rose_data_export::EffectsExport::from_files_as(toml, |name| {
            let path = dir.join(rose_data_export::data_file_as(name, toml));
            match std::fs::read_to_string(&path) {
                Ok(text) => Ok(Some(text)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(anyhow::Error::new(e).context(path.display().to_string())),
            }
        })?;
        if let Some(export) = export {
            godot::global::godot_print!(
                "rose: {} effect files and {} attack effects from {}",
                export.effect_files.len(),
                export.attack_effects.len(),
                dir.display()
            );
            return Ok(Arc::new(export.to_database()?));
        }
    }
    godot::global::godot_warn!("rose: no effect export (export/data/effect-files.toml), the effects come from the iROSE tables");
    rose_data_irose::get_effect_database(vfs)
}

/// The skies and the interface's pictures and cursors from the export, or without one the
/// iROSE sky table and the client's own pictures and cursors. Only the game reads them.
fn load_interface(
    vfs: &VirtualFilesystem,
) -> Result<(Arc<SkyboxDatabase>, rose_data_export::interface::InterfaceSettingsExport), anyhow::Error> {
    if let Some(dir) = find_export() {
        let toml = rose_data_export::Format::Toml;
        let export = rose_data_export::InterfaceExport::from_files_as(toml, |name| {
            let path = dir.join(rose_data_export::data_file_as(name, toml));
            match std::fs::read_to_string(&path) {
                Ok(text) => Ok(Some(text)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(anyhow::Error::new(e).context(path.display().to_string())),
            }
        })?;
        if let Some(export) = export {
            godot::global::godot_print!("rose: {} skies and the interface from {}", export.skies.len(), dir.display());
            return Ok((Arc::new(export.to_database()?), export.interface));
        }
    }
    godot::global::godot_warn!("rose: no sky export (export/data/skies.toml), the skies come from the iROSE tables");
    Ok((rose_data_irose::get_skybox_database(vfs)?, Default::default()))
}

pub fn open(data_idx: &Path) -> Result<(), anyhow::Error> {
    if GAME_DATA.get().is_some() {
        return Ok(());
    }

    let root = data_idx.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let vfs = VirtualFilesystem::new(vec![
        Box::new(VfsIndex::load(data_idx)?),
        Box::new(HostFilesystemDevice::new(root.clone())),
    ]);
    let strings = rose_data_irose::get_string_database(&vfs, 1)?;
    let world = load_world(&vfs, &strings)?;
    let zone_list = rose_data_irose::get_zone_list_from_settings(strings.clone(), &world.zone_settings);
    let (skybox, interface) = load_interface(&vfs)?;
    let items = load_items(&vfs, strings.clone())?;
    let (skills, status_effects, job_classes) = load_skills(&vfs, strings.clone())?;
    let quests = load_quests(&vfs, strings.clone())?;
    let npcs = load_npcs(&vfs, strings)?;
    let dialogs = load_dialogs(&vfs)?;
    let npc_chr = vfs.read_file::<ChrFile, _>("3DDATA/NPC/LIST_NPC.CHR")?;
    let motions = rose_data_irose::get_character_motion_database(
        &vfs,
        &CharacterMotionDatabaseOptions { load_frame_data: false },
    )?;

    let decoder = rose_data_irose::get_data_decoder();
    let sounds = load_sounds(&vfs)?;
    let effects = load_effects(&vfs)?;
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
        dialogs,
        craft_recipes: world.craft_recipes,
        character_creator: world.character_creator,
        sounds,
        effects,
        interface,
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
