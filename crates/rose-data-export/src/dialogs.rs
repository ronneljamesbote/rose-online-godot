//! NPC dialogs: `dialogs.toml`, one `[[dialog]]` per dialog (iROSE's CON files with their
//! Lua scripts and the ULNGTB_CON.LTB text). The keys are the property names of the Godot
//! resources they are exported from: `RoseDialog`, `RoseDialogMenu`, `RoseDialogLine`,
//! `RoseDialogCase` and the blocks, `RoseDialogIf<kind>` (conditions) and `RoseDialogDo<kind>`
//! (actions).
//!
//! A dialog's `[[dialog.menu]]` tables hold `[[dialog.menu.line]]` tables: what the NPC says
//! (`kind = "Npc"`) or an answer (`kind = "Answer"`), its text, the menu it leads to
//! (`next_menu`, -1 for none) and its cases: `show_if` (whether it shows) and `on_choose`
//! (what picking it does). A case has `[[...condition]]` and `[[...action]]` tables, each with
//! a `kind` and that kind's fields. How a dialog runs: `rose_data::Dialog`.
//!
//! A quest step is named by its name (`124-01`), a quest by its number and an item by its
//! export name (`quest/601`). Only the game reads dialogs (the server checks the quest steps
//! a dialog asks for again).

use std::collections::{BTreeSet, HashSet};

use anyhow::{bail, Context};
use rose_data::{
    AbilityType, Dialog, DialogAction, DialogCase, DialogCondition, DialogDatabase, DialogLine, DialogLineKind,
    DialogMenu, DialogQuestValue, DialogVariable,
};
use serde::{Deserialize, Serialize};

use crate::npcs::{item_name, parse_item_name};
use crate::quests::Compare;
use crate::Format;

pub const DIALOGS_FILE: &str = "dialogs";

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

fn yes() -> bool {
    true
}

fn is_yes(value: &bool) -> bool {
    *value
}

fn no_menu() -> i32 {
    -1
}

fn is_no_menu(value: &i32) -> bool {
    *value == -1
}

/// Which variable list a `Variable` condition reads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariableList {
    #[default]
    Episode,
    Job,
    Planet,
}

pub const VARIABLE_LIST_NAMES: [&str; 3] = ["Episode", "Job", "Planet"];

/// Which quest value a `QuestValue` condition reads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestValueType {
    #[default]
    Switch,
    Variable,
}

pub const QUEST_VALUE_NAMES: [&str; 2] = ["Switch", "Variable"];

/// The window an `Open` action opens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Window {
    #[default]
    Shop,
    Storage,
    Refining,
    Disassembly,
    Repair,
    /// Not in the game yet (the game says so).
    Appraisal,
    Delivery,
}

pub const WINDOW_NAMES: [&str; 7] = ["Shop", "Storage", "Refining", "Disassembly", "Repair", "Appraisal", "Delivery"];

/// What a case checks. A value that isn't there reads as -1.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum DialogConditionExport {
    /// The quest step's conditions pass (or, `passes = false`, don't).
    QuestStep {
        step: String,
        #[serde(default = "yes", skip_serializing_if = "is_yes")]
        passes: bool,
    },
    /// The quest is in your quest list (or, `active = false`, isn't).
    Quest {
        quest: u16,
        #[serde(default = "yes", skip_serializing_if = "is_yes")]
        active: bool,
    },
    /// How many of `item` you carry for `quest`.
    QuestItem { quest: u16, item: String, operator: Compare, value: i32 },
    Ability { ability: AbilityType, operator: Compare, value: i32 },
    /// The NPC's own quest switch.
    NpcSwitch { operator: Compare, value: i32 },
    QuestSwitch { switch: u16, operator: Compare, value: i32 },
    Variable { variable: VariableList, index: u16, operator: Compare, value: i32 },
    QuestValue { quest: u16, value_type: QuestValueType, index: u16, operator: Compare, value: i32 },
    QuestCount { operator: Compare, value: i32 },
}

