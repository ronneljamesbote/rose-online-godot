//! The NPC dialogs as Godot resources: `RoseDialog` (one per dialog in `res://data/dialogs/`)
//! holding its menus (`RoseDialogMenu`), their lines (`RoseDialogLine`) and the cases that
//! decide whether a line shows and what an answer does (`RoseDialogCase`), made of blocks, one
//! class per kind: `RoseDialogIf<kind>` (conditions, 9) and `RoseDialogDo<kind>` (actions, 5).
//! Their properties are the export's keys (`crates/rose-data-export/src/dialogs.rs`);
//! `data_resources.rs` reads and writes them. How a dialog runs: `rose_data::Dialog`.

use godot::{
    classes::{IResource, Resource},
    prelude::*,
    register::info::{PropertyHint, PropertyInfo},
};
use rose_data::AbilityType;
use rose_data_export::{
    dialogs::{ACTION_KINDS, CONDITION_KINDS, LINE_KIND_NAMES, QUEST_VALUE_NAMES, VARIABLE_LIST_NAMES, WINDOW_NAMES},
    quests::COMPARE_NAMES,
};

use crate::quest_blocks::{RoseQuest, RoseQuestTrigger};

/// The class name prefixes of the blocks.
pub const CONDITION_PREFIX: &str = "RoseDialogIf";
pub const ACTION_PREFIX: &str = "RoseDialogDo";

/// The Inspector's list of block classes for an array of conditions or actions.
fn block_classes(action: bool) -> String {
    let (prefix, kinds): (&str, &[&str]) =
        if action { (ACTION_PREFIX, &ACTION_KINDS) } else { (CONDITION_PREFIX, &CONDITION_KINDS) };
    kinds.iter().map(|kind| format!("{prefix}{kind}")).collect::<Vec<_>>().join(",")
}

/// An array property of resources of `classes` ("<object type>/<resource hint>:<classes>").
fn array_hint(property: &mut PropertyInfo, classes: &str) {
    property.hint_info.hint_string = GString::from(format!("24/17:{classes}").as_str());
}

/// The item classes an item link takes.
const ITEM_CLASSES: &str = "RoseItemFace,RoseItemHead,RoseItemBody,RoseItemHands,RoseItemFeet,RoseItemBack,RoseItemJewellery,RoseItemWeapon,RoseItemSubWeapon,RoseItemConsumable,RoseItemGem,RoseItemMaterial,RoseItemQuest,RoseItemVehicle";

/// The dropdowns of a block's text fields, and the classes its item link takes.
fn block_hint(property: &mut PropertyInfo) {
    let name = property.property_name.to_string();
    if property.variant_type == VariantType::OBJECT && name == "item" {
        property.hint_info.hint_string = GString::from(ITEM_CLASSES);
        return;
    }
    if property.variant_type != VariantType::STRING {
        return;
    }
    let options: Vec<String> = match name.as_str() {
        "operator" => COMPARE_NAMES.iter().map(|n| n.to_string()).collect(),
        "variable" => VARIABLE_LIST_NAMES.iter().map(|n| n.to_string()).collect(),
        "value_type" => QUEST_VALUE_NAMES.iter().map(|n| n.to_string()).collect(),
        "window" => WINDOW_NAMES.iter().map(|n| n.to_string()).collect(),
        "kind" => LINE_KIND_NAMES.iter().map(|n| n.to_string()).collect(),
        "ability" => SCRIPT_ABILITIES.iter().map(|a| format!("{a:?}")).collect(),
        _ => return,
    };
    property.hint_info.hint = PropertyHint::ENUM;
    property.hint_info.hint_string = GString::from(options.join(",").as_str());
}

/// The character values a dialog can check (iROSE's script variables).
const SCRIPT_ABILITIES: [AbilityType; 15] = [
    AbilityType::Level,
    AbilityType::Job,
    AbilityType::Union,
    AbilityType::Gender,
    AbilityType::Birthstone,
    AbilityType::Rank,
    AbilityType::Fame,
    AbilityType::Strength,
    AbilityType::Dexterity,
    AbilityType::Intelligence,
    AbilityType::Concentration,
    AbilityType::Charm,
    AbilityType::Sense,
    AbilityType::Experience,
    AbilityType::BonusPoint,
];

/// What a line shows as in the Inspector's lists: who says it and how it starts.
fn line_label(kind: &str, text: &str) -> String {
    let who = if kind == "Answer" { "You" } else { "NPC" };
    let flat: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let short: String = flat.chars().take(48).collect();
    if short.len() < flat.len() {
        format!("{who}: {short}…")
    } else {
        format!("{who}: {short}")
    }
}

