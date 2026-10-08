//! Sounds from the client data (WAV and OGG files, cached by path) and which sound each
//! game event plays. Ported from rose-offline-client's animation_sound_system.rs,
//! background_music_system.rs and npc_idle_sound_system.rs.

use std::{cell::RefCell, collections::HashMap};

use godot::{
    classes::{AudioStream, AudioStreamOggVorbis, AudioStreamWav},
    prelude::*,
};
use rose_data::{NpcData, NpcId, SkillId, SoundId, ZoneId};

use crate::data;

thread_local! {
    static STREAMS: RefCell<HashMap<String, Option<Gd<AudioStream>>>> = RefCell::new(HashMap::new());
}

/// Sound played for a footstep on an object (bridges, floors) or where the tile has none.
const DEFAULT_STEP_SOUND: u16 = 653;

/// Hit sounds are looked up by the material of what is hit; characters are flesh (1).
const PLAYER_HIT_MATERIAL: usize = 1;

/// An audio file from the data by path, or None if it is missing or not WAV/OGG.
pub fn load_stream(path: &str) -> Option<Gd<AudioStream>> {
    let key = path.replace('\\', "/").to_ascii_uppercase();
    if let Some(cached) = STREAMS.with_borrow(|cache| cache.get(&key).cloned()) {
        return cached;
    }
    let stream = data::read_bytes(path).and_then(|bytes| {
        let buffer = PackedByteArray::from(bytes.as_slice());
        if key.ends_with(".OGG") {
            AudioStreamOggVorbis::load_from_buffer(&buffer).map(|s| s.upcast::<AudioStream>())
        } else {
            AudioStreamWav::load_from_buffer(&buffer).map(|s| s.upcast::<AudioStream>())
        }
    });
    if stream.is_none() {
        godot_warn!("rose: could not load sound {path}");
    }
    STREAMS.with_borrow_mut(|cache| cache.insert(key, stream.clone()));
    stream
}

pub fn sound(id: Option<SoundId>) -> Option<Gd<AudioStream>> {
    let data = data::get()?.sounds.get_sound(id?)?;
    load_stream(&data.path.path().to_string_lossy())
}

fn npc(npc_id: i32) -> Option<&'static NpcData> {
    data::get()?.npcs.get_npc(NpcId::new(u16::try_from(npc_id).ok()?)?)
}

fn sound_id(id: i32) -> Option<SoundId> {
    SoundId::new(u16::try_from(id).ok()?)
}

/// Drops the cached sounds while the engine is still running.
pub fn clear_cache() {
    STREAMS.with_borrow_mut(|cache| cache.clear());
}

/// Sound lookups for GDScript. Every function returns null when there is no sound.
#[derive(GodotClass)]
#[class(base=RefCounted, init)]
pub struct RoseSound {
    base: Base<RefCounted>,
}

#[godot_api]
impl RoseSound {
    /// An audio file from the data by path (e.g. SOUND/BGM/CANYON01_JUNON.OGG).
    #[func]
    fn stream(path: GString) -> Option<Gd<AudioStream>> {
        load_stream(&path.to_string())
    }

    /// A sound from FILE_SOUND.STB by row.
    #[func]
    fn by_id(id: i32) -> Option<Gd<AudioStream>> {
        sound(sound_id(id))
    }

    /// AnimationEventFlags bits of a ZMO frame event id (0 when it does nothing).
    #[func]
    fn event_flags(event: i32) -> i64 {
        data::get()
            .and_then(|g| g.event_flags.get(usize::try_from(event).ok()?))
            .map_or(0, |flags| flags.bits() as i64)
    }

    /// The zone's background music for day (morning and day) or night (evening and night).
    #[func]
    fn zone_music(zone_id: i32, night: bool) -> Option<Gd<AudioStream>> {
        let zone = data::get()?.zone_list.get_zone(ZoneId::new(u16::try_from(zone_id).ok()?)?)?;
        let path = if night { zone.background_music_night.as_ref() } else { zone.background_music_day.as_ref() }?;
        load_stream(&path.path().to_string_lossy())
    }

