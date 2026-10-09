//! Quests: QSD triggers run on the server, from an NPC conversation (the client asks with
//! `quest_trigger`), a monster's death trigger or a level up. Conditions come from the shared
//! rose-quest crate; rewards follow rose-offline's quest_system. A trigger chain works on a
//! copy of the character and writes it back once at the end.

use rand::Rng;
use rose_data::{AbilityType, EquipmentItem, Item, NpcId, SkillId, WorldTicks};
use rose_file_readers::{QsdEquation, QsdItem, QsdNpcMessageType, QsdReward, QsdRewardOperator, QsdSpawnMonsterLocation};
use rose_game_common::components::{ActiveQuest, DroppedItem, Money};
use rose_game_data::GameData;
use rose_quest::{QuestCharacter, QuestContext, QuestNpc, QuestParty, QuestWorld, RealTime};
use spacetimedb::{Identity, ReducerContext};

use crate::{
    character, combat, game_data::game, items, my_player, now_us, npc, player, position, world, world_rates_row,
    world::MonsterSpawn, xp_event, Player, XpEvent,
};
use spacetimedb::Table;

/// Longest trigger chain one run follows (the data's longest is far shorter).
const MAX_CHAIN: usize = 64;
/// Spawn id for quest monsters: they belong to no spawn point and do not respawn.
const QUEST_SPAWN_ID: u32 = u32::MAX;

struct ServerWorld<'a> {
    ctx: &'a ReducerContext,
    game: &'a GameData,
    identity: Identity,
    zone_id: u16,
    t: i64,
}

impl QuestWorld for ServerWorld<'_> {
    fn job_classes(&self) -> Option<&rose_data::JobClassDatabase> {
        Some(&self.game.job_class)
    }

    fn world_ticks(&self) -> WorldTicks {
        rose_quest::world_ticks(self.t)
    }

    fn real_time(&self) -> RealTime {
        RealTime::from_unix_us(self.t)
    }

    fn random_percent(&mut self) -> Option<u8> {
        Some(self.ctx.rng().gen_range(0..100))
    }

    fn find_npc(&self, npc_id: u16) -> Option<QuestNpc> {
        let mut found = None;
        for n in self.ctx.db.npc().iter().filter(|n| n.npc_id == npc_id) {
            let here = n.zone_id == self.zone_id;
            if found.is_none() || here {
                found = Some(n);
            }
            if here {
                break;
            }
        }
        let n = found?;
        let (x, y) = position(self.ctx, n.entity_id, self.t).unwrap_or((0.0, 0.0));
        Some(QuestNpc { entity_id: n.entity_id, zone_id: n.zone_id, x, y })
    }

    fn npc_variable(&self, npc_entity_id: u64, variable_id: usize) -> Option<i32> {
        self.ctx.db.npc().entity_id().find(npc_entity_id)?.variables.get(variable_id).copied()
    }

    fn party(&self) -> Option<QuestParty> {
        crate::party::quest_party(self.ctx, self.identity)
    }
}

/// What a trigger chain does after the character is written back.
enum After {
    Xp(u64),
    Stamina(i32),
    Drop(Item),
    Teleport(u16, f32, f32),
    Spawn { npc_id: u16, count: usize, zone_id: u16, x: f32, y: f32, range: f32 },
    Notice(String),
}

/// The character a trigger chain works on.
struct Run<'a> {
    ctx: &'a ReducerContext,
    game: &'a GameData,
    p: Player,
    ch: QuestCharacter,
    after: Vec<After>,
    t: i64,
}

pub fn quest_character(ctx: &ReducerContext, game: &GameData, p: &Player, t: i64) -> QuestCharacter {
    let c = p.entity_id.and_then(|id| ctx.db.combat().entity_id().find(id));
    let (x, y) = p.entity_id.and_then(|id| position(ctx, id, t)).unwrap_or((p.last_x, p.last_y));
    QuestCharacter {
        gender: p.gender,
        face: p.face,
        hair: p.hair,
        job: p.job,
        level: p.level,
        xp: p.xp,
        stat_points: p.stat_points,
        skill_points: p.skill_points,
        basic_stats: p.basic_stats(),
        ability_values: items::player_ability_values(ctx, game, p),
        hp: c.as_ref().map_or(p.last_hp, |c| c.hp),
        mp: c.as_ref().map_or(p.last_mp, |c| c.mp),
        zone_id: p.zone_id,
        x,
        y,
        team: 2,
        equipment: p.equipment(),
        inventory: p.inventory(),
        skill_list: p.skill_list(),
        quest_state: p.quest_state(),
        union_membership: p.union_membership(),
    }
}

