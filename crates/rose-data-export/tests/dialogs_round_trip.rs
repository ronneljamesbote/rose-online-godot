//! NPC dialogs turned into blocks do what their Lua scripts did. Each CON file is run the way
//! the game ran it before (the Lua 4 VM in `lua4_vm/`, with the conversation code of the old
//! `godot/rust/src/conversation.rs`) and as blocks (converted, written as the export, read
//! back), on the same made-up answers to every script function, and each conversation is
//! followed through its answers: what the NPC says, the answers shown and the actions must be
//! the same. Needs the client data: ROSE_DATA_IDX=<path to data.idx> (skipped without it).

mod lua4_vm;

use std::{
    any::Any,
    collections::HashSet,
    sync::{Arc, OnceLock},
};

use lua4_vm::{Lua4Function, Lua4VM, Lua4VMError, Lua4VMRustClosures, Lua4Value};
use rose_data::{
    AbilityType, Dialog, DialogAction, DialogCondition, DialogContext, DialogQuestValue, DialogVariable, DialogView,
};
use rose_data_export::{Format, DialogsExport};
use rose_file_readers::{ConFile, ConMessageType, LtbFile, VfsIndex, VirtualFilesystem};

/// The quest steps that aren't in the game: they never pass (the game's check finds no step).
static MISSING_STEPS: OnceLock<HashSet<String>> = OnceLock::new();
/// The quests that aren't in the game: nobody has them.
static MISSING_QUESTS: OnceLock<HashSet<usize>> = OnceLock::new();

/// The NPC's entity, as QF_getEventOwner answers it.
const NPC: f64 = 77.0;

/// Made-up answers: the same function and arguments always get the same answer, picked from
/// `pool` (the numbers the script compares with, and one either side) by a hash with `seed`.
struct Answers {
    seed: u64,
    pool: Vec<i32>,
}

impl Answers {
    fn hash(&self, key: &str) -> u64 {
        let mut h = self.seed ^ 0xcbf29ce484222325;
        for b in key.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h ^ (h >> 29)
    }

    fn number(&self, key: &str) -> i32 {
        self.pool[(self.hash(key) % self.pool.len() as u64) as usize]
    }

    fn passes(&self, step: &str) -> bool {
        if MISSING_STEPS.get().is_some_and(|m| m.contains(step)) {
            return false;
        }
        self.hash(&format!("step {step}")) % 2 == 0
    }

    /// QF_findQuest: -1 or a slot.
    fn slot(&self, quest: usize) -> i32 {
        if MISSING_QUESTS.get().is_some_and(|m| m.contains(&quest)) {
            return -1;
        }
        (self.hash(&format!("quest {quest}")) % 12) as i32 - 2
    }

    fn quest_slot(&self, quest: usize) -> i32 {
        self.slot(quest).max(-1)
    }
}

/// The old conversation: the script functions as the game answered them, on `Answers`.
struct OldScript<'a> {
    answers: &'a Answers,
    actions: Vec<String>,
}

struct Owner;