    /// A footstep on terrain tile `tile` (RoseZone.get_tile_index), or on an object when -1.
    #[func]
    fn footstep(zone_id: i32, tile: i32) -> Option<Gd<AudioStream>> {
        let game_data = data::get()?;
        let default = || sound(SoundId::new(DEFAULT_STEP_SOUND));
        let Ok(tile) = usize::try_from(tile) else { return default() };
        let zone_type = ZoneId::new(u16::try_from(zone_id).ok()?)
            .and_then(|id| game_data.zone_list.get_zone(id))
            .and_then(|zone| zone.footstep_type)
            .unwrap_or(0) as usize;
        match game_data.sounds.get_step_sound(tile, zone_type) {
            Some(step) => load_stream(&step.path.path().to_string_lossy()),
            None => default(),
        }
    }

    /// The swing sound: the weapon's, else the monster's attack sound, else bare hands.
    #[func]
    fn attack_start(weapon: i32, npc_id: i32) -> Option<Gd<AudioStream>> {
        let items = &data::get()?.items;
        let id = if let Some(weapon) = (weapon > 0).then(|| items.get_weapon_item(weapon as usize)).flatten() {
            weapon.attack_start_sound_id
        } else if let Some(npc) = npc(npc_id) {
            npc.attack_sound_id
        } else {
            items.get_weapon_item(0).and_then(|w| w.attack_start_sound_id)
        };
        sound(id)
    }

    /// The sound of a weapon (0: bare hands or a monster) hitting a monster, or a player
    /// when `target_npc` is 0.
    #[func]
    fn attack_hit(weapon: i32, target_npc: i32) -> Option<Gd<AudioStream>> {
        let game_data = data::get()?;
        let material = if target_npc > 0 { npc(target_npc).map_or(0, |n| n.hit_sound_material_type as usize) } else { PLAYER_HIT_MATERIAL };
        let hit_type = game_data.items.get_weapon_item(weapon.max(0) as usize).map_or(0, |w| w.attack_hit_sound_index as usize);
        let hit = game_data.sounds.get_hit_sound(hit_type, material)?;
        load_stream(&hit.path.path().to_string_lossy())
    }

    /// A bow, crossbow, gun or launcher firing (its own sound, else its projectile's).
    #[func]
    fn weapon_fire(weapon: i32) -> Option<Gd<AudioStream>> {
        let game_data = data::get()?;
        let weapon = game_data.items.get_weapon_item(weapon.max(0) as usize)?;
        let id = weapon.attack_fire_sound_id.or_else(|| {
            weapon.bullet_effect_id.and_then(|id| game_data.effects.get_effect(id)).and_then(|e| e.fire_sound_id)
        });
        sound(id)
    }

    /// A skill's sound: "fire" (its bullet leaves), "hit", "dummy0" or "dummy1" (extra hits).
    #[func]
    fn skill(skill_id: i32, kind: GString) -> Option<Gd<AudioStream>> {
        let skill = data::get()?.skills.get_skill(SkillId::new(u16::try_from(skill_id).ok()?)?)?;
        let id = match kind.to_string().as_str() {
            "fire" => skill.bullet_fire_sound_id,
            "hit" => skill.hit_sound_id,
            "dummy0" => skill.hit_dummy_sound_id[0],
            "dummy1" => skill.hit_dummy_sound_id[1],
            _ => None,
        };
        sound(id)
    }

    /// A monster or NPC sound: "idle", "attack", "hurt", "die" or "spawn".
    #[func]
    fn npc(npc_id: i32, kind: GString) -> Option<Gd<AudioStream>> {
        let npc = npc(npc_id)?;
        let id = match kind.to_string().as_str() {
            "idle" => npc.normal_effect_sound_id,
            "attack" => npc.attack_sound_id,
            "hurt" => npc.hitted_sound_id,
            "die" => npc.die_sound_id,
            "spawn" => npc.create_sound_id,
            _ => None,
        };
        sound(id)
    }
}
