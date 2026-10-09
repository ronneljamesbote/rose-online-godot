//! Reading and changing a character's values by AbilityType, as rose-offline's
//! ability_values_get_value / ability_values_add_value (used by item requirements, potions,
//! and later quests). Values the module does not track yet (stamina, unions, fame) read as 0.

use rose_data::AbilityType;
use rose_game_common::components::{AbilityValues, Money};

use crate::{Combat, Player};

pub fn get_value(p: &Player, c: Option<&Combat>, av: &AbilityValues, ability: AbilityType) -> Option<i32> {
    Some(match ability {
        AbilityType::Gender => p.gender as i32,
        AbilityType::Job => p.job as i32,
        AbilityType::Face => p.face as i32,
        AbilityType::Hair => p.hair as i32,
        AbilityType::Strength => av.get_strength(),
        AbilityType::Dexterity => av.get_dexterity(),
        AbilityType::Intelligence => av.get_intelligence(),
        AbilityType::Concentration => av.get_concentration(),
        AbilityType::Charm => av.get_charm(),
        AbilityType::Sense => av.get_sense(),
        AbilityType::Attack => av.get_attack_power(),
        AbilityType::Defence => av.get_defence(),
        AbilityType::Hit => av.get_hit(),
        AbilityType::Resistance => av.get_resistance(),
        AbilityType::Avoid => av.get_avoid(),
        AbilityType::AttackSpeed => av.get_attack_speed(),
        AbilityType::Critical => av.get_critical(),
        AbilityType::Speed => av.get_run_speed() as i32,
        AbilityType::Skillpoint => p.skill_points as i32,
        AbilityType::BonusPoint => p.stat_points as i32,
        AbilityType::Experience => p.xp.min(i32::MAX as u64) as i32,
        AbilityType::Level => p.level as i32,
        AbilityType::Money => p.inventory().money.0.min(i32::MAX as i64) as i32,
        AbilityType::MaxHealth => av.get_max_health(),
        AbilityType::MaxMana => av.get_max_mana(),
        AbilityType::Health => c.map_or(0, |c| c.hp),
        AbilityType::Mana => c.map_or(0, |c| c.mp),
        AbilityType::SaveMana => av.get_save_mana(),
        AbilityType::DropRate => av.get_drop_rate(),
        AbilityType::Fuel => crate::vehicle::engine_life(p),
        AbilityType::Union | AbilityType::Rank | AbilityType::Fame | AbilityType::Stamina => 0,
        _ => return None,
    })
}

fn add(value: u32, add: i32) -> u32 {
    (value as i64 + add as i64).clamp(0, u32::MAX as i64) as u32
}

/// Add to a value. Returns false for types that can't be changed (yet).
/// Basic stat changes need `character::refresh_player` afterwards.
pub fn add_value(p: &mut Player, c: Option<&mut Combat>, ability: AbilityType, value: i32) -> bool {
    match ability {
        AbilityType::Strength => p.strength = (p.strength + value).max(0),
        AbilityType::Dexterity => p.dexterity = (p.dexterity + value).max(0),
        AbilityType::Intelligence => p.intelligence = (p.intelligence + value).max(0),
        AbilityType::Concentration => p.concentration = (p.concentration + value).max(0),
        AbilityType::Charm => p.charm = (p.charm + value).max(0),
        AbilityType::Sense => p.sense = (p.sense + value).max(0),
        AbilityType::BonusPoint => p.stat_points = add(p.stat_points, value),
        AbilityType::Skillpoint => p.skill_points = add(p.skill_points, value),
        AbilityType::Experience => p.xp = (p.xp as i64 + value as i64).max(0) as u64,
        AbilityType::Money => {
            let mut inventory = p.inventory();
            inventory.money = Money((inventory.money.0 + value as i64).max(0));
            p.set_inventory(&inventory);
        }
        AbilityType::Health => match c {
            Some(c) => c.hp = (c.hp + value).clamp(0, c.max_hp),
            None => return false,
        },
        AbilityType::Mana => match c {
            Some(c) => c.mp = (c.mp + value).clamp(0, c.max_mp),
            None => return false,
        },
        _ => return false,
    }
    true
}