impl OldScript<'_> {
    fn call(&mut self, name: &str, args: &[Lua4Value]) -> Option<Vec<Lua4Value>> {
        let a = self.answers;
        let num = |i: usize| args.get(i).and_then(|v| v.to_usize().ok());
        let one = |v: i32| Some(vec![Lua4Value::Number(v as f64)]);
        match name {
            "QF_checkQuestCondition" => one(a.passes(&args.first()?.to_string().ok()?) as i32),
            "QF_doQuestTrigger" => {
                let step = args.first()?.to_string().ok()?;
                let ok = a.passes(&step);
                if ok {
                    self.actions.push(format!("step {step}"));
                }
                one(ok as i32)
            }
            "QF_findQuest" => one(num(0).map_or(-1, |q| a.quest_slot(q))),
            "QF_getEventOwner" => {
                let owner = args.first().and_then(|o| o.to_user_type::<Owner>().ok());
                Some(vec![Lua4Value::Number(if owner.is_some() { NPC } else { 0.0 })])
            }
            "QF_getEpisodeVAR" | "QF_getJobVAR" | "QF_getPlanetVAR" | "QF_getUserSwitch" => {
                one(num(0).map_or(-1, |i| a.number(&format!("{name} {i}"))))
            }
            "QF_getQuestCount" => one(a.number("quest count")),
            "QF_getQuestItemQuantity" => {
                let value = (|| {
                    let slot = a.quest_slot(num(0)?);
                    if slot < 0 {
                        return None;
                    }
                    let item = rose_data_irose::decode_item_base1000(num(1)?)?;
                    Some(a.number(&format!("item {} {:?} {}", num(0)?, item.item_type, item.item_number)))
                })();
                one(value.unwrap_or(-1))
            }
            "QF_getQuestSwitch" | "QF_getQuestVar" => {
                let value = num(0).and_then(|slot| Some(a.number(&format!("{name} {slot} {}", num(1)?))));
                one(value.unwrap_or(-1))
            }
            "QF_getNpcQuestZeroVal" => {
                let entity = num(0)?;
                one(if entity as f64 == NPC { a.number("npc switch") } else { 0 })
            }
            "GF_getVariable" => one(a.number(&format!("ability {}", args.first()?.to_i32().ok()?))),
            "GF_openStore" => {
                num(0)?;
                self.actions.push("open Shop".into());
                Some(vec![])
            }
            "GF_openBank" => self.push("open Storage"),
            "GF_openUpgrade" => self.push("open Refining"),
            "GF_openSeparate" => self.push("open Disassembly"),
            "GF_repair" => self.push("open Repair"),
            "GF_appraisal" => self.push("open Appraisal"),
            "GF_openDeliveryStore" => self.push("open Delivery"),
            "GF_setRevivePosition" => self.push("save point"),
            "GF_organizeClan" => self.push("found clan"),
            "GF_disorganizeClan" => self.push("disband clan"),
            "GF_getReviveZoneName" => Some(vec![Lua4Value::Nil]),
            "GF_GetMotionUseFile" => one(0),
            "GF_SetMotion" => Some(vec![]),
            _ => None,
        }
    }

    fn push(&mut self, what: &str) -> Option<Vec<Lua4Value>> {
        self.actions.push(what.into());
        Some(vec![])
    }
}

impl Lua4VMRustClosures for OldScript<'_> {
    fn call_rust_closure(&mut self, name: &str, parameters: Vec<Lua4Value>) -> Result<Vec<Lua4Value>, Lua4VMError> {
        self.call(name, &parameters).ok_or_else(|| Lua4VMError::GlobalNotFound(name.to_string()))
    }
}

const SV: [&str; 15] = [
    "SV_SEX", "SV_BIRTH", "SV_CLASS", "SV_UNION", "SV_RANK", "SV_FAME", "SV_STR", "SV_DEX", "SV_INT", "SV_CON", "SV_CHA",
    "SV_SEN", "SV_EXP", "SV_LEVEL", "SV_POINT",
];
const FUNCTIONS: [&str; 28] = [
    "QF_checkQuestCondition", "QF_doQuestTrigger", "QF_findQuest", "QF_getEventOwner", "QF_getEpisodeVAR", "QF_getJobVAR",
    "QF_getPlanetVAR", "QF_getQuestCount", "QF_getQuestID", "QF_getQuestItemQuantity", "QF_getQuestSwitch",
    "QF_getQuestVar", "QF_getUserSwitch", "QF_getNpcQuestZeroVal", "GF_getVariable", "GF_openStore", "GF_openBank",
    "GF_organizeClan", "GF_disorganizeClan", "GF_GetMotionUseFile", "GF_SetMotion", "GF_appraisal", "GF_repair",
    "GF_openUpgrade", "GF_openSeparate", "GF_openDeliveryStore", "GF_setRevivePosition", "GF_getReviveZoneName",
];

/// The old client's conversation (conversation.rs before the dialogs became blocks).
#[derive(Clone)]
struct OldConversation<'a> {
    con: &'a ConFile,
    text: &'a LtbFile,
    vm: Lua4VM,
    owner: Arc<dyn Any + Send + Sync>,
    message: Option<String>,
    responses: Vec<(String, String, i32)>,
}

