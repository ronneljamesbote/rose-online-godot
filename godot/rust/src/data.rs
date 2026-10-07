//! Process-wide access to the ROSE data files and the parsed data tables.

use std::{
    path::Path,
    sync::{Arc, OnceLock},
};

use rose_data::{
    CharacterMotionDatabase, CharacterMotionDatabaseOptions, DataDecoder, ItemDatabase, JobClassDatabase, NpcDatabase, NpcDatabaseOptions,
    QuestDatabase, SkillDatabase, SkyboxDatabase, StatusEffectDatabase, ZoneList,
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
}

static GAME_DATA: OnceLock<GameData> = OnceLock::new();

pub fn open(data_idx: &Path) -> Result<(), anyhow::Error> {
    if GAME_DATA.get().is_some() {
        return Ok(());
    }

    let vfs = VirtualFilesystem::new(vec![
        Box::new(VfsIndex::load(data_idx)?),
        Box::new(HostFilesystemDevice::new(
            data_idx.parent().map(|p| p.to_path_buf()).unwrap_or_default(),
        )),
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

    let decoder = rose_data_irose::get_data_decoder();
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
    });
    Ok(())
}

pub fn get() -> Option<&'static GameData> {
    GAME_DATA.get()
}

pub fn read_bytes(path: &str) -> Option<Vec<u8>> {
    match get()?.vfs.open_file(path).ok()? {
        VfsFile::Buffer(buffer) => Some(buffer),
        VfsFile::View(view) => Some(view.into()),
    }
}

pub fn read_file<T: RoseFile>(path: &str) -> Option<T>
where
    T::ReadOptions: Default,
{
    get()?.vfs.read_file::<T, _>(path).ok()
}