/// An NPC dialog: what the NPC says when you talk to it and the answers you can pick. Talking
/// to the NPC shows menu 0: of its lines whose `show_if` passes, the **last** NPC line is what
/// the NPC says, and the answers are the lines of the menu that line leads to. Picking an
/// answer runs its `on_choose` and shows the menu it leads to (none: the conversation ends).
/// An NPC placed in a zone names its dialog by `key`. Edit it here, then run `--export-data`;
/// the wiki page "NPC dialogs" has the rules.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseDialog {
    /// What NPCs placed in a zone call it by (iROSE's CON file name, `EM20-001.con`). Keep it
    /// unique; the file is named after it.
    #[export]
    key: GString,
    /// Who it is for, a note for whoever edits it (not shown in the game).
    #[export]
    name: GString,
    /// Whether the NPC talks at all: the first case whose conditions pass decides (none: it
    /// always talks).
    #[export]
    open_if: Array<Option<Gd<Resource>>>,
    /// The menus, numbered from 0 (`RoseDialogMenu`). Menu 0 is what the NPC says first.
    #[export]
    menus: Array<Option<Gd<Resource>>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseDialog {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        match property.property_name.to_string().as_str() {
            "open_if" => array_hint(property, "RoseDialogCase"),
            "menus" => array_hint(property, "RoseDialogMenu"),
            _ => {}
        }
    }
}

/// One menu of a dialog (`RoseDialog`): NPC lines and answers. Lines lead to menus by their
/// number in the dialog's `menus`.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseDialogMenu {
    /// From the top (`RoseDialogLine`).
    #[export]
    lines: Array<Option<Gd<Resource>>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseDialogMenu {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        if property.property_name.to_string() == "lines" {
            array_hint(property, "RoseDialogLine");
        }
    }
}

/// A line of a dialog menu (`RoseDialogMenu`): something the NPC says, or an answer you can
/// pick.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseDialogLine {
    /// `Npc`: what the NPC says; the answers are the lines of `next_menu`. `Answer`: a choice;
    /// `next_menu` is what comes after it.
    #[export]
    #[var(get = get_kind, set = set_kind)]
    #[init(val = GString::from("Npc"))]
    kind: GString,
    /// The words. `<NAME>` and `<LEVEL>` are the player's; `{br}` breaks the line, `{b}`...`{/b}`
    /// is bold and `{fc=N}`...`{/fc}` a colour (1 to 17).
    #[export(multiline)]
    #[var(get = get_text, set = set_text)]
    text: GString,
    /// The menu it leads to, by its number in the dialog's `menus`; -1: none (an answer that
    /// ends the conversation).
    #[export]
    #[init(val = -1)]
    next_menu: i64,
    /// Whether the line shows: the first case whose conditions pass decides; no case passing
    /// hides it. Empty: it always shows.
    #[export]
    show_if: Array<Option<Gd<Resource>>>,
    /// For an answer, what picking it does: the first case whose conditions pass runs its
    /// actions.
    #[export]
    on_choose: Array<Option<Gd<Resource>>>,
    base: Base<Resource>,
}

/// Changing the kind or the words renames the line, so the menu's list shows it as it is.
#[godot_api]
impl RoseDialogLine {
    #[func]
    fn get_kind(&self) -> GString {
        self.kind.clone()
    }

    #[func]
    fn set_kind(&mut self, kind: GString) {
        self.kind = kind;
        self.relabel();
    }

    #[func]
    fn get_text(&self) -> GString {
        self.text.clone()
    }

    #[func]
    fn set_text(&mut self, text: GString) {
        self.text = text;
        self.relabel();
    }

    fn relabel(&mut self) {
        let label = GString::from(line_label(&self.kind.to_string(), &self.text.to_string()).as_str());
        if self.base().get_name() != label {
            self.base_mut().set_name(&label);
        }
    }
}

#[godot_api]
impl IResource for RoseDialogLine {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        match property.property_name.to_string().as_str() {
            "show_if" | "on_choose" => array_hint(property, "RoseDialogCase"),
            _ => block_hint(property),
        }
    }
}

/// A case of a dialog line (`RoseDialogLine`) or of a dialog's opening: when every condition
/// passes (from the top), it runs every action (from the top), and in a `show_if` it shows the
/// line when `show` is on. Only the first case whose conditions pass counts.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseDialogCase {
    /// What must all be true (`RoseDialogIf...` blocks). Empty: always.
    #[export]
    conditions: Array<Option<Gd<Resource>>>,
    /// What it does (`RoseDialogDo...` blocks).
    #[export]
    actions: Array<Option<Gd<Resource>>>,
    /// In a `show_if` (or `open_if`): whether the line shows (the NPC talks). Not used in
    /// `on_choose`.
    #[export]
    #[init(val = true)]
    show: bool,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseDialogCase {
    fn on_validate_property(&self, property: &mut PropertyInfo) {
        match property.property_name.to_string().as_str() {
            "conditions" => array_hint(property, &block_classes(false)),
            "actions" => array_hint(property, &block_classes(true)),
            _ => {}
        }
    }
}