impl<'a> OldConversation<'a> {
    fn open(con: &'a ConFile, text: &'a LtbFile, strict: bool, cx: &mut OldScript) -> Option<Self> {
        let mut vm = Lua4VM::new();
        vm.strict = strict;
        for (i, name) in SV.iter().enumerate() {
            vm.set_global(name.to_string(), Lua4Value::Number(i as f64));
        }
        for name in FUNCTIONS {
            vm.set_global(name.to_string(), Lua4Value::RustClosure(name.to_string()));
        }
        let function = Lua4Function::from_bytes(&con.script_binary).ok()?;
        vm.call_lua_function(cx, &function, &[]).ok()?;
        let owner: Arc<dyn Any + Send + Sync> = Arc::new(Owner);
        let open_check = con.initial_messages.first().map(|m| m.condition_function.clone()).unwrap_or_default();
        if !open_check.is_empty() {
            match vm.call_global_closure(cx, &open_check, &[Lua4Value::UserData(owner.clone())]) {
                Ok(values) if values.first().and_then(|v| v.to_i32().ok()).unwrap_or(0) >= 1 => {}
                _ => return None,
            }
        }
        let mut conversation = Self { con, text, vm, owner, message: None, responses: Vec::new() };
        conversation.run_menu(cx, 0, 0).then_some(conversation)
    }

    fn run_menu(&mut self, cx: &mut OldScript, menu_index: i32, depth: usize) -> bool {
        if menu_index < 0 || menu_index as usize >= self.con.menus.len() || depth > 32 {
            return false;
        }
        let mut any = false;
        for message in &self.con.menus[menu_index as usize].messages {
            let condition = message.condition_function.clone();
            if !condition.is_empty() {
                match self.vm.call_global_closure(cx, &condition, &[Lua4Value::UserData(self.owner.clone())]) {
                    Ok(values) if values.first().and_then(|v| v.to_i32().ok()).unwrap_or(0) != 0 => {}
                    _ => continue,
                }
            }
            let Some(text) = self.text.get_string(message.string_id as usize, 2) else { continue };
            // The game showed a NUL at the end as nothing.
            let text = text.replace('\0', "");
            if matches!(message.message_type, ConMessageType::NextMessage | ConMessageType::ShowMessage) {
                self.message = Some(text);
                self.responses.clear();
                self.run_menu(cx, message.message_value, depth + 1);
            } else {
                self.responses.push((text, message.action_function.clone(), message.message_value));
            }
            any = true;
        }
        any
    }

    fn choose(&mut self, cx: &mut OldScript, index: usize) -> bool {
        let Some((_, action, next)) = self.responses.get(index).cloned() else { return true };
        if !action.is_empty() {
            let _ = self.vm.call_global_closure(cx, &action, &[Lua4Value::UserData(self.owner.clone())]);
        }
        self.run_menu(cx, next, 0)
    }
}

/// The blocks on the same answers.
struct NewScript<'a> {
    answers: &'a Answers,
    actions: Vec<String>,
}

