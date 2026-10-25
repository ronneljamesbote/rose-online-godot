//! NPC conversations: the NPC's dialog (`RoseDialog`, exported to `export/data/dialogs.toml`)
//! holds menus of lines, each with cases that decide whether it shows and what choosing it
//! does. Quest steps a choice runs are checked here and then sent to the server, which
//! checks them again and applies the rewards.

use rose_data::{AbilityType, Dialog, DialogAction, DialogCondition, DialogContext, DialogQuestValue, DialogVariable, DialogView};
use rose_quest::{QuestCharacter, QuestNpc, QuestParty, QuestWorld, RealTime};

use crate::data::GameData;

/// What a conversation asks the game to do.
#[derive(Debug, Clone)]
pub enum Action {
    QuestTrigger(String),
    OpenStore(u64),
    /// A window for the NPC we talk to: "bank", "refine", "disassemble" or "repair".
    OpenWindow(&'static str),
    Notice(String),
    /// Save this zone as where we get up after dying.
    SetSavePoint,
}

/// A town NPC as the scripts see it.
#[derive(Clone, Debug)]
pub struct WorldNpc {
    pub npc_id: u16,
    pub npc: QuestNpc,
    pub variables: Vec<i32>,
}

/// The world as the client knows it, for quest conditions.
pub struct ClientWorld {
    pub t: i64,
    pub zone_id: u16,
    pub npcs: Vec<WorldNpc>,
    pub party: Option<QuestParty>,
}

impl QuestWorld for ClientWorld {
    fn world_ticks(&self) -> rose_data::WorldTicks {
        rose_quest::world_ticks(self.t)
    }

    fn real_time(&self) -> RealTime {
        RealTime::from_unix_us(self.t)
    }

    /// Random rolls are the server's to make.
    fn random_percent(&mut self) -> Option<u8> {
        None
    }

    fn find_npc(&self, npc_id: u16) -> Option<QuestNpc> {
        let mut matches = self.npcs.iter().filter(|n| n.npc_id == npc_id);
        let first = matches.next()?;
        if first.npc.zone_id == self.zone_id {
            return Some(first.npc);
        }
        Some(matches.find(|n| n.npc.zone_id == self.zone_id).unwrap_or(first).npc)
    }

    fn npc_variable(&self, npc_entity_id: u64, variable_id: usize) -> Option<i32> {
        self.npcs.iter().find(|n| n.npc.entity_id == npc_entity_id)?.variables.get(variable_id).copied()
    }

    fn party(&self) -> Option<QuestParty> {
        self.party
    }

    fn job_classes(&self) -> Option<&rose_data::JobClassDatabase> {
        crate::data::get().map(|g| &g.job_classes)
    }
}

/// Everything the script functions read, plus where they put what they ask for.
pub struct ScriptContext<'a> {
    pub game: &'static GameData,
    pub ch: &'a QuestCharacter,
    pub world: &'a mut ClientWorld,
    pub name: &'a str,
    pub actions: &'a mut Vec<Action>,
    /// The NPC we talk to.
    pub npc_entity: Option<u64>,
}

impl ScriptContext<'_> {
    fn sv(&self, ability: AbilityType) -> i32 {
        let ch = self.ch;
        match ability {
            AbilityType::Gender => ch.gender as i32,
            AbilityType::Job => ch.job as i32,
            AbilityType::Union => ch.union_membership.current_union.map_or(0, |u| u.get() as i32),
            AbilityType::Strength => ch.basic_stats.strength,
            AbilityType::Dexterity => ch.basic_stats.dexterity,
            AbilityType::Intelligence => ch.basic_stats.intelligence,
            AbilityType::Concentration => ch.basic_stats.concentration,
            AbilityType::Charm => ch.basic_stats.charm,
            AbilityType::Sense => ch.basic_stats.sense,
            AbilityType::Experience => ch.xp.min(i32::MAX as u64) as i32,
            AbilityType::Level => ch.level as i32,
            AbilityType::BonusPoint => ch.stat_points as i32,
            _ => 0,
        }
    }

    fn passes(&mut self, trigger: &str) -> bool {
        rose_quest::check_trigger(&self.game.quests, self.game.decoder.as_ref(), self.ch, self.world, trigger)
    }
}