impl<'a> Run<'a> {
    fn new(ctx: &'a ReducerContext, game: &'a GameData, p: Player) -> Self {
        let t = now_us(ctx);
        let ch = quest_character(ctx, game, &p, t);
        Self { ctx, game, p, ch, after: Vec::new(), t }
    }

    /// Copy the working character onto the player row (not saved yet).
    fn sync_to_player(&mut self) {
        let (p, ch) = (&mut self.p, &self.ch);
        p.gender = ch.gender;
        p.face = ch.face;
        p.hair = ch.hair;
        p.job = ch.job;
        p.level = ch.level;
        p.xp = ch.xp;
        p.stat_points = ch.stat_points;
        p.skill_points = ch.skill_points;
        p.set_basic_stats(&ch.basic_stats);
        p.set_inventory(&ch.inventory);
        p.set_skill_list(&ch.skill_list);
        p.set_quest_state(&ch.quest_state);
        p.set_union_membership(&ch.union_membership);
    }

    fn finish(mut self) {
        self.sync_to_player();
        let (ctx, game) = (self.ctx, self.game);
        ctx.db.player().identity().update(self.p.clone());
        if let Some(mut c) = self.p.entity_id.and_then(|id| ctx.db.combat().entity_id().find(id)) {
            if c.dead_until_us.is_none() {
                c.hp = self.ch.hp.clamp(1, c.max_hp.max(1));
                c.mp = self.ch.mp.clamp(0, c.max_mp);
                ctx.db.combat().entity_id().update(c);
            }
        }
        character::refresh_player(ctx, game, &self.p, false);
        let identity = self.p.identity;
        for after in self.after {
            match after {
                After::Xp(xp) => {
                    character::reward_xp(ctx, game, identity, xp);
                    let level = ctx.db.player().identity().find(identity).map_or(0, |p| p.level);
                    ctx.db.xp_event().insert(XpEvent { identity, xp, level });
                }
                After::Stamina(value) => crate::stamina::add(ctx, identity, value),
                After::Drop(item) => {
                    let Some(p) = ctx.db.player().identity().find(identity) else { continue };
                    let at = p.entity_id.and_then(|id| position(ctx, id, self.t)).unwrap_or((p.last_x, p.last_y));
                    items::drop_on_ground(ctx, p.zone_id, at, &DroppedItem::Item(item), Some(identity));
                }
                After::Teleport(zone_id, x, y) => {
                    if let Some(mut p) = ctx.db.player().identity().find(identity) {
                        world::teleport(ctx, &mut p, zone_id, (x, y));
                    }
                }
                After::Spawn { npc_id, count, zone_id, x, y, range } => {
                    let spawn = MonsterSpawn {
                        spawn_id: QUEST_SPAWN_ID,
                        zone_id,
                        x,
                        y,
                        range,
                        interval_secs: 0,
                        limit_count: 0,
                        tactic_points: 0,
                        basic: Vec::new(),
                        tactic: Vec::new(),
                        current_tactics_value: 0,
                        next_check_at_us: 0,
                    };
                    for _ in 0..count.min(20) {
                        world::spawn_monster(ctx, game, &spawn, npc_id);
                    }
                }
                After::Notice(text) => items::notify(ctx, identity, text),
            }
        }
    }

    fn quest_name(&self, quest_id: usize) -> String {
        self.game.quests.get_quest_data(quest_id).map_or_else(|| format!("quest {quest_id}"), |q| q.name.to_string())
    }