impl DialogContext for NewScript<'_> {
    fn check(&mut self, condition: &DialogCondition) -> bool {
        let a = self.answers;
        let cmp = |value: i32, operator, against| rose_data::dialog_compare(value, operator, against);
        match condition {
            DialogCondition::QuestStep { trigger, passes } => a.passes(trigger) == *passes,
            DialogCondition::Quest { quest, active } => (a.quest_slot(*quest as usize) >= 0) == *active,
            DialogCondition::QuestItem { quest, item, operator, value } => {
                let count = if a.quest_slot(*quest as usize) < 0 {
                    -1
                } else {
                    a.number(&format!("item {quest} {:?} {}", item.item_type, item.item_number))
                };
                cmp(count, *operator, *value)
            }
            DialogCondition::Ability { ability, operator, value } => {
                let index = SV.iter().position(|s| sv_ability(s) == *ability).unwrap();
                cmp(a.number(&format!("ability {index}")), *operator, *value)
            }
            DialogCondition::NpcSwitch { operator, value } => cmp(a.number("npc switch"), *operator, *value),
            DialogCondition::QuestSwitch { switch, operator, value } => {
                cmp(a.number(&format!("QF_getUserSwitch {switch}")), *operator, *value)
            }
            DialogCondition::Variable { variable, index, operator, value } => {
                let name = match variable {
                    DialogVariable::Episode => "QF_getEpisodeVAR",
                    DialogVariable::Job => "QF_getJobVAR",
                    DialogVariable::Planet => "QF_getPlanetVAR",
                };
                cmp(a.number(&format!("{name} {index}")), *operator, *value)
            }
            DialogCondition::QuestValue { quest, value_type, index, operator, value } => {
                let slot = a.quest_slot(*quest as usize);
                let name = match value_type {
                    DialogQuestValue::Switch => "QF_getQuestSwitch",
                    DialogQuestValue::Variable => "QF_getQuestVar",
                };
                let v = if slot < 0 { -1 } else { a.number(&format!("{name} {slot} {index}")) };
                cmp(v, *operator, *value)
            }
            DialogCondition::QuestCount { operator, value } => cmp(a.number("quest count"), *operator, *value),
        }
    }

    fn act(&mut self, action: &DialogAction) {
        let what = match action {
            DialogAction::QuestStep { trigger } => {
                if !self.answers.passes(trigger) {
                    return;
                }
                format!("step {trigger}")
            }
            DialogAction::OpenShop => "open Shop".into(),
            DialogAction::OpenStorage => "open Storage".into(),
            DialogAction::OpenRefining => "open Refining".into(),
            DialogAction::OpenDisassembly => "open Disassembly".into(),
            DialogAction::OpenRepair => "open Repair".into(),
            DialogAction::Appraisal => "open Appraisal".into(),
            DialogAction::OpenDelivery => "open Delivery".into(),
            DialogAction::SetSavePoint => "save point".into(),
            DialogAction::FoundClan => "found clan".into(),
            DialogAction::DisbandClan => "disband clan".into(),
        };
        self.actions.push(what);
    }
}

fn sv_ability(name: &str) -> AbilityType {
    match name {
        "SV_SEX" => AbilityType::Gender,
        "SV_BIRTH" => AbilityType::Birthstone,
        "SV_CLASS" => AbilityType::Job,
        "SV_UNION" => AbilityType::Union,
        "SV_RANK" => AbilityType::Rank,
        "SV_FAME" => AbilityType::Fame,
        "SV_STR" => AbilityType::Strength,
        "SV_DEX" => AbilityType::Dexterity,
        "SV_INT" => AbilityType::Intelligence,
        "SV_CON" => AbilityType::Concentration,
        "SV_CHA" => AbilityType::Charm,
        "SV_SEN" => AbilityType::Sense,
        "SV_EXP" => AbilityType::Experience,
        "SV_LEVEL" => AbilityType::Level,
        _ => AbilityType::BonusPoint,
    }
}

/// The numbers a script compares with, one either side, and -1 to 2.
fn pool(con: &ConFile) -> Vec<i32> {
    let mut out = vec![-1, 0, 1, 2];
    fn walk(f: &Lua4Function, out: &mut Vec<i32>) {
        for i in &f.instructions {
            if let rose_file_readers::lua4::Lua4Instruction::OP_PUSHINT(v) = *i {
                out.extend([v - 1, v, v + 1]);
            }
        }
        out.extend(f.constant_numbers.iter().map(|n| *n as i32));
        for g in &f.constant_functions {
            walk(g, out);
        }
    }
    if let Ok(main) = Lua4Function::from_bytes(&con.script_binary) {
        walk(&main, &mut out);
    }
    out.sort();
    out.dedup();
    out
}

/// What one step of a conversation shows: the NPC's line and the answers, then the actions.
fn old_view(c: &OldConversation, actions: &[String]) -> String {
    format!("{:?} {:?} {:?}", c.message, c.responses.iter().map(|r| &r.0).collect::<Vec<_>>(), actions)
}

fn new_view(d: &Dialog, v: &DialogView, actions: &[String]) -> String {
    let text = |at: &(usize, usize)| d.line(*at).unwrap().text.clone();
    format!("{:?} {:?} {:?}", v.message.as_ref().map(text), v.answers.iter().map(text).collect::<Vec<_>>(), actions)
}

/// Answers deep a walk follows, and conversation steps one set of answers may try.
const MAX_DEPTH: usize = 8;
const MAX_STEPS: usize = 400;

