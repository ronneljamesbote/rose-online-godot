//! NPC conversations: the NPC's CON file holds menus of messages and a compiled Lua 4 script
//! whose functions decide which messages show and what a choice does (rose-offline-client's
//! conversation_dialog_system and its QF_/GF_ script functions). Quest triggers a choice
//! runs are checked here and then sent to the server, which checks them again and applies
//! the rewards.

use std::{any::Any, sync::Arc};

use rose_data::NpcConversationId;
use rose_file_readers::{ConFile, ConMessageType};
use rose_quest::{QuestCharacter, QuestNpc, QuestParty, QuestWorld, RealTime};

use crate::{
    data::{self, GameData},
    lua4::{Lua4Function, Lua4VM, Lua4VMError, Lua4VMRustClosures, Lua4Value},
};

/// Script variable ids for GF_getVariable (lua_game_constants).
const SV_CONSTANTS: [(&str, i32); 15] = [
    ("SV_SEX", 0),
    ("SV_BIRTH", 1),
    ("SV_CLASS", 2),
    ("SV_UNION", 3),
    ("SV_RANK", 4),
    ("SV_FAME", 5),
    ("SV_STR", 6),
    ("SV_DEX", 7),
    ("SV_INT", 8),
    ("SV_CON", 9),
    ("SV_CHA", 10),
    ("SV_SEN", 11),
    ("SV_EXP", 12),
    ("SV_LEVEL", 13),
    ("SV_POINT", 14),
];

/// Script functions this client answers. The NPC motion and effect functions only change
/// how the NPC looks while talking, so they do nothing yet.
const FUNCTIONS: [&str; 28] = [
    "QF_checkQuestCondition",
    "QF_doQuestTrigger",
    "QF_findQuest",
    "QF_getEventOwner",
    "QF_getEpisodeVAR",
    "QF_getJobVAR",
    "QF_getPlanetVAR",
    "QF_getQuestCount",
    "QF_getQuestID",
    "QF_getQuestItemQuantity",
    "QF_getQuestSwitch",
    "QF_getQuestVar",
    "QF_getUserSwitch",
    "QF_getNpcQuestZeroVal",
    "GF_getVariable",
    "GF_openStore",
    "GF_openBank",
    "GF_organizeClan",
    "GF_disorganizeClan",
    "GF_GetMotionUseFile",
    "GF_SetMotion",
    "GF_appraisal",
    "GF_repair",
    "GF_openUpgrade",
    "GF_openSeparate",
    "GF_openDeliveryStore",
    "GF_setRevivePosition",
    "GF_getReviveZoneName",
];

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
    /// The zone our save point is in.
    pub save_zone_name: Option<String>,
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
}

/// The conversation's owner, passed to the script's functions as user data.
struct Owner {
    entity_id: Option<u64>,
}

fn arg_usize(args: &[Lua4Value], i: usize) -> Option<usize> {
    args.get(i)?.to_usize().ok()
}

