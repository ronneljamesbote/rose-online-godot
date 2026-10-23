//! Who sees whom. Every entity's zone sector is kept in `entity_sector`; a client subscribes
//! to the entities in its own sector and the eight around it, as iROSE's server sends them
//! (the zone's sector size, `sector_size` in its settings, 50 to 120 m). Monsters in the rest of
//! the zone, and in other zones, are never sent.

use std::cell::Cell;

use rose_game_data::GameData;
use spacetimedb::{ReducerContext, Table};

use crate::{entity, motion, EntityKind};

/// How often moving entities are checked for a sector change.
const REFRESH_US: i64 = 200_000;

#[spacetimedb::table(accessor = entity_sector, public)]
#[derive(Clone, PartialEq)]
pub struct EntitySector {
    #[primary_key]
    pub entity_id: u64,
    #[index(btree)]
    pub zone_id: u16,
    pub sector_x: i32,
    pub sector_y: i32,
    /// Players are sent zone-wide to party members (their HP bars), so they are marked.
    pub is_player: bool,
}

thread_local! {
    static LAST_REFRESH_US: Cell<i64> = const { Cell::new(0) };
}

/// The sector a point of a zone is in (rose-offline's zone sectors; points outside the zone
/// fall into the nearest one).
pub fn sector_of(game: &GameData, zone_id: u16, x: f32, y: f32) -> (i32, i32) {
    let Some(zone) = rose_data::ZoneId::new(zone_id).and_then(|id| game.zones.get_zone(id)) else { return (0, 0) };
    let size = zone.sector_size.max(1) as f32;
    let sx = ((x - zone.sectors_base_position.x) / size).floor() as i32;
    let sy = ((y - zone.sectors_base_position.y) / size).floor() as i32;
    (sx.clamp(0, zone.num_sectors_x as i32), sy.clamp(0, zone.num_sectors_y as i32))
}

/// Puts every entity in the sector it is in now: new entities get their row and moved ones
/// a new sector (despawn drops the row). Rows are only written when they change, so a
/// client only hears about an entity that crossed into its view. About 3 ms for 1,800
/// entities, five times a second.
pub fn refresh(ctx: &ReducerContext, game: &GameData, t: i64) {
    if t - LAST_REFRESH_US.get() < REFRESH_US {
        return;
    }
    LAST_REFRESH_US.set(t);
    for e in ctx.db.entity().iter() {
        let Some(m) = ctx.db.motion().entity_id().find(e.entity_id) else { continue };
        let (x, y) = m.position_at(t);
        let (sector_x, sector_y) = sector_of(game, e.zone_id, x, y);
        let row = EntitySector {
            entity_id: e.entity_id,
            zone_id: e.zone_id,
            sector_x,
            sector_y,
            is_player: e.kind == EntityKind::Player,
        };
        match ctx.db.entity_sector().entity_id().find(e.entity_id) {
            Some(old) if old == row => {}
            Some(_) => {
                ctx.db.entity_sector().entity_id().update(row);
            }
            None => {
                ctx.db.entity_sector().insert(row);
            }
        }
    }
}

/// Drops an entity's sector row (it left the world).
pub fn forget(ctx: &ReducerContext, entity_id: u64) {
    ctx.db.entity_sector().entity_id().delete(entity_id);
}