    fn expire_time(&self, quest_id: usize) -> Option<WorldTicks> {
        self.game
            .quests
            .get_quest_data(quest_id)
            .and_then(|q| q.time_limit)
            .map(|limit| rose_quest::world_ticks(self.t) + limit)
    }

    fn reward_value(&self, equation: usize, value: i32, dup_count: i32) -> Option<i32> {
        let equation = <QsdEquation as num_traits::FromPrimitive>::from_usize(equation)?;
        Some(self.game.ability_value_calculator.calculate_reward_value(
            &equation,
            value,
            dup_count,
            self.ch.level as i32,
            self.ch.basic_stats.charm,
            0,
            world_rates_row(self.ctx).reward_rate,
        ))
    }

    /// An item for the inventory; if it is full the item lands at the character's feet.
    fn give_item(&mut self, item: Item) {
        let name = items::item_name(self.game, &item);
        match self.ch.inventory.try_add_item(item) {
            Ok(_) => self.after.push(After::Notice(format!("Received {name}"))),
            Err(item) => {
                self.after.push(After::Notice(format!("Received {name} (inventory full, it is on the ground)")));
                self.after.push(After::Drop(item));
            }
        }
    }

    fn set_ability(&mut self, ability: AbilityType, value: i32) -> bool {
        let ch = &mut self.ch;
        match ability {
            AbilityType::Gender => ch.gender = (value != 0) as u8,
            AbilityType::Face => ch.face = value as u8,
            AbilityType::Hair => ch.hair = value as u8,
            AbilityType::Job => ch.job = value as u16,
            AbilityType::Strength => ch.basic_stats.strength = value,
            AbilityType::Dexterity => ch.basic_stats.dexterity = value,
            AbilityType::Intelligence => ch.basic_stats.intelligence = value,
            AbilityType::Concentration => ch.basic_stats.concentration = value,
            AbilityType::Charm => ch.basic_stats.charm = value,
            AbilityType::Sense => ch.basic_stats.sense = value,
            AbilityType::Health => ch.hp = value,
            AbilityType::Mana => ch.mp = value,
            AbilityType::Experience => ch.xp = value.max(0) as u64,
            AbilityType::Union => ch.union_membership.current_union = std::num::NonZeroUsize::new(value.max(0) as usize),
            AbilityType::Level | AbilityType::Rank | AbilityType::Fame | AbilityType::Stamina => {}
            other => match rose_quest::union_point_index(other) {
                Some(i) => ch.union_membership.points[i] = value.max(0) as u32,
                None => return false,
            },
        }
        true
    }

    fn add_ability(&mut self, ability: AbilityType, value: i32) -> bool {
        fn add(current: i32, value: i32) -> i32 {
            (current + value).max(0)
        }
        fn add_u32(current: u32, value: i32) -> u32 {
            (current as i64 + value as i64).clamp(0, u32::MAX as i64) as u32
        }
        let ch = &mut self.ch;
        match ability {
            AbilityType::Strength => ch.basic_stats.strength = add(ch.basic_stats.strength, value),
            AbilityType::Dexterity => ch.basic_stats.dexterity = add(ch.basic_stats.dexterity, value),
            AbilityType::Intelligence => ch.basic_stats.intelligence = add(ch.basic_stats.intelligence, value),
            AbilityType::Concentration => ch.basic_stats.concentration = add(ch.basic_stats.concentration, value),
            AbilityType::Charm => ch.basic_stats.charm = add(ch.basic_stats.charm, value),
            AbilityType::Sense => ch.basic_stats.sense = add(ch.basic_stats.sense, value),
            AbilityType::BonusPoint => ch.stat_points = add_u32(ch.stat_points, value),
            AbilityType::Skillpoint => ch.skill_points = add_u32(ch.skill_points, value),
            AbilityType::Health => ch.hp = add(ch.hp, value),
            AbilityType::Mana => ch.mp = add(ch.mp, value),
            AbilityType::Experience => {
                if value > 0 {
                    self.after.push(After::Xp(value as u64));
                } else {
                    ch.xp = (ch.xp as i64 + value as i64).max(0) as u64;
                }
            }
            AbilityType::Money => {
                let money = ch.inventory.money.0 + value as i64;
                if money < 0 {
                    return false;
                }
                ch.inventory.money = Money(money);
                let text = if value >= 0 { format!("Received {value} Zuly") } else { format!("Paid {} Zuly", -value) };
                self.after.push(After::Notice(text));
            }
            AbilityType::Stamina => self.after.push(After::Stamina(value)),
            AbilityType::Fame => {}
            other => match rose_quest::union_point_index(other) {
                Some(i) => ch.union_membership.points[i] = add_u32(ch.union_membership.points[i], value),
                None => return false,
            },
        }
        true
    }

