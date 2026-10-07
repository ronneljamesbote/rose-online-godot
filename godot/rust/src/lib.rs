//! Godot 4 extension that reads ROSE Online data files and builds Godot nodes from them.

use godot::prelude::*;

mod character;
mod conversation;
mod data;
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
            items::clear_cache();
            material::clear_cache();
            mesh::clear_cache();
            texture::clear_cache();
        }
    }
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
                false
            }
        }
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
}