impl ScriptContext<'_> {
    fn check(&mut self, name: &str) -> bool {
        rose_quest::check_trigger(&self.game.quests, self.game.decoder.as_ref(), self.ch, self.world, name)
    }

    fn sv(&self, id: i32) -> i32 {
        let ch = self.ch;
        match id {
            0 => ch.gender as i32,
            2 => ch.job as i32,
            3 => ch.union_membership.current_union.map_or(0, |u| u.get() as i32),
            6 => ch.basic_stats.strength,
            7 => ch.basic_stats.dexterity,
            8 => ch.basic_stats.intelligence,
            9 => ch.basic_stats.concentration,
            10 => ch.basic_stats.charm,
            11 => ch.basic_stats.sense,
            12 => ch.xp.min(i32::MAX as u64) as i32,
            13 => ch.level as i32,
            14 => ch.stat_points as i32,
            _ => 0,
        }
    }

    fn call(&mut self, name: &str, args: &[Lua4Value]) -> Option<Vec<Lua4Value>> {
        let qs = &self.ch.quest_state;
        let one = |v: i32| Some(vec![v.into()]);
        match name {
            "QF_checkQuestCondition" => {
                let trigger = args.first()?.to_string().ok()?;
                let ok = self.check(&trigger);
                one(ok as i32)
            }
            "QF_doQuestTrigger" => {
                let trigger = args.first()?.to_string().ok()?;
                let ok = self.check(&trigger);
                if ok {
                    self.actions.push(Action::QuestTrigger(trigger));
                }
                one(ok as i32)
            }
            "QF_findQuest" => one(arg_usize(args, 0).and_then(|id| qs.find_active_quest_index(id)).map_or(-1, |i| i as i32)),
            "QF_getEventOwner" => {
                let owner = args.first().and_then(|a| a.to_user_type::<Owner>().ok()).and_then(|o| o.entity_id);
                Some(vec![owner.map_or(0, |id| id as usize).into()])
            }
            "QF_getEpisodeVAR" => one(arg_usize(args, 0).and_then(|i| qs.episode_variables.get(i)).map_or(-1, |v| *v as i32)),
            "QF_getJobVAR" => one(arg_usize(args, 0).and_then(|i| qs.job_variables.get(i)).map_or(-1, |v| *v as i32)),
            "QF_getPlanetVAR" => one(arg_usize(args, 0).and_then(|i| qs.planet_variables.get(i)).map_or(-1, |v| *v as i32)),
            "QF_getQuestCount" => one(qs.active_quests.iter().filter(|q| q.is_some()).count() as i32),
            "QF_getQuestID" => one(arg_usize(args, 0).and_then(|i| qs.get_quest(i)).map_or(-1, |q| q.quest_id as i32)),
            "QF_getQuestItemQuantity" => {
                let value = (|| {
                    let quest = qs.find_active_quest(arg_usize(args, 0)?)?;
                    let reference = self.game.decoder.decode_item_base1000(arg_usize(args, 1)?)?;
                    Some(quest.find_item(reference).map_or(0, |i| i.get_quantity() as i32))
                })();
                one(value.unwrap_or(-1))
            }
            "QF_getQuestSwitch" => {
                let value = (|| Some(*qs.get_quest(arg_usize(args, 0)?)?.switches.get(arg_usize(args, 1)?)? as i32))();
                one(value.unwrap_or(-1))
            }
            "QF_getQuestVar" => {
                let value = (|| Some(*qs.get_quest(arg_usize(args, 0)?)?.variables.get(arg_usize(args, 1)?)? as i32))();
                one(value.unwrap_or(-1))
            }
            "QF_getUserSwitch" => {
                one(arg_usize(args, 0).and_then(|i| qs.quest_switches.get(i).map(|s| *s as i32)).unwrap_or(-1))
            }
            // Object variable 0 of the NPC (the event owner's entity), which town NPCs'
            // scripts use to switch their quests on and off.
            "QF_getNpcQuestZeroVal" => {
                let entity_id = arg_usize(args, 0)? as u64;
                one(self.world.npc_variable(entity_id, 0).unwrap_or(0))
            }
            "GF_getVariable" => one(self.sv(args.first()?.to_i32().ok()?)),
            "GF_openStore" => {
                self.actions.push(Action::OpenStore(arg_usize(args, 0)? as u64));
                Some(vec![])
            }
            "GF_openBank" => {
                self.actions.push(Action::OpenWindow("bank"));
                Some(vec![])
            }
            "GF_openUpgrade" => {
                self.actions.push(Action::OpenWindow("refine"));
                Some(vec![])
            }
            "GF_openSeparate" => {
                self.actions.push(Action::OpenWindow("disassemble"));
                Some(vec![])
            }
            "GF_repair" => {
                self.actions.push(Action::OpenWindow("repair"));
                Some(vec![])
            }
            "GF_appraisal" | "GF_openDeliveryStore" => {
                self.actions.push(Action::Notice("That service is not in the game yet".into()));
                Some(vec![])
            }
            "GF_setRevivePosition" => {
                self.actions.push(Action::SetSavePoint);
                Some(vec![])
            }
            "GF_getReviveZoneName" => {
                Some(vec![self.world.save_zone_name.clone().map_or(Lua4Value::Nil, Lua4Value::String)])
            }
            "GF_GetMotionUseFile" => one(0),
            "GF_SetMotion" | "GF_organizeClan" | "GF_disorganizeClan" => Some(vec![]),
            _ => None,
        }
    }
}

impl Lua4VMRustClosures for ScriptContext<'_> {
    fn call_rust_closure(&mut self, name: &str, parameters: Vec<Lua4Value>) -> Result<Vec<Lua4Value>, Lua4VMError> {
        self.call(name, &parameters).ok_or_else(|| Lua4VMError::GlobalNotFound(name.to_string()))
    }
}

/// One choice under the NPC's message.
#[derive(Clone, Debug)]
pub struct Response {
    /// BBCode.
    pub text: String,
    action_function: String,
    menu_index: i32,
}