    fn ability_reward(&mut self, ability_type: usize, operator: QsdRewardOperator, value: i32) -> bool {
        let Some(ability) = self.game.data_decoder.decode_ability_type(ability_type) else { return false };
        match operator {
            QsdRewardOperator::Set => self.set_ability(ability, value),
            QsdRewardOperator::Add => self.add_ability(ability, value),
            QsdRewardOperator::Subtract => self.add_ability(ability, -value),
            QsdRewardOperator::Zero => self.set_ability(ability, 0),
            QsdRewardOperator::One => self.set_ability(ability, 1),
        }
    }

    fn add_item(&mut self, cx: &QuestContext, item: QsdItem, quantity: usize) -> bool {
        let Some(reference) = self.game.data_decoder.decode_item_reference(item.item_number, item.item_type) else {
            return false;
        };
        let Some(data) = self.game.items.get_base_item(reference) else { return false };
        let Some(item) = Item::from_item_data(data, quantity as u32) else { return false };
        if reference.item_type.is_quest_item() {
            let Some(quest) = cx.selected_quest_index.and_then(|i| self.ch.quest_state.get_quest_mut(i)) else {
                return false;
            };
            let name = data.name;
            let ok = quest.try_add_item(item).is_ok();
            if ok {
                self.after.push(After::Notice(format!("Quest item: {name} ({})", self.quest_item_count(cx, reference))));
            }
            ok
        } else {
            self.give_item(item);
            true
        }
    }

    fn quest_item_count(&self, cx: &QuestContext, reference: rose_data::ItemReference) -> u32 {
        cx.selected_quest_index
            .and_then(|i| self.ch.quest_state.get_quest(i))
            .and_then(|q| q.find_item(reference))
            .map_or(0, |i| i.get_quantity())
    }

    fn remove_item(&mut self, cx: &QuestContext, item: QsdItem, quantity: usize) -> bool {
        let Some(reference) = self.game.data_decoder.decode_item_reference(item.item_number, item.item_type) else {
            return false;
        };
        if reference.item_type.is_quest_item() {
            cx.selected_quest_index
                .and_then(|i| self.ch.quest_state.get_quest_mut(i))
                .and_then(|q| q.try_take_item(reference, quantity as u32))
                .is_some()
        } else {
            self.ch.inventory.try_take_item(reference, quantity as u32).is_some()
        }
    }

    fn calculated_item(&mut self, equation: usize, value: i32, item: QsdItem, gem: Option<std::num::NonZeroUsize>) -> bool {
        let Some(reference) = self.game.data_decoder.decode_item_reference(item.item_number, item.item_type) else {
            return false;
        };
        let Some(data) = self.game.items.get_base_item(reference) else { return false };
        let item = if reference.item_type.is_stackable_item() {
            let Some(quantity) = self.reward_value(equation, value, 0) else { return true };
            if quantity <= 0 {
                return true;
            }
            Item::from_item_data(data, quantity as u32)
        } else if let Some(mut equipment) = EquipmentItem::new(reference, data.durability) {
            if let Some(gem) = gem.filter(|g| g.get() < 300) {
                equipment.is_appraised = true;
                equipment.has_socket = false;
                equipment.gem = gem.get() as u16;
            }
            if equipment.gem == 0 {
                match data.rare_type {
                    1 => {
                        equipment.has_socket = true;
                        equipment.is_appraised = true;
                    }
                    2 if data.quality as i32 + 60 > self.ctx.rng().gen_range(0..400) => {
                        equipment.has_socket = true;
                        equipment.is_appraised = true;
                    }
                    _ => {}
                }
            }
            Some(Item::Equipment(equipment))
        } else {
            None
        };
        if let Some(item) = item {
            self.give_item(item);
        }
        true
    }

