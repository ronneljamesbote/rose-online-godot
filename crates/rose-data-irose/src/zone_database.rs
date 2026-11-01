use std::sync::Arc;

use glam::{Quat, Vec2, Vec3, Vec3Swizzles};
use log::debug;

use rose_data::{
    NpcConversationId, NpcId, SkyboxId, StringDatabase, ZoneData, ZoneDatabase, ZoneEventObject,
    ZoneId, ZoneList, ZoneListEntry, ZoneMonsterSpawnPoint, ZoneNpcSpawn, ZoneSettings, WORLD_TICKS_PER_DAY,
};
use rose_file_readers::{
    stb_column, IfoEventObject, IfoFile, IfoMonsterSpawn, IfoMonsterSpawnPoint, IfoNpc,
    IfoReadOptions, StbFile, VfsPath, VfsPathBuf, VirtualFilesystem, ZonFile, ZonReadOptions,
};

const MIN_SECTOR_SIZE: u32 = 5000;
const MAX_SECTOR_SIZE: u32 = 12000;

pub struct StbZone(pub StbFile);

#[allow(dead_code)]
impl StbZone {
    pub fn rows(&self) -> usize {
        self.0.rows()
    }

    stb_column! { 1, get_zone_file, &str }
    stb_column! { 2, get_zone_start_event_position_name, &str }
    stb_column! { 3, get_zone_revive_event_position_name, &str }
    stb_column! { 0, get_zone_is_underground, bool }
    stb_column! { 5, get_zone_background_music_day, &str }
    stb_column! { 6, get_zone_background_music_night, &str }
    stb_column! { 7, get_zone_skybox_id, SkyboxId }
    stb_column! { 8, get_zone_minimap_filename, &str }
    stb_column! { 9, get_zone_minimap_start_x, u32 }
    stb_column! { 10, get_zone_minimap_start_y, u32 }
    stb_column! { 11, get_zone_deco_table, &str }
    stb_column! { 12, get_zone_cnst_table, &str }
    stb_column! { 13, get_zone_day_cycle_time, u32 }
    stb_column! { 14, get_zone_morning_time, u32 }
    stb_column! { 15, get_zone_day_time, u32 }
    stb_column! { 16, get_zone_evening_time, u32 }
    stb_column! { 17, get_zone_night_time, u32 }
    stb_column! { 18, get_zone_pvp_state, u32 }
    stb_column! { 19, get_zone_planet, u32 }
    stb_column! { 20, get_zone_footstep_type, u32 }
    stb_column! { 21, get_zone_camera_type, u32 }
    stb_column! { 22, get_zone_join_trigger, &str }
    stb_column! { 23, get_zone_kill_trigger, &str }
    stb_column! { 24, get_zone_dead_trigger, &str }
    stb_column! { 25, get_zone_sector_size, u32 }
    stb_column! { 26, get_zone_string_id, &str }
    stb_column! { 27, get_zone_weather_type, u32 }
    stb_column! { 28, get_zone_party_xp_a, u32 }
    stb_column! { 29, get_zone_party_xp_b, u32 }
    stb_column! { 30, get_zone_vehicle_use_flags, u32 }
    stb_column! { 31, get_zone_revive_zone_no, u32 }
    stb_column! { 32, get_zone_revive_pos_x, u32 }
    stb_column! { 33, get_zone_revive_pos_y, u32 }
}

pub enum LoadZoneError {
    NotExists,
    ZonFileInvalidPath,
    ZonFileNotFound,
}

