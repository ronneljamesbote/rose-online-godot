//! NPC dialogs: what an NPC says when you talk to it, the answers you can pick, and what an
//! answer does. iROSE kept each one in a CON file (menus of lines) with a Lua script deciding
//! which lines show and what an answer does; here the script is made of condition and action
//! blocks (`DialogCondition`, `DialogAction`) grouped in cases (`DialogCase`).
//!
//! A dialog is a list of menus. Talking to the NPC shows menu 0: of its lines whose
//! condition passes, the **last** NPC line is what the NPC says and the answers come from the
//! menu that line points to (so a plain greeting comes first and quest lines after it).
//! Picking an answer runs its actions and shows the menu it points to; an answer pointing
//! nowhere ends the conversation. Only the game runs dialogs: an action that changes
//! something (a quest step) is checked again by the server.

use std::collections::HashMap;

use rose_file_readers::QsdConditionOperator;

use crate::{AbilityType, ItemReference};

/// Which quest value a `DialogCondition::QuestValue` reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogQuestValue {
    Switch,
    Variable,
}

/// Which of the character's quest variable lists a `DialogCondition::Variable` reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogVariable {
    Episode,
    Job,
    Planet,
}

/// One thing a case checks. Values that aren't there (a quest you don't have, a variable
/// number past the end) read as -1, as iROSE's script functions answered.
#[derive(Clone, Debug, PartialEq)]
pub enum DialogCondition {
    /// The quest step's conditions pass (`passes`), or don't. iROSE: QF_checkQuestCondition.
    QuestStep { trigger: String, passes: bool },
    /// The quest is in your quest list (`active`), or isn't. iROSE: QF_findQuest.
    Quest { quest: u16, active: bool },
    /// How many of `item` you carry for `quest` (-1 without the quest).
    QuestItem { quest: u16, item: ItemReference, operator: QsdConditionOperator, value: i32 },
    /// A value of the character (level, job, union, a stat ...). iROSE: GF_getVariable.
    Ability { ability: AbilityType, operator: QsdConditionOperator, value: i32 },
    /// The NPC's own quest switch (its object variable 0), which turns its quests on and off.
    NpcSwitch { operator: QsdConditionOperator, value: i32 },
    /// One of the character's quest switches (0 or 1). iROSE: QF_getUserSwitch.
    QuestSwitch { switch: u16, operator: QsdConditionOperator, value: i32 },
    /// An episode, job or planet variable of the character.
    Variable { variable: DialogVariable, index: u16, operator: QsdConditionOperator, value: i32 },
    /// A switch or variable of one of your quests (-1 without the quest).
    QuestValue { quest: u16, value_type: DialogQuestValue, index: u16, operator: QsdConditionOperator, value: i32 },
    /// How many quests you have.
    QuestCount { operator: QsdConditionOperator, value: i32 },
}

/// One thing a case does.
#[derive(Clone, Debug, PartialEq)]
pub enum DialogAction {
    /// Ask the server to run the quest step (when its conditions pass here first).
    QuestStep { trigger: String },
    /// Open the NPC's shop.
    OpenShop,
    OpenStorage,
    OpenRefining,
    OpenDisassembly,
    OpenRepair,
    /// Save this town as where you get up after dying.
    SetSavePoint,
    /// Not in the game yet: iROSE's item appraisal, item delivery and clans.
    Appraisal,
    OpenDelivery,
    FoundClan,
    DisbandClan,
}

