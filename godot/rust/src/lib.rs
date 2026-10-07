//! Godot 4 extension that reads ROSE Online data files and builds Godot nodes from them.

use godot::prelude::*;

mod character;
mod data;
mod material;
mod mesh;
mod texture;
mod zone;

struct RoseExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RoseExtension {}

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
}
