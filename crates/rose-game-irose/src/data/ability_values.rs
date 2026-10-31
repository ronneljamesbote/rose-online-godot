use core::f32;
use log::error;
use rand::Rng;
use rose_data_irose::IroseSkillPageType;
use std::{num::NonZeroU32, sync::Arc};

use rose_data::{
    AbilityType, AmmoIndex, EquipmentIndex, EquipmentItem, Item, ItemClass, ItemDatabase,
    ItemReference, ItemType, ItemWeaponType, NpcDatabase, NpcId, SkillAddAbility, SkillDamageType,
    SkillData, SkillDatabase, VehiclePartIndex,
};
use rose_file_readers::QsdEquation;
use rose_game_common::{
    components::{
        AbilityValues, BasicStatType, BasicStats, CharacterInfo, DamageCategory, DamageType,
        Equipment, EquipmentItemDatabase, ItemSlot, Level, Money, SkillList, StatusEffects,
    },
    data::{AbilityValueCalculator, Damage, PassiveRecoveryState},
};

const MAX_BASIC_STAT_VALUE: i32 = 300;

/// Experience needed to go from `level` to the next one.
pub fn levelup_require_xp(level: u32) -> u64 {
    match level as u64 {
        0..=15 => (((level + 3) * (level + 5) * (level + 10)) as f64 * 0.7) as u64,
        16..=60 => (((level - 5) * (level + 2) * (level + 2)) as f64 * 2.2) as u64,
        61..=113 => (((level - 11) * (level) * (level + 4)) as f64 * 2.5) as u64,
        114..=150 => (((level - 31) * (level - 20) * (level + 4)) as f64 * 3.8) as u64,
        151..=189 => (((level - 67) * (level - 20) * (level - 10)) as f64 * 6.0) as u64,
        190..=u64::MAX => {
            ((level - 90) * (level - 120) * (level - 60) * (level - 170) * (level - 188)) as u64
        }
    }
}

/// Stat points it costs to raise a basic stat from `current`, or None at the maximum.
pub fn basic_stat_increase_cost(current: i32) -> Option<u32> {
    if current >= MAX_BASIC_STAT_VALUE {
        None
    } else {
        Some((current as f32 * 0.2) as u32)
    }
}

/// What an NPC store charges for one of this item (rose-offline's
/// calculate_npc_store_item_buy_price, usable without a calculator).
pub fn npc_store_buy_price(
    item_database: &ItemDatabase,
    item: ItemReference,
    buy_skill_value: i32,
    item_rate: i32,
    town_rate: i32,
) -> Option<i32> {
    let item_data = item_database.get_base_item(item)?;

    match item.item_type {
        ItemType::Face
        | ItemType::Head
        | ItemType::Body
        | ItemType::Hands
        | ItemType::Feet
        | ItemType::Back
        | ItemType::Weapon
        | ItemType::SubWeapon
        | ItemType::Vehicle => Some(
            (item_data.base_price as f32
                * (item_data.quality as f32 + 50.0)
                * (1.0 - buy_skill_value as f32 * 0.01)
                / 100.0
                + 0.5) as i32,
        ),
        ItemType::Consumable
        | ItemType::Material
        | ItemType::Jewellery
        | ItemType::Gem
        | ItemType::Quest => {
            let item_rate = if matches!(
                item_data.class,
                ItemClass::Medicine
                    | ItemClass::Food
                    | ItemClass::Metal
                    | ItemClass::OtherworldlyMetal
                    | ItemClass::StoneMaterial
                    | ItemClass::WoodenMaterial
                    | ItemClass::Leather
                    | ItemClass::Cloth
                    | ItemClass::RefiningMaterial
                    | ItemClass::Chemicals
            ) {
                item_rate
            } else {
                town_rate
            };

            Some(
                (item_data.base_price as f32
                    * (1.0 + (item_rate as f32 - 50.0) * item_data.price_rate as f32 / 1000.0)
                    * (1.0 - buy_skill_value as f32 * 0.01)
                    + 0.5) as i32,
            )
        }
    }
}

/// What an NPC store pays for one of this item (rose-offline's
/// calculate_npc_store_item_sell_price, usable without a calculator).
pub fn npc_store_sell_price(
    item_database: &ItemDatabase,
    item: &Item,
    sell_skill_value: i32,
    world_rate: i32,
    item_rate: i32,
    town_rate: i32,
) -> Option<i32> {
    let item_data = item_database.get_base_item(item.get_item_reference())?;
    match item.get_item_type() {
        ItemType::Face
        | ItemType::Head
        | ItemType::Body
        | ItemType::Hands
        | ItemType::Feet
        | ItemType::Back
        | ItemType::Weapon
        | ItemType::SubWeapon
        | ItemType::Vehicle => {
            let item = item.as_equipment()?;
            let gem_base_price = if item.is_appraised {
                item_database
                    .get_base_item(ItemReference::gem(item.gem as usize))
                    .map(|gem_item_data| gem_item_data.base_price)
                    .unwrap_or(0)
            } else {
                0
            } as f32;
            Some(
                ((item_data.base_price as f32
                    * (40.0 + item.grade as f32)
                    * (200.0 + item.durability as f32)
                    * (200.0 - world_rate as f32)
                    * (1.0 + sell_skill_value as f32 * 0.01)
                    * ((4000.0 + item.life as f32) / 14000.0)
                    / 1000000.0)
                    + gem_base_price * 0.2) as i32,
            )
        }
        ItemType::Consumable
        | ItemType::Material
        | ItemType::Jewellery
        | ItemType::Gem
        | ItemType::Quest => {
            let item_rate = if matches!(
                item_data.class,
                ItemClass::Medicine
                    | ItemClass::Food
                    | ItemClass::Metal
                    | ItemClass::OtherworldlyMetal
                    | ItemClass::StoneMaterial
                    | ItemClass::WoodenMaterial
                    | ItemClass::Leather
                    | ItemClass::Cloth
                    | ItemClass::RefiningMaterial
                    | ItemClass::Chemicals
            ) {
                item_rate
            } else {
                town_rate
            };

            Some(
                (item_data.base_price as f32
                    * (1000.0 + (item_rate as f32 - 50.0) * item_data.price_rate as f32)
                    * (1.0 + sell_skill_value as f32 * 0.01)
                    * (200.0 - world_rate as f32)
                    / 180000.0) as i32,
            )
        }
    }
}

pub struct AbilityValuesData {
    item_database: Arc<ItemDatabase>,
    skill_database: Arc<SkillDatabase>,
    npc_database: Arc<NpcDatabase>,
}

pub fn get_ability_value_calculator(
    item_database: Arc<ItemDatabase>,
    skill_database: Arc<SkillDatabase>,
    npc_database: Arc<NpcDatabase>,
) -> Box<impl AbilityValueCalculator + Send + Sync> {
    Box::new(AbilityValuesData {
        item_database,
        skill_database,
        npc_database,
    })
}

impl AbilityValueCalculator for AbilityValuesData {
    fn calculate_npc(
        &self,
        npc_id: NpcId,
        status_effects: &StatusEffects,
        owner_level: Option<i32>,
        summon_skill_level: Option<i32>,
    ) -> Option<AbilityValues> {
        let npc_data = self.npc_database.get_npc(npc_id)?;
        let mut level = npc_data.level;
        let mut max_health = npc_data.level * npc_data.health_points;
        let mut attack_power = npc_data.attack;
        let mut hit = npc_data.hit;
        let mut defence = npc_data.defence;
        let mut resistance = npc_data.resistance;
        let mut avoid = npc_data.avoid;

        if let Some(owner_level) = owner_level {
            let summon_skill_level = summon_skill_level.unwrap_or(1);

            level = owner_level;
            max_health =
                (npc_data.health_points * (summon_skill_level + 16) * (owner_level + 85)) / 2600;
            attack_power = (attack_power * (summon_skill_level + 22) * (owner_level + 100)) / 4000;
            hit = (hit * (summon_skill_level + 30) * (owner_level + 50)) / 3200;
            defence = (defence * (summon_skill_level + 30) * (owner_level + 80)) / 4400;
            resistance = (resistance * (summon_skill_level + 24) * (owner_level + 90)) / 3600;
            avoid = (avoid * (summon_skill_level + 22) * (owner_level + 90)) / 3400;
        }

        Some(AbilityValues {
            is_driving: false,
            damage_category: DamageCategory::Npc,
            walk_speed: npc_data.walk_speed as f32,
            run_speed: npc_data.run_speed as f32,
            vehicle_move_speed: 0.0,
            level,
            strength: 0,
            dexterity: 0,
            intelligence: npc_data.level,
            concentration: 0,
            charm: 0,
            sense: npc_data.level,
            max_health,
            max_mana: 100,
            additional_health_recovery: 0,
            additional_mana_recovery: 0,
            attack_damage_type: if npc_data.is_attack_magic_damage {
                DamageType::Magic
            } else {
                DamageType::Physical
            },
            attack_power,
            attack_speed: npc_data.attack_speed,
            passive_attack_speed: 0,
            attack_range: npc_data.attack_range,
            hit,
            defence,
            resistance,
            // Monsters crit a flat 5% (critical_chance_of); they have no critical rating.
            critical: 0,
            avoid,
            vehicle_attack_power: 0,
            vehicle_attack_range: 0,
            vehicle_attack_speed: 0,
            vehicle_hit: 0,
            vehicle_defence: 0,
            vehicle_critical: 0,
            vehicle_avoid: 0,
            max_damage_sources: ((npc_data.health_points / 8) + 4) as usize,
            drop_rate: 0,
            max_weight: 0,
            summon_owner_level: owner_level,
            summon_skill_level,
            adjust: status_effects.into(),
            npc_store_buy_rate: 0,
            npc_store_sell_rate: 0,
            save_mana: 0,
        })
    }