/// When every condition passes, the case runs its actions, and for a line's condition the
/// line shows when `show` is set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DialogCase {
    pub conditions: Vec<DialogCondition>,
    pub actions: Vec<DialogAction>,
    pub show: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogLineKind {
    /// What the NPC says. `next_menu` holds the answers.
    Npc,
    /// An answer you can pick. `next_menu` is what comes after it (None ends the conversation).
    Answer,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DialogLine {
    pub kind: DialogLineKind,
    /// With ROSE's tags ({br}, {b}, {fc=N}, <NAME>, <LEVEL>).
    pub text: String,
    /// The first case whose conditions pass decides (no case passing hides the line); none at
    /// all: the line always shows.
    pub show_if: Vec<DialogCase>,
    /// When picked: the first case whose conditions pass runs its actions.
    pub on_choose: Vec<DialogCase>,
    pub next_menu: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DialogMenu {
    pub lines: Vec<DialogLine>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Dialog {
    /// What NPC placements call it by (iROSE's CON file name, `EM20-001.con`).
    pub key: String,
    /// A note for whoever edits it (which NPC).
    pub name: String,
    /// Whether the NPC talks at all (first case decides, none: always).
    pub open_if: Vec<DialogCase>,
    pub menus: Vec<DialogMenu>,
}

#[derive(Default)]
pub struct DialogDatabase {
    dialogs: HashMap<String, Dialog>,
}

impl DialogDatabase {
    pub fn new(dialogs: impl IntoIterator<Item = Dialog>) -> Self {
        Self { dialogs: dialogs.into_iter().map(|d| (d.key.to_lowercase(), d)).collect() }
    }

    /// By its key, any case (`EM20-001.con`, `em20-001.CON`).
    pub fn get(&self, key: &str) -> Option<&Dialog> {
        self.dialogs.get(&key.to_lowercase())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Dialog> {
        self.dialogs.values()
    }

    pub fn len(&self) -> usize {
        self.dialogs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.dialogs.is_empty()
    }
}

/// What a running dialog asks the game: whether a condition passes, and to carry out an
/// action (later, once the dialog has finished deciding).
pub trait DialogContext {
    fn check(&mut self, condition: &DialogCondition) -> bool;
    fn act(&mut self, action: &DialogAction);
}

/// Runs the first case whose conditions pass: its actions, and its `show`. None of them: false.
pub fn run_cases(cases: &[DialogCase], cx: &mut impl DialogContext) -> bool {
    for case in cases {
        if case.conditions.iter().all(|c| cx.check(c)) {
            for action in &case.actions {
                cx.act(action);
            }
            return case.show;
        }
    }
    false
}

pub fn compare(value: i32, operator: QsdConditionOperator, against: i32) -> bool {
    match operator {
        QsdConditionOperator::Equals => value == against,
        QsdConditionOperator::GreaterThan => value > against,
        QsdConditionOperator::GreaterThanEqual => value >= against,
        QsdConditionOperator::LessThan => value < against,
        QsdConditionOperator::LessThanEqual => value <= against,
        QsdConditionOperator::NotEqual => value != against,
    }
}

/// What the dialog shows now: the NPC's line and the answers (menu and line numbers).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DialogView {
    pub message: Option<(usize, usize)>,
    pub answers: Vec<(usize, usize)>,
}

/// NPC lines leading to NPC lines deeper than this stop (a loop in a dialog's menus).
const MAX_DEPTH: usize = 32;

impl Dialog {
    pub fn line(&self, at: (usize, usize)) -> Option<&DialogLine> {
        self.menus.get(at.0)?.lines.get(at.1)
    }

    /// Talk to the NPC. None when it has nothing to say.
    pub fn open(&self, cx: &mut impl DialogContext) -> Option<DialogView> {
        if !self.open_if.is_empty() && !run_cases(&self.open_if, cx) {
            return None;
        }
        let mut view = DialogView::default();
        self.run_menu(Some(0), &mut view, cx, 0).then_some(view)
    }

    /// Pick answer `index` of `view`. False when the conversation is over.
    pub fn choose(&self, view: &mut DialogView, index: usize, cx: &mut impl DialogContext) -> bool {
        let Some(line) = view.answers.get(index).and_then(|at| self.line(*at)) else { return true };
        run_cases(&line.on_choose, cx);
        let next = line.next_menu;
        self.run_menu(next, view, cx, 0)
    }

    fn run_menu(&self, menu: Option<usize>, view: &mut DialogView, cx: &mut impl DialogContext, depth: usize) -> bool {
        let Some(index) = menu else { return false };
        let Some(menu) = self.menus.get(index) else { return false };
        if depth > MAX_DEPTH {
            return false;
        }
        let mut any = false;
        for (i, line) in menu.lines.iter().enumerate() {
            if !line.show_if.is_empty() && !run_cases(&line.show_if, cx) {
                continue;
            }
            match line.kind {
                DialogLineKind::Npc => {
                    view.message = Some((index, i));
                    view.answers.clear();
                    self.run_menu(line.next_menu, view, cx, depth + 1);
                }
                DialogLineKind::Answer => view.answers.push((index, i)),
            }
            any = true;
        }
        any
    }
}
