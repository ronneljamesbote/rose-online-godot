//! The game data made in the Godot editor. Each thing (an item, later a monster, a skill, a
//! quest ...) is a resource in `godot/data/`; `--export-data` writes them as TOML files to
//! `overrides/data/`, and this crate reads those files into the game's own databases
//! (`rose-data`), the same ones `rose-data-irose` fills from the iROSE tables. Server and
//! client both load the export, so they always see the same values.
//!
//! So far: items (`items/<type>.toml`, see [`items`]), and monsters, NPCs, drop tables and
//! shop tabs (`monsters.toml`, `npcs.toml`, `drop-tables.toml`, `shop-tabs.toml`, see
//! [`npcs`]).

pub mod items;
pub mod npcs;

pub use items::{compare_item_databases, Format, ItemsExport};
pub use npcs::{compare_drop_rows, compare_npc_databases, NpcModels, NpcsExport};

/// The export folder in the repository (and next to the game in the Windows build).
pub const DATA_DIR: &str = "overrides/data";
/// Where the upload puts the export among the game files the server reads.
pub const DATA_GAME_DIR: &str = "OVERRIDES/DATA";

/// The item files' folder inside the export.
pub const ITEMS_DIR: &str = "items";

/// The path of an item export file (`weapon` -> `items/weapon.toml`) inside the export folder.
pub fn items_file(name: &str) -> String {
    items_file_as(name, Format::Toml)
}

/// The same in `format` (`items/weapon.json`).
pub fn items_file_as(name: &str, format: Format) -> String {
    format!("{ITEMS_DIR}/{name}.{}", format.extension())
}

/// The path of a file at the top of the export (`monsters` -> `monsters.toml`) in `format`.
pub fn data_file_as(name: &str, format: Format) -> String {
    format!("{name}.{}", format.extension())
}