    fn calculated_money(&mut self, cx: &QuestContext, equation: usize, value: i32) -> bool {
        // The selected quest's last variable counts repeats, and resets when paid.
        let quest = cx.selected_quest_index.and_then(|i| self.ch.quest_state.get_quest(i));
        let dup_count = quest.and_then(|q| q.variables.last()).map_or(0, |v| *v as i32);
        let Some(money) = self.reward_value(equation, value, dup_count) else { return true };
        self.ch.inventory.money = Money(self.ch.inventory.money.0 + money.max(0) as i64);
        if let Some(last) = cx
            .selected_quest_index
            .and_then(|i| self.ch.quest_state.get_quest_mut(i))
            .and_then(|q| q.variables.last_mut())
        {
            *last = 0;
        }
        self.after.push(After::Notice(format!("Received {money} Zuly")));
        true
    }

    fn add_skill(&mut self, id: usize) -> bool {
        let Some(skill_id) = SkillId::new(id as u16) else { return false };
        self.sync_to_player();
        // As rose-offline, a skill the character can't learn does not fail the trigger.
        if let Err(e) = crate::skills::learn_skill(self.ctx, self.game, &mut self.p, skill_id) {
            log::debug!("quest skill {id}: {e}");
        }
        self.ch.skill_list = self.p.skill_list();
        self.ch.skill_points = self.p.skill_points;
        true
    }

    fn remove_skill(&mut self, id: usize) -> bool {
        let Some(data) = SkillId::new(id as u16).and_then(|id| self.game.skills.get_skill(id)) else { return false };
        let Some((slot, _)) = self.ch.skill_list.find_skill_exact(data) else { return false };
        if let Some(slot) = self.ch.skill_list.get_slot_mut(slot) {
            *slot = None;
        }
        true
    }

    fn reset_basic_stats(&mut self) -> bool {
        let start = &self.game.character_creator.genders[if self.ch.gender == 1 { 1 } else { 0 }];
        self.ch.basic_stats = start.basic_stats.clone();
        self.ch.stat_points = (2..=self.ch.level)
            .map(|level| self.game.ability_value_calculator.calculate_levelup_reward_stat_points(level))
            .sum();
        true
    }

    fn reset_skills(&mut self) -> bool {
        for page in self.ch.skill_list.pages[1..].iter_mut() {
            for skill in page.skills.iter_mut() {
                *skill = None;
            }
        }
        self.ch.skill_points = (2..=self.ch.level)
            .map(|level| self.game.ability_value_calculator.calculate_levelup_reward_skill_points(level))
            .sum();
        true
    }

    fn npc_variable_reward(&mut self, cx: &QuestContext, variable_id: usize, operator: QsdRewardOperator, value: i32) -> bool {
        let Some(mut n) = cx.selected_npc.and_then(|n| self.ctx.db.npc().entity_id().find(n.entity_id)) else {
            return false;
        };
        let Some(v) = n.variables.get_mut(variable_id) else { return false };
        *v = rose_quest::reward_operator(operator, *v, value);
        self.ctx.db.npc().entity_id().update(n);
        true
    }

    fn spawn_monster(&mut self, cx: &QuestContext, npc: usize, count: usize, location: QsdSpawnMonsterLocation, distance: i32) -> bool {
        let Some(npc_id) = NpcId::new(npc as u16) else { return true };
        let at = match location {
            QsdSpawnMonsterLocation::QuestOwner => Some((self.ch.zone_id, self.ch.x, self.ch.y)),
            QsdSpawnMonsterLocation::SelectedNpc => cx.selected_npc.map(|n| (n.zone_id, n.x, n.y)),
            QsdSpawnMonsterLocation::SelectedEvent => None,
            QsdSpawnMonsterLocation::Position { zone, x, y } => Some((zone as u16, x, y)),
        };
        if let Some((zone_id, x, y)) = at {
            self.after.push(After::Spawn { npc_id: npc_id.get(), count, zone_id, x, y, range: distance.max(100) as f32 });
        }
        true
    }

