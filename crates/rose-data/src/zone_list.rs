use std::sync::Arc;

use rose_file_readers::VfsPathBuf;

use crate::{SkyboxId, StringDatabase, ZoneId};

pub struct ZoneListEntry {
    pub id: ZoneId,
    pub name: &'static str,
    pub description: &'static str,
    pub minimap_path: Option<VfsPathBuf>,
    pub minimap_start_x: u32,
    pub minimap_start_y: u32,
    pub zon_file_path: VfsPathBuf,
    pub zsc_cnst_path: VfsPathBuf,
    pub zsc_deco_path: VfsPathBuf,
    /// As ZoneData::pvp_state.
    pub pvp_state: u32,
    pub day_cycle: u32,
    pub morning_time: u32,
    pub day_time: u32,
    pub evening_time: u32,
    pub night_time: u32,
    pub skybox_id: Option<SkyboxId>,
    /// The music: a Godot audio file or a sound file in the client data, as written.
    pub background_music_day: Option<String>,
    pub background_music_night: Option<String>,
    pub footstep_type: Option<u32>,
}

pub struct ZoneList {
    _string_database: Arc<StringDatabase>,
    /// The names and descriptions the entries point into (see `ZoneDatabase::with_text`).
    _text: Vec<String>,
    zones: Vec<Option<ZoneListEntry>>,
}

impl ZoneList {
    pub fn new(string_database: Arc<StringDatabase>, zones: Vec<Option<ZoneListEntry>>) -> Self {
        Self {
            _string_database: string_database,
            _text: Vec::new(),
            zones,
        }
    }

    /// Keeps `text` alive as long as the list: the entries' `&'static str` names and
    /// descriptions may point into it.
    pub fn with_text(mut self, text: Vec<String>) -> Self {
        self._text = text;
        self
    }

    pub fn len(&self) -> usize {
        self.zones.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &ZoneListEntry> {
        self.zones
            .iter()
            .filter_map(|zone_list_entry| zone_list_entry.as_ref())
    }

    pub fn get_zone(&self, id: ZoneId) -> Option<&ZoneListEntry> {
        match self.zones.get(id.get() as usize) {
            Some(inner) => inner.as_ref(),
            None => None,
        }
    }
}