impl DialogContext for ScriptContext<'_> {
    fn check(&mut self, condition: &DialogCondition) -> bool {
        let qs = &self.ch.quest_state;
        let cmp = rose_data::dialog_compare;
        match condition {
            DialogCondition::QuestStep { trigger, passes } => self.passes(trigger) == *passes,
            DialogCondition::Quest { quest, active } => qs.find_active_quest_index(*quest as usize).is_some() == *active,
            DialogCondition::QuestItem { quest, item, operator, value } => {
                let count = qs
                    .find_active_quest(*quest as usize)
                    .map_or(-1, |q| q.find_item(*item).map_or(0, |i| i.get_quantity() as i32));
                cmp(count, *operator, *value)
            }
            DialogCondition::Ability { ability, operator, value } => cmp(self.sv(*ability), *operator, *value),
            // Object variable 0 of the NPC we talk to, which town NPCs use to switch their
            // quests on and off.
            DialogCondition::NpcSwitch { operator, value } => {
                let v = self.npc_entity.and_then(|e| self.world.npc_variable(e, 0)).unwrap_or(0);
                cmp(v, *operator, *value)
            }
            DialogCondition::QuestSwitch { switch, operator, value } => {
                let v = qs.quest_switches.get(*switch as usize).map_or(-1, |s| *s as i32);
                cmp(v, *operator, *value)
            }
            DialogCondition::Variable { variable, index, operator, value } => {
                let list: &[u16] = match variable {
                    DialogVariable::Episode => &qs.episode_variables,
                    DialogVariable::Job => &qs.job_variables,
                    DialogVariable::Planet => &qs.planet_variables,
                };
                cmp(list.get(*index as usize).map_or(-1, |v| *v as i32), *operator, *value)
            }
            DialogCondition::QuestValue { quest, value_type, index, operator, value } => {
                let v = qs.find_active_quest(*quest as usize).and_then(|q| match value_type {
                    DialogQuestValue::Switch => q.switches.get(*index as usize).map(|s| *s as i32),
                    DialogQuestValue::Variable => q.variables.get(*index as usize).map(|v| *v as i32),
                });
                cmp(v.unwrap_or(-1), *operator, *value)
            }
            DialogCondition::QuestCount { operator, value } => {
                cmp(qs.active_quests.iter().filter(|q| q.is_some()).count() as i32, *operator, *value)
            }
        }
    }

    fn act(&mut self, action: &DialogAction) {
        let action = match action {
            DialogAction::QuestStep { trigger } => {
                if !self.passes(trigger) {
                    return;
                }
                Action::QuestTrigger(trigger.clone())
            }
            DialogAction::OpenShop => match self.npc_entity {
                Some(entity) => Action::OpenStore(entity),
                None => return,
            },
            DialogAction::OpenStorage => Action::OpenWindow("bank"),
            DialogAction::OpenRefining => Action::OpenWindow("refine"),
            DialogAction::OpenDisassembly => Action::OpenWindow("disassemble"),
            DialogAction::OpenRepair => Action::OpenWindow("repair"),
            DialogAction::SetSavePoint => Action::SetSavePoint,
            DialogAction::Appraisal | DialogAction::OpenDelivery => {
                Action::Notice("That service is not in the game yet".into())
            }
            DialogAction::FoundClan | DialogAction::DisbandClan => return,
        };
        self.actions.push(action);
    }
}

pub struct Conversation {
    pub npc_entity: Option<u64>,
    pub title: String,
    /// BBCode.
    pub message: String,
    /// The answers you can pick (BBCode).
    pub responses: Vec<String>,
    dialog: &'static Dialog,
    view: DialogView,
}