    fn calculate(
        &self,
        character_info: &CharacterInfo,
        level: &Level,
        equipment: &Equipment,
        basic_stats: &BasicStats,
        skill_list: &SkillList,
        status_effects: &StatusEffects,
    ) -> AbilityValues {
        let equipment_ability_values =
            calculate_equipment_ability_values(&self.item_database, equipment);
        let vehicle_ability_values =
            calculate_vehicle_ability_values(&self.item_database, equipment);
        let passive_ability_values =
            calculate_passive_skill_ability_values(&self.skill_database, skill_list);

        let vehicle_basic_stats = basic_stats.clone();
        let basic_stats = BasicStats {
            strength: (basic_stats.strength
                + passive_ability_values.value.strength
                + passive_ability_values.rate.strength
                + equipment_ability_values.strength),
            dexterity: (basic_stats.dexterity
                + passive_ability_values.value.dexterity
                + passive_ability_values.rate.dexterity
                + equipment_ability_values.dexterity),
            intelligence: (basic_stats.intelligence
                + passive_ability_values.value.intelligence
                + passive_ability_values.rate.intelligence
                + equipment_ability_values.intelligence),
            concentration: (basic_stats.concentration
                + passive_ability_values.value.concentration
                + passive_ability_values.rate.concentration
                + equipment_ability_values.concentration),
            charm: (basic_stats.charm
                + passive_ability_values.value.charm
                + passive_ability_values.rate.charm
                + equipment_ability_values.charm),
            sense: (basic_stats.sense
                + passive_ability_values.value.sense
                + passive_ability_values.rate.sense
                + equipment_ability_values.sense),
        };

        /*
        TODO:
        Cal_MaxWEIGHT ();
        m_fRateUseMP
        job based += stats + immunity
        */
        let (attack_speed, passive_attack_speed) = calculate_attack_speed(
            &self.item_database,
            &basic_stats,
            equipment,
            &equipment_ability_values,
            &passive_ability_values,
        );

        let (job_add_max_health, job_add_attack, job_add_defence, job_add_resistance) =
            match character_info.job {
                121 | 122 | 221 | 222 | 321 | 322 | 421 | 422 => (300, 30, 25, 20),
                _ => (0, 0, 0, 0),
            };

        AbilityValues {
            is_driving: false,
            damage_category: DamageCategory::Character,
            walk_speed: 200.0,
            run_speed: calculate_run_speed(
                &self.item_database,
                &basic_stats,
                &equipment_ability_values,
                equipment,
                &passive_ability_values,
            ),
            vehicle_move_speed: calculate_vehicle_move_speed(
                &self.item_database,
                &vehicle_ability_values,
                equipment,
            ),
            max_health: calculate_max_health(
                character_info,
                level,
                &basic_stats,
                &equipment_ability_values,
                &passive_ability_values,
            ) + job_add_max_health,
            max_mana: calculate_max_mana(
                character_info,
                level,
                &basic_stats,
                &equipment_ability_values,
                &passive_ability_values,
            ),
            level: level.level as i32,
            strength: basic_stats.strength,
            dexterity: basic_stats.dexterity,
            intelligence: basic_stats.intelligence,
            concentration: basic_stats.concentration,
            charm: basic_stats.charm,
            sense: basic_stats.sense,
            additional_health_recovery: passive_ability_values.value.recover_health
                + (equipment_ability_values.recover_health as f32
                    * (passive_ability_values.rate.recover_health as f32 / 100.0))
                    as i32,
            additional_mana_recovery: passive_ability_values.value.recover_mana
                + (equipment_ability_values.recover_mana as f32
                    * (passive_ability_values.rate.recover_mana as f32 / 100.0))
                    as i32,
            attack_damage_type: self
                .item_database
                .get_equipped_weapon_item_data(equipment, EquipmentIndex::Weapon)
                .map(|item| {
                    if item.is_magic_damage {
                        DamageType::Magic
                    } else {
                        DamageType::Physical
                    }
                })
                .unwrap_or(DamageType::Physical),
            attack_power: calculate_attack_power(
                &self.item_database,
                &basic_stats,
                level,
                &equipment_ability_values,
                equipment,
                &passive_ability_values,
            ) + job_add_attack,
            attack_speed,
            passive_attack_speed,
            attack_range: calculate_attack_range(&self.item_database, equipment),
            hit: calculate_hit(
                &self.item_database,
                &basic_stats,
                &equipment_ability_values,
                equipment,
                &passive_ability_values,
            ),
            defence: calculate_defence(
                &self.item_database,
                &basic_stats,
                level,
                &equipment_ability_values,
                equipment,
                &passive_ability_values,
                false,
            ) + job_add_defence,
            resistance: calculate_resistance(
                &self.item_database,
                &basic_stats,
                level,
                &equipment_ability_values,
                equipment,
                &passive_ability_values,
            ) + job_add_resistance,
            critical: calculate_critical(
                &basic_stats,
                &equipment_ability_values,
                &passive_ability_values,
            ),
            avoid: calculate_avoid(
                &self.item_database,
                &basic_stats,
                level,
                equipment,
                &equipment_ability_values,
                &passive_ability_values,
            ),
            vehicle_attack_power: calculate_vehicle_attack_power(
                &self.item_database,
                &vehicle_basic_stats,
                level,
                &vehicle_ability_values,
                equipment,
            ) + job_add_attack,
            vehicle_attack_range: calculate_vehicle_attack_range(&self.item_database, equipment),
            vehicle_attack_speed: calculate_vehicle_attack_speed(
                &self.item_database,
                equipment,
                &vehicle_ability_values,
            ),
            vehicle_hit: calculate_vehicle_hit(
                &self.item_database,
                &vehicle_basic_stats,
                level,
                &vehicle_ability_values,
                equipment,
                &passive_ability_values,
            ),
            vehicle_defence: calculate_defence(
                &self.item_database,
                &basic_stats,
                level,
                &equipment_ability_values,
                equipment,
                &passive_ability_values,
                true,
            ) + job_add_defence,
            vehicle_critical: calculate_vehicle_critical(
                &vehicle_basic_stats,
                level,
                &vehicle_ability_values,
                &passive_ability_values,
            ),
            vehicle_avoid: calculate_vehicle_avoid(
                &vehicle_basic_stats,
                level,
                &vehicle_ability_values,
                &passive_ability_values,
            ),
            max_damage_sources: 0,
            drop_rate: calculate_drop_rate(&equipment_ability_values, &passive_ability_values),
            save_mana: calculate_save_mana(&equipment_ability_values, &passive_ability_values),
            max_weight: calculate_max_weight(
                &self.item_database,
                level,
                &basic_stats,
                equipment,
                &equipment_ability_values,
                &passive_ability_values,
            ),
            summon_owner_level: None,
            summon_skill_level: None,
            adjust: status_effects.into(),
            npc_store_buy_rate: passive_ability_values.value.buy_skill,
            npc_store_sell_rate: passive_ability_values.value.sell_skill,
        }
    }

    fn calculate_damage(
        &self,
        attacker: &AbilityValues,
        defender: &AbilityValues,
        hit_count: i32,
    ) -> Damage {
        let mut rng = crate::rng::SharedRng;
        if !roll_hit(&mut rng, attacker, defender) {
            return Damage {
                amount: 0,
                apply_hit_stun: false,
                is_critical: false,
            };
        }
        // Staffs, wands and magic monsters do magic damage: magic resistance only.
        let magic = matches!(attacker.get_attack_damage_type(), DamageType::Magic);
        let hit = DamageHit {
            base_attack: attacker.get_attack_power() as f32,
            power: 0,
            magic,
            hit_count,
            pvp_cap: PVP_CAP_NORMAL,
            pvp_cap_critical: PVP_CAP_CRITICAL,
        };
        roll_damage(&mut rng, attacker, defender, &hit)
    }

    fn calculate_skill_adjust_value(
        &self,
        skill_add_ability: &SkillAddAbility,
        caster_intelligence: i32,
        ability_value: i32,
    ) -> i32 {
        ((ability_value * skill_add_ability.rate) as f32 / 100.0
            + skill_add_ability.value as f32 * (caster_intelligence as f32 + 300.0) / 315.0)
            as i32
    }

    fn calculate_skill_damage(
        &self,
        attacker: &AbilityValues,
        defender: &AbilityValues,
        skill_data: &SkillData,
        hit_count: i32,
    ) -> Damage {
        let mut rng = crate::rng::SharedRng;
        let missed = match skill_data.damage_type {
            // Weapon skills share the plain-attack hit chance (wiki/rules/combat.md "Hit or miss").
            SkillDamageType::WeaponAttack => !roll_hit(&mut rng, attacker, defender),
            // Continuous attacks keep their own success roll, now only for hit or miss.
            SkillDamageType::ContinuousAttack => {
                let success = ((attacker.get_level() + 8) - defender.get_level()
                    + rng.gen_range(1..=80)) as f32
                    * (attacker.get_hit() as f32 - defender.get_avoid() as f32 * 0.6
                        + rng.gen_range(1..=50) as f32
                        + 50.0)
                    / 90.0;
                success < 10.0
            }
            // Magic never misses.
            SkillDamageType::MagicAttack | SkillDamageType::NaturalMagic => false,
        };
        if missed {
            return Damage {
                amount: 0,
                is_critical: false,
                apply_hit_stun: false,
            };
        }
        let magic = matches!(
            skill_data.damage_type,
            SkillDamageType::MagicAttack | SkillDamageType::NaturalMagic
        );
        let hit = DamageHit {
            base_attack: if magic {
                magic_attack(attacker)
            } else {
                attacker.get_attack_power() as f32
            },
            power: skill_data.power as i32,
            magic,
            hit_count,
            pvp_cap: PVP_CAP_SKILL,
            pvp_cap_critical: PVP_CAP_SKILL,
        };
        roll_damage(&mut rng, attacker, defender, &hit)
    }

    fn calculate_give_xp(
        &self,
        attacker_level: i32,
        attacker_damage: i32,
        defender_level: i32,
        defender_max_hp: i32,
        defender_reward_xp: i32,
        world_xp_rate: i32,
    ) -> i32 {
        let level_difference = attacker_level - defender_level;
        let attacker_damage = attacker_damage as f32;
        let defender_level = defender_level as f32;
        let defender_max_hp = defender_max_hp as f32;
        let defender_reward_xp = defender_reward_xp as f32;
        let world_xp_rate = world_xp_rate as f32;

        (if level_difference < 3 {
            ((defender_level + 3.0)
                * defender_reward_xp
                * (attacker_damage + defender_max_hp / 15.0 + 30.0)
                * world_xp_rate)
                / defender_max_hp
                / 370.0
        } else {
            ((defender_level + 3.0)
                * defender_reward_xp
                * (attacker_damage + defender_max_hp / 15.0 + 30.0)
                * world_xp_rate)
                / defender_max_hp
                / (level_difference + 3) as f32
                / 60.0
        }) as i32
    }

    fn calculate_give_stamina(
        &self,
        experience_points: i32,
        level: i32,
        world_stamina_rate: i32,
    ) -> i32 {
        (((experience_points + 100) as f32 / (level + 6) as f32)
            * (world_stamina_rate as f32 / 80.0)) as i32
    }

    fn calculate_basic_stat_increase_cost(
        &self,
        basic_stats: &BasicStats,
        basic_stat_type: BasicStatType,
    ) -> Option<u32> {
        let current = match basic_stat_type {
            BasicStatType::Strength => basic_stats.strength,
            BasicStatType::Dexterity => basic_stats.dexterity,
            BasicStatType::Intelligence => basic_stats.intelligence,
            BasicStatType::Concentration => basic_stats.concentration,
            BasicStatType::Charm => basic_stats.charm,
            BasicStatType::Sense => basic_stats.sense,
        };

        if current >= MAX_BASIC_STAT_VALUE {
            None
        } else {
            Some((current as f32 * 0.2) as u32)
        }
    }

    fn calculate_levelup_require_xp(&self, level: u32) -> u64 {
        levelup_require_xp(level)
    }

    fn calculate_levelup_reward_skill_points(&self, level: u32) -> u32 {
        (level + 2) / 2
    }

    fn calculate_levelup_reward_stat_points(&self, level: u32) -> u32 {
        (level as f32 * 0.8) as u32 + 10
    }

    fn calculate_reward_value(
        &self,
        equation: &QsdEquation,
        base_reward_value: i32,
        dup_count: i32,
        level: i32,
        charm: i32,
        fame: i32,
        world_reward_rate: i32,
    ) -> i32 {
        match equation {
            QsdEquation::ExpUnleveled => {
                ((base_reward_value + 30) * (charm + 10) * world_reward_rate * (fame + 20)
                    / (level + 70))
                    / 30000
                    + base_reward_value
            }
            QsdEquation::ExpLeveled => {
                base_reward_value * (level + 3) * (level + charm / 2 + 40) * world_reward_rate
                    / 10000
            }
            QsdEquation::MoneyStatic => base_reward_value * dup_count,
            QsdEquation::MoneyScaled | QsdEquation::Item => {
                ((base_reward_value + 20) * (charm + 10) * world_reward_rate * (fame + 20)
                    / (level + 70))
                    / 30000
                    + base_reward_value
            }
            QsdEquation::Unknown => {
                ((base_reward_value + 2) * (level + charm + 40) * (fame + 40) * world_reward_rate)
                    / 140000
            }
            QsdEquation::Unknown2 => {
                ((base_reward_value + 20) * (level + charm) * (fame + 20) * world_reward_rate)
                    / 3000000
                    + base_reward_value
            }
        }
    }

