//! Godot 4 extension that reads ROSE Online data files and builds Godot nodes from them.

use godot::prelude::*;

mod character;
mod data;
mod items;
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
}