/// Both conversations after the same answers so far, with what each has done.
#[derive(Clone)]
struct Both<'a> {
    old: OldConversation<'a>,
    new: DialogView,
    old_actions: Vec<String>,
    new_actions: Vec<String>,
}

/// Follows every answer from `at` (up to `MAX_DEPTH` deep), comparing what both show and do.
/// The first difference found, if any.
fn walk(both: Both, dialog: &Dialog, answers: &Answers, depth: usize, steps: &mut usize, seen: &mut HashSet<(usize, usize)>) -> Option<String> {
    let a = old_view(&both.old, &both.old_actions);
    let b = new_view(dialog, &both.new, &both.new_actions);
    seen.extend(both.new.message.iter().chain(&both.new.answers).copied());
    if a != b {
        return Some(format!("after {depth} answers:\n  old {a}\n  new {b}"));
    }
    if depth >= MAX_DEPTH {
        return None;
    }
    for pick in 0..both.old.responses.len() {
        *steps += 1;
        if *steps > MAX_STEPS {
            return None;
        }
        let mut next = both.clone();
        let mut old_cx = OldScript { answers, actions: std::mem::take(&mut next.old_actions) };
        let mut new_cx = NewScript { answers, actions: std::mem::take(&mut next.new_actions) };
        let go_on_old = next.old.choose(&mut old_cx, pick);
        let go_on_new = dialog.choose(&mut next.new, pick, &mut new_cx);
        (next.old_actions, next.new_actions) = (old_cx.actions, new_cx.actions);
        if go_on_old != go_on_new {
            return Some(format!("answer {pick} after {depth}: goes on {go_on_old} / {go_on_new}"));
        }
        if !go_on_old {
            if next.old_actions != next.new_actions {
                return Some(format!("answer {pick} ends: {:?} / {:?}", next.old_actions, next.new_actions));
            }
            continue;
        }
        if let Some(d) = walk(next, dialog, answers, depth + 1, steps, seen) {
            return Some(d);
        }
    }
    None
}

/// Runs one dialog both ways on `seeds` sets of answers, following every answer. Returns how
/// many sets of answers there were and how many differed (and prints the first difference).
fn compare_runs(con: &ConFile, text: &LtbFile, dialog: &Dialog, seeds: u64, strict: bool, seen: &mut HashSet<(usize, usize)>) -> (usize, usize) {
    let pool = pool(con);
    let (mut runs, mut differ) = (0, 0);
    for seed in 0..seeds {
        let answers = Answers { seed, pool: pool.clone() };
        let mut old_cx = OldScript { answers: &answers, actions: Vec::new() };
        let mut new_cx = NewScript { answers: &answers, actions: Vec::new() };
        let old = OldConversation::open(con, text, strict, &mut old_cx);
        let new = dialog.open(&mut new_cx);
        runs += 1;
        let difference = match (old, new) {
            (None, None) => None,
            (Some(old), Some(new)) => {
                let both = Both { old, new, old_actions: old_cx.actions, new_actions: new_cx.actions };
                walk(both, dialog, &answers, 0, &mut 0, seen)
            }
            (o, n) => Some(format!("opens {} / {}", o.is_some(), n.is_some())),
        };
        if let Some(d) = difference {
            differ += 1;
            if differ == 1 && !strict {
                eprintln!("{} answers {seed}: {d}", dialog.key);
            }
        }
    }
    (runs, differ)
}