    fn calculate_npc_store_item_buy_price(
        &self,
        item_database: &ItemDatabase,
        item: ItemReference,
        buy_skill_value: i32,
        item_rate: i32,
        town_rate: i32,
    ) -> Option<i32> {
        npc_store_buy_price(item_database, item, buy_skill_value, item_rate, town_rate)
    }

    fn calculate_npc_store_item_sell_price(
        &self,
        item_database: &ItemDatabase,
        item: &Item,
        sell_skill_value: i32,
        world_rate: i32,
        item_rate: i32,
        town_rate: i32,
    ) -> Option<i32> {
        npc_store_sell_price(item_database, item, sell_skill_value, world_rate, item_rate, town_rate)
    }

    fn calculate_passive_recover_hp(
        &self,
        ability_values: &AbilityValues,
        recovery_state: PassiveRecoveryState,
    ) -> i32 {
        match recovery_state {
            PassiveRecoveryState::Normal => {
                ((ability_values.get_additional_health_recovery() as f32
                    + (ability_values.get_concentration() as f32 + 40.0) / 6.0)
                    / 6.0) as i32
            }
            PassiveRecoveryState::Sitting => {
                (ability_values.get_additional_health_recovery() as f32
                    + ((((ability_values.get_concentration() as f32 + 30.0) / 8.0) * 23.0) / 10.0))
                    as i32
            }
        }
    }

    fn calculate_passive_recover_mp(
        &self,
        ability_values: &AbilityValues,
        recovery_state: PassiveRecoveryState,
    ) -> i32 {
        match recovery_state {
            PassiveRecoveryState::Normal => 0,
            PassiveRecoveryState::Sitting => {
                (ability_values.get_additional_mana_recovery() as f32
                    + ((((ability_values.get_concentration() as f32 + 20.0) / 10.0) * 20.0) / 7.0))
                    as i32
            }
        }
    }

    fn calculate_decrease_weapon_life(
        &self,
        is_driving: bool,
        equipment: &Equipment,
    ) -> Option<ItemSlot> {
        let (equipment_item, item_slot) = if is_driving {
            (
                equipment.get_vehicle_item(VehiclePartIndex::Arms),
                ItemSlot::Vehicle(VehiclePartIndex::Arms),
            )
        } else {
            (
                equipment.get_equipment_item(EquipmentIndex::Weapon),
                ItemSlot::Equipment(EquipmentIndex::Weapon),
            )
        };

        let equipment_item = equipment_item?;
        if equipment_item.life == 0 {
            return None;
        }

        let mut rng = crate::rng::SharedRng;
        if rng.gen_range(1..=710) >= equipment_item.durability as i32 + 600 {
            Some(item_slot)
        } else {
            None
        }
    }

    fn calculate_decrease_armour_life(
        &self,
        is_driving: bool,
        equipment: &Equipment,
        damage: &Damage,
    ) -> Option<ItemSlot> {
        let mut rng = crate::rng::SharedRng;
        let rand_value = rng.gen_range(1..=400);

        if rand_value >= 101 {
            return None;
        }

        let (equipment_item, item_slot) = if is_driving {
            let vehicle_part_index = if rand_value >= 51 {
                VehiclePartIndex::Body
            } else if rand_value >= 21 {
                VehiclePartIndex::Leg
            } else {
                VehiclePartIndex::Arms
            };

            (
                equipment.get_vehicle_item(vehicle_part_index),
                ItemSlot::Vehicle(vehicle_part_index),
            )
        } else {
            let equipment_index = if equipment
                .get_equipment_item(EquipmentIndex::SubWeapon)
                .filter(|item| item.life > 0)
                .and_then(|item| self.item_database.get_base_item(item.item))
                .map_or(false, |item_data| {
                    matches!(item_data.class, ItemClass::Shield)
                }) {
                if rand_value > 60 {
                    EquipmentIndex::SubWeapon
                } else if rand_value > 30 {
                    EquipmentIndex::Body
                } else if rand_value > 22 {
                    EquipmentIndex::Feet
                } else if rand_value > 16 {
                    EquipmentIndex::Head
                } else if rand_value > 9 {
                    EquipmentIndex::Back
                } else if rand_value > 5 {
                    EquipmentIndex::Hands
                } else {
                    EquipmentIndex::Face
                }
            } else if rand_value > 64 {
                EquipmentIndex::Body
            } else if rand_value > 46 {
                EquipmentIndex::Feet
            } else if rand_value > 31 {
                EquipmentIndex::Hands
            } else if rand_value > 15 {
                EquipmentIndex::Back
            } else if rand_value > 7 {
                EquipmentIndex::Head
            } else {
                EquipmentIndex::Face
            };

            (
                equipment.get_equipment_item(equipment_index),
                ItemSlot::Equipment(equipment_index),
            )
        };

        let equipment_item = equipment_item?;
        if equipment_item.life == 0 {
            return None;
        }

        if rng.gen_range(1..=120)
            - (equipment_item.durability as i32 + 10 - (damage.amount / 10) as i32)
            >= 0
        {
            Some(item_slot)
        } else {
            None
        }
    }

    fn calculate_repair_from_npc_price(&self, item: &EquipmentItem) -> Money {
        let base_item_price = self
            .item_database
            .get_base_item(item.item)
            .map_or(0, |item_data| item_data.base_price);

        Money(
            (((base_item_price + 1000) as f32 / 400000.0)
                * (item.durability + 10) as f32
                * (1100 - item.life) as f32) as i64,
        )
    }

    fn calculate_clan_max_members(&self, level: NonZeroU32) -> usize {
        match level.get() {
            1 => 15,
            2 => 20,
            3 => 25,
            4 => 30,
            5 => 36,
            6 => 43,
            _ => 50,
        }
    }
}

/// Hit chance tuning (wiki/rules/combat.md "Hit or miss"): how much the defender's avoid
/// weighs against the attacker's hit.
pub const HIT_AVOID_WEIGHT: f32 = 0.3;
/// Percent added per level the attacker is above the defender (taken off per level below).
pub const HIT_PERCENT_PER_LEVEL: f32 = 1.5;
/// Level difference counted at most, either way.
pub const HIT_LEVEL_DIFFERENCE_CAP: i32 = 10;
/// Lowest and highest hit chance in percent: every swing can hit and every swing can miss.
pub const HIT_CHANCE_MIN: f32 = 5.0;
pub const HIT_CHANCE_MAX: f32 = 95.0;

/// Percent chance that a plain attack or weapon skill lands, the same for players, monsters
/// and PvP: `100 * hit / (hit + avoid * 0.3) + 1.5 * clamp(level difference, -10, 10)`,
/// clamped to 5..95.
pub fn hit_chance(attacker: &AbilityValues, defender: &AbilityValues) -> f32 {
    hit_chance_from(
        attacker.get_hit(),
        defender.get_avoid(),
        attacker.get_level() - defender.get_level(),
    )
}

/// `hit_chance` from the raw numbers.
pub fn hit_chance_from(hit: i32, avoid: i32, level_difference: i32) -> f32 {
    let hit = hit.max(0) as f32;
    let avoid = avoid.max(0) as f32 * HIT_AVOID_WEIGHT;
    let base = if hit + avoid > 0.0 {
        100.0 * hit / (hit + avoid)
    } else {
        100.0
    };
    let level = level_difference.clamp(-HIT_LEVEL_DIFFERENCE_CAP, HIT_LEVEL_DIFFERENCE_CAP) as f32
        * HIT_PERCENT_PER_LEVEL;
    (base + level).clamp(HIT_CHANCE_MIN, HIT_CHANCE_MAX)
}

/// One roll against `hit_chance`.
fn roll_hit(rng: &mut impl Rng, attacker: &AbilityValues, defender: &AbilityValues) -> bool {
    rng.gen_range(0.0f32..100.0) < hit_chance(attacker, defender)
}

// ---------------------------------------------------------------------------------------------
// Damage (wiki/rules/combat.md "Damage", wiki/rules/skills.md "Skill damage"). Every tuning
// constant of the damage, critical and level-gap rules is here.
// ---------------------------------------------------------------------------------------------

/// Raw damage is attack × this for players (and anything a player drives).
pub const PLAYER_RAW_SCALE: f32 = 1.2;
/// Raw damage is attack × this for monsters and summons.
pub const MONSTER_RAW_SCALE: f32 = 0.5;
/// Defence (or magic resistance) takes off `DEF / (DEF + 0.75 × raw + 80)` of a hit.
pub const DEFENCE_RAW_WEIGHT: f32 = 0.75;
pub const DEFENCE_CONSTANT: f32 = 80.0;
/// Defence takes off at most this much of a hit.
pub const MAX_DAMAGE_REDUCTION: f32 = 0.75;
/// Each hit rolls between 90% and 110% of its raw damage.
pub const DAMAGE_SPREAD_MIN: f32 = 0.9;
pub const DAMAGE_SPREAD_MAX: f32 = 1.1;
/// Percent more (or less) damage per level the attacker is above (or below) the defender.
pub const LEVEL_GAP_PERCENT_PER_LEVEL: f32 = 2.0;
/// Level difference counted at most, either way.
pub const LEVEL_GAP_CAP: i32 = 10;
/// Player critical chance in percent: `5 + 45 × rating / (rating + 1.5 × level + 60)`, at most 50.
pub const CRITICAL_CHANCE_BASE: f32 = 5.0;
pub const CRITICAL_CHANCE_SPAN: f32 = 45.0;
pub const CRITICAL_LEVEL_WEIGHT: f32 = 1.5;
pub const CRITICAL_CONSTANT: f32 = 60.0;
pub const CRITICAL_CHANCE_MAX: f32 = 50.0;
/// Player critical damage in percent of a normal hit: 150 + 1.2 per point of Sense.
pub const CRITICAL_DAMAGE_BASE: f32 = 150.0;
pub const CRITICAL_DAMAGE_PER_SENSE: f32 = 1.2;
/// Monsters and summons crit a flat 5% of the time, for 150%.
pub const MONSTER_CRITICAL_CHANCE: f32 = 5.0;
pub const MONSTER_CRITICAL_DAMAGE: f32 = 150.0;
/// A player hitting a player does half damage, and at most this share of the target's max HP.
pub const PVP_DAMAGE_SCALE: f32 = 0.5;
pub const PVP_CAP_NORMAL: f32 = 0.25;
pub const PVP_CAP_CRITICAL: f32 = 0.35;
pub const PVP_CAP_SKILL: f32 = 0.45;
pub const MIN_DAMAGE: f32 = 1.0;
pub const MAX_DAMAGE: f32 = 2047.0;

/// Share of a hit (0 to 0.75) that defence or magic resistance takes off:
/// `DEF / (DEF + 0.75 × raw + 80)`. A bigger hit loses a smaller share.
pub fn damage_reduction(defence: f32, raw: f32) -> f32 {
    let defence = defence.max(0.0);
    let raw = raw.max(0.0);
    (defence / (defence + DEFENCE_RAW_WEIGHT * raw + DEFENCE_CONSTANT)).min(MAX_DAMAGE_REDUCTION)
}

