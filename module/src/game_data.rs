//! The game databases (items, monsters, skills, zones, ...) inside the module.
//!
//! The host uploads the client files the databases are built from (`server/upload-game-data.sh`
//! runs the import tool's `pack` and calls the reducers below), so no game data
//! ships with the module. Each module instance builds the databases once from the
//! `game_file` table and keeps them until the next upload bumps the version.

use std::{cell::RefCell, rc::Rc};

use base64::Engine;
use rose_file_readers::{MemoryFilesystemDevice, VirtualFilesystem};
use rose_game_data::GameData;
use spacetimedb::{ReducerContext, SpacetimeType, Table};

use crate::require_admin;

#[spacetimedb::table(accessor = game_file)]
pub struct GameFile {
    #[primary_key]
    pub path: String,
    pub data: Vec<u8>,
}

/// Whether the server has game data. Public so clients can say why nothing spawns.
#[spacetimedb::table(accessor = game_data_status, public)]
#[derive(Clone)]
pub struct GameDataStatus {
    #[primary_key]
    pub id: u8,
    pub version: u64,
    pub files: u32,
    pub bytes: u64,
    pub ready: bool,
    pub error: String,
}

#[derive(SpacetimeType)]
pub struct GameFileUpload {
    pub path: String,
    /// File contents, base64.
    pub data: String,
}

thread_local! {
    static CACHE: RefCell<Option<(u64, Rc<GameData>)>> = const { RefCell::new(None) };
}

/// The game databases, built on first use after each upload.
pub fn game(ctx: &ReducerContext) -> Result<Rc<GameData>, String> {
    let status = ctx
        .db
        .game_data_status()
        .id()
        .find(0)
        .filter(|s| s.ready)
        .ok_or("the server has no game data yet")?;
    if let Some(data) = CACHE.with(|c| {
        c.borrow().as_ref().filter(|(version, _)| *version == status.version).map(|(_, d)| d.clone())
    }) {
        return Ok(data);
    }
    let data = Rc::new(load(ctx)?);
    CACHE.with(|c| *c.borrow_mut() = Some((status.version, data.clone())));
    Ok(data)
}

fn load(ctx: &ReducerContext) -> Result<GameData, String> {
    let mut device = MemoryFilesystemDevice::default();
    for file in ctx.db.game_file().iter() {
        device.insert(&file.path, file.data);
    }
    let vfs = VirtualFilesystem::new(vec![Box::new(device)]);
    rose_game_data::load_game_data(&vfs).map_err(|e| format!("loading game data: {e:#}"))
}

fn status(ctx: &ReducerContext) -> GameDataStatus {
    ctx.db.game_data_status().id().find(0).unwrap_or(GameDataStatus {
        id: 0,
        version: 0,
        files: 0,
        bytes: 0,
        ready: false,
        error: String::new(),
    })
}

fn save_status(ctx: &ReducerContext, s: GameDataStatus) {
    if ctx.db.game_data_status().id().find(0).is_some() {
        ctx.db.game_data_status().id().update(s);
    } else {
        ctx.db.game_data_status().insert(s);
    }
}

/// Start a new upload: forget the old files.
#[spacetimedb::reducer]
pub fn begin_game_data_upload(ctx: &ReducerContext) -> Result<(), String> {
    require_admin(ctx)?;
    let paths: Vec<String> = ctx.db.game_file().iter().map(|f| f.path).collect();
    for path in paths {
        ctx.db.game_file().path().delete(path);
    }
    let mut s = status(ctx);
    s.ready = false;
    s.files = 0;
    s.bytes = 0;
    s.error.clear();
    save_status(ctx, s);
    Ok(())
}

#[spacetimedb::reducer]
pub fn upload_game_files(ctx: &ReducerContext, files: Vec<GameFileUpload>) -> Result<(), String> {
    require_admin(ctx)?;
    let engine = base64::engine::general_purpose::STANDARD;
    let mut s = status(ctx);
    for file in files {
        let data = engine.decode(file.data.as_bytes()).map_err(|e| format!("{}: {e}", file.path))?;
        s.files += 1;
        s.bytes += data.len() as u64;
        ctx.db.game_file().path().delete(file.path.clone());
        ctx.db.game_file().insert(GameFile { path: file.path, data });
    }
    save_status(ctx, s);
    Ok(())
}

/// Build the databases from the uploaded files and set up zones and spawn points from them.
#[spacetimedb::reducer]
pub fn finish_game_data_upload(ctx: &ReducerContext) -> Result<(), String> {
    require_admin(ctx)?;
    let mut s = status(ctx);
    s.version += 1;
    s.ready = true;
    s.error.clear();
    save_status(ctx, s.clone());
    match game(ctx) {
        Ok(game) => {
            crate::world::setup_zones(ctx, &game);
            log::info!("game data ready: {} files, {} bytes", s.files, s.bytes);
            Ok(())
        }
        Err(error) => {
            s.ready = false;
            s.error = error.clone();
            save_status(ctx, s);
            // Keep the failure visible in the status row rather than rolling it back.
            log::error!("{error}");
            Ok(())
        }
    }
}
