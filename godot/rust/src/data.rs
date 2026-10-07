//! Process-wide access to the ROSE data files and the parsed data tables.

use std::{
    path::Path,
    sync::{Arc, OnceLock},
};

use rose_data::{CharacterMotionDatabase, CharacterMotionDatabaseOptions, ItemDatabase, SkyboxDatabase, ZoneList};
use rose_file_readers::{HostFilesystemDevice, RoseFile, VfsFile, VfsIndex, VirtualFilesystem};

pub struct GameData {
    pub vfs: VirtualFilesystem,
    pub zone_list: ZoneList,
    pub skybox: Arc<SkyboxDatabase>,
    pub items: ItemDatabase,
    pub motions: CharacterMotionDatabase,
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
    let items = rose_data_irose::get_item_database(&vfs, strings)?;
    let motions = rose_data_irose::get_character_motion_database(
        &vfs,
        &CharacterMotionDatabaseOptions { load_frame_data: false },
    )?;

    let _ = GAME_DATA.set(GameData { vfs, zone_list, skybox, items, motions });
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