#[test]
fn dialogs_do_what_their_scripts_did() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    let vfs = VirtualFilesystem::new(vec![Box::new(VfsIndex::load(std::path::Path::new(&data_idx)).unwrap())]);
    let (db, notes) = rose_data_irose::get_dialog_database(&vfs).unwrap();
    for note in notes.iter().filter(|n| !n.contains("no text left out")) {
        eprintln!("note: {note}");
    }
    // Through the export (without the quest steps that aren't in the game, as --convert-data
    // writes it) and back.
    let strings = rose_data_irose::get_string_database(&vfs, 1).unwrap();
    let quests = rose_data_irose::get_quest_database(&vfs, strings).unwrap();
    let mut export = DialogsExport::from_database(&db);
    // The quests the game has: LIST_QUEST's, and those a step names.
    let export_quests = rose_data_export::QuestsExport::from_database(
        &quests,
        &rose_data_export::IroseCodes {
            decode_ability: rose_data_irose::decode_ability_type,
            encode_ability: rose_data_irose::encode_ability_type,
            decode_item_type: rose_data_irose::decode_item_type,
            encode_item_type: rose_data_irose::encode_item_type,
        },
    )
    .unwrap();
    let mut known: HashSet<u16> = export_quests.quests.iter().map(|q| q.id).collect();
    for trigger in &export_quests.triggers {
        known.extend(trigger.condition.iter().filter_map(|c| c.quest()));
        known.extend(trigger.reward.iter().filter_map(|r| r.quest()));
    }
    let (mut steps, mut missing_quests) = (HashSet::new(), HashSet::new());
    for note in export.without_missing(|step| quests.get_trigger_by_name(step).is_some(), |q| known.contains(&q)) {
        eprintln!("note: {note}");
        let what = note.split(": ").nth(1).unwrap();
        if let Some(step) = what.strip_prefix("quest step ") {
            steps.insert(step.split(' ').next().unwrap().to_string());
        } else if let Some(quest) = what.strip_prefix("quest ") {
            missing_quests.insert(quest.split(' ').next().unwrap().parse().unwrap());
        }
    }
    MISSING_STEPS.set(steps).unwrap();
    MISSING_QUESTS.set(missing_quests).unwrap();
    let db = export.to_database().unwrap();
    // The repository's export (from the resources) is the same, but for the names (there the
    // NPC's, here LIST_EVENT's).
    let exported = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../export/data/dialogs.toml");
    if let Ok(text) = std::fs::read_to_string(&exported) {
        let mut repo = DialogsExport::from_files_as(Format::Toml, |_| Ok(Some(text.clone()))).unwrap().unwrap();
        let mut ours = DialogsExport::from_database(&db);
        for d in repo.dialogs.iter_mut().chain(ours.dialogs.iter_mut()) {
            d.name.clear();
        }
        let differences =
            rose_data_export::compare_dialog_databases(&ours.to_database().unwrap(), &repo.to_database().unwrap());
        assert!(differences.is_empty(), "export/data/dialogs.toml: {}", differences.join("\n"));
        eprintln!("export/data/dialogs.toml: the same dialogs");
    }
    let files = export.to_files_as(Format::Toml).unwrap();
    let read = DialogsExport::from_files_as(Format::Toml, |name| Ok(files.iter().find(|(n, _)| *n == name).map(|(_, t)| t.clone())))
        .unwrap()
        .unwrap();
    let back = read.to_database().unwrap();
    let differences = rose_data_export::compare_dialog_databases(&db, &back);
    assert!(differences.is_empty(), "{}", differences.join("\n"));

    let text = vfs.read_file::<LtbFile, _>(rose_data_irose::DIALOG_TEXT_FILE).unwrap();
    let seeds: u64 = std::env::var("DIALOG_SEEDS").ok().and_then(|s| s.parse().ok()).unwrap_or(300);
    let conversations = rose_data_irose::get_npc_conversations(&vfs).unwrap();
    let (mut runs, mut differ, mut strict_differ, mut dialogs, mut lines, mut lines_seen) = (0, 0, 0, 0, 0, 0);
    let mut differing = Vec::new();
    for dialog in back.iter() {
        let data = &conversations[&dialog.key];
        let con = vfs.read_file::<ConFile, _>(data.filename.as_str()).unwrap();
        let mut seen = HashSet::new();
        let (r, d) = compare_runs(&con, &text, dialog, seeds, false, &mut seen);
        let (_, s) = compare_runs(&con, &text, dialog, seeds, true, &mut HashSet::new());
        lines_seen += seen.len();
        lines += dialog.menus.iter().map(|m| m.lines.len()).sum::<usize>();
        runs += r;
        differ += d;
        strict_differ += s;
        dialogs += 1;
        if d > 0 {
            differing.push(dialog.key.clone());
        }
        if s > 0 {
            eprintln!("{}: {s} of {r} runs differ from the game's strict VM (unset globals)", dialog.key);
        }
    }
    let bytes: usize = files.iter().map(|(_, t)| t.len()).sum();
    eprintln!(
        "{dialogs} dialogs, {runs} runs, {differ} differ, {strict_differ} differ from the strict VM; {lines_seen} of {lines} lines shown; export {bytes} bytes"
    );
    assert!(differing.is_empty(), "dialogs that differ: {differing:?}");
}