    fn npc_message(&mut self, message_type: QsdNpcMessageType, string_id: usize) -> bool {
        if let Some(text) = self.game.quests.get_quest_string(string_id as u16) {
            let text = match message_type {
                QsdNpcMessageType::Chat => text.clone(),
                QsdNpcMessageType::Shout | QsdNpcMessageType::Announce => format!("[Announcement] {text}"),
            };
            self.after.push(After::Notice(text));
        }
        true
    }

    /// Apply one trigger's rewards in order (quest_trigger_apply_rewards). Stops at the first
    /// that fails; what earlier rewards changed stays, as in rose-offline.
    fn apply_rewards(&mut self, cx: &mut QuestContext, trigger: &rose_data::QuestTrigger) -> bool {
        for reward in trigger.rewards.iter() {
            let ok = match *reward {
                QsdReward::RemoveSelectedQuest => match cx.selected_quest_index {
                    Some(index) => match self.ch.quest_state.get_quest_slot_mut(index) {
                        Some(slot) => {
                            if let Some(quest) = slot.take() {
                                let name = self.quest_name(quest.quest_id);
                                self.after.push(After::Notice(format!("Quest complete: {name}")));
                            }
                            true
                        }
                        None => false,
                    },
                    None => false,
                },
                QsdReward::AddQuest { id } => {
                    let expire = self.expire_time(id);
                    match self.ch.quest_state.try_add_quest(ActiveQuest::new(id, expire)) {
                        Some(index) => {
                            if cx.selected_quest_index.is_none() {
                                cx.selected_quest_index = Some(index);
                            }
                            let name = self.quest_name(id);
                            self.after.push(After::Notice(format!("New quest: {name}")));
                            true
                        }
                        None => {
                            self.after.push(After::Notice("Your quest list is full".into()));
                            false
                        }
                    }
                }
                QsdReward::ChangeSelectedQuest { id, keep_data } => {
                    let expire = self.expire_time(id);
                    match cx.selected_quest_index.and_then(|i| self.ch.quest_state.get_quest_mut(i)) {
                        Some(quest) => {
                            if keep_data {
                                quest.quest_id = id;
                            } else {
                                *quest = ActiveQuest::new(id, expire);
                            }
                            true
                        }
                        None => false,
                    }
                }
                QsdReward::SelectQuest { id } => match self.ch.quest_state.find_active_quest_index(id) {
                    Some(index) => {
                        cx.selected_quest_index = Some(index);
                        true
                    }
                    None => false,
                },
                QsdReward::AbilityValue { ability_type, operator, value } => {
                    self.ability_reward(ability_type.get(), operator, value)
                }
                QsdReward::AddItem { item, quantity } => self.add_item(cx, item, quantity),
                QsdReward::RemoveItem { item, quantity } => self.remove_item(cx, item, quantity),
                QsdReward::AddSkill { id } => self.add_skill(id),
                QsdReward::RemoveSkill { id } => self.remove_skill(id),
                QsdReward::ResetBasicStats => self.reset_basic_stats(),
                QsdReward::ResetSkills => self.reset_skills(),
                QsdReward::SetQuestSwitch { id, value } => match self.ch.quest_state.quest_switches.get_mut(id) {
                    Some(mut switch) => {
                        *switch = value;
                        true
                    }
                    None => false,
                },
                QsdReward::ClearAllSwitches => {
                    self.ch.quest_state.quest_switches.fill(false);
                    true
                }
                QsdReward::ClearSwitchGroup { group } => {
                    for i in (32 * group)..(32 * (group + 1)) {
                        if let Some(mut switch) = self.ch.quest_state.quest_switches.get_mut(i) {
                            *switch = false;
                        }
                    }
                    true
                }
                QsdReward::CalculatedExperiencePoints { equation, value } => {
                    if let Some(xp) = self.reward_value(equation, value, 0).filter(|xp| *xp > 0) {
                        self.after.push(After::Xp(xp as u64));
                    }
                    true
                }
                QsdReward::CalculatedItem { equation, value, item, gem } => self.calculated_item(equation, value, item, gem),
                QsdReward::CalculatedMoney { equation, value } => self.calculated_money(cx, equation, value),
                // Client-side effects (event dialogs).
                QsdReward::CallLuaFunction { .. } => true,
                QsdReward::Teleport { zone, x, y } => {
                    self.after.push(After::Teleport(zone as u16, x as f32, y as f32));
                    true
                }
                QsdReward::Trigger { ref name } => {
                    cx.next_trigger_name = Some(name.clone());
                    true
                }
                QsdReward::QuestVariable { variable_type, variable_id, operator, value } => {
                    match rose_quest::get_quest_variable(&self.ch, cx, rose_quest::world_ticks(self.t), variable_type, variable_id) {
                        Some(current) => {
                            let value = rose_quest::reward_operator(operator, current, value);
                            rose_quest::set_quest_variable(&mut self.ch, cx, variable_type, variable_id, value);
                            true
                        }
                        None => false,
                    }
                }
                QsdReward::SetHealthManaPercent { health_percent, mana_percent } => {
                    self.ch.hp = self.ch.ability_values.get_max_health() * health_percent as i32 / 100;
                    self.ch.mp = self.ch.ability_values.get_max_mana() * mana_percent as i32 / 100;
                    true
                }
                QsdReward::ObjectVariable { variable_id, operator, value, .. } => {
                    self.npc_variable_reward(cx, variable_id, operator, value)
                }
                QsdReward::SpawnMonster { npc, count, location, distance, .. } => {
                    self.spawn_monster(cx, npc, count, location, distance)
                }
                QsdReward::NpcMessage { message_type, string_id } => self.npc_message(message_type, string_id),
                QsdReward::SetRevivePosition { x, y } => {
                    crate::death::save(self.ctx, self.p.identity, self.ch.zone_id, (x, y));
                    true
                }
                // Teams are not in the game yet.
                QsdReward::SetTeamNumber { .. } => true,
                // No clans yet.
                QsdReward::ClanLevelIncrease
                | QsdReward::ClanMoney { .. }
                | QsdReward::ClanPoints { .. }
                | QsdReward::AddClanSkill { .. }
                | QsdReward::RemoveClanSkill { .. }
                | QsdReward::ClanPointContribution { .. }
                | QsdReward::TeleportNearbyClanMembers { .. } => false,
                _ => {
                    log::warn!("unimplemented quest reward {reward:?}");
                    false
                }
            };
            if !ok {
                log::debug!(target: "quest", "reward failed {reward:?}");
                return false;
            }
        }
        true
    }
}