fn create_monster_spawn(
    spawn: &IfoMonsterSpawnPoint,
    object_offset: Vec3,
) -> ZoneMonsterSpawnPoint {
    let transform_spawn_list = |spawn_list: &Vec<IfoMonsterSpawn>| {
        spawn_list
            .iter()
            .map(|&IfoMonsterSpawn { id, count }| (NpcId::new(id as u16).unwrap(), count as usize))
            .collect()
    };

    ZoneMonsterSpawnPoint {
        position: Vec3::new(
            spawn.object.position.x,
            spawn.object.position.y,
            spawn.object.position.z,
        ) + object_offset,
        basic_spawns: transform_spawn_list(&spawn.basic_spawns),
        tactic_spawns: transform_spawn_list(&spawn.tactic_spawns),
        interval: spawn.interval,
        limit_count: spawn.limit_count,
        range: spawn.range,
        tactic_points: spawn.tactic_points,
    }
}

fn create_npc_spawn(npc: &IfoNpc, object_offset: Vec3) -> ZoneNpcSpawn {
    ZoneNpcSpawn {
        npc_id: NpcId::new(npc.object.object_id as u16).unwrap(),
        position: Vec3::new(
            npc.object.position.x,
            npc.object.position.y,
            npc.object.position.z,
        ) + object_offset,
        direction: Quat::from_xyzw(
            npc.object.rotation.x,
            npc.object.rotation.y,
            npc.object.rotation.z,
            npc.object.rotation.w,
        )
        .to_euler(glam::EulerRot::XYZ)
        .2
        .to_degrees(),
        conversation: NpcConversationId::new(npc.quest_file_name.to_string()),
    }
}

fn create_event_object(
    event_object: &IfoEventObject,
    object_offset: Vec3,
    map_chunk_x: i32,
    map_chunk_y: i32,
) -> ZoneEventObject {
    ZoneEventObject {
        event_id: event_object.object.event_id,
        map_chunk_x,
        map_chunk_y,
        position: Vec3::new(
            event_object.object.position.x,
            event_object.object.position.y,
            event_object.object.position.z,
        ) + object_offset,
    }
}

/// Gives `text` a `'static` lifetime by keeping it in `keep`, which the database owns.
fn keep(keep: &mut Vec<String>, text: &str) -> &'static str {
    if text.is_empty() {
        return "";
    }
    let owned = text.to_string();
    // SAFETY: the String's buffer never moves or changes while it sits in `keep`, and
    // `keep` lives in the database (`with_text`) as long as the data pointing into it.
    let text: &'static str = unsafe { std::mem::transmute::<&str, &'static str>(owned.as_str()) };
    keep.push(owned);
    text
}

fn path_or_none(path: &str) -> Option<VfsPathBuf> {
    (!path.is_empty()).then(|| VfsPathBuf::new(path))
}

/// One LIST_ZONE.STB row as the zone's settings (name and description from
/// LIST_ZONE_S.STL); None for a row without a map file, which the game doesn't use.
fn zone_settings(data: &StbZone, string_database: &StringDatabase, id: usize) -> Option<ZoneSettings> {
    let map_file = data.get_zone_file(id)?;
    let zone_strings = string_database.get_zone(data.get_zone_string_id(id).unwrap_or(""));
    let text = |s: Option<&str>| s.unwrap_or("").to_string();
    Some(ZoneSettings {
        id: ZoneId::new(id as u16)?,
        name: zone_strings.as_ref().map_or(String::new(), |x| x.name.to_string()),
        description: zone_strings.as_ref().map_or(String::new(), |x| x.description.to_string()),
        map_file: map_file.to_string(),
        start_spot: text(data.get_zone_start_event_position_name(id)),
        revive_spot: text(data.get_zone_revive_event_position_name(id)),
        music_day: text(data.get_zone_background_music_day(id)),
        music_night: text(data.get_zone_background_music_night(id)),
        skybox_id: data.get_zone_skybox_id(id),
        minimap: text(data.get_zone_minimap_filename(id)),
        minimap_start_x: data.get_zone_minimap_start_x(id).unwrap_or(0),
        minimap_start_y: data.get_zone_minimap_start_y(id).unwrap_or(0),
        decoration_models: text(data.get_zone_deco_table(id)),
        building_models: text(data.get_zone_cnst_table(id)),
        day_cycle: data.get_zone_day_cycle_time(id).unwrap_or(WORLD_TICKS_PER_DAY as u32),
        morning_time: data.get_zone_morning_time(id).unwrap_or((WORLD_TICKS_PER_DAY / 6) as u32),
        day_time: data.get_zone_day_time(id).unwrap_or((2 * WORLD_TICKS_PER_DAY / 6) as u32),
        evening_time: data.get_zone_evening_time(id).unwrap_or((4 * WORLD_TICKS_PER_DAY / 6) as u32),
        night_time: data.get_zone_night_time(id).unwrap_or((5 * WORLD_TICKS_PER_DAY / 6) as u32),
        pvp_state: data.get_zone_pvp_state(id).unwrap_or(0),
        planet: data.get_zone_planet(id).unwrap_or(0),
        footstep_type: data.get_zone_footstep_type(id),
        sector_size: data.get_zone_sector_size(id).unwrap_or(0).clamp(MIN_SECTOR_SIZE, MAX_SECTOR_SIZE),
        vehicle_use_flags: data.get_zone_vehicle_use_flags(id).unwrap_or(0),
        decoration_fade_distance: rose_data::DEFAULT_DECORATION_FADE_DISTANCE,
    })
}

