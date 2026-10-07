//! Characters: creation, ability values, experience and levels, stat points.
//! Rules come from rose-offline (rose-game-irose's ability value calculator and the
//! experience_points_system), with its components stored as JSON on the player row.

use glam::Vec3;
use rose_data::{
    CharacterMotionAction, EquipmentItem, ItemClass, ItemReference, ItemType, StackableItem,
    ZoneId,
};
use rose_data_irose::{IroseSkillPageType, SKILL_PAGE_SIZE};
use rose_game_common::components::{
    AbilityValues, BasicStatType, BasicStats, CharacterGender, CharacterInfo, Equipment, Hotbar,
    Inventory, Level, QuestState, SkillList, SkillPage, StatusEffects, UnionMembership,
};
use rose_game_data::GameData;
use serde::{de::DeserializeOwned, Serialize};
use spacetimedb::{Identity, ReducerContext, Table};

use crate::{combat, game_data::game, my_player, player, stats, Player, Stats};

const SHORT_SWORD: usize = 2;
const SHORT_BOW: usize = 202;
const STARTING_ARROWS: u32 = 999;

fn to_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap_or_default()
}

fn from_json<T: DeserializeOwned + Default>(json: &str) -> T {
    serde_json::from_str(json).unwrap_or_default()
}

impl Player {
    pub fn gender(&self) -> CharacterGender {
        if self.gender == 1 {
            CharacterGender::Female
        } else {
            CharacterGender::Male
        }
    }

    pub fn basic_stats(&self) -> BasicStats {
        BasicStats {
            strength: self.strength,
            dexterity: self.dexterity,
            intelligence: self.intelligence,
            concentration: self.concentration,
            charm: self.charm,
            sense: self.sense,
        }
    }

    pub fn set_basic_stats(&mut self, s: &BasicStats) {
        self.strength = s.strength;
        self.dexterity = s.dexterity;
        self.intelligence = s.intelligence;
        self.concentration = s.concentration;
        self.charm = s.charm;
        self.sense = s.sense;
    }

    pub fn equipment(&self) -> Equipment {
        from_json(&self.equipment)
    }

    pub fn set_equipment(&mut self, equipment: &Equipment) {
        self.equipment = to_json(equipment);
    }

    pub fn inventory(&self) -> Inventory {
        from_json(&self.inventory)
    }

    pub fn set_inventory(&mut self, inventory: &Inventory) {
        self.inventory = to_json(inventory);
    }

    pub fn skill_list(&self) -> SkillList {
        from_json(&self.skill_list)
    }

    pub fn set_skill_list(&mut self, skill_list: &SkillList) {
        self.skill_list = to_json(skill_list);
    }

    pub fn quest_state(&self) -> QuestState {
        from_json(&self.quest_state)
    }

    pub fn set_quest_state(&mut self, quest_state: &QuestState) {
        self.quest_state = to_json(quest_state);
    }

    pub fn union_membership(&self) -> UnionMembership {
        from_json(&self.union_membership)
    }

    pub fn set_union_membership(&mut self, union_membership: &UnionMembership) {
        self.union_membership = to_json(union_membership);
    }

    pub fn character_info(&self) -> CharacterInfo {
        CharacterInfo {
            name: self.name.clone(),
            gender: self.gender(),
            race: 0,
            birth_stone: 0,
            job: self.job,
            face: self.face,
            hair: self.hair,
            rank: 0,
            fame: 0,
            fame_b: 0,
            fame_g: 0,
            revive_zone_id: ZoneId::new(self.zone_id.max(1)).unwrap(),
            revive_position: Vec3::ZERO,
            unique_id: 0,
        }
    }
}