/// Run a trigger chain for a character (rose-offline's quest_system). True if any trigger
/// in the chain passed its conditions and rewards.
pub fn run_trigger(ctx: &ReducerContext, game: &GameData, identity: Identity, name: &str) -> bool {
    let Some(p) = ctx.db.player().identity().find(identity) else { return false };
    let Some(first) = rose_quest::find_trigger(&game.quests, name) else { return false };
    let mut run = Run::new(ctx, game, p);
    let mut world = ServerWorld { ctx, game, identity, zone_id: run.ch.zone_id, t: run.t };
    let mut cx = QuestContext::default();
    let mut trigger = Some(first);
    let mut success = false;
    let mut steps = 0;
    while let Some(t) = trigger {
        steps += 1;
        if steps > MAX_CHAIN {
            log::warn!("quest trigger chain from {name} is too long");
            break;
        }
        if rose_quest::check_conditions(game.data_decoder.as_ref(), &run.ch, &mut world, &mut cx, t)
            && run.apply_rewards(&mut cx, t)
        {
            success = true;
            trigger = cx.next_trigger_name.take().and_then(|n| game.quests.get_trigger_by_name(&n));
        } else {
            trigger = t.next_trigger_name.as_ref().and_then(|n| game.quests.get_trigger_by_name(n));
        }
    }
    run.finish();
    success
}