/// Replace <NAME> and <LEVEL> and turn ROSE's {br} {b} {fc=N} tags into BBCode.
pub fn format_text(text: &str, name: &str, level: u32) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find(['<', '{']) {
        let (before, tag) = rest.split_at(start);
        out += &escape(before);
        let close = if tag.starts_with('<') { '>' } else { '}' };
        let Some(end) = tag.find(close) else {
            rest = tag;
            break;
        };
        let (tag, after) = tag.split_at(end + 1);
        let lower = tag.to_lowercase();
        out += &match lower.as_str() {
            "<name>" => escape(name),
            "<level>" => level.to_string(),
            "{br}" => "\n".into(),
            "{b}" => "[b]".into(),
            "{/b}" => "[/b]".into(),
            "{/fc}" => "[/color]".into(),
            t if t.starts_with("{fc=") => {
                let colour = match t[4..t.len() - 1].parse::<i32>().unwrap_or(-1) {
                    1 => "#800000",
                    2 => "#008000",
                    3 => "#000080",
                    4 => "#808000",
                    5 => "#800080",
                    6 => "#008080",
                    7 => "#808080",
                    8 => "#c0c0c0",
                    9 => "#c0dcc0",
                    10 => "#c0c0dc",
                    11 => "#a6caf0",
                    12 => "#ff6060",
                    13 => "#60ff60",
                    14 => "#8080ff",
                    15 => "#ffff00",
                    16 => "#00ffff",
                    17 => "#fffbf0",
                    _ => "#ffffff",
                };
                format!("[color={colour}]")
            }
            _ if close == '>' => escape(tag),
            _ => String::new(),
        };
        rest = after;
    }
    out += &escape(rest);
    balance_color(out)
}

fn escape(text: &str) -> String {
    text.replace('[', "[lb]")
}

/// Close any [color] the text left open, and drop stray closers.
fn balance_color(text: String) -> String {
    let mut depth = 0i32;
    let mut out = String::with_capacity(text.len());
    let mut rest = text.as_str();
    while let Some(i) = rest.find("[/color]") {
        let (before, after) = rest.split_at(i);
        depth += before.matches("[color=").count() as i32;
        out += before;
        if depth > 0 {
            out += "[/color]";
            depth -= 1;
        }
        rest = &after[8..];
    }
    depth += rest.matches("[color=").count() as i32;
    out += rest;
    for _ in 0..depth {
        out += "[/color]";
    }
    out
}

impl Conversation {
    /// Open an NPC's dialog (`key`, the zone spawn's conversation). None when the NPC has
    /// nothing to say (no dialog, or its opening check says no).
    pub fn open(key: &str, npc_entity: Option<u64>, title: String, cx: &mut ScriptContext) -> Option<Self> {
        let dialog = cx.game.dialogs.get(key)?;
        cx.npc_entity = npc_entity;
        let view = dialog.open(cx)?;
        let mut conversation = Self { npc_entity, title, message: String::new(), responses: Vec::new(), dialog, view };
        conversation.refresh(cx);
        Some(conversation)
    }

    /// The view's NPC line and answers as text, with the player's name and level filled in.
    fn refresh(&mut self, cx: &ScriptContext) {
        let dialog = self.dialog;
        let text =
            |at: &(usize, usize)| dialog.line(*at).map(|l| format_text(&l.text, cx.name, cx.ch.level)).unwrap_or_default();
        self.message = self.view.message.as_ref().map(text).unwrap_or_default();
        self.responses = self.view.answers.iter().map(text).collect();
    }

    /// Pick a response. False when the conversation is over.
    pub fn choose(&mut self, cx: &mut ScriptContext, index: usize) -> bool {
        cx.npc_entity = self.npc_entity;
        let open = self.dialog.choose(&mut self.view, index, cx);
        self.refresh(cx);
        open
    }
}