/// Every zone's settings from LIST_ZONE.STB, by id (the rows with a map file).
pub fn get_zone_settings(
    vfs: &VirtualFilesystem,
    string_database: &StringDatabase,
) -> Result<Vec<ZoneSettings>, anyhow::Error> {
    let data = StbZone(vfs.read_file::<StbFile, _>("3DDATA/STB/LIST_ZONE.STB")?);
    Ok((1..data.rows()).filter_map(|id| zone_settings(&data, string_database, id)).collect())
}

fn load_zone(
    vfs: &VirtualFilesystem,
    settings: &ZoneSettings,
    name: &'static str,
    description: &'static str,
) -> Result<ZoneData, LoadZoneError> {
    let id = settings.id.get();
    let zone_file = VfsPath::from(settings.map_file.as_str());
    let zone_base_directory = zone_file
        .path()
        .parent()
        .ok_or(LoadZoneError::ZonFileInvalidPath)?;

    let zon_file: ZonFile = vfs
        .read_file_with(
            &zone_file,
            &ZonReadOptions {
                skip_zone_info: false,
                skip_event_positions: false,
                skip_textures: true,
                skip_tiles: true,
            },
        )
        .map_err(|_| LoadZoneError::ZonFileNotFound)?;
    let mut monster_spawns = Vec::new();
    let mut npcs = Vec::new();
    let mut event_objects = Vec::new();
    let mut warp_gates = Vec::new();

    let mut num_blocks = 0;
    let mut min_block_x = None;
    let mut min_block_y = None;
    let mut max_block_x = None;
    let mut max_block_y = None;

    let objects_offset = Vec3::new(
        (64.0 / 2.0) * (zon_file.grid_size * zon_file.grid_per_patch * 16.0)
            + (zon_file.grid_size * zon_file.grid_per_patch * 16.0) / 2.0,
        (64.0 / 2.0) * (zon_file.grid_size * zon_file.grid_per_patch * 16.0)
            + (zon_file.grid_size * zon_file.grid_per_patch * 16.0) / 2.0,
        0.0,
    );

    let ifo_read_options = IfoReadOptions {
        skip_event_objects: false,
        skip_monster_spawns: false,
        skip_npcs: false,
        skip_animated_objects: true,
        skip_collision_objects: true,
        skip_cnst_objects: true,
        skip_deco_objects: true,
        skip_effect_objects: true,
        skip_sound_objects: true,
        skip_water_planes: true,
        skip_warp_objects: false,
    };

    for block_y in 0..64u32 {
        for block_x in 0..64u32 {
            // Most of the 64x64 blocks have no file; checking first skips building an error for each.
            let ifo_path = zone_base_directory.join(format!("{}_{}.IFO", block_x, block_y));
            if !vfs.exists(ifo_path.as_path()) {
                continue;
            }
            if let Ok(ifo_file) = vfs.read_file_with::<IfoFile, _>(ifo_path, &ifo_read_options) {
                monster_spawns.extend(
                    ifo_file
                        .monster_spawns
                        .iter()
                        .map(|x| create_monster_spawn(x, objects_offset)),
                );
                npcs.extend(
                    ifo_file
                        .npcs
                        .iter()
                        .map(|x| create_npc_spawn(x, objects_offset)),
                );
                event_objects.extend(ifo_file.event_objects.iter().map(|event_object| {
                    create_event_object(
                        event_object,
                        objects_offset,
                        block_x as i32,
                        block_y as i32,
                    )
                }));
                warp_gates.extend(ifo_file.warps.iter().map(|w| w.warp_id));
                num_blocks += 1;

                min_block_x = Some(min_block_x.map_or(block_x, |value| u32::min(value, block_x)));
                min_block_y = Some(min_block_y.map_or(block_y, |value| u32::min(value, block_y)));
                max_block_x = Some(max_block_x.map_or(block_x, |value| u32::max(value, block_x)));
                max_block_y = Some(max_block_y.map_or(block_y, |value| u32::max(value, block_y)));
            }
        }
    }

    if min_block_x.is_none()
        || min_block_y.is_none()
        || max_block_x.is_none()
        || max_block_y.is_none()
    {
        return Err(LoadZoneError::NotExists);
    }

    // Block files count rows from the north: row b covers y from (64 - b) to (65 - b) blocks.
    let min_x = min_block_x.unwrap();
    let min_y = 64 - max_block_y.unwrap();
    let max_x = max_block_x.unwrap() + 1;
    let max_y = 65 - min_block_y.unwrap();

    let sector_size = settings.sector_size.clamp(MIN_SECTOR_SIZE, MAX_SECTOR_SIZE);
    let block_size = 16.0 * zon_file.grid_per_patch * zon_file.grid_size;
    let num_blocks_x = max_x - min_x;
    let num_blocks_y = max_y - min_y;
    let num_sectors_x = ((num_blocks_x as f32 * block_size) / sector_size as f32) as u32;
    let num_sectors_y = ((num_blocks_y as f32 * block_size) / sector_size as f32) as u32;

    let start_event_position_name = settings.start_spot.as_str();
    let revive_event_position_name = settings.revive_spot.as_str();
    let mut start_position = Vec3::new(0.0, 0.0, 0.0);
    let mut revive_positions = Vec::new();
    for (name, position) in zon_file.event_positions.iter() {
        let position = Vec3::new(position.x, position.y, position.z).xzy() + objects_offset;

        if name == start_event_position_name {
            start_position = position;
        }

        if name == revive_event_position_name {
            revive_positions.push(position);
        }
    }

    debug!(
        "Loaded zone {} {} blocks: {}, spawns: {}, npcs: {}, sectors ({}, {}), start: {}",
        id,
        name,
        num_blocks,
        monster_spawns.len(),
        npcs.len(),
        num_sectors_x,
        num_sectors_y,
        start_position.xy(),
    );
    Ok(ZoneData {
        id: settings.id,
        name,
        description,
        sector_size,
        grid_per_patch: zon_file.grid_per_patch,
        grid_size: zon_file.grid_size,
        event_objects,
        monster_spawns,
        npcs,
        warp_gates,
        sectors_base_position: Vec2::new((min_x as f32) * block_size, (min_y as f32) * block_size),
        num_sectors_x,
        num_sectors_y,
        start_position,
        revive_positions,
        event_positions: zon_file
            .event_positions
            .into_iter()
            .map(|(name, position)| {
                (
                    name,
                    Vec3::new(position.x, position.y, position.z).xzy() + objects_offset,
                )
            })
            .collect(),
        pvp_state: settings.pvp_state,
        planet: settings.planet,
        vehicle_use_flags: settings.vehicle_use_flags,
        day_cycle: settings.day_cycle,
        morning_time: settings.morning_time,
        day_time: settings.day_time,
        evening_time: settings.evening_time,
        night_time: settings.night_time,
        skybox_id: settings.skybox_id,
    })
}