/// Percent chance of a critical hit for a critical rating at a level.
pub fn critical_chance(rating: i32, level: i32) -> f32 {
    let rating = rating.max(0) as f32;
    let level = level.max(1) as f32;
    (CRITICAL_CHANCE_BASE
        + CRITICAL_CHANCE_SPAN * rating
            / (rating + CRITICAL_LEVEL_WEIGHT * level + CRITICAL_CONSTANT))
        .min(CRITICAL_CHANCE_MAX)
}

/// Percent chance that this attacker's hit is critical.
pub fn critical_chance_of(attacker: &AbilityValues) -> f32 {
    match attacker.get_damage_category() {
        DamageCategory::Character => critical_chance(attacker.get_critical(), attacker.get_level()),
        DamageCategory::Npc => MONSTER_CRITICAL_CHANCE,
    }
}

/// A critical hit of this attacker does this percent of a normal hit.
pub fn critical_damage_percent(attacker: &AbilityValues) -> f32 {
    match attacker.get_damage_category() {
        DamageCategory::Character => {
            CRITICAL_DAMAGE_BASE + CRITICAL_DAMAGE_PER_SENSE * attacker.get_sense().max(0) as f32
        }
        DamageCategory::Npc => MONSTER_CRITICAL_DAMAGE,
    }
}

/// Damage multiplier for the level difference: ±2% per level, at most 10 levels counted.
pub fn level_gap_multiplier(attacker_level: i32, defender_level: i32) -> f32 {
    let gap = (attacker_level - defender_level).clamp(-LEVEL_GAP_CAP, LEVEL_GAP_CAP) as f32;
    1.0 + gap * LEVEL_GAP_PERCENT_PER_LEVEL / 100.0
}

/// The attack magic skills use: attack × 0.8 + INT × 1.2.
pub fn magic_attack(attacker: &AbilityValues) -> f32 {
    attacker.get_attack_power().max(0) as f32 * 0.8 + attacker.get_intelligence().max(0) as f32 * 1.2
}

fn raw_scale(attacker: &AbilityValues) -> f32 {
    match attacker.get_damage_category() {
        DamageCategory::Character => PLAYER_RAW_SCALE,
        DamageCategory::Npc => MONSTER_RAW_SCALE,
    }
}

/// What a hit is made of: the attack it starts from, a skill's power, and how it is defended.
pub struct DamageHit {
    /// Attack (or magic attack for magic skills).
    pub base_attack: f32,
    /// Skill power in percent on top of the attack; 0 for a plain attack.
    pub power: i32,
    /// Magic: reduced by magic resistance instead of defence.
    pub magic: bool,
    /// Hits in the attack animation.
    pub hit_count: i32,
    /// Most of the target's max HP one hit may take between players.
    pub pvp_cap: f32,
    pub pvp_cap_critical: f32,
}

/// The damage of a hit that landed, with the random parts given: `spread` is 0.9 to 1.1 and
/// `critical` whether it is critical. Not rounded.
pub fn hit_damage(
    attacker: &AbilityValues,
    defender: &AbilityValues,
    hit: &DamageHit,
    spread: f32,
    critical: bool,
) -> f32 {
    let raw = hit.base_attack.max(0.0)
        * raw_scale(attacker)
        * (1.0 + hit.power.max(0) as f32 / 100.0)
        * spread;
    let defence = if hit.magic {
        defender.get_resistance()
    } else {
        defender.get_defence()
    } as f32;
    let mut damage = raw * (1.0 - damage_reduction(defence, raw));
    if critical {
        damage *= critical_damage_percent(attacker) / 100.0;
    }
    damage *= level_gap_multiplier(attacker.get_level(), defender.get_level());
    damage += damage * attacker.get_additional_damage_multiplier();
    damage *= hit.hit_count.max(1) as f32;

    if attacker.get_damage_category() == DamageCategory::Character
        && defender.get_damage_category() == DamageCategory::Character
    {
        damage *= PVP_DAMAGE_SCALE;
        let cap = if critical {
            hit.pvp_cap_critical
        } else {
            hit.pvp_cap
        };
        damage = damage.min(defender.get_max_health() as f32 * cap);
    }

    damage.clamp(MIN_DAMAGE, MAX_DAMAGE)
}

/// Rolls the spread and the critical, then works out the hit.
fn roll_damage(
    rng: &mut impl Rng,
    attacker: &AbilityValues,
    defender: &AbilityValues,
    hit: &DamageHit,
) -> Damage {
    let spread = rng.gen_range(DAMAGE_SPREAD_MIN..=DAMAGE_SPREAD_MAX);
    let is_critical = rng.gen_range(0.0f32..100.0) < critical_chance_of(attacker);
    let damage = hit_damage(attacker, defender, hit, spread, is_critical);
    // iROSE's hard-hit check; the server works it out but doesn't apply hit stun yet.
    let apply_hit_stun = is_critical
        || (damage * (rng.gen_range(1..=100) as f32 + 100.0)
            / (defender.get_avoid().max(0) as f32 + 40.0)
            / 14.0)
            >= 10.0;
    Damage {
        amount: damage as u32,
        is_critical,
        apply_hit_stun,
    }
}

#[derive(Default)]
struct EquipmentAbilityValue {
    pub gender: i32,
    pub birthstone: i32,
    pub job: i32,
    pub union: i32,
    pub rank: i32,
    pub fame: i32,
    pub face: i32,
    pub hair: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub concentration: i32,
    pub charm: i32,
    pub sense: i32,
    pub health: i32,
    pub mana: i32,
    pub attack: i32,
    pub defence: i32,
    pub hit: i32,
    pub resistance: i32,
    pub avoid: i32,
    pub move_speed: i32,
    pub attack_speed: i32,
    pub max_weight: i32,
    pub critical: i32,
    pub recover_health: i32,
    pub recover_mana: i32,
    pub save_mana: i32,
    pub experience: i32,
    pub level: i32,
    pub bonus_point: i32,
    pub pvp_flag: i32,
    pub team_number: i32,
    pub head_size: i32,
    pub body_size: i32,
    pub skillpoint: i32,
    pub max_health: i32,
    pub max_mana: i32,
    pub money: i32,
    pub race: i32,
    pub drop_rate: i32,
    pub fame_g: i32,
    pub fame_b: i32,
    pub current_planet: i32,
    pub stamina: i32,
    pub fuel: i32,
    pub immunity: i32,
    pub union_point1: i32,
    pub union_point2: i32,
    pub union_point3: i32,
    pub union_point4: i32,
    pub union_point5: i32,
    pub union_point6: i32,
    pub union_point7: i32,
    pub union_point8: i32,
    pub union_point9: i32,
    pub union_point10: i32,
    pub guild_number: i32,
    pub guild_score: i32,
    pub guild_position: i32,
    pub bank_free: i32,
    pub bank_addon: i32,
    pub store_skin: i32,
    pub vehicle_health: i32,
}

impl EquipmentAbilityValue {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_ability_value(&mut self, ability_type: AbilityType, value: i32) {
        match ability_type {
            AbilityType::Gender => self.gender += value,
            AbilityType::Birthstone => self.birthstone += value,
            AbilityType::Job => self.job += value,
            AbilityType::Union => self.union += value,
            AbilityType::Rank => self.rank += value,
            AbilityType::Fame => self.fame += value,
            AbilityType::Face => self.face += value,
            AbilityType::Hair => self.hair += value,
            AbilityType::Strength => self.strength += value,
            AbilityType::Dexterity => self.dexterity += value,
            AbilityType::Intelligence => self.intelligence += value,
            AbilityType::Concentration => self.concentration += value,
            AbilityType::Charm => self.charm += value,
            AbilityType::Sense => self.sense += value,
            AbilityType::Health => self.health += value,
            AbilityType::Mana => self.mana += value,
            AbilityType::Attack => self.attack += value,
            AbilityType::Defence => self.defence += value,
            AbilityType::Hit => self.hit += value,
            AbilityType::Resistance => self.resistance += value,
            AbilityType::Avoid => self.avoid += value,
            AbilityType::Speed => self.move_speed += value,
            AbilityType::AttackSpeed => self.attack_speed += value,
            AbilityType::Weight => self.max_weight += value,
            AbilityType::Critical => self.critical += value,
            AbilityType::RecoverHealth => self.recover_health += value,
            AbilityType::RecoverMana => self.recover_mana += value,
            AbilityType::SaveMana => self.save_mana += value,
            AbilityType::Experience => self.experience += value,
            AbilityType::Level => self.level += value,
            AbilityType::BonusPoint => self.bonus_point += value,
            AbilityType::PvpFlag => self.pvp_flag += value,
            AbilityType::TeamNumber => self.team_number += value,
            AbilityType::HeadSize => self.head_size += value,
            AbilityType::BodySize => self.body_size += value,
            AbilityType::Skillpoint => self.skillpoint += value,
            AbilityType::MaxHealth => self.max_health += value,
            AbilityType::MaxMana => self.max_mana += value,
            AbilityType::Money => self.money += value,
            AbilityType::Race => self.race += value,
            AbilityType::DropRate => self.drop_rate += value,
            AbilityType::FameG => self.fame_g += value,
            AbilityType::FameB => self.fame_b += value,
            AbilityType::CurrentPlanet => self.current_planet += value,
            AbilityType::Stamina => self.stamina += value,
            AbilityType::Fuel => self.fuel += value,
            AbilityType::Immunity => self.immunity += value,
            AbilityType::UnionPoint1 => self.union_point1 += value,
            AbilityType::UnionPoint2 => self.union_point2 += value,
            AbilityType::UnionPoint3 => self.union_point3 += value,
            AbilityType::UnionPoint4 => self.union_point4 += value,
            AbilityType::UnionPoint5 => self.union_point5 += value,
            AbilityType::UnionPoint6 => self.union_point6 += value,
            AbilityType::UnionPoint7 => self.union_point7 += value,
            AbilityType::UnionPoint8 => self.union_point8 += value,
            AbilityType::UnionPoint9 => self.union_point9 += value,
            AbilityType::UnionPoint10 => self.union_point10 += value,
            AbilityType::GuildNumber => self.guild_number += value,
            AbilityType::GuildScore => self.guild_score += value,
            AbilityType::GuildPosition => self.guild_position += value,
            AbilityType::BankFree => self.bank_free += value,
            AbilityType::BankAddon => self.bank_addon += value,
            AbilityType::StoreSkin => self.store_skin += value,
            AbilityType::VehicleHealth => self.vehicle_health += value,
            _ => {
                error!("Item has unimplemented ability type {:?}", ability_type)
            }
        }
    }
}

fn calculate_equipment_ability_values(
    item_database: &ItemDatabase,
    equipment: &Equipment,
) -> EquipmentAbilityValue {
    let mut result = EquipmentAbilityValue::new();

    for item in equipment.iter_equipped_items() {
        if item.is_appraised || item.has_socket {
            if let Some(item_data) = item_database.get_gem_item(item.gem as usize) {
                for (ability, value) in item_data.gem_add_ability.iter() {
                    result.add_ability_value(*ability, *value);
                }
            }
        }

        if let Some(item_data) = item_database.get_base_item(item.into()) {
            // TODO: Check item_stb.get_item_union_requirement(item_number)
            for (ability, value) in item_data.add_ability.iter() {
                result.add_ability_value(*ability, *value);
            }
        }
    }

    result
}

