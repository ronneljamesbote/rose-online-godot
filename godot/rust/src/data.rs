//! Process-wide access to the ROSE data files and the parsed data tables.

use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use rose_data::{
    AnimationEventFlags, CharacterMotionDatabase, CharacterMotionDatabaseOptions, DataDecoder, EffectDatabase, ItemDatabase,
    JobClassDatabase, NpcDatabase, NpcDatabaseOptions, QuestDatabase, SkillDatabase, SkyboxDatabase, SoundDatabase,
    StatusEffectDatabase, ZoneList,
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
}

static GAME_DATA: OnceLock<GameData> = OnceLock::new();

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
    let zone_list = rose_data_irose::get_zone_list(&vfs, strings.clone())?;
    let skybox = rose_data_irose::get_skybox_database(&vfs)?;
    let items = rose_data_irose::get_item_database(&vfs, strings.clone())?;
    let job_classes = rose_data_irose::get_job_class_database(&vfs, strings.clone())?;
    let skills = rose_data_irose::get_skill_database(&vfs, strings.clone())?;
    let status_effects = rose_data_irose::get_status_effect_database(&vfs, strings.clone())?;
    let quests = rose_data_irose::get_quest_database(&vfs, strings.clone())?;
    let npcs = rose_data_irose::get_npc_database(&vfs, strings, &NpcDatabaseOptions { load_frame_data: false })?;
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