/// The client's conversation script asks for a quest trigger (QF_doQuestTrigger). The
/// server checks the conditions again, so a client can't skip them.
#[spacetimedb::reducer]
pub fn quest_trigger(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let game = game(ctx)?;
    let (p, id) = my_player(ctx)?;
    if !crate::is_alive(ctx, id) {
        return Err("dead".into());
    }
    if rose_quest::find_trigger(&game.quests, &name).is_none() {
        return Err("no such quest trigger".into());
    }
    if !run_trigger(ctx, &game, p.identity, &name) {
        log::debug!("quest trigger {name} failed for {}", p.name);
    }
    Ok(())
}

/// Give up a quest from the quest list.
#[spacetimedb::reducer]
pub fn abandon_quest(ctx: &ReducerContext, slot: u8, quest_id: u32) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, _) = my_player(ctx)?;
    let mut state = p.quest_state();
    let quest = state.get_quest_slot_mut(slot as usize).ok_or("no such quest slot")?;
    if quest.as_ref().map(|q| q.quest_id) != Some(quest_id as usize) {
        return Err("that quest is not in that slot".into());
    }
    *quest = None;
    p.set_quest_state(&state);
    ctx.db.player().identity().update(p.clone());
    let name = game.quests.get_quest_data(quest_id as usize).map_or("the quest", |q| q.name);
    items::notify(ctx, p.identity, format!("Abandoned {name}"));
    Ok(())
}

/// Debug: run a quest trigger for a player by name, skipping nothing.
#[spacetimedb::reducer]
pub fn admin_quest_trigger(ctx: &ReducerContext, player_name: String, trigger: String) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let game = game(ctx)?;
    let p = ctx.db.player().iter().find(|p| p.name == player_name).ok_or("no such player")?;
    let ok = run_trigger(ctx, &game, p.identity, &trigger);
    log::info!("admin quest trigger {trigger} for {player_name}: {ok}");
    Ok(())
}

/// Debug: give a player a quest, with `quantity` of quest item `item_number` in it (0 for
/// none), as if a trigger had. For testing the later steps of a quest chain.
#[spacetimedb::reducer]
pub fn admin_give_quest(ctx: &ReducerContext, player_name: String, quest_id: u32, item_number: u32, quantity: u32) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let game = game(ctx)?;
    let p = ctx.db.player().iter().find(|p| p.name == player_name).ok_or("no such player")?;
    let mut rewards = vec![QsdReward::AddQuest { id: quest_id as usize }];
    if quantity > 0 {
        let item = QsdItem { item_number: item_number as usize, item_type: 13 };
        rewards.push(QsdReward::AddItem { item, quantity: quantity as usize });
    }
    let trigger = rose_data::QuestTrigger { name: String::new(), conditions: Vec::new(), rewards, next_trigger_name: None };
    let mut run = Run::new(ctx, &game, p);
    let ok = run.apply_rewards(&mut QuestContext::default(), &trigger);
    run.finish();
    if ok { Ok(()) } else { Err("could not add the quest".into()) }
}

/// Debug: set one of a player's job variables (the job change quests keep their progress
/// there), for testing a job chain again on the same character.
#[spacetimedb::reducer]
pub fn admin_set_job_var(ctx: &ReducerContext, player_name: String, index: u32, value: u16) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let mut p = ctx.db.player().iter().find(|p| p.name == player_name).ok_or("no such player")?;
    let mut state = p.quest_state();
    *state.job_variables.get_mut(index as usize).ok_or("no such job variable")? = value;
    p.set_quest_state(&state);
    ctx.db.player().identity().update(p);
    Ok(())
}

/// Debug: drop all of a player's active quests, for testing a quest chain again.
#[spacetimedb::reducer]
pub fn admin_clear_quests(ctx: &ReducerContext, player_name: String) -> Result<(), String> {
    crate::require_admin(ctx)?;
    let mut p = ctx.db.player().iter().find(|p| p.name == player_name).ok_or("no such player")?;
    let mut state = p.quest_state();
    for quest in state.active_quests.iter_mut() {
        *quest = None;
    }
    p.set_quest_state(&state);
    ctx.db.player().identity().update(p);
    Ok(())
}