/// A new character from INIT_AVATAR: starting stats, clothes, items and basic skills.
pub fn new_player(game: &GameData, identity: Identity, name: String, gender: u8, zone: (u16, f32, f32)) -> Player {
    let start = &game.character_creator.genders[if gender == 1 { 1 } else { 0 }];
    let items = &game.items;

    let mut equipment = Equipment::default();
    for item in &start.equipped_items {
        if let Some(item) = items.get_base_item(*item).and_then(EquipmentItem::from_item_data) {
            equipment.equip_item(item).ok();
        }
    }
    let mut inventory = Inventory::default();
    for item in &start.inventory_equipment {
        if let Some(item) = items.get_base_item(*item).and_then(EquipmentItem::from_item_data) {
            inventory.try_add_item(item.into()).ok();
        }
    }
    for (item, quantity) in start.inventory_consumables.iter().chain(&start.inventory_materials) {
        if let Some(item) = items.get_base_item(*item).and_then(|d| StackableItem::from_item_data(d, *quantity)) {
            inventory.try_add_item(item.into()).ok();
        }
    }
    let mut skill_list = SkillList {
        pages: vec![
            SkillPage::new(IroseSkillPageType::Basic as usize, SKILL_PAGE_SIZE),
            SkillPage::new(IroseSkillPageType::Active as usize, SKILL_PAGE_SIZE),
            SkillPage::new(IroseSkillPageType::Passive as usize, SKILL_PAGE_SIZE),
            SkillPage::new(IroseSkillPageType::Clan as usize, SKILL_PAGE_SIZE),
        ],
    };
    for skill_id in &game.character_creator.skills {
        if let Some(skill) = game.skills.get_skill(*skill_id) {
            skill_list.add_skill(skill);
        }
    }

    let mut player = Player {
        identity,
        entity_id: None,
        name,
        online: true,
        connection: None,
        zone_id: zone.0,
        last_x: zone.1,
        last_y: zone.2,
        last_hp: 0,
        last_mp: 0,
        gender,
        face: 1,
        hair: 0,
        job: 0,
        level: 1,
        xp: 0,
        stat_points: 0,
        skill_points: 0,
        strength: 0,
        dexterity: 0,
        intelligence: 0,
        concentration: 0,
        charm: 0,
        sense: 0,
        equipment: String::new(),
        inventory: String::new(),
        skill_list: to_json(&skill_list),
        quest_state: to_json(&QuestState::default()),
        hotbar: to_json(&Hotbar::default()),
        union_membership: to_json(&UnionMembership::default()),
    };
    player.set_basic_stats(&start.basic_stats);
    starter_kit(game, &mut equipment, &mut inventory);
    player.set_equipment(&equipment);
    player.set_inventory(&inventory);
    player
}

/// Our addition to INIT_AVATAR, which has no weapon: a Short Sword in hand, and a Short Bow
/// with arrows in the bag to try ranged combat.
fn starter_kit(game: &GameData, equipment: &mut Equipment, inventory: &mut Inventory) {
    let weapon = |number| {
        game.items
            .get_base_item(ItemReference::new(ItemType::Weapon, number))
            .and_then(EquipmentItem::from_item_data)
    };
    if let Some(sword) = weapon(SHORT_SWORD) {
        equipment.equip_item(sword).ok();
    }
    if let Some(bow) = weapon(SHORT_BOW) {
        inventory.try_add_item(bow.into()).ok();
    }
    let arrows = game
        .items
        .iter_items(ItemType::Material)
        .filter_map(|item| game.items.get_base_item(item))
        .find(|item| item.class == ItemClass::Arrow)
        .and_then(|item| StackableItem::from_item_data(item, STARTING_ARROWS));
    if let Some(arrows) = arrows {
        inventory.try_add_item(arrows.into()).ok();
    }
}

/// A player's ability values, with the status effects on their character.
pub fn ability_values(ctx: &ReducerContext, game: &GameData, player: &Player) -> AbilityValues {
    let effects = player.entity_id.map_or_else(StatusEffects::default, |id| crate::skills::status_effects(ctx, id));
    game.ability_value_calculator.calculate(
        &player.character_info(),
        &Level::new(player.level),
        &player.equipment(),
        &player.basic_stats(),
        &player.skill_list(),
        &effects,
    )
}

/// Stats row for a player's entity: ability values plus the attack motion of their weapon.
pub fn player_stats(ctx: &ReducerContext, game: &GameData, entity_id: u64, player: &Player) -> Stats {
    let av = ability_values(ctx, game, player);
    let weapon_motion = player
        .equipment()
        .get_equipment_item(rose_data::EquipmentIndex::Weapon)
        .and_then(|item| game.items.get_weapon_item(item.item.item_number))
        .map_or(0, |weapon| weapon.motion_type as usize);
    let attack = game.motions.get_character_action_motion(
        CharacterMotionAction::Attack,
        weapon_motion,
        player.gender as usize,
    );
    let attack_motion_ms = attack.map_or(1000, |m| m.duration.as_millis() as i32).max(300);
    let attack_hit_ms = attack
        .and_then(|m| m.first_attack_time)
        .map_or(attack_motion_ms / 2, |d| d.as_millis() as i32)
        .min(attack_motion_ms);
    let hit_count = attack.map_or(1, |m| m.total_attack_frames.max(1) as i32);
    Stats::from_ability_values(entity_id, &av, true, attack_motion_ms, attack_hit_ms, hit_count)
}