macro_rules! dialog_block {
    ($(#[$doc:meta])* $name:ident { $($(#[$fdoc:meta])* $field:ident: $ty:ty $(= $init:expr)?,)* }) => {
        $(#[$doc])*
        #[derive(GodotClass)]
        #[class(tool, base=Resource, init)]
        pub struct $name {
            $(
                $(#[$fdoc])*
                #[export]
                $(#[init(val = $init)])?
                $field: $ty,
            )*
            base: Base<Resource>,
        }

        #[godot_api]
        impl IResource for $name {
            fn on_validate_property(&self, property: &mut PropertyInfo) {
                block_hint(property);
            }
        }
    };
}

dialog_block!(
    /// A quest step's conditions pass (`passes` on), or fail (off), checked as the step would
    /// be. A condition of a dialog case (`RoseDialogCase`).
    RoseDialogIfQuestStep {
        /// The step (drag a RoseQuestTrigger in from `res://data/quest_triggers/`).
        step: Option<Gd<RoseQuestTrigger>>,
        /// On: passes when the step's conditions pass. Off: when they fail.
        passes: bool = true,
    }
);

dialog_block!(
    /// The quest is in your quest list (`active` on), or isn't (off). A condition of a dialog
    /// case (`RoseDialogCase`).
    RoseDialogIfQuest {
        /// The quest (drag a RoseQuest in).
        quest: Option<Gd<RoseQuest>>,
        active: bool = true,
    }
);

dialog_block!(
    /// How many of an item you carry for a quest (its own item list) compares to a number;
    /// without the quest it counts as -1. A condition of a dialog case (`RoseDialogCase`).
    RoseDialogIfQuestItem {
        /// The quest.
        quest: Option<Gd<RoseQuest>>,
        /// The item (drag an item resource in, usually a quest item).
        item: Option<Gd<Resource>>,
        /// How to compare: the count on the left, `value` on the right.
        operator: GString = GString::from(">="),
        value: i64,
    }
);

dialog_block!(
    /// One of your values (level, job, union, a stat ...) compares to a number. A condition of
    /// a dialog case (`RoseDialogCase`).
    RoseDialogIfAbility {
        /// Which value. Birthstone, rank and fame are always 0 in this game.
        ability: GString = GString::from("Level"),
        /// How to compare: your value on the left, `value` on the right.
        operator: GString = GString::from(">="),
        value: i64,
    }
);

dialog_block!(
    /// The NPC's own quest switch (its object variable 0, which its AI sets) compares to a
    /// number: town NPCs turn their quests on and off with it. A condition of a dialog case
    /// (`RoseDialogCase`).
    RoseDialogIfNpcSwitch {
        operator: GString = GString::from("=="),
        value: i64,
    }
);

dialog_block!(
    /// One of your quest switches (0 or 1; -1 past the last one) compares to a number. A
    /// condition of a dialog case (`RoseDialogCase`).
    RoseDialogIfQuestSwitch {
        /// Which switch (from 0).
        switch: i64,
        operator: GString = GString::from("=="),
        value: i64,
    }
);

dialog_block!(
    /// One of your episode, job or planet variables (-1 past the last one) compares to a
    /// number. A condition of a dialog case (`RoseDialogCase`).
    RoseDialogIfVariable {
        /// Which list.
        variable: GString = GString::from("Episode"),
        /// Which one (from 0).
        index: i64,
        operator: GString = GString::from("=="),
        value: i64,
    }
);

dialog_block!(
    /// A switch or variable of one of your quests compares to a number; without the quest it
    /// is -1. A condition of a dialog case (`RoseDialogCase`).
    RoseDialogIfQuestValue {
        /// The quest.
        quest: Option<Gd<RoseQuest>>,
        /// Its switches or its variables.
        value_type: GString = GString::from("Switch"),
        /// Which one (from 0).
        index: i64,
        operator: GString = GString::from("=="),
        value: i64,
    }
);

dialog_block!(
    /// How many quests you have compares to a number. A condition of a dialog case
    /// (`RoseDialogCase`).
    RoseDialogIfQuestCount {
        operator: GString = GString::from("<"),
        value: i64,
    }
);

dialog_block!(
    /// Runs a quest step: the game checks its conditions, then asks the server, which checks
    /// them again and gives its rewards. An action of a dialog case (`RoseDialogCase`).
    RoseDialogDoQuestStep {
        /// The step (drag a RoseQuestTrigger in from `res://data/quest_triggers/`).
        step: Option<Gd<RoseQuestTrigger>>,
    }
);

dialog_block!(
    /// Opens a window for this NPC: its shop, storage, refining, disassembly or repair.
    /// Appraisal and Delivery aren't in the game yet (the game says so). An action of a dialog
    /// case (`RoseDialogCase`).
    RoseDialogDoOpen {
        window: GString = GString::from("Shop"),
    }
);

dialog_block!(
    /// Saves this town as where you get up after dying. An action of a dialog case
    /// (`RoseDialogCase`).
    RoseDialogDoSetSavePoint {}
);

dialog_block!(
    /// Founds a clan. Clans aren't in the game yet: does nothing. An action of a dialog case
    /// (`RoseDialogCase`).
    RoseDialogDoFoundClan {}
);

dialog_block!(
    /// Disbands your clan. Clans aren't in the game yet: does nothing. An action of a dialog
    /// case (`RoseDialogCase`).
    RoseDialogDoDisbandClan {}
);
