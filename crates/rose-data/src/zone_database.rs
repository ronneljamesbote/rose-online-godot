use glam::{Vec2, Vec3, Vec3Swizzles};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, num::NonZeroU16, str::FromStr, sync::Arc};

use crate::{NpcConversationId, NpcId, SkyboxId, StringDatabase};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct ZoneId(pub NonZeroU16);

id_wrapper_impl!(ZoneId, NonZeroU16, u16);

pub struct ZoneMonsterSpawnPoint {
    pub position: Vec3,
    pub basic_spawns: Vec<(NpcId, usize)>,
    pub tactic_spawns: Vec<(NpcId, usize)>,
    pub interval: u32,
    pub limit_count: u32,
    pub range: u32,
    pub tactic_points: u32,
}

pub struct ZoneNpcSpawn {
    pub npc_id: NpcId,
    pub position: Vec3,
    pub direction: f32,
    pub conversation: NpcConversationId,
}

pub struct ZoneEventObject {
    pub event_id: u16,
    pub map_chunk_x: i32,
    pub map_chunk_y: i32,
    pub position: Vec3,
}

/// A zone's settings: its iROSE LIST_ZONE.STB row, or its `RoseZoneSettings` resource in the
/// data export. The zone database and the zone list are built from these (with the zone's
/// map files for its spawns, NPCs and named spots). Empty text means none.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneSettings {
    pub id: ZoneId,
    pub name: String,
    pub description: String,
    /// The zone's ZON map file (its blocks' IFO and HIM files sit next to it).
    pub map_file: String,
    /// The named spot players arrive at (`start`).
    pub start_spot: String,
    /// The named spots fallen players get up at (the nearest of that name).
    pub revive_spot: String,
    pub music_day: String,
    pub music_night: String,
    pub skybox_id: Option<SkyboxId>,
    pub minimap: String,
    pub minimap_start_x: u32,
    pub minimap_start_y: u32,
    /// The ZSC model lists of the zone's decorations and buildings.
    pub decoration_models: String,
    pub building_models: String,
    pub day_cycle: u32,
    pub morning_time: u32,
    pub day_time: u32,
    pub evening_time: u32,
    pub night_time: u32,
    /// As ZoneData::pvp_state.
    pub pvp_state: u32,
    /// As ZoneData::planet.
    pub planet: u32,
    pub footstep_type: Option<u32>,
    /// Centimetres per side of a visibility sector (clamped to 5000 to 12000).
    pub sector_size: u32,
    /// As ZoneData::vehicle_use_flags.
    pub vehicle_use_flags: u32,
    /// Metres from the camera at which the game client fades out the zone's small decorations
    /// (0 = never). Not in iROSE: [`DEFAULT_DECORATION_FADE_DISTANCE`] for every zone.
    pub decoration_fade_distance: u32,
}

/// The decoration fade distance of a zone that doesn't set one (`ZoneSettings`).
pub const DEFAULT_DECORATION_FADE_DISTANCE: u32 = 250;

pub struct ZoneData {
    pub id: ZoneId,
    pub name: &'static str,
    pub description: &'static str,
    pub sector_size: u32,
    pub grid_per_patch: f32,
    pub grid_size: f32,
    pub event_objects: Vec<ZoneEventObject>,
    pub monster_spawns: Vec<ZoneMonsterSpawnPoint>,
    pub npcs: Vec<ZoneNpcSpawn>,
    pub sectors_base_position: Vec2,
    pub num_sectors_x: u32,
    pub num_sectors_y: u32,
    pub start_position: Vec3,
    pub revive_positions: Vec<Vec3>,
    pub event_positions: HashMap<String, Vec3>,
    pub day_cycle: u32,
    pub morning_time: u32,
    pub day_time: u32,
    pub evening_time: u32,
    pub night_time: u32,
    pub skybox_id: Option<SkyboxId>,
    /// LIST_ZONE.STB's PvP column: 0 none, 1 all except clan, 2 all except party, 3 all,
    /// 11 clan zone.
    pub pvp_state: u32,
    /// LIST_ZONE.STB's planet column (1 Junon, 2 Luna, ...): a save point only works on the
    /// planet it was saved on.
    pub planet: u32,
    /// LIST_ZONE.STB column 30: 1 no carts, 2 no castle gear, 3 neither.
    pub vehicle_use_flags: u32,
}

impl ZoneData {
    pub fn get_closest_revive_position(&self, origin: Vec3) -> Option<Vec3> {
        let mut closest = None;

        for revive_position in self.revive_positions.iter() {
            let distance = revive_position.xy().distance(origin.xy());

            if closest.map_or(true, |(d, _)| distance < d) {
                closest = Some((distance, revive_position));
            }
        }

        closest.map(|(_, p)| *p)
    }
}

pub struct ZoneDatabase {
    _string_database: Arc<StringDatabase>,
    /// The names and descriptions the zones point into when they don't come from the string
    /// database (the data export, see `with_text`).
    _text: Vec<String>,
    zones: Vec<Option<ZoneData>>,
}

impl ZoneDatabase {
    pub fn new(string_database: Arc<StringDatabase>, zones: Vec<Option<ZoneData>>) -> Self {
        Self {
            _string_database: string_database,
            _text: Vec::new(),
            zones,
        }
    }

    /// Keeps `text` alive as long as the database: the zones' `&'static str` names and
    /// descriptions may point into it.
    pub fn with_text(mut self, text: Vec<String>) -> Self {
        self._text = text;
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = &ZoneData> {
        self.zones.iter().filter_map(|zone_data| zone_data.as_ref())
    }

    pub fn get_zone_mut(&mut self, id: ZoneId) -> Option<&mut ZoneData> {
        self.zones.get_mut(id.get() as usize).and_then(|inner| inner.as_mut())
    }

    pub fn get_zone(&self, id: ZoneId) -> Option<&ZoneData> {
        match self.zones.get(id.get() as usize) {
            Some(inner) => inner.as_ref(),
            None => None,
        }
    }
}