fn calculate_vehicle_ability_values(
    item_database: &ItemDatabase,
    equipment: &Equipment,
) -> EquipmentAbilityValue {
    let mut result = EquipmentAbilityValue::new();

    for item in equipment.iter_equipped_vehicles() {
        if item.is_appraised || item.has_socket {
            if let Some(item_data) = item_database.get_gem_item(item.gem as usize) {
                for (ability, value) in item_data.gem_add_ability.iter() {
                    result.add_ability_value(*ability, *value);
                }
            }
        }

        if let Some(item_data) = item_database.get_base_item(item.into()) {
            // TODO: Check item_stb.get_item_union_requirement(item_number)
            for (ability, value) in item_data.add_ability.iter() {
                result.add_ability_value(*ability, *value);
            }
        }
    }

    result
}

#[derive(Default)]
struct PassiveSkillAbilities {
    strength: i32,
    dexterity: i32,
    intelligence: i32,
    concentration: i32,
    charm: i32,
    sense: i32,
    attack_power_unarmed: i32,
    attack_power_one_handed: i32,
    attack_power_two_handed: i32,
    attack_power_bow: i32,
    attack_power_gun: i32,
    attack_power_staff_wand: i32,
    attack_power_auto_bow: i32,
    attack_power_katar_pair: i32,
    attack_speed_bow: i32,
    attack_speed_gun: i32,
    attack_speed_pair: i32,
    move_speed: i32,
    defence: i32,
    max_health: i32,
    max_mana: i32,
    recover_health: i32,
    recover_mana: i32,
    max_weight: i32,
    buy_skill: i32,
    sell_skill: i32,
    save_mana: i32,
    max_summons: i32,
    drop_rate: i32,
    resistance: i32,
    hit: i32,
    critical: i32,
    avoid: i32,
    shield_defence: i32,
    immunity: i32,
}

impl PassiveSkillAbilities {
    fn get_passive_weapon_attack_power(&self, weapon_type: Option<ItemWeaponType>) -> i32 {
        match weapon_type {
            Some(ItemWeaponType::OneHanded) => self.attack_power_one_handed,
            Some(ItemWeaponType::TwoHanded) => self.attack_power_two_handed,
            Some(ItemWeaponType::Bow) => self.attack_power_bow,
            Some(ItemWeaponType::Gun) | Some(ItemWeaponType::Launcher) => self.attack_power_gun,
            Some(ItemWeaponType::MagicMelee) | Some(ItemWeaponType::MagicRanged) => {
                self.attack_power_staff_wand
            }
            Some(ItemWeaponType::Crossbow) => self.attack_power_auto_bow,
            Some(ItemWeaponType::Katar) | Some(ItemWeaponType::DualWield) => {
                self.attack_power_katar_pair
            }
            None => self.attack_power_unarmed,
        }
    }
}

#[derive(Default)]
struct PassiveSkillAbilityValues {
    pub value: PassiveSkillAbilities,
    pub rate: PassiveSkillAbilities,
}

impl PassiveSkillAbilityValues {
    pub fn new() -> Self {
        Default::default()
    }

    fn get_passive_weapon_attack_power_value(&self, weapon_type: Option<ItemWeaponType>) -> i32 {
        self.value.get_passive_weapon_attack_power(weapon_type)
    }

    fn get_passive_weapon_attack_power_rate(&self, weapon_type: Option<ItemWeaponType>) -> i32 {
        self.rate.get_passive_weapon_attack_power(weapon_type)
    }

    fn add_ability(abilities: &mut PassiveSkillAbilities, ability_type: AbilityType, value: i32) {
        match ability_type {
            AbilityType::Strength => abilities.strength += value,
            AbilityType::Dexterity => abilities.dexterity += value,
            AbilityType::Intelligence => abilities.intelligence += value,
            AbilityType::Concentration => abilities.concentration += value,
            AbilityType::Charm => abilities.charm += value,
            AbilityType::Sense => abilities.sense += value,
            AbilityType::PassiveAttackPowerUnarmed => abilities.attack_power_unarmed += value,
            AbilityType::PassiveAttackPowerOneHanded => abilities.attack_power_one_handed += value,
            AbilityType::PassiveAttackPowerTwoHanded => abilities.attack_power_two_handed += value,
            AbilityType::PassiveAttackPowerBow => abilities.attack_power_bow += value,
            AbilityType::PassiveAttackPowerGun => abilities.attack_power_gun += value,
            AbilityType::PassiveAttackPowerStaffWand => abilities.attack_power_staff_wand += value,
            AbilityType::PassiveAttackPowerAutoBow => abilities.attack_power_auto_bow += value,
            AbilityType::PassiveAttackPowerKatarPair => abilities.attack_power_katar_pair += value,
            AbilityType::PassiveAttackSpeedBow => abilities.attack_speed_bow += value,
            AbilityType::PassiveAttackSpeedGun => abilities.attack_speed_gun += value,
            AbilityType::PassiveAttackSpeedPair => abilities.attack_speed_pair += value,
            AbilityType::PassiveMoveSpeed => abilities.move_speed += value,
            AbilityType::PassiveDefence => abilities.defence += value,
            AbilityType::PassiveMaxHealth => abilities.max_health += value,
            AbilityType::PassiveMaxMana => abilities.max_mana += value,
            AbilityType::PassiveRecoverHealth => abilities.recover_health += value,
            AbilityType::PassiveRecoverMana => abilities.recover_mana += value,
            AbilityType::PassiveWeight => abilities.max_weight += value,
            AbilityType::PassiveBuySkill => abilities.buy_skill += value,
            AbilityType::PassiveSellSkill => abilities.sell_skill += value,
            AbilityType::PassiveSaveMana => abilities.save_mana += value,
            AbilityType::PassiveMaxSummons => abilities.max_summons += value,
            AbilityType::PassiveDropRate => abilities.drop_rate += value,
            AbilityType::PassiveResistance => abilities.resistance += value,
            AbilityType::PassiveHit => abilities.hit += value,
            AbilityType::PassiveCritical => abilities.critical += value,
            AbilityType::PassiveAvoid => abilities.avoid += value,
            AbilityType::PassiveShieldDefence => abilities.shield_defence += value,
            AbilityType::PassiveImmunity => abilities.immunity += value,
            _ => {
                error!(
                    "Passive skill has unimplemented ability type {:?}",
                    ability_type
                )
            }
        }
    }

    pub fn add_ability_rate(&mut self, ability_type: AbilityType, value: i32) {
        Self::add_ability(&mut self.rate, ability_type, value);
    }

    pub fn add_ability_value(&mut self, ability_type: AbilityType, value: i32) {
        Self::add_ability(&mut self.value, ability_type, value);
    }
}

fn calculate_passive_skill_ability_values(
    skill_database: &SkillDatabase,
    skill_list: &SkillList,
) -> PassiveSkillAbilityValues {
    let mut result = PassiveSkillAbilityValues::new();

    if let Some(passive_skills) = skill_list.get_page(IroseSkillPageType::Passive as usize) {
        for skill_id in passive_skills.skills.iter().filter_map(|x| *x) {
            if let Some(skill_data) = skill_database.get_skill(skill_id) {
                for add_ability in skill_data.add_ability.iter().filter_map(|x| x.as_ref()) {
                    if add_ability.rate != 0 {
                        result.add_ability_rate(add_ability.ability_type, add_ability.rate);
                    } else {
                        result.add_ability_value(add_ability.ability_type, add_ability.value);
                    }
                }
            }
        }
    }

    result
}