/// The zones from LIST_ZONE.STB and their map files.
pub fn get_zone_database(
    vfs: &VirtualFilesystem,
    string_database: Arc<StringDatabase>,
) -> Result<ZoneDatabase, anyhow::Error> {
    let settings = get_zone_settings(vfs, &string_database)?;
    Ok(get_zone_database_from_settings(vfs, string_database, &settings))
}

/// The zones from their settings (iROSE's or the data export's) and their map files. A zone
/// whose map files are missing isn't there, as in iROSE.
pub fn get_zone_database_from_settings(
    vfs: &VirtualFilesystem,
    string_database: Arc<StringDatabase>,
    settings: &[ZoneSettings],
) -> ZoneDatabase {
    let mut text = Vec::new();
    let mut zones: Vec<Option<ZoneData>> = Vec::new();
    for zone in settings {
        let at = zone.id.get() as usize;
        if zones.len() <= at {
            zones.resize_with(at + 1, || None);
        }
        let name = keep(&mut text, &zone.name);
        let description = keep(&mut text, &zone.description);
        zones[at] = load_zone(vfs, zone, name, description).ok();
    }
    ZoneDatabase::new(string_database, zones).with_text(text)
}

fn zone_list_entry(settings: &ZoneSettings, text: &mut Vec<String>) -> ZoneListEntry {
    ZoneListEntry {
        id: settings.id,
        name: keep(text, &settings.name),
        description: keep(text, &settings.description),
        minimap_path: path_or_none(&settings.minimap),
        minimap_start_x: settings.minimap_start_x,
        minimap_start_y: settings.minimap_start_y,
        zon_file_path: VfsPathBuf::new(&settings.map_file),
        zsc_cnst_path: VfsPathBuf::new(&settings.building_models),
        zsc_deco_path: VfsPathBuf::new(&settings.decoration_models),
        pvp_state: settings.pvp_state,
        day_cycle: settings.day_cycle,
        morning_time: settings.morning_time,
        day_time: settings.day_time,
        evening_time: settings.evening_time,
        night_time: settings.night_time,
        skybox_id: settings.skybox_id,
        background_music_day: (!settings.music_day.is_empty()).then(|| settings.music_day.clone()),
        background_music_night: (!settings.music_night.is_empty()).then(|| settings.music_night.clone()),
        footstep_type: settings.footstep_type,
        decoration_fade_distance: settings.decoration_fade_distance,
    }
}

/// The zone list (what the client needs to draw a zone) from LIST_ZONE.STB.
pub fn get_zone_list(
    vfs: &VirtualFilesystem,
    string_database: Arc<StringDatabase>,
) -> Result<ZoneList, anyhow::Error> {
    let settings = get_zone_settings(vfs, &string_database)?;
    Ok(get_zone_list_from_settings(string_database, &settings))
}

/// The zone list from the zones' settings (iROSE's or the data export's).
pub fn get_zone_list_from_settings(string_database: Arc<StringDatabase>, settings: &[ZoneSettings]) -> ZoneList {
    let mut text = Vec::new();
    let mut zones: Vec<Option<ZoneListEntry>> = vec![None]; // Zone ID 0
    for zone in settings {
        let at = zone.id.get() as usize;
        if zones.len() <= at {
            zones.resize_with(at + 1, || None);
        }
        zones[at] = Some(zone_list_entry(zone, &mut text));
    }
    ZoneList::new(string_database, zones).with_text(text)
}
