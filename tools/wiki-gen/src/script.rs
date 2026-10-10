//! NPC dialogs: a CON file's menus of messages, and what its Lua 4 script functions check
//! and do. The script is read, not run: each function's calls are collected with their
//! literal arguments (QF_doQuestTrigger("x"), GF_openStore(...), GF_getVariable(SV_LEVEL) >= 10).

use std::{collections::HashMap, sync::Arc};

use rose_file_readers::{ConFile, ConMessageType};

use crate::lua4::{Lua4Function, Lua4Instruction};

#[derive(Clone, Debug, PartialEq)]
enum Sym {
    Str(String),
    Num(f64),
    Global(String),
    Call(String, Vec<Sym>),
    Unknown,
}

/// One thing a script function checks or does.
#[derive(Clone, Debug, PartialEq)]
pub enum Effect {
    /// QF_checkQuestCondition: passes when the quest trigger's conditions pass.
    CheckTrigger(String),
    /// QF_doQuestTrigger: runs the quest trigger (checks it, then gives its rewards).
    RunTrigger(String),
    /// GF_ functions that open a window or change something.
    Service(&'static str),
    /// QF_findQuest: whether the quest is in your quest list.
    HasQuest(usize, bool),
    /// QF_getQuestItemQuantity: how many of a quest item you carry for a quest.
    Carries { quest: usize, item: usize, op: &'static str, count: i64 },
    /// A comparison the script makes, already in words.
    Check(String),
}

/// What each global function of a dialog script does, by function name.
pub struct Script {
    functions: HashMap<String, Arc<Lua4Function>>,
}

impl Script {
    pub fn load(con: &ConFile) -> Option<Self> {
        let main = Lua4Function::from_bytes(&con.script_binary).ok()?;
        let mut functions = HashMap::new();
        let mut pending: Option<Arc<Lua4Function>> = None;
        for instruction in &main.instructions {
            match *instruction {
                Lua4Instruction::OP_CLOSURE(k, _) => pending = main.constant_functions.get(k as usize).cloned(),
                Lua4Instruction::OP_SETGLOBAL(k) => {
                    if let (Some(f), Some(name)) = (pending.take(), main.constant_strings.get(k as usize)) {
                        functions.insert(name.clone(), f);
                    }
                }
                _ => pending = None,
            }
        }
        Some(Self { functions })
    }

    pub fn dump(&self) {
        let mut names: Vec<_> = self.functions.keys().collect();
        names.sort();
        for n in names {
            let f = &self.functions[n];
            eprintln!("function {n} strings {:?} numbers {:?}", f.constant_strings, f.constant_numbers);
            for i in &f.instructions {
                eprintln!("    {i:?}");
            }
        }
    }

    /// A check function that only ever answers 0 (menus kept in the file but never shown).
    pub fn never(&self, name: &str) -> bool {
        let Some(f) = self.functions.get(name) else { return false };
        let mut last_int = None;
        let mut returns = 0;
        for i in &f.instructions {
            match *i {
                Lua4Instruction::OP_CALL(..) | Lua4Instruction::OP_TAILCALL(..) => return false,
                Lua4Instruction::OP_PUSHINT(v) => last_int = Some(v),
                Lua4Instruction::OP_RETURN(_) => {
                    if last_int != Some(0) {
                        return false;
                    }
                    returns += 1;
                }
                _ => {}
            }
        }
        returns > 0
    }

    pub fn effects(&self, name: &str) -> Vec<Effect> {
        let mut out = Vec::new();
        self.collect(name, 0, &mut out);
        out
    }

    fn collect(&self, name: &str, depth: usize, out: &mut Vec<Effect>) {
        let Some(function) = self.functions.get(name) else { return };
        if depth > 4 {
            return;
        }
        let mut stack: Vec<Sym> = vec![Sym::Unknown; function.num_parameters as usize];
        let push_effect = |out: &mut Vec<Effect>, e: Effect| {
            if !out.contains(&e) {
                out.push(e);
            }
        };
        for instruction in &function.instructions {
            match *instruction {
                Lua4Instruction::OP_END | Lua4Instruction::OP_RETURN(_) => {}
                Lua4Instruction::OP_CALL(a, results) | Lua4Instruction::OP_TAILCALL(a, results) => {
                    let base = a as usize;
                    if base >= stack.len() {
                        stack.truncate(base);
                        continue;
                    }
                    let args = stack.split_off(base + 1);
                    let callee = stack.pop().unwrap_or(Sym::Unknown);
                    if let Sym::Global(callee) = callee {
                        if self.functions.contains_key(&callee) {
                            self.collect(&callee, depth + 1, out);
                        } else if let Some(e) = call_effect(&callee, &args) {
                            push_effect(out, e);
                        }
                        let results = if results == 255 { 1 } else { results };
                        for i in 0..results {
                            stack.push(if i == 0 { Sym::Call(callee.clone(), args.clone()) } else { Sym::Unknown });
                        }
                    } else {
                        for _ in 0..results.min(8) {
                            stack.push(Sym::Unknown);
                        }
                    }
                }
                Lua4Instruction::OP_PUSHNIL(n) => stack.extend(std::iter::repeat(Sym::Unknown).take(n as usize)),
                Lua4Instruction::OP_POP(n) => {
                    let keep = stack.len().saturating_sub(n as usize);
                    stack.truncate(keep);
                }
                Lua4Instruction::OP_PUSHINT(v) => stack.push(Sym::Num(v as f64)),
                Lua4Instruction::OP_PUSHSTRING(k) => {
                    stack.push(function.constant_strings.get(k as usize).cloned().map_or(Sym::Unknown, Sym::Str))
                }
                Lua4Instruction::OP_PUSHNUM(k) => {
                    stack.push(function.constant_numbers.get(k as usize).copied().map_or(Sym::Unknown, Sym::Num))
                }
                Lua4Instruction::OP_PUSHNEGNUM(k) => {
                    stack.push(function.constant_numbers.get(k as usize).map_or(Sym::Unknown, |n| Sym::Num(-n)))
                }
                Lua4Instruction::OP_GETLOCAL(i) => stack.push(stack.get(i as usize).cloned().unwrap_or(Sym::Unknown)),
                Lua4Instruction::OP_GETGLOBAL(k) => {
                    stack.push(function.constant_strings.get(k as usize).cloned().map_or(Sym::Unknown, Sym::Global))
                }
                Lua4Instruction::OP_SETLOCAL(i) => {
                    let v = stack.pop().unwrap_or(Sym::Unknown);
                    if let Some(slot) = stack.get_mut(i as usize) {
                        *slot = v;
                    }
                }
                Lua4Instruction::OP_SETGLOBAL(_) => {
                    stack.pop();
                }
                Lua4Instruction::OP_JMPNE(_)
                | Lua4Instruction::OP_JMPEQ(_)
                | Lua4Instruction::OP_JMPLT(_)
                | Lua4Instruction::OP_JMPLE(_)
                | Lua4Instruction::OP_JMPGT(_)
                | Lua4Instruction::OP_JMPGE(_) => {
                    let rhs = stack.pop().unwrap_or(Sym::Unknown);
                    let lhs = stack.pop().unwrap_or(Sym::Unknown);
                    // An `if a OP b then` jumps over its body when the test fails, so the
                    // source's test is the opposite of the jump.
                    let op = match *instruction {
                        Lua4Instruction::OP_JMPNE(_) => "=",
                        Lua4Instruction::OP_JMPEQ(_) => "≠",
                        Lua4Instruction::OP_JMPLT(_) => "≥",
                        Lua4Instruction::OP_JMPLE(_) => ">",
                        Lua4Instruction::OP_JMPGT(_) => "≤",
                        _ => "<",
                    };
                    if let Some(e) = comparison(&lhs, op, &rhs) {
                        push_effect(out, e);
                    }
                }
                Lua4Instruction::OP_JMPT(_) | Lua4Instruction::OP_JMPF(_) => {
                    stack.pop();
                }
                Lua4Instruction::OP_PUSHNILJMP => stack.push(Sym::Unknown),
                Lua4Instruction::OP_CLOSURE(_, b) => {
                    let keep = stack.len().saturating_sub(b as usize);
                    stack.truncate(keep);
                    stack.push(Sym::Unknown);
                }
                Lua4Instruction::OP_ADD
                | Lua4Instruction::OP_SUB
                | Lua4Instruction::OP_MULT
                | Lua4Instruction::OP_DIV
                | Lua4Instruction::OP_POW
                | Lua4Instruction::OP_GETTABLE => {
                    stack.pop();
                    stack.pop();
                    stack.push(Sym::Unknown);
                }
                Lua4Instruction::OP_ADDI(_)
                | Lua4Instruction::OP_MINUS
                | Lua4Instruction::OP_NOT
                | Lua4Instruction::OP_GETDOTTED(_)
                | Lua4Instruction::OP_GETINDEXED(_) => {
                    stack.pop();
                    stack.push(Sym::Unknown);
                }
                Lua4Instruction::OP_CONCAT(n) => {
                    let keep = stack.len().saturating_sub(n as usize);
                    stack.truncate(keep);
                    stack.push(Sym::Unknown);
                }
                // Jumps that keep or drop a value (and/or), loops and tables: rare in dialog
                // scripts; the stack picture may drift, which only loses detail.
                _ => {}
            }
        }
    }
}

fn arg_str(args: &[Sym], i: usize) -> Option<String> {
    match args.get(i)? {
        Sym::Str(s) => Some(s.clone()),
        _ => None,
    }
}

fn call_effect(name: &str, args: &[Sym]) -> Option<Effect> {
    Some(match name {
        "QF_checkQuestCondition" => Effect::CheckTrigger(arg_str(args, 0)?),
        "QF_doQuestTrigger" => Effect::RunTrigger(arg_str(args, 0)?),
        "GF_openStore" => Effect::Service("opens the shop"),
        "GF_openBank" => Effect::Service("opens storage"),
        "GF_openUpgrade" => Effect::Service("opens refining"),
        "GF_openSeparate" => Effect::Service("opens disassembly"),
        "GF_repair" => Effect::Service("opens repair"),
        "GF_appraisal" => Effect::Service("appraises items (not in game yet)"),
        "GF_openDeliveryStore" => Effect::Service("opens item delivery (not in game yet)"),
        "GF_setRevivePosition" => Effect::Service("saves this town as where you get up after dying"),
        "GF_organizeClan" => Effect::Service("founds a clan (not in game yet)"),
        "GF_disorganizeClan" => Effect::Service("disbands your clan (not in game yet)"),
        _ => return None,
    })
}

const SV_NAMES: [(&str, &str); 15] = [
    ("SV_SEX", "gender"),
    ("SV_BIRTH", "birthstone"),
    ("SV_CLASS", "job"),
    ("SV_UNION", "union"),
    ("SV_RANK", "rank"),
    ("SV_FAME", "fame"),
    ("SV_STR", "STR"),
    ("SV_DEX", "DEX"),
    ("SV_INT", "INT"),
    ("SV_CON", "CON"),
    ("SV_CHA", "CHA"),
    ("SV_SEN", "SEN"),
    ("SV_EXP", "experience"),
    ("SV_LEVEL", "level"),
    ("SV_POINT", "stat points"),
];

fn describe(sym: &Sym) -> Option<String> {
    Some(match sym {
        Sym::Num(n) => format!("{n}"),
        Sym::Str(s) => format!("\"{s}\""),
        Sym::Call(name, args) => match name.as_str() {
            "GF_getVariable" => match args.first()? {
                Sym::Global(g) => format!("your {}", SV_NAMES.iter().find(|(k, _)| k == g)?.1),
                Sym::Num(n) => format!("your {}", SV_NAMES.get(*n as usize)?.1),
                _ => return None,
            },
            "QF_getQuestCount" => "your number of active quests".into(),
            "QF_findQuest" => format!("quest {} in your quest list", describe(args.first()?)?),
            "QF_getEpisodeVAR" => format!("episode variable {}", describe(args.first()?)?),
            "QF_getJobVAR" => format!("job variable {}", describe(args.first()?)?),
            "QF_getPlanetVAR" => format!("planet variable {}", describe(args.first()?)?),
            "QF_getUserSwitch" => format!("quest switch {}", describe(args.first()?)?),
            "QF_getNpcQuestZeroVal" => "the NPC's quest switch".into(),
            _ => return None,
        },
        _ => return None,
    })
}

fn comparison(lhs: &Sym, op: &'static str, rhs: &Sym) -> Option<Effect> {
    if let (Sym::Call(name, args), Sym::Num(n)) = (lhs, rhs) {
        let num = |i: usize| match args.get(i) {
            Some(Sym::Num(v)) => Some(*v as usize),
            _ => None,
        };
        match name.as_str() {
            // Quest checks already show as their own effect.
            "QF_checkQuestCondition" | "QF_doQuestTrigger" => return None,
            "QF_findQuest" => {
                let quest = num(0)?;
                let has = match (op, *n as i64) {
                    ("=", -1) | ("<", 0) | ("≤", -1) => false,
                    ("≥", 0) | (">", -1) | ("≠", -1) => true,
                    _ => return Some(Effect::Check(format!("quest {quest} slot {op} {n}"))),
                };
                return Some(Effect::HasQuest(quest, has));
            }
            "QF_getQuestItemQuantity" => {
                return Some(Effect::Carries { quest: num(0)?, item: num(1)?, op, count: *n as i64 })
            }
            _ => {}
        }
    }
    Some(Effect::Check(format!("{} {op} {}", describe(lhs)?, describe(rhs)?)))
}

/// One line of the dialog tree.
pub enum Line {
    /// The NPC's message, with the lines under it (the player's choices).
    Npc { text: u32, condition: String, children: Vec<Line> },
    /// A choice the player picks; `next` is what the NPC says after it.
    Choice { text: u32, condition: String, action: String, next: Vec<Line> },
    /// The same menu again, shown once already above.
    Again,
}

/// The dialog as a tree, from menu 0, the way iROSE walks it (CEvent::Conversation).
pub fn tree(con: &ConFile, script: Option<&Script>) -> Vec<Line> {
    let mut seen = Vec::new();
    menu(con, script, 0, &mut seen, 0)
}

fn menu(con: &ConFile, script: Option<&Script>, index: i32, seen: &mut Vec<i32>, depth: usize) -> Vec<Line> {
    if index < 0 || index as usize >= con.menus.len() {
        return Vec::new();
    }
    if seen.contains(&index) || depth > 12 {
        return vec![Line::Again];
    }
    seen.push(index);
    let mut out = Vec::new();
    for message in &con.menus[index as usize].messages {
        let condition = message.condition_function.clone();
        if !condition.is_empty() && script.is_some_and(|s| s.never(&condition)) {
            continue;
        }
        match message.message_type {
            ConMessageType::NextMessage | ConMessageType::ShowMessage => out.push(Line::Npc {
                text: message.string_id,
                condition,
                children: menu(con, script, message.message_value, seen, depth + 1),
            }),
            _ => out.push(Line::Choice {
                text: message.string_id,
                condition,
                action: message.action_function.clone(),
                next: menu(con, script, message.message_value, seen, depth + 1),
            }),
        }
    }
    out
}