/// Every script function a dialog uses (its opening check, each line's condition and each
/// answer's action), on its own: the cases give the same answer and actions as the function
/// on many sets of answers, also for lines a conversation rarely reaches.
#[test]
fn each_condition_and_action_does_what_its_function_did() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    use rose_data_irose::{DialogFunctionUse, DialogScript};
    let vfs = VirtualFilesystem::new(vec![Box::new(VfsIndex::load(std::path::Path::new(&data_idx)).unwrap())]);
    let seeds: u64 = std::env::var("DIALOG_SEEDS").ok().and_then(|s| s.parse().ok()).unwrap_or(300);
    let mut keys: Vec<_> = rose_data_irose::get_npc_conversations(&vfs).unwrap().into_iter().collect();
    keys.sort_by(|a, b| a.0.cmp(&b.0));
    let (mut functions, mut checks, mut failed) = (0, 0, Vec::new());
    for (key, data) in keys {
        let Ok(con) = vfs.read_file::<ConFile, _>(data.filename.as_str()) else { continue };
        let Ok(script) = DialogScript::load(&con) else { continue };
        let mut uses = vec![(con.initial_messages.first().map(|m| m.condition_function.clone()).unwrap_or_default(), DialogFunctionUse::Opening)];
        for message in con.menus.iter().flat_map(|m| &m.messages) {
            uses.push((message.condition_function.clone(), DialogFunctionUse::Line));
            uses.push((message.action_function.clone(), DialogFunctionUse::Action));
        }
        uses.sort_by(|a, b| (&a.0, a.1 as u8).cmp(&(&b.0, b.1 as u8)));
        uses.dedup();
        let pool = pool(&con);
        for (function, used) in uses {
            if function.is_empty() {
                continue;
            }
            let cases = script.cases(&function, used, &key, &mut Vec::new()).unwrap();
            functions += 1;
            for seed in 0..seeds {
                let answers = Answers { seed, pool: pool.clone() };
                let mut old_cx = OldScript { answers: &answers, actions: Vec::new() };
                let mut vm = Lua4VM::new();
                for (i, name) in SV.iter().enumerate() {
                    vm.set_global(name.to_string(), Lua4Value::Number(i as f64));
                }
                for name in FUNCTIONS {
                    vm.set_global(name.to_string(), Lua4Value::RustClosure(name.to_string()));
                }
                let main = Lua4Function::from_bytes(&con.script_binary).unwrap();
                vm.call_lua_function(&mut old_cx, &main, &[]).unwrap();
                let owner: Arc<dyn Any + Send + Sync> = Arc::new(Owner);
                let answer = vm.call_global_closure(&mut old_cx, &function, &[Lua4Value::UserData(owner)]);
                let number = answer.ok().and_then(|v| v.first().and_then(|v| v.to_i32().ok())).unwrap_or(0);
                let old = match used {
                    DialogFunctionUse::Opening => (number >= 1, old_cx.actions),
                    DialogFunctionUse::Line => (number != 0, old_cx.actions),
                    DialogFunctionUse::Action => (false, old_cx.actions),
                };
                let mut new_cx = NewScript { answers: &answers, actions: Vec::new() };
                let shows = match &cases {
                    None => false,
                    Some(c) if c.is_empty() => used != DialogFunctionUse::Action,
                    Some(c) => rose_data::run_dialog_cases(c, &mut new_cx),
                };
                let new = (shows && used != DialogFunctionUse::Action, new_cx.actions);
                checks += 1;
                if old != new {
                    failed.push(format!("{key} {function} ({used:?}) answers {seed}: {old:?} / {new:?}"));
                    break;
                }
            }
        }
    }
    eprintln!("{functions} functions checked on {checks} sets of answers");
    assert!(failed.is_empty(), "{}", failed.join("\n"));
}