pub struct Conversation {
    pub npc_entity: Option<u64>,
    pub title: String,
    /// BBCode.
    pub message: String,
    pub responses: Vec<Response>,
    con: ConFile,
    vm: Lua4VM,
    owner: Arc<dyn Any + Send + Sync>,
}

/// An NPC's conversation file, from its zone spawn's conversation id.
pub fn conversation_path(game: &GameData, conversation: &str) -> Option<String> {
    game.npcs.get_conversation(&NpcConversationId::new(conversation.to_string())).map(|c| c.filename.clone())
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
    /// Load an NPC's conversation and run its opening check. None when the NPC has nothing
    /// to say (no file, or the opening function says no).
    pub fn open(path: &str, npc_entity: Option<u64>, title: String, cx: &mut ScriptContext) -> Option<Self> {
        let con = data::read_file::<ConFile>(path)?;
        let mut vm = Lua4VM::new();
        for (name, value) in SV_CONSTANTS {
            vm.set_global(name.to_string(), value.into());
        }
        for name in FUNCTIONS {
            vm.set_global(name.to_string(), Lua4Value::RustClosure(name.to_string()));
        }
        let function = Lua4Function::from_bytes(&con.script_binary).ok()?;
        if let Err(e) = vm.call_lua_function(cx, &function, &[]) {
            godot::global::godot_warn!("rose: conversation {path}: {e}");
            return None;
        }
        let owner: Arc<dyn Any + Send + Sync> = Arc::new(Owner { entity_id: npc_entity });
        let open_check = con.initial_messages.first().map(|m| m.condition_function.clone()).unwrap_or_default();
        if !open_check.is_empty() {
            let result = vm.call_global_closure(cx, &open_check, &[Lua4Value::UserData(owner.clone())]);
            match result {
                Ok(values) if values.first().and_then(|v| v.to_i32().ok()).unwrap_or(0) >= 1 => {}
                Ok(_) => return None,
                Err(e) => {
                    godot::global::godot_warn!("rose: conversation {path} {open_check}: {e}");
                    return None;
                }
            }
        }
        let mut conversation =
            Self { npc_entity, title, message: String::new(), responses: Vec::new(), con, vm, owner };
        conversation.run_menu(cx, 0).then_some(conversation)
    }

    fn run_menu(&mut self, cx: &mut ScriptContext, menu_index: i32) -> bool {
        if menu_index < 0 || menu_index as usize >= self.con.menus.len() {
            return false;
        }
        let mut any = false;
        let count = self.con.menus[menu_index as usize].messages.len();
        for i in 0..count {
            let message = &self.con.menus[menu_index as usize].messages[i];
            let (condition, message_type, string_id, value, action) = (
                message.condition_function.clone(),
                &message.message_type,
                message.string_id,
                message.message_value,
                message.action_function.clone(),
            );
            let shows_menu = matches!(message_type, ConMessageType::NextMessage | ConMessageType::ShowMessage);
            if !condition.is_empty() {
                match self.vm.call_global_closure(cx, &condition, &[Lua4Value::UserData(self.owner.clone())]) {
                    Ok(values) if values.first().and_then(|v| v.to_i32().ok()).unwrap_or(0) != 0 => {}
                    Ok(_) => continue,
                    Err(e) => {
                        godot::global::godot_warn!("rose: conversation function {condition}: {e}");
                        continue;
                    }
                }
            }
            let Some(text) = cx.game.ltb_event.get_string(string_id as usize, 2) else { continue };
            let text = format_text(&text, cx.name, cx.ch.level);
            if shows_menu {
                // The NPC's message, then the choices under it. Like iROSE (CEvent::Conversation)
                // the loop goes on: a later message whose check passes replaces this one, so
                // the last match wins (a plain greeting comes first, quest lines after it).
                self.message = text;
                self.responses.clear();
                self.run_menu(cx, value);
            } else {
                self.responses.push(Response { text, action_function: action, menu_index: value });
            }
            any = true;
        }
        any
    }

    /// Pick a response. False when the conversation is over.
    pub fn choose(&mut self, cx: &mut ScriptContext, index: usize) -> bool {
        let Some(response) = self.responses.get(index).cloned() else { return true };
        if !response.action_function.is_empty() {
            if let Err(e) =
                self.vm.call_global_closure(cx, &response.action_function, &[Lua4Value::UserData(self.owner.clone())])
            {
                godot::global::godot_warn!("rose: conversation action {}: {e}", response.action_function);
            }
        }
        self.run_menu(cx, response.menu_index)
    }
}
