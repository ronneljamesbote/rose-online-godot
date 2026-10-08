//! Player against player fights, in the zones LIST_ZONE.STB marks for PvP (the Junon Cartel,
//! the Crusader Training Camp, Lion's Plains, the clan fields). Like the iROSE server
//! (CObjAVT::Is_ALLIED): in "all except party" zones party members stay allies, and with no
//! clans yet "all except clan" zones let everyone fight. Elsewhere players can't hurt
//! each other.

use rose_data::ZoneId;
use rose_game_data::GameData;
use spacetimedb::{ReducerContext, Table};

use crate::{entity, party, player, EntityKind};

pub const PVP_ALL_EXCEPT_CLAN: u32 = 1;
pub const PVP_ALL_EXCEPT_PARTY: u32 = 2;
pub const PVP_ALL: u32 = 3;

/// The zone's PvP state, 0 when players can't fight there.
pub fn zone_pvp(game: &GameData, zone_id: u16) -> u32 {
    ZoneId::new(zone_id).and_then(|z| game.zones.get_zone(z)).map_or(0, |z| z.pvp_state)
}

/// Whether these two player entities may hurt each other.
pub fn players_hostile(ctx: &ReducerContext, game: &GameData, a: u64, b: u64) -> bool {
    if a == b {
        return false;
    }
    let (Some(ea), Some(eb)) = (ctx.db.entity().entity_id().find(a), ctx.db.entity().entity_id().find(b)) else { return false };
    if ea.kind != EntityKind::Player || eb.kind != EntityKind::Player || ea.zone_id != eb.zone_id {
        return false;
    }
    match zone_pvp(game, ea.zone_id) {
        PVP_ALL_EXCEPT_PARTY => {
            let identity = |id| ctx.db.player().iter().find(|p| p.entity_id == Some(id)).map(|p| p.identity);
            match (identity(a), identity(b)) {
                (Some(x), Some(y)) => !party::same_party(ctx, x, y),
                _ => false,
            }
        }
        PVP_ALL_EXCEPT_CLAN | PVP_ALL => true,
        _ => false,
    }
}

/// Whether `attacker` (a player or a summon) may attack `target`: wild monsters always,
/// players and their summons where PvP allows.
pub fn can_attack(ctx: &ReducerContext, game: &GameData, attacker: u64, target: u64) -> bool {
    let attacker = crate::skills::controller(ctx, attacker);
    match ctx.db.entity().entity_id().find(target).map(|e| e.kind) {
        Some(EntityKind::Monster) => match crate::skills::owner_of(ctx, target) {
            Some(owner) => players_hostile(ctx, game, attacker, owner),
            None => true,
        },
        Some(EntityKind::Player) => players_hostile(ctx, game, attacker, target),
        _ => false,
    }
}