fn calculate_run_speed(
    item_database: &ItemDatabase,
    basic_stats: &BasicStats,
    equipment_ability_values: &EquipmentAbilityValue,
    equipment: &Equipment,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> f32 {
    let mut item_speed = 20f32;

    item_speed += equipment
        .get_equipment_item(EquipmentIndex::Feet)
        .filter(|item| !item.is_broken())
        .and_then(|item| item_database.get_feet_item(item.item.item_number))
        .or_else(|| item_database.get_feet_item(0))
        .map(|item_data| item_data.move_speed)
        .unwrap_or(0) as f32;

    item_speed += equipment
        .get_equipment_item(EquipmentIndex::Back)
        .filter(|item| !item.is_broken())
        .and_then(|item| item_database.get_back_item(item.item.item_number))
        .map(|item_data| item_data.move_speed)
        .unwrap_or(0) as f32;

    let item_run_speed = item_speed * (basic_stats.dexterity as f32 + 500.0) / 100.0
        + equipment_ability_values.move_speed as f32;

    let passive_run_speed = passive_ability_values.value.move_speed as f32
        + item_run_speed * (passive_ability_values.rate.move_speed as f32 / 100.0);

    item_run_speed + passive_run_speed
}

fn calculate_vehicle_move_speed(
    item_database: &ItemDatabase,
    vehicle_ability_values: &EquipmentAbilityValue,
    equipment: &Equipment,
) -> f32 {
    let engine_item = equipment.get_vehicle_item(VehiclePartIndex::Engine);
    let leg_item = equipment.get_vehicle_item(VehiclePartIndex::Leg);

    let broken = engine_item.map_or(true, |item| item.is_broken())
        || leg_item.map_or(true, |item| item.is_broken());

    let item_speed = if broken {
        200.0
    } else {
        engine_item
            .and_then(|item| item_database.get_vehicle_item(item.item.item_number))
            .map(|item_data| item_data.move_speed)
            .unwrap_or(0) as f32
            * leg_item
                .and_then(|item| item_database.get_vehicle_item(item.item.item_number))
                .map(|item_data| item_data.move_speed)
                .unwrap_or(0) as f32
            / 10.0
    };

    // TODO: Limit speed to 300 if > max weight

    item_speed + vehicle_ability_values.move_speed as f32
}

fn calculate_max_health(
    character_info: &CharacterInfo,
    level: &Level,
    basic_stats: &BasicStats,
    equipment_ability_values: &EquipmentAbilityValue,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    let (level_add, level_multiplier, strength_multipler) = match character_info.job {
        111 => (7, 12, 2),
        121 => (-3, 14, 2),
        122 => (2, 13, 2),

        211 => (11, 10, 2),
        221 => (11, 10, 2),
        222 => (5, 11, 2),

        311 => (10, 11, 2),
        321 => (2, 13, 2),
        322 => (11, 11, 2),

        411 => (12, 10, 2),
        421 => (13, 10, 2),
        422 => (6, 11, 2),

        _ => (12, 8, 2),
    };

    let max_health = (level.level as i32 + level_add) * level_multiplier
        + basic_stats.strength * strength_multipler
        + equipment_ability_values.max_health;

    let passive_max_health = passive_ability_values.value.max_health
        + ((max_health as f32) * ((passive_ability_values.rate.max_health as f32) / 100.0)) as i32;

    max_health + passive_max_health
}

fn calculate_max_mana(
    character_info: &CharacterInfo,
    level: &Level,
    basic_stats: &BasicStats,
    equipment_ability_values: &EquipmentAbilityValue,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    let (level_add, level_multiplier, int_multipler) = match character_info.job {
        111 => (3, 4.0, 4),
        121 => (0, 4.5, 4),
        122 => (-6, 5.0, 4),

        211 => (0, 6.0, 4),
        221 => (-7, 7.0, 4),
        222 => (-4, 6.5, 4),

        311 => (4, 4.0, 4),
        321 => (4, 4.0, 4),
        322 => (0, 4.5, 4),

        411 => (3, 4.0, 4),
        421 => (3, 4.0, 4),
        422 => (0, 4.5, 4),

        _ => (4, 3.0, 4),
    };

    let max_mana = ((level.level as i32 + level_add) as f32 * level_multiplier) as i32
        + basic_stats.intelligence * int_multipler
        + equipment_ability_values.max_mana;

    let passive_max_mana = passive_ability_values.value.max_mana
        + ((max_mana as f32) * ((passive_ability_values.rate.max_mana as f32) / 100.0)) as i32;

    max_mana + passive_max_mana
}

fn calculate_attack_power(
    item_database: &ItemDatabase,
    basic_stats: &BasicStats,
    level: &Level,
    equipment_ability_values: &EquipmentAbilityValue,
    equipment: &Equipment,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    let dexterity = basic_stats.dexterity as f32;
    let concentration = basic_stats.concentration as f32;
    let strength = basic_stats.strength as f32;
    let intelligence = basic_stats.intelligence as f32;
    let level = level.level as f32;

    let get_ammo_quality = |item_database: &ItemDatabase, equipment: &Equipment, ammo_index| {
        equipment
            .get_ammo_item(ammo_index)
            .and_then(|item| item_database.get_material_item(item.item.item_number))
            .map(|item| item.item_data.quality)
            .unwrap_or(0) as f32
    };

    let weapon = equipment
        .get_equipment_item(EquipmentIndex::Weapon)
        .filter(|item| !item.is_broken())
        .and_then(|item| {
            item_database
                .get_weapon_item(item.item.item_number)
                .map(|item_data| (item, item_data))
        });

    let weapon_attack = weapon
        .map(|(weapon, weapon_data)| {
            weapon_data.attack_power as f32
                + item_database
                    .get_item_grade(weapon.grade)
                    .map(|grade| grade.attack)
                    .unwrap_or(0) as f32
        })
        .unwrap_or(0.0);

    let weapon_type =
        weapon.and_then(|(_, weapon_data)| ItemWeaponType::from(weapon_data.item_data.class));

    // Flat attack: the stats' flat parts, level, weapon, ammunition, item bonuses and passive
    // amounts. The weapon stat then adds 0.2% increased attack per point, in one pool with
    // the weapon passives' percent (wiki/rules/stats.md "Attack").
    let flat_attack = match weapon_type {
        Some(ItemWeaponType::Bow) | Some(ItemWeaponType::Crossbow) => {
            let ammo_quality = get_ammo_quality(item_database, equipment, AmmoIndex::Arrow);
            dexterity * 0.62 + strength * 0.2 + level * 0.2 + ammo_quality * 1.5 + weapon_attack + 8.0
        }
        Some(ItemWeaponType::Gun) => {
            let ammo_quality = get_ammo_quality(item_database, equipment, AmmoIndex::Bullet);
            dexterity * 0.4 + concentration * 0.5 + level * 0.2 + ammo_quality * 1.6 + weapon_attack + 8.0
        }
        Some(ItemWeaponType::Launcher) => {
            let ammo_quality = get_ammo_quality(item_database, equipment, AmmoIndex::Throw);
            strength * 0.52 + concentration * 0.5 + level * 0.2 + ammo_quality * 2.0 + weapon_attack + 12.0
        }
        Some(ItemWeaponType::OneHanded) | Some(ItemWeaponType::TwoHanded) => {
            strength * 0.75 + level * 0.2 + weapon_attack
        }
        Some(ItemWeaponType::MagicMelee) => {
            strength * 0.4 + intelligence * 0.4 + level * 0.2 + weapon_attack
        }
        Some(ItemWeaponType::MagicRanged) => intelligence * 0.6 + level * 0.2 + weapon_attack,
        Some(ItemWeaponType::DualWield) => {
            strength * 0.63 + dexterity * 0.45 + level * 0.2 + weapon_attack
        }
        Some(ItemWeaponType::Katar) => {
            strength * 0.42 + dexterity * 0.55 + level * 0.2 + weapon_attack
        }
        None => strength * 0.5 + dexterity * 0.3 + level * 0.2,
    } + equipment_ability_values.attack as f32
        + passive_ability_values.get_passive_weapon_attack_power_value(weapon_type) as f32;

    let increased_percent = weapon_stat(weapon_type, basic_stats) as f32 * INCREASED_ATTACK_PER_POINT
        + passive_ability_values.get_passive_weapon_attack_power_rate(weapon_type) as f32;

    (flat_attack * (1.0 + increased_percent / 100.0)) as i32
}

/// Percent increased attack per point of the weapon's stat.
pub const INCREASED_ATTACK_PER_POINT: f32 = 0.2;
/// Percent increased attack speed per point of the speed stat.
pub const ATTACK_SPEED_PER_POINT: f32 = 0.3;

/// Bows, crossbows, katars and dual swords are Dexterity weapons.
fn is_dexterity_weapon(weapon_type: Option<ItemWeaponType>) -> bool {
    matches!(
        weapon_type,
        Some(ItemWeaponType::Bow)
            | Some(ItemWeaponType::Crossbow)
            | Some(ItemWeaponType::Katar)
            | Some(ItemWeaponType::DualWield)
    )
}

/// The stat that adds increased attack: DEX for bows, crossbows, katars and dual swords,
/// INT for staffs and wands, CON for guns, STR for everything else (and no weapon).
pub fn weapon_stat(weapon_type: Option<ItemWeaponType>, basic_stats: &BasicStats) -> i32 {
    match weapon_type {
        _ if is_dexterity_weapon(weapon_type) => basic_stats.dexterity,
        Some(ItemWeaponType::MagicMelee) | Some(ItemWeaponType::MagicRanged) => {
            basic_stats.intelligence
        }
        Some(ItemWeaponType::Gun) => basic_stats.concentration,
        _ => basic_stats.strength,
    }
}

/// The stat that adds attack speed: STR for Dexterity weapons, DEX for everything else.
pub fn attack_speed_stat(weapon_type: Option<ItemWeaponType>, basic_stats: &BasicStats) -> i32 {
    if is_dexterity_weapon(weapon_type) {
        basic_stats.strength
    } else {
        basic_stats.dexterity
    }
}

/// Percent attack speed the stat adds with this weapon (for the character window).
pub fn attack_speed_stat_percent(weapon_type: Option<ItemWeaponType>, basic_stats: &BasicStats) -> f32 {
    attack_speed_stat(weapon_type, basic_stats).max(0) as f32 * ATTACK_SPEED_PER_POINT
}

fn calculate_vehicle_attack_power(
    item_database: &ItemDatabase,
    basic_stats: &BasicStats,
    level: &Level,
    vehicle_ability_values: &EquipmentAbilityValue,
    equipment: &Equipment,
) -> i32 {
    let concentration = basic_stats.concentration as f32;
    let level = level.level as f32;

    let arms_attack_power = equipment
        .get_vehicle_item(VehiclePartIndex::Arms)
        .filter(|item| !item.is_broken())
        .and_then(|item| {
            item_database
                .get_vehicle_item(item.item.item_number)
                .map(|item_data| item_data.attack_power as f32)
        })
        .unwrap_or(0.0);

    let attack_power =
        level * 3.0 + concentration + arms_attack_power + vehicle_ability_values.attack as f32;

    attack_power as i32
}

fn calculate_attack_speed(
    item_database: &ItemDatabase,
    basic_stats: &BasicStats,
    equipment: &Equipment,
    equipment_ability_values: &EquipmentAbilityValue,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> (i32, i32) {
    let (weapon_attack_speed, weapon_item_class) = item_database
        .get_weapon_item(
            equipment
                .get_equipment_item(EquipmentIndex::Weapon)
                .map(|item| item.item.item_number)
                .unwrap_or(0),
        )
        .map(|weapon| (weapon.attack_speed, Some(weapon.item_data.class)))
        .unwrap_or((0, None));

    let attack_speed = 1500.0 / (weapon_attack_speed + 5) as f32;

    let (passive_value, passive_rate) = match weapon_item_class {
        Some(ItemClass::Bow) => (
            passive_ability_values.value.attack_speed_bow,
            passive_ability_values.rate.attack_speed_bow,
        ),
        Some(ItemClass::Gun) | Some(ItemClass::Launcher) => (
            passive_ability_values.value.attack_speed_gun,
            passive_ability_values.rate.attack_speed_gun,
        ),
        Some(ItemClass::Katar) | Some(ItemClass::DualSwords) => (
            passive_ability_values.value.attack_speed_pair,
            passive_ability_values.rate.attack_speed_pair,
        ),
        _ => (0, 0),
    };

    // The speed stat's percent and the passives' percent add into one pool.
    let weapon_type = weapon_item_class.and_then(ItemWeaponType::from);
    let stat_percent = attack_speed_stat_percent(weapon_type, basic_stats);
    let passive_attack_speed = passive_value as f32 + attack_speed * (passive_rate as f32 / 100.0);

    (
        (attack_speed * (1.0 + (passive_rate as f32 + stat_percent) / 100.0)
            + passive_value as f32
            + equipment_ability_values.attack_speed as f32) as i32,
        passive_attack_speed as i32,
    )
}

fn calculate_vehicle_attack_speed(
    item_database: &ItemDatabase,
    equipment: &Equipment,
    vehicle_ability_values: &EquipmentAbilityValue,
) -> i32 {
    let arms_attack_speed = equipment
        .get_vehicle_item(VehiclePartIndex::Arms)
        .filter(|item| !item.is_broken())
        .and_then(|item| {
            item_database
                .get_vehicle_item(item.item.item_number)
                .map(|item_data| item_data.attack_speed as f32)
        })
        .unwrap_or(0.0);

    let attack_speed = 1500.0 / (arms_attack_speed + 5.0);

    (attack_speed + vehicle_ability_values.attack_speed as f32) as i32
}

fn calculate_attack_range(item_database: &ItemDatabase, equipment: &Equipment) -> i32 {
    let weapon_attack_range = item_database
        .get_weapon_item(
            equipment
                .get_equipment_item(EquipmentIndex::Weapon)
                .map(|item| item.item.item_number)
                .unwrap_or(0),
        )
        .map(|weapon| weapon.attack_range)
        .unwrap_or(70);

    let scale = 1.0;

    weapon_attack_range + (scale * 120.0) as i32
}

fn calculate_vehicle_attack_range(item_database: &ItemDatabase, equipment: &Equipment) -> i32 {
    let arms_attack_range = equipment
        .get_vehicle_item(VehiclePartIndex::Arms)
        .and_then(|item| {
            item_database
                .get_vehicle_item(item.item.item_number)
                .map(|item_data| item_data.attack_range)
        })
        .unwrap_or(0);

    let scale = 1.0;

    arms_attack_range + (scale * 120.0) as i32
}

fn calculate_hit(
    item_database: &ItemDatabase,
    basic_stats: &BasicStats,
    equipment_ability_values: &EquipmentAbilityValue,
    equipment: &Equipment,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    let concentration = basic_stats.concentration as f32;

    let hit = if let Some((weapon, weapon_data)) = equipment
        .get_equipment_item(EquipmentIndex::Weapon)
        .filter(|item| !item.is_broken())
        .and_then(|item| {
            item_database
                .get_weapon_item(item.item.item_number)
                .map(|item_data| (item, item_data))
        }) {
        let weapon_quality = weapon_data.item_data.quality as f32;
        let weapon_durability = weapon.durability as f32;
        let grade_hit = item_database
            .get_item_grade(weapon.grade)
            .map(|grade| grade.hit)
            .unwrap_or(0) as f32;

        (concentration + 10.0) * 0.8 + weapon_quality * 0.6 + grade_hit + weapon_durability * 0.8
    } else {
        (concentration + 10.0) * 0.5 + 15.0
    } + equipment_ability_values.hit as f32;

    let passive_hit_rate = passive_ability_values.rate.hit as f32 / 100.0;
    let passive_hit = passive_ability_values.value.hit as f32 + (hit * passive_hit_rate);

    (hit + passive_hit) as i32
}

fn calculate_vehicle_hit(
    item_database: &ItemDatabase,
    basic_stats: &BasicStats,
    level: &Level,
    vehicle_ability_values: &EquipmentAbilityValue,
    equipment: &Equipment,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    let concentration = basic_stats.concentration as f32;
    let level = level.level as f32;

    let hit = if let Some(arms_item_quality) = equipment
        .get_vehicle_item(VehiclePartIndex::Arms)
        .filter(|item| !item.is_broken())
        .and_then(|item| {
            item_database
                .get_vehicle_item(item.item.item_number)
                .map(|item_data| item_data.item_data.quality as f32)
        }) {
        (concentration + 10.0) * 0.8 + level * 0.5 + arms_item_quality * 1.2
    } else {
        0.0
    } + vehicle_ability_values.hit as f32;

    let passive_hit_rate = passive_ability_values.rate.hit as f32 / 100.0;
    let passive_hit = passive_ability_values.value.hit as f32 + (hit * passive_hit_rate);

    (hit + passive_hit) as i32
}

fn calculate_defence(
    item_database: &ItemDatabase,
    basic_stats: &BasicStats,
    level: &Level,
    equipment_ability_values: &EquipmentAbilityValue,
    equipment: &Equipment,
    passive_ability_values: &PassiveSkillAbilityValues,
    for_vehicle: bool,
) -> i32 {
    let mut item_defence = 0;

    for item in equipment.iter_equipped_items().filter(|item| item.life > 0) {
        if let Some(item_data) = item_database.get_base_item(item.into()) {
            if item_data.defence > 0 {
                let grade_defence = item_database
                    .get_item_grade(item.grade)
                    .map(|grade| grade.defence)
                    .unwrap_or(0);
                item_defence += item_data.defence as i32 + grade_defence;
            }
        }
    }

    if for_vehicle {
        for item in equipment
            .iter_equipped_vehicles()
            .filter(|item| item.life > 0)
        {
            item_defence += item_database
                .get_item_grade(item.grade)
                .map(|grade| grade.defence)
                .unwrap_or(0);
        }
    }

    let strength = basic_stats.strength as f32;
    let level = level.level as f32;
    let defence = item_defence as f32
        + (strength + 5.0) * 0.35
        + (level + 15.0) * 0.7
        + equipment_ability_values.defence as f32;

    let passive_defence_rate = passive_ability_values.rate.defence as f32 / 100.0;
    let passive_defence =
        passive_ability_values.value.defence as f32 + (defence * passive_defence_rate);

    let mut defence = (defence + passive_defence) as i32;

    if let Some(offhand_item) = equipment.get_equipment_item(EquipmentIndex::SubWeapon) {
        if let Some(ItemClass::Shield) = item_database
            .get_base_item(offhand_item.into())
            .map(|x| x.class)
        {
            let passive_shield_defence_rate =
                passive_ability_values.rate.shield_defence as f32 / 100.0;
            let passive_shield_defence = passive_ability_values.value.shield_defence as f32
                + (defence as f32 * passive_shield_defence_rate);
            defence += passive_shield_defence as i32;
        }
    }

    defence
}

fn calculate_resistance(
    item_database: &ItemDatabase,
    basic_stats: &BasicStats,
    level: &Level,
    equipment_ability_values: &EquipmentAbilityValue,
    equipment: &Equipment,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    let mut item_resistance = 0;

    for item in equipment.iter_equipped_items().filter(|item| item.life > 0) {
        if let Some(item_data) = item_database.get_base_item(item.into()) {
            if item_data.resistance > 0 {
                let grade_resistance = item_database
                    .get_item_grade(item.grade)
                    .map(|grade| grade.resistance)
                    .unwrap_or(0);
                item_resistance += item_data.resistance as i32 + grade_resistance;
            }
        }
    }

    let intelligence = basic_stats.intelligence as f32;
    let level = level.level as f32;
    let resistance = item_resistance as f32
        + (intelligence + 5.0) * 0.6
        + (level + 15.0) * 0.8
        + equipment_ability_values.resistance as f32;

    let passive_resistance_rate = passive_ability_values.rate.resistance as f32 / 100.0;
    let passive_resistance =
        passive_ability_values.value.resistance as f32 + (resistance * passive_resistance_rate);

    (resistance + passive_resistance) as i32
}

fn calculate_critical(
    basic_stats: &BasicStats,
    equipment_ability_values: &EquipmentAbilityValue,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    let concentration = basic_stats.concentration as f32;
    let sense = basic_stats.sense as f32;
    // Critical rating: one per point of CON and of SEN (wiki/rules/combat.md "Critical hits").
    let critical = concentration + sense + equipment_ability_values.critical as f32;

    let passive_critical_rate = passive_ability_values.rate.critical as f32 / 100.0;
    let passive_critical =
        passive_ability_values.value.critical as f32 + (critical * passive_critical_rate);

    (critical + passive_critical) as i32
}

fn calculate_vehicle_critical(
    basic_stats: &BasicStats,
    level: &Level,
    vehicle_ability_values: &EquipmentAbilityValue,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    let level = level.level as f32;
    let sense = basic_stats.sense as f32;
    let critical = sense * 0.8 + level * 0.3 + vehicle_ability_values.critical as f32;

    let passive_critical_rate = passive_ability_values.rate.critical as f32 / 100.0;
    let passive_critical =
        passive_ability_values.value.critical as f32 + (critical * passive_critical_rate);

    (critical + passive_critical) as i32
}

fn calculate_avoid(
    item_database: &ItemDatabase,
    basic_stats: &BasicStats,
    level: &Level,
    equipment: &Equipment,
    equipment_ability_values: &EquipmentAbilityValue,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    const AVOID_DURABILITY_ITEMS: [EquipmentIndex; 6] = [
        EquipmentIndex::Head,
        EquipmentIndex::Body,
        EquipmentIndex::Back,
        EquipmentIndex::Hands,
        EquipmentIndex::Feet,
        EquipmentIndex::SubWeapon,
    ];

    // Get total durability for specific set of equipment
    let equipment_durability: i32 = AVOID_DURABILITY_ITEMS
        .iter()
        .filter_map(|x| equipment.get_equipment_item(*x))
        .filter(|x| x.life > 0)
        .map(|item| item.durability as i32)
        .sum();

    // Count grade on all items which have defence stat > 0
    let mut equipment_total_grade = 0;
    for item in equipment.iter_equipped_items().filter(|item| item.life > 0) {
        if let Some(item_data) = item_database.get_base_item(item.into()) {
            if item_data.defence > 0 {
                equipment_total_grade += item.grade as i32;
            }
        }
    }

    let dexterity = basic_stats.dexterity as f32;
    let level = level.level as f32;
    let avoid = (dexterity * 1.9 + level * 0.3 + 10.0) * 0.4
        + (equipment_durability as f32) * 0.3
        + equipment_total_grade as f32
        + equipment_ability_values.avoid as f32;

    let passive_avoid_rate = passive_ability_values.rate.avoid as f32 / 100.0;
    let passive_avoid = passive_ability_values.value.avoid as f32 + (avoid * passive_avoid_rate);

    (avoid + passive_avoid) as i32
}

fn calculate_vehicle_avoid(
    basic_stats: &BasicStats,
    level: &Level,
    vehicle_ability_values: &EquipmentAbilityValue,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    let dexterity = basic_stats.dexterity as f32;
    let level = level.level as f32;
    let avoid = (dexterity + 10.0) * 0.8 + level * 0.5 + vehicle_ability_values.avoid as f32;

    let passive_avoid_rate = passive_ability_values.rate.avoid as f32 / 100.0;
    let passive_avoid = passive_ability_values.value.avoid as f32 + (avoid * passive_avoid_rate);

    (avoid + passive_avoid) as i32
}

fn calculate_drop_rate(
    equipment_ability_values: &EquipmentAbilityValue,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    let drop_rate = equipment_ability_values.drop_rate as f32;
    let passive_drop_rate = passive_ability_values.value.drop_rate as f32
        + (drop_rate * passive_ability_values.rate.drop_rate as f32 / 100.0);

    (drop_rate + passive_drop_rate) as i32
}

fn calculate_save_mana(
    equipment_ability_values: &EquipmentAbilityValue,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    let save_mana = equipment_ability_values.save_mana as f32;
    let passive_save_mana = passive_ability_values.value.save_mana as f32
        + (save_mana * passive_ability_values.rate.save_mana as f32 / 100.0);

    (save_mana + passive_save_mana) as i32
}

fn calculate_max_weight(
    item_database: &ItemDatabase,
    level: &Level,
    basic_stats: &BasicStats,
    equipment: &Equipment,
    equipment_ability_values: &EquipmentAbilityValue,
    passive_ability_values: &PassiveSkillAbilityValues,
) -> i32 {
    let mut max_weight = 1100
        + level.level as i32 * 5
        + basic_stats.strength * 6
        + equipment_ability_values.max_weight;

    // If user has a bag equipped, apply max weight passives
    if equipment
        .get_equipment_item(EquipmentIndex::Back)
        .filter(|x| x.life > 0)
        .and_then(|x| item_database.get_base_item(x.item))
        .filter(|x| matches!(x.class, ItemClass::Bag))
        .is_some()
    {
        max_weight += (passive_ability_values.value.max_weight as f32
            + max_weight as f32 * passive_ability_values.rate.max_weight as f32 / 100.0)
            as i32;
    }

    max_weight
}

#[cfg(test)]
mod hit_chance_tests {
    use super::hit_chance_from;

    /// Typical player (hit 63 + 2.2L) against a median monster (avoid 20 + 1.35L).
    fn typical(level: i32, monster_above: i32) -> f32 {
        let monster = level + monster_above;
        let hit = (63.0 + 2.2 * level as f32) as i32;
        let avoid = (20.0 + 1.35 * monster as f32) as i32;
        hit_chance_from(hit, avoid, -monster_above)
    }

    #[test]
    fn same_level_is_about_87_percent() {
        let c = typical(30, 0);
        assert!((86.0..=89.0).contains(&c), "{c}");
    }

    #[test]
    fn twenty_levels_above_is_about_69_percent() {
        let c = typical(30, 20);
        assert!((66.0..=72.0).contains(&c), "{c}");
    }

    #[test]
    fn level_difference_counts_ten_levels_at_most() {
        assert_eq!(hit_chance_from(100, 100, 10), hit_chance_from(100, 100, 50));
        assert_eq!(
            hit_chance_from(100, 100, -10),
            hit_chance_from(100, 100, -50)
        );
        assert!(
            (hit_chance_from(100, 100, 0) - hit_chance_from(100, 100, -10) - 15.0).abs() < 0.01
        );
    }

    #[test]
    fn clamped_to_5_and_95() {
        assert_eq!(hit_chance_from(1000, 0, 10), 95.0);
        assert_eq!(hit_chance_from(0, 1000, -10), 5.0);
        assert_eq!(hit_chance_from(0, 0, 0), 95.0);
    }

    #[test]
    fn avoid_matters() {
        assert!(hit_chance_from(100, 200, 0) < hit_chance_from(100, 100, 0) - 5.0);
    }
}

#[cfg(test)]
mod damage_tests {
    use super::*;
    use rose_game_common::components::AbilityValuesAdjust;

    fn adjust() -> AbilityValuesAdjust {
        AbilityValuesAdjust {
            additional_damage_multiplier: 0.0,
            attack_speed: 0,
            attack_power: 0,
            avoid: 0,
            critical: 0,
            defence: 0,
            hit: 0,
            resistance: 0,
            max_health: 0,
            max_mana: 0,
            run_speed: 0.0,
        }
    }

    struct Who {
        player: bool,
        level: i32,
        attack: i32,
        defence: i32,
        resistance: i32,
        critical: i32,
        sense: i32,
        intelligence: i32,
        max_health: i32,
    }

    fn values(w: Who) -> AbilityValues {
        AbilityValues {
            is_driving: false,
            damage_category: if w.player {
                DamageCategory::Character
            } else {
                DamageCategory::Npc
            },
            level: w.level,
            walk_speed: 200.0,
            run_speed: 400.0,
            vehicle_move_speed: 0.0,
            strength: 0,
            dexterity: 0,
            intelligence: w.intelligence,
            concentration: 0,
            charm: 0,
            sense: w.sense,
            max_health: w.max_health,
            max_mana: 100,
            additional_health_recovery: 0,
            additional_mana_recovery: 0,
            attack_damage_type: DamageType::Physical,
            attack_power: w.attack,
            attack_speed: 100,
            passive_attack_speed: 0,
            attack_range: 100,
            hit: 100,
            defence: w.defence,
            resistance: w.resistance,
            critical: w.critical,
            avoid: 50,
            vehicle_attack_power: 0,
            vehicle_attack_range: 0,
            vehicle_attack_speed: 0,
            vehicle_hit: 0,
            vehicle_defence: 0,
            vehicle_critical: 0,
            vehicle_avoid: 0,
            max_damage_sources: 0,
            drop_rate: 0,
            max_weight: 0,
            summon_owner_level: None,
            summon_skill_level: None,
            adjust: adjust(),
            npc_store_buy_rate: 0,
            npc_store_sell_rate: 0,
            save_mana: 0,
        }
    }

    fn player(level: i32, attack: i32, critical: i32, sense: i32) -> AbilityValues {
        values(Who {
            player: true,
            level,
            attack,
            defence: 100,
            resistance: 80,
            critical,
            sense,
            intelligence: 15,
            max_health: 1000,
        })
    }

    fn monster(level: i32, attack: i32, defence: i32, resistance: i32) -> AbilityValues {
        values(Who {
            player: false,
            level,
            attack,
            defence,
            resistance,
            critical: 0,
            sense: level,
            intelligence: level,
            max_health: 2000,
        })
    }

    fn plain(magic: bool) -> DamageHit {
        DamageHit {
            base_attack: 0.0,
            power: 0,
            magic,
            hit_count: 1,
            pvp_cap: PVP_CAP_NORMAL,
            pvp_cap_critical: PVP_CAP_CRITICAL,
        }
    }

    fn hit_of(a: &AbilityValues, power: i32, magic: bool) -> DamageHit {
        DamageHit {
            base_attack: a.get_attack_power() as f32,
            power,
            ..plain(magic)
        }
    }

    #[test]
    fn worked_example_level_60() {
        // wiki/rules/combat.md: attack 180 against defence 160: raw 216,
        // 160 / (160 + 162 + 80) = 39.8% off, 130 damage.
        let a = player(60, 180, 0, 0);
        let d = monster(60, 220, 160, 120);
        let damage = hit_damage(&a, &d, &hit_of(&a, 0, false), 1.0, false);
        assert_eq!(damage as u32, 130, "{damage}");
        // Critical with SEN 50: 130.0 × 2.1 = 273.
        let a = player(60, 180, 0, 50);
        let crit = hit_damage(&a, &d, &hit_of(&a, 0, false), 1.0, true);
        assert_eq!(crit as u32, 273, "{crit}");
    }

    #[test]
    fn reduction_is_capped_at_75_percent() {
        assert_eq!(damage_reduction(100_000.0, 10.0), 0.75);
        assert_eq!(damage_reduction(0.0, 100.0), 0.0);
        // A bigger hit loses a smaller share.
        assert!(damage_reduction(200.0, 400.0) < damage_reduction(200.0, 100.0));
    }

    #[test]
    fn magic_uses_resistance_only() {
        let a = player(60, 180, 0, 0);
        let thick = monster(60, 220, 1000, 120);
        let thin = monster(60, 220, 0, 120);
        let h = hit_of(&a, 0, true);
        assert_eq!(hit_damage(&a, &thick, &h, 1.0, false), hit_damage(&a, &thin, &h, 1.0, false));
    }

    #[test]
    fn critical_chance_curve() {
        assert_eq!(critical_chance(0, 60), 5.0);
        // Level 60, rating 100: 5 + 45 × 100 / 250 = 23%.
        assert!((critical_chance(100, 60) - 23.0).abs() < 0.01);
        // The same rating counts for less at a higher level.
        assert!(critical_chance(100, 120) < critical_chance(100, 60));
        assert!(critical_chance(1_000_000, 1) <= CRITICAL_CHANCE_MAX);
        let m = monster(100, 400, 300, 200);
        assert_eq!(critical_chance_of(&m), 5.0);
        assert_eq!(critical_damage_percent(&m), 150.0);
        assert_eq!(critical_damage_percent(&player(60, 100, 0, 50)), 210.0);
    }

    #[test]
    fn level_gap_counts_ten_levels_at_most() {
        assert_eq!(level_gap_multiplier(60, 60), 1.0);
        assert!((level_gap_multiplier(65, 60) - 1.1).abs() < 1e-6);
        assert_eq!(level_gap_multiplier(80, 60), level_gap_multiplier(70, 60));
        assert_eq!(level_gap_multiplier(30, 60), level_gap_multiplier(50, 60));
        assert!((level_gap_multiplier(50, 60) - 0.8).abs() < 1e-6);
    }

    #[test]
    fn spread_is_90_to_110_percent() {
        let a = player(60, 180, 0, 0);
        let d = monster(60, 220, 0, 0);
        let h = hit_of(&a, 0, false);
        let low = hit_damage(&a, &d, &h, DAMAGE_SPREAD_MIN, false);
        let high = hit_damage(&a, &d, &h, DAMAGE_SPREAD_MAX, false);
        assert!((high / low - 1.1 / 0.9).abs() < 1e-3);
    }

    #[test]
    fn pvp_is_halved_and_capped() {
        let a = player(60, 180, 0, 0);
        let d = player(60, 180, 0, 0);
        let m = values(Who {
            player: false,
            level: 60,
            attack: 0,
            defence: 100,
            resistance: 80,
            critical: 0,
            sense: 0,
            intelligence: 0,
            max_health: 1000,
        });
        let h = hit_of(&a, 0, false);
        let pvp = hit_damage(&a, &d, &h, 1.0, false);
        let pve = hit_damage(&a, &m, &h, 1.0, false);
        assert!((pvp - pve * 0.5).abs() < 0.01, "{pvp} {pve}");
        let big = hit_of(&player(60, 5000, 0, 0), 0, false);
        assert_eq!(hit_damage(&a, &d, &big, 1.0, false), 250.0);
        assert_eq!(hit_damage(&a, &d, &big, 1.0, true), 350.0);
    }

    #[test]
    fn skill_power_adds_percent_of_attack() {
        let a = player(60, 200, 0, 0);
        let d = monster(60, 220, 0, 0);
        let plain = hit_damage(&a, &d, &hit_of(&a, 0, false), 1.0, false);
        let skill = hit_damage(&a, &d, &hit_of(&a, 100, false), 1.0, false);
        assert!((skill / plain - 2.0).abs() < 1e-3);
        assert_eq!(magic_attack(&a), 200.0 * 0.8 + 15.0 * 1.2);
    }

    #[test]
    fn damage_is_kept_between_1_and_2047() {
        let weak = player(1, 1, 0, 0);
        let d = monster(1, 1, 100_000, 0);
        assert_eq!(hit_damage(&weak, &d, &hit_of(&weak, 0, false), 1.0, false), 1.0);
        let strong = player(200, 100_000, 0, 0);
        assert_eq!(hit_damage(&strong, &d, &hit_of(&strong, 0, false), 1.0, false), 2047.0);
    }

    /// Basic stats a level buys when its stat points go in these shares (STR, DEX, CON, SEN).
    fn buy(level: i32, shares: [f32; 4]) -> [i32; 4] {
        let total: i32 = (2..=level).map(|l| (l as f32 * 0.8) as i32 + 10).sum();
        let mut stats = [15, 15, 15, 10];
        for (stat, share) in stats.iter_mut().zip(shares) {
            let mut budget = total as f32 * share;
            while *stat < 300 && (*stat / 5) as f32 <= budget {
                budget -= (*stat / 5) as f32;
                *stat += 1;
            }
        }
        stats
    }

    /// Damage per second of a level 60 sword user against a median level 60 monster, with
    /// `share` of the stat points in STR and the rest split over DEX, CON and SEN.
    fn sword_dps(share: f32) -> f32 {
        let rest = (1.0 - share) / 3.0;
        let [str_, dex, con, sen] = buy(60, [share, rest, rest, rest]);
        let level = 60;
        // Median level 60 sword (attack 62, speed 9, quality 60, durability 52).
        let attack = (str_ as f32 * 0.75 + level as f32 * 0.2 + 62.0)
            * (1.0 + str_ as f32 * INCREASED_ATTACK_PER_POINT / 100.0);
        let speed = 1500.0 / 14.0 * (1.0 + dex as f32 * ATTACK_SPEED_PER_POINT / 100.0);
        let hit = ((con + 10) as f32 * 0.8 + 60.0 * 0.6 + 52.0 * 0.8) as i32;
        let a = player(level, attack as i32, con + sen, sen);
        let d = monster(60, 221, 162, 125);
        let chance = critical_chance_of(&a) / 100.0;
        let h = hit_of(&a, 0, false);
        let per_hit: f32 = [0.9, 0.95, 1.0, 1.05, 1.1]
            .iter()
            .map(|&s| {
                hit_damage(&a, &d, &h, s, false) * (1.0 - chance) + hit_damage(&a, &d, &h, s, true) * chance
            })
            .sum::<f32>()
            / 5.0;
        per_hit * hit_chance_from(hit, 87, 0) / 100.0 * speed / 100.0
    }

    #[test]
    fn putting_every_point_in_the_weapon_stat_is_not_best() {
        let shares: Vec<f32> = (0..=10).map(|i| i as f32 / 10.0).collect();
        let dps: Vec<f32> = shares.iter().map(|&s| sword_dps(s)).collect();
        let best = dps.iter().cloned().fold(0.0, f32::max);
        let best_share = shares[dps.iter().position(|&d| d == best).unwrap()];
        assert!((0.4..=0.8).contains(&best_share), "best at {best_share}");
        assert!(dps[10] < best * 0.93, "all in {} best {}", dps[10], best);
    }

    #[test]
    fn swings_to_kill_stay_in_a_band() {
        // Median monsters (HP, defence) and a typical sword user's attack, from the game data.
        for (level, hp, defence, attack) in
            [(10, 250.0, 45, 41), (30, 765.0, 83, 87), (60, 2160.0, 162, 184), (100, 4100.0, 303, 343)]
        {
            let a = player(level, attack, 0, 0);
            let d = monster(level, 0, defence, 0);
            let per_hit = hit_damage(&a, &d, &hit_of(&a, 0, false), 1.0, false);
            let swings = hp / per_hit;
            assert!((5.0..=22.0).contains(&swings), "level {level}: {swings}");
        }
    }
}