/// Recalculate a player's stats after a level, stat or equipment change.
pub fn refresh_player(ctx: &ReducerContext, game: &GameData, player: &Player, full_heal: bool) {
    let Some(id) = player.entity_id else { return };
    let stats = player_stats(ctx, game, id, player);
    if let Some(mut c) = ctx.db.combat().entity_id().find(id) {
        c.max_hp = stats.max_hp;
        c.max_mp = stats.max_mp;
        if full_heal && c.dead_until_us.is_none() {
            c.hp = c.max_hp;
            c.mp = c.max_mp;
        }
        c.hp = c.hp.min(c.max_hp);
        c.mp = c.mp.min(c.max_mp);
        ctx.db.combat().entity_id().update(c);
    }
    if ctx.db.stats().entity_id().find(id).is_some() {
        ctx.db.stats().entity_id().update(stats);
    } else {
        ctx.db.stats().insert(stats);
    }
}

/// Add experience, levelling up as often as it covers (rose-offline's experience_points_system).
pub fn reward_xp(ctx: &ReducerContext, game: &GameData, identity: Identity, xp: u64) {
    let Some(mut p) = ctx.db.player().identity().find(identity) else { return };
    p.xp = p.xp.saturating_add(xp);
    let level_before = p.level;
    loop {
        let need = game.ability_value_calculator.calculate_levelup_require_xp(p.level);
        if p.xp < need {
            break;
        }
        p.level += 1;
        p.xp -= need;
        p.skill_points += game.ability_value_calculator.calculate_levelup_reward_skill_points(p.level);
        p.stat_points += game.ability_value_calculator.calculate_levelup_reward_stat_points(p.level);
    }
    let levelled = p.level != level_before;
    ctx.db.player().identity().update(p.clone());
    if levelled {
        refresh_player(ctx, game, &p, true);
        // Every level gained runs its level up trigger (experience_points_system).
        for level in (level_before + 1)..=p.level {
            crate::quests::run_trigger(ctx, game, identity, &format!("levelup_{level}"));
        }
    }
}

fn stat_type(stat: u8) -> Option<BasicStatType> {
    Some(match stat {
        0 => BasicStatType::Strength,
        1 => BasicStatType::Dexterity,
        2 => BasicStatType::Intelligence,
        3 => BasicStatType::Concentration,
        4 => BasicStatType::Charm,
        5 => BasicStatType::Sense,
        _ => return None,
    })
}

/// Spend stat points on one basic stat: 0 STR, 1 DEX, 2 INT, 3 CON, 4 CHA, 5 SEN.
#[spacetimedb::reducer]
pub fn add_basic_stat(ctx: &ReducerContext, stat: u8) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, _) = my_player(ctx)?;
    let stat = stat_type(stat).ok_or("no such stat")?;
    let mut basic = p.basic_stats();
    let cost = game
        .ability_value_calculator
        .calculate_basic_stat_increase_cost(&basic, stat)
        .ok_or("that stat is at its maximum")?;
    if p.stat_points < cost {
        return Err(format!("needs {cost} stat points"));
    }
    p.stat_points -= cost;
    basic.set(stat, basic.get(stat) + 1);
    p.set_basic_stats(&basic);
    ctx.db.player().identity().update(p.clone());
    refresh_player(ctx, &game, &p, false);
    Ok(())
}

/// Debug: set a player's basic stat by name (stat as in add_basic_stat).
#[spacetimedb::reducer]
pub fn set_basic_stat(ctx: &ReducerContext, name: String, stat: u8, value: i32) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let game = game(ctx)?;
    let stat = stat_type(stat).ok_or("no such stat")?;
    let mut p = ctx.db.player().iter().find(|p| p.name == name).ok_or("no such player")?;
    let mut basic = p.basic_stats();
    basic.set(stat, value.max(1));
    p.set_basic_stats(&basic);
    ctx.db.player().identity().update(p.clone());
    refresh_player(ctx, &game, &p, false);
    Ok(())
}

/// Debug: give a player (by name) Zuly.
#[spacetimedb::reducer]
pub fn give_money(ctx: &ReducerContext, name: String, amount: i64) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let mut p = ctx.db.player().iter().find(|p| p.name == name).ok_or("no such player")?;
    let mut inventory = p.inventory();
    inventory.money = rose_game_common::components::Money((inventory.money.0 + amount).max(0));
    p.set_inventory(&inventory);
    ctx.db.player().identity().update(p);
    Ok(())
}