pub const CONDITION_KINDS: [&str; 9] =
    ["QuestStep", "Quest", "QuestItem", "Ability", "NpcSwitch", "QuestSwitch", "Variable", "QuestValue", "QuestCount"];

/// What a case does.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum DialogActionExport {
    /// Runs the quest step (the server checks it again).
    QuestStep { step: String },
    Open { window: Window },
    SetSavePoint,
    /// Not in the game yet.
    FoundClan,
    DisbandClan,
}

pub const ACTION_KINDS: [&str; 5] = ["QuestStep", "Open", "SetSavePoint", "FoundClan", "DisbandClan"];

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DialogCaseExport {
    #[serde(skip_serializing_if = "is_default")]
    pub condition: Vec<DialogConditionExport>,
    #[serde(skip_serializing_if = "is_default")]
    pub action: Vec<DialogActionExport>,
    /// For a line's `show_if`: whether the line shows. Not used in `on_choose`.
    #[serde(skip_serializing_if = "is_yes")]
    pub show: bool,
}

impl Default for DialogCaseExport {
    fn default() -> Self {
        Self { condition: Vec::new(), action: Vec::new(), show: true }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineKind {
    #[default]
    Npc,
    Answer,
}

pub const LINE_KIND_NAMES: [&str; 2] = ["Npc", "Answer"];

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DialogLineExport {
    pub kind: LineKind,
    #[serde(skip_serializing_if = "is_default")]
    pub text: String,
    /// The menu it leads to: an NPC line's answers, or what comes after an answer. -1: none
    /// (an answer that ends the conversation).
    #[serde(default = "no_menu", skip_serializing_if = "is_no_menu")]
    pub next_menu: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub show_if: Vec<DialogCaseExport>,
    #[serde(skip_serializing_if = "is_default")]
    pub on_choose: Vec<DialogCaseExport>,
}

impl Default for DialogLineExport {
    fn default() -> Self {
        Self { kind: LineKind::Npc, text: String::new(), next_menu: -1, show_if: Vec::new(), on_choose: Vec::new() }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DialogMenuExport {
    #[serde(skip_serializing_if = "is_default")]
    pub line: Vec<DialogLineExport>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DialogExport {
    /// What NPC placements call it (`EM20-001.con`).
    pub key: String,
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    #[serde(skip_serializing_if = "is_default")]
    pub open_if: Vec<DialogCaseExport>,
    #[serde(skip_serializing_if = "is_default")]
    pub menu: Vec<DialogMenuExport>,
}

fn condition_export(c: &DialogCondition) -> DialogConditionExport {
    use DialogConditionExport as E;
    match c {
        DialogCondition::QuestStep { trigger, passes } => E::QuestStep { step: trigger.clone(), passes: *passes },
        DialogCondition::Quest { quest, active } => E::Quest { quest: *quest, active: *active },
        DialogCondition::QuestItem { quest, item, operator, value } => {
            E::QuestItem { quest: *quest, item: item_name(*item), operator: (*operator).into(), value: *value }
        }
        DialogCondition::Ability { ability, operator, value } => E::Ability { ability: *ability, operator: (*operator).into(), value: *value },
        DialogCondition::NpcSwitch { operator, value } => E::NpcSwitch { operator: (*operator).into(), value: *value },
        DialogCondition::QuestSwitch { switch, operator, value } => {
            E::QuestSwitch { switch: *switch, operator: (*operator).into(), value: *value }
        }
        DialogCondition::Variable { variable, index, operator, value } => E::Variable {
            variable: match variable {
                DialogVariable::Episode => VariableList::Episode,
                DialogVariable::Job => VariableList::Job,
                DialogVariable::Planet => VariableList::Planet,
            },
            index: *index,
            operator: (*operator).into(),
            value: *value,
        },
        DialogCondition::QuestValue { quest, value_type, index, operator, value } => E::QuestValue {
            quest: *quest,
            value_type: match value_type {
                DialogQuestValue::Switch => QuestValueType::Switch,
                DialogQuestValue::Variable => QuestValueType::Variable,
            },
            index: *index,
            operator: (*operator).into(),
            value: *value,
        },
        DialogCondition::QuestCount { operator, value } => E::QuestCount { operator: (*operator).into(), value: *value },
    }
}

impl DialogConditionExport {
    fn to_data(&self) -> Result<DialogCondition, anyhow::Error> {
        use DialogConditionExport as E;
        Ok(match self {
            E::QuestStep { step, passes } => DialogCondition::QuestStep { trigger: step.clone(), passes: *passes },
            E::Quest { quest, active } => DialogCondition::Quest { quest: *quest, active: *active },
            E::QuestItem { quest, item, operator, value } => {
                DialogCondition::QuestItem { quest: *quest, item: parse_item_name(item)?, operator: (*operator).into(), value: *value }
            }
            E::Ability { ability, operator, value } => DialogCondition::Ability { ability: *ability, operator: (*operator).into(), value: *value },
            E::NpcSwitch { operator, value } => DialogCondition::NpcSwitch { operator: (*operator).into(), value: *value },
            E::QuestSwitch { switch, operator, value } => {
                DialogCondition::QuestSwitch { switch: *switch, operator: (*operator).into(), value: *value }
            }
            E::Variable { variable, index, operator, value } => DialogCondition::Variable {
                variable: match variable {
                    VariableList::Episode => DialogVariable::Episode,
                    VariableList::Job => DialogVariable::Job,
                    VariableList::Planet => DialogVariable::Planet,
                },
                index: *index,
                operator: (*operator).into(),
                value: *value,
            },
            E::QuestValue { quest, value_type, index, operator, value } => DialogCondition::QuestValue {
                quest: *quest,
                value_type: match value_type {
                    QuestValueType::Switch => DialogQuestValue::Switch,
                    QuestValueType::Variable => DialogQuestValue::Variable,
                },
                index: *index,
                operator: (*operator).into(),
                value: *value,
            },
            E::QuestCount { operator, value } => DialogCondition::QuestCount { operator: (*operator).into(), value: *value },
        })
    }

    /// The quest step it names.
    pub fn step(&self) -> Option<&str> {
        match self {
            DialogConditionExport::QuestStep { step, .. } => Some(step),
            _ => None,
        }
    }

    /// The quest it names.
    pub fn quest(&self) -> Option<u16> {
        match self {
            DialogConditionExport::Quest { quest, .. }
            | DialogConditionExport::QuestItem { quest, .. }
            | DialogConditionExport::QuestValue { quest, .. } => Some(*quest),
            _ => None,
        }
    }
}

fn action_export(a: &DialogAction) -> DialogActionExport {
    use DialogActionExport as E;
    match a {
        DialogAction::QuestStep { trigger } => E::QuestStep { step: trigger.clone() },
        DialogAction::OpenShop => E::Open { window: Window::Shop },
        DialogAction::OpenStorage => E::Open { window: Window::Storage },
        DialogAction::OpenRefining => E::Open { window: Window::Refining },
        DialogAction::OpenDisassembly => E::Open { window: Window::Disassembly },
        DialogAction::OpenRepair => E::Open { window: Window::Repair },
        DialogAction::Appraisal => E::Open { window: Window::Appraisal },
        DialogAction::OpenDelivery => E::Open { window: Window::Delivery },
        DialogAction::SetSavePoint => E::SetSavePoint,
        DialogAction::FoundClan => E::FoundClan,
        DialogAction::DisbandClan => E::DisbandClan,
    }
}

impl DialogActionExport {
    fn to_data(&self) -> DialogAction {
        use DialogActionExport as E;
        match self {
            E::QuestStep { step } => DialogAction::QuestStep { trigger: step.clone() },
            E::Open { window } => match window {
                Window::Shop => DialogAction::OpenShop,
                Window::Storage => DialogAction::OpenStorage,
                Window::Refining => DialogAction::OpenRefining,
                Window::Disassembly => DialogAction::OpenDisassembly,
                Window::Repair => DialogAction::OpenRepair,
                Window::Appraisal => DialogAction::Appraisal,
                Window::Delivery => DialogAction::OpenDelivery,
            },
            E::SetSavePoint => DialogAction::SetSavePoint,
            E::FoundClan => DialogAction::FoundClan,
            E::DisbandClan => DialogAction::DisbandClan,
        }
    }

    pub fn step(&self) -> Option<&str> {
        match self {
            DialogActionExport::QuestStep { step } => Some(step),
            _ => None,
        }
    }
}

impl DialogCaseExport {
    fn from_data(case: &DialogCase, show_used: bool) -> Self {
        Self {
            condition: case.conditions.iter().map(condition_export).collect(),
            action: case.actions.iter().map(action_export).collect(),
            show: !show_used || case.show,
        }
    }

    fn to_data(&self, show_used: bool) -> Result<DialogCase, anyhow::Error> {
        Ok(DialogCase {
            conditions: self.condition.iter().map(|c| c.to_data()).collect::<Result<_, _>>()?,
            actions: self.action.iter().map(|a| a.to_data()).collect(),
            show: show_used && self.show,
        })
    }
}

impl DialogExport {
    pub fn from_data(dialog: &Dialog) -> Self {
        Self {
            key: dialog.key.clone(),
            name: dialog.name.clone(),
            open_if: dialog.open_if.iter().map(|c| DialogCaseExport::from_data(c, true)).collect(),
            menu: dialog
                .menus
                .iter()
                .map(|m| DialogMenuExport {
                    line: m
                        .lines
                        .iter()
                        .map(|l| DialogLineExport {
                            kind: match l.kind {
                                DialogLineKind::Npc => LineKind::Npc,
                                DialogLineKind::Answer => LineKind::Answer,
                            },
                            text: l.text.clone(),
                            next_menu: l.next_menu.map_or(-1, |m| m as i32),
                            show_if: l.show_if.iter().map(|c| DialogCaseExport::from_data(c, true)).collect(),
                            on_choose: l.on_choose.iter().map(|c| DialogCaseExport::from_data(c, false)).collect(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    pub fn to_data(&self) -> Result<Dialog, anyhow::Error> {
        let menus = self
            .menu
            .iter()
            .enumerate()
            .map(|(m, menu)| {
                let lines = menu
                    .line
                    .iter()
                    .enumerate()
                    .map(|(i, line)| {
                        let read = || -> Result<DialogLine, anyhow::Error> {
                            Ok(DialogLine {
                                kind: match line.kind {
                                    LineKind::Npc => DialogLineKind::Npc,
                                    LineKind::Answer => DialogLineKind::Answer,
                                },
                                text: line.text.clone(),
                                show_if: line.show_if.iter().map(|c| c.to_data(true)).collect::<Result<_, _>>()?,
                                on_choose: line.on_choose.iter().map(|c| c.to_data(false)).collect::<Result<_, _>>()?,
                                next_menu: usize::try_from(line.next_menu).ok(),
                            })
                        };
                        read().with_context(|| format!("menu {m} line {i}"))
                    })
                    .collect::<Result<_, _>>()?;
                Ok(DialogMenu { lines })
            })
            .collect::<Result<_, anyhow::Error>>()?;
        Ok(Dialog {
            key: self.key.clone(),
            name: self.name.clone(),
            open_if: self.open_if.iter().map(|c| c.to_data(true)).collect::<Result<_, _>>()?,
            menus,
        })
    }

    /// Every case of the dialog (opening, and each line's).
    pub fn cases(&self) -> impl Iterator<Item = &DialogCaseExport> {
        self.open_if
            .iter()
            .chain(self.menu.iter().flat_map(|m| m.line.iter().flat_map(|l| l.show_if.iter().chain(&l.on_choose))))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DialogFile {
    #[serde(default)]
    dialog: Vec<DialogExport>,
}

/// Every dialog, as exported.
#[derive(Debug, Default, PartialEq)]
pub struct DialogsExport {
    pub dialogs: Vec<DialogExport>,
}

impl DialogsExport {
    /// Sorted by key.
    pub fn from_database(db: &DialogDatabase) -> Self {
        let mut dialogs: Vec<DialogExport> = db.iter().map(DialogExport::from_data).collect();
        dialogs.sort_by(|a, b| a.key.to_lowercase().cmp(&b.key.to_lowercase()));
        Self { dialogs }
    }

    pub fn to_database(&self) -> Result<DialogDatabase, anyhow::Error> {
        self.check()?;
        let dialogs = self
            .dialogs
            .iter()
            .map(|d| d.to_data().with_context(|| format!("dialog {}", d.key)))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(DialogDatabase::new(dialogs))
    }

    /// Keys used twice (in any case), and menus a line leads to that aren't there.
    pub fn check(&self) -> Result<(), anyhow::Error> {
        let mut keys = HashSet::new();
        for dialog in &self.dialogs {
            if dialog.key.trim().is_empty() {
                bail!("{DIALOGS_FILE}: a dialog has no key");
            }
            if !keys.insert(dialog.key.to_lowercase()) {
                bail!("{DIALOGS_FILE}: dialog {} is there twice", dialog.key);
            }
            if dialog.menu.is_empty() {
                bail!("dialog {}: it has no menu (menu 0 is what the NPC says first)", dialog.key);
            }
            for (m, menu) in dialog.menu.iter().enumerate() {
                for (i, line) in menu.line.iter().enumerate() {
                    if line.next_menu < -1 || line.next_menu >= dialog.menu.len() as i32 {
                        bail!(
                            "dialog {} menu {m} line {i}: leads to menu {}, which isn't there (menus 0 to {}, -1 for none)",
                            dialog.key,
                            line.next_menu,
                            dialog.menu.len() - 1
                        );
                    }
                }
            }
        }
        Ok(())
    }

    /// The quest steps and quests the dialogs name, to check they are there.
    pub fn named(&self) -> (BTreeSet<&str>, BTreeSet<u16>) {
        let mut steps = BTreeSet::new();
        let mut quests = BTreeSet::new();
        for case in self.dialogs.iter().flat_map(|d| d.cases()) {
            steps.extend(case.condition.iter().filter_map(DialogConditionExport::step));
            steps.extend(case.action.iter().filter_map(DialogActionExport::step));
            quests.extend(case.condition.iter().filter_map(DialogConditionExport::quest));
        }
        (steps, quests)
    }

    /// Takes out what names a quest step or a quest that isn't there (`has_step` and
    /// `has_quest` say which are), keeping what the dialogs do: such a step never passes and
    /// running it does nothing, and nobody can have such a quest (so its items and values read
    /// -1). A condition whose answer is then known is taken out when it passes, and the case
    /// with it when it fails; running such a step is dropped. A line no case can show any more
    /// goes; a dialog no case can open keeps a case that says no. Returns a note per step or
    /// quest taken out.
    pub fn without_missing(&mut self, has_step: impl Fn(&str) -> bool, has_quest: impl Fn(u16) -> bool) -> Vec<String> {
        let mut notes = BTreeSet::new();
        let step_note = |step: &str| format!("quest step {step} isn't in the game (it never passed; taken out)");
        let quest_note = |quest: u16| format!("quest {quest} isn't in the game (nobody could have it; taken out)");
        // The answer of a condition on something that isn't there, and what to note.
        let known = |c: &DialogConditionExport| -> Option<(bool, String)> {
            let cmp = |value: i32, operator: &Compare, against: i32| rose_data::dialog_compare(value, (*operator).into(), against);
            match c {
                DialogConditionExport::QuestStep { step, passes } if !has_step(step) => Some((!*passes, step_note(step))),
                DialogConditionExport::Quest { quest, active } if !has_quest(*quest) => Some((!*active, quest_note(*quest))),
                DialogConditionExport::QuestItem { quest, operator, value, .. }
                | DialogConditionExport::QuestValue { quest, operator, value, .. }
                    if !has_quest(*quest) =>
                {
                    Some((cmp(-1, operator, *value), quest_note(*quest)))
                }
                _ => None,
            }
        };
        let mut fold = |key: &str, cases: &mut Vec<DialogCaseExport>| {
            cases.retain(|case| {
                let fails = case.condition.iter().filter_map(&known).find(|(passes, _)| !passes);
                if let Some((_, note)) = &fails {
                    notes.insert(format!("{key}: {note}"));
                }
                fails.is_none()
            });
            for case in cases.iter_mut() {
                case.condition.retain(|c| match known(c) {
                    Some((_, note)) => {
                        notes.insert(format!("{key}: {note}"));
                        false
                    }
                    None => true,
                });
                case.action.retain(|a| match a {
                    DialogActionExport::QuestStep { step } if !has_step(step) => {
                        notes.insert(format!("{key}: {}", step_note(step)));
                        false
                    }
                    _ => true,
                });
            }
        };
        for dialog in &mut self.dialogs {
            let key = dialog.key.clone();
            if !dialog.open_if.is_empty() {
                fold(&key, &mut dialog.open_if);
                if dialog.open_if.is_empty() {
                    dialog.open_if.push(DialogCaseExport { show: false, ..Default::default() });
                }
            }
            for menu in &mut dialog.menu {
                menu.line.retain_mut(|line| {
                    fold(&key, &mut line.on_choose);
                    if line.show_if.is_empty() {
                        return true;
                    }
                    fold(&key, &mut line.show_if);
                    !line.show_if.is_empty()
                });
            }
        }
        notes.into_iter().collect()
    }

    pub fn to_files_as(&self, format: Format) -> Result<Vec<(&'static str, String)>, anyhow::Error> {
        let mut dialog = self.dialogs.clone();
        dialog.sort_by(|a, b| a.key.to_lowercase().cmp(&b.key.to_lowercase()));
        Ok(vec![(DIALOGS_FILE, format.write(DIALOGS_FILE, &DialogFile { dialog })?)])
    }

    /// None when the file isn't there.
    pub fn from_files_as(
        format: Format,
        mut read: impl FnMut(&str) -> Result<Option<String>, anyhow::Error>,
    ) -> Result<Option<Self>, anyhow::Error> {
        let Some(text) = read(DIALOGS_FILE)? else { return Ok(None) };
        Ok(Some(Self { dialogs: format.parse::<DialogFile>(DIALOGS_FILE, &text)?.dialog }))
    }
}

/// Every difference between two dialog databases, as their export.
pub fn compare_dialog_databases(a: &DialogDatabase, b: &DialogDatabase) -> Vec<String> {
    let a = DialogsExport::from_database(a);
    let b = DialogsExport::from_database(b);
    let mut out = Vec::new();
    let keys: BTreeSet<String> = a.dialogs.iter().chain(&b.dialogs).map(|d| d.key.to_lowercase()).collect();
    for key in keys {
        let find = |e: &DialogsExport| e.dialogs.iter().find(|d| d.key.to_lowercase() == key).cloned();
        match (find(&a), find(&b)) {
            (Some(x), Some(y)) if x != y => out.push(format!("dialog {key} differs")),
            (Some(_), None) => out.push(format!("dialog {key} is only in the first")),
            (None, Some(_)) => out.push(format!("dialog {key} is only in the second")),
            _ => {}
        }
    }
    out
}
