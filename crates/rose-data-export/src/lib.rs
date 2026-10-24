//! What the Godot editor exports for the server and the game (`export/` in the repository,
//! written by `--export-data`), and the loaders that read it.
//!
//! - `export/data/`: the game data made in the Godot editor. Each thing (an item, a monster,
//!   a skill, a zone ...) is a resource in `godot/data/`; this crate reads the exported
//!   files into the game's own databases (`rose-data`), the same ones `rose-data-irose`
//!   fills from the iROSE tables. So far: items (`items/<type>.toml`, see [`items`]),
//!   monsters, NPCs, drop tables and shop tabs (`monsters.toml`, `npcs.toml`,
//!   `drop-tables.toml`, `shop-tabs.toml`, see [`npcs`]), skills, status effects and job
//!   classes (`skills.toml`, `status-effects.toml`, `job-classes.toml`, see [`skills`]), and
//!   the world: zone settings, warp gates, crafting recipes and new characters
//!   (`zones.toml`, `warp-gates.toml`, `craft-recipes.toml`, `new-characters.toml`, see
//!   [`world`]), and the sounds (`sounds.toml`, `hit-sounds.toml`, `footsteps.toml`, see
//!   [`sounds`]; only the game reads them), and the effects (`effect-files.toml`,
//!   `attack-effects.toml`, see [`effects`]; only the game reads them too).
//! - `export/zones/`: the spawns, NPCs and named spots of the zones edited as Godot scenes
//!   (see [`zone_layouts`]).
//! - `export/motions.toml`: the length and hit frames of each motion, from the hit markers
//!   of the Godot animations (see [`motions`]).
//!
//! Server and client both load the export, so they always see the same values.

pub mod effects;
pub mod items;
pub mod motions;
pub mod npcs;
pub mod skills;
pub mod sounds;
pub mod world;
pub mod zone_layouts;

pub use effects::{compare_effect_databases, effect_stem, EffectsExport};
pub use items::{compare_item_databases, Format, ItemsExport};
pub use motions::{MotionTiming, MotionTimings, MOTIONS_FILE, MOTIONS_GAME_FILE};
pub use npcs::{compare_drop_rows, compare_npc_databases, NpcModels, NpcsExport};
pub use skills::{compare_skill_databases, SkillsExport};
pub use sounds::{compare_sound_databases, sound_stem, SoundsExport};
pub use world::{compare_warp_gates, compare_zone_settings, WorldExport};
pub use zone_layouts::{zone_game_file, ZoneEvent, ZoneLayout, ZoneNpc, ZoneSpawn, ZONES_DIR};

/// The export folder in the repository (and next to the game in the Windows build).
pub const EXPORT_DIR: &str = "export";
/// The game data's folder in it.
pub const DATA_DIR: &str = "export/data";
/// Where the upload puts the game data among the game files the server reads.
pub const DATA_GAME_DIR: &str = "EXPORT/DATA";

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
