//! Godot 4 extension that reads ROSE Online data files and builds Godot nodes from them.

use godot::prelude::*;

mod audio;
mod character;
mod conversation;
mod data;
mod effect;
mod items;
mod lua4;
mod material;
mod mesh;
#[allow(clippy::all, unused)]
mod module_bindings;
mod net;
mod skills;
mod texture;
mod zone;

struct RoseExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RoseExtension {
    /// The caches hold Godot objects, so they must be emptied before the engine shuts down.
    fn on_stage_deinit(stage: InitStage) {
        if stage == InitStage::MainLoop {
            audio::clear_cache();
            effect::clear_cache();
            items::clear_cache();
            material::clear_cache();
            mesh::clear_cache();
            texture::clear_cache();
        }
    }
}

thread_local! {
    static OPEN_ERROR: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

/// Entry point for GDScript: opens the client data once per process.
#[derive(GodotClass)]
#[class(base=RefCounted, init)]
struct RoseData {
    base: Base<RefCounted>,
}

#[godot_api]
impl RoseData {
    /// Opens a ROSE install by its data.idx path. Safe to call more than once.
    #[func]
    fn open(data_idx: GString) -> bool {
        match data::open(std::path::Path::new(&data_idx.to_string())) {
            Ok(()) => true,
            Err(error) => {
                godot_error!("rose: could not open {data_idx}: {error:#}");
                OPEN_ERROR.with_borrow_mut(|e| *e = format!("{error:#}"));
                false
            }
        }
    }

    /// Why the last open() failed, for showing to the player.
    #[func]
    fn open_error() -> GString {
        OPEN_ERROR.with_borrow(|e| GString::from(e.as_str()))
    }

    /// The inventory icon with this index, or null.
    #[func]
    fn item_icon(index: i32) -> Option<Gd<godot::classes::Texture2D>> {
        items::item_icon(index.max(0) as u32)
    }

    /// A texture from the client data by path (e.g. 3DDATA/CONTROL/RES/LOADING.DDS), or null.
    #[func]
    fn texture(path: GString) -> Option<Gd<godot::classes::Texture2D>> {
        texture::load_texture(&path.to_string())
    }

    /// Item numbers a new character wears (INIT_AVATAR.STB), as RoseCharacter.build takes
    /// them: head, body, hands, feet, weapon (with the server's starting Short Sword).
    #[func]
    fn starting_look(female: bool) -> PackedInt32Array {
        static CREATOR: std::sync::OnceLock<Option<rose_game_data::CharacterCreator>> = std::sync::OnceLock::new();
        let creator = CREATOR.get_or_init(|| data::get().and_then(|g| rose_game_data::CharacterCreator::load(&g.vfs).ok()));
        let mut look = [0, 1, 1, 1, 2];
        if let Some(c) = creator {
            for item in &c.genders[female as usize].equipped_items {
                let slot = match item.item_type {
                    rose_data::ItemType::Head => 0,
                    rose_data::ItemType::Body => 1,
                    rose_data::ItemType::Hands => 2,
                    rose_data::ItemType::Feet => 3,
                    _ => continue,
                };
                look[slot] = item.item_number as i32;
            }
        }
        PackedInt32Array::from(&look)
    }

    /// The zone's name from LIST_ZONE.STB, or "".
    #[func]
    fn zone_name(zone_id: i32) -> GString {
        let name = data::get()
            .and_then(|g| rose_data::ZoneId::new(zone_id.clamp(0, u16::MAX as i32) as u16).and_then(|id| g.zone_list.get_zone(id)))
            .map_or(String::new(), |z| z.name.to_string());
        GString::from(name.as_str())
    }

    /// The zone's minimap from LIST_ZONE.STB: {path, start_x, start_y}, or {} when it has none.
    #[func]
    fn zone_minimap(zone_id: i32) -> VarDictionary {
        let mut dict = VarDictionary::new();
        let zone = data::get()
            .and_then(|g| rose_data::ZoneId::new(zone_id.clamp(0, u16::MAX as i32) as u16).and_then(|id| g.zone_list.get_zone(id)));
        if let Some(zone) = zone {
            if let Some(path) = &zone.minimap_path {
                dict.set("path", path.path().to_string_lossy().to_string());
                dict.set("start_x", zone.minimap_start_x as i64);
                dict.set("start_y", zone.minimap_start_y as i64);
            }
        }
        dict
    }
}

/// Reads TOML text (the player-editable UI theme files) for GDScript.
#[derive(GodotClass)]
#[class(base=RefCounted, init)]
struct RoseToml {
    base: Base<RefCounted>,
}

#[godot_api]
impl RoseToml {
    /// Parses TOML into a Dictionary. On a syntax error the result is
    /// {"__error": "line N, column M: message"} instead.
    #[func]
    fn parse(text: GString) -> VarDictionary {
        match text.to_string().parse::<toml::Table>() {
            Ok(table) => toml_table(&table),
            Err(error) => {
                let mut dict = VarDictionary::new();
                dict.set("__error", error.message().to_string() + &toml_error_place(&text.to_string(), error.span()));
                dict
            }
        }
    }
}

fn toml_error_place(text: &str, span: Option<std::ops::Range<usize>>) -> String {
    let Some(span) = span else { return String::new() };
    let before = &text[..span.start.min(text.len())];
    let line = before.matches('\n').count() + 1;
    let column = before.len() - before.rfind('\n').map_or(0, |i| i + 1) + 1;
    format!(" (line {line}, column {column})")
}

fn toml_table(table: &toml::Table) -> VarDictionary {
    let mut dict = VarDictionary::new();
    for (key, value) in table {
        dict.set(key.as_str(), &toml_value(value));
    }
    dict
}

fn toml_value(value: &toml::Value) -> Variant {
    match value {
        toml::Value::String(s) => s.to_variant(),
        toml::Value::Integer(i) => i.to_variant(),
        toml::Value::Float(f) => f.to_variant(),
        toml::Value::Boolean(b) => b.to_variant(),
        toml::Value::Datetime(d) => d.to_string().to_variant(),
        toml::Value::Array(items) => {
            let mut array = VarArray::new();
            for item in items {
                array.push(&toml_value(item));
            }
            array.to_variant()
        }
        toml::Value::Table(table) => toml_table(table).to_variant(),
    }
}
