//! Lists the client files the server's game databases read, or packs them for upload.
//!
//! Usage:
//!   rose-stdb-import list <data.idx>       files per folder
//!   rose-stdb-import time <data.idx>       how long the databases take to build from memory
//!   rose-stdb-import pack <data.idx> <dir> upload batches for server/upload-game-data.sh
//!
//! `pack` writes upload-NNN.json files, each the JSON argument list of the module's
//! `upload_game_files` reducer (well under the server's request size limit), plus manifest.json.
//! It first applies our data changes: the file named by ROSE_OVERRIDES, else
//! overrides/game-data.toml in the current folder (see overrides/README.md).

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::Mutex,
    time::Instant,
};

use base64::Engine;
use rose_file_readers::{
    HostFilesystemDevice, VfsFile, VfsIndex, VfsPath, VirtualFilesystem, VirtualFilesystemDevice,
};
use serde_json::json;

/// Passes reads through to another device and remembers every file that was read.
struct RecordingDevice<D> {
    inner: D,
    read: &'static Mutex<BTreeMap<PathBuf, Vec<u8>>>,
}

impl<D: VirtualFilesystemDevice> VirtualFilesystemDevice for RecordingDevice<D> {
    fn open_file(&self, path: &VfsPath) -> Result<VfsFile<'_>, anyhow::Error> {
        let file = self.inner.open_file(path)?;
        let bytes = match &file {
            VfsFile::Buffer(data) => data.clone(),
            VfsFile::View(data) => data.to_vec(),
        };
        self.read.lock().unwrap().insert(path.path().to_path_buf(), bytes);
        Ok(file)
    }

    fn exists(&self, path: &VfsPath) -> bool {
        self.inner.exists(path)
    }
}

/// Raw bytes per upload call; SpacetimeDB refuses request bodies much over 1 MB.
const MAX_CALL_BYTES: usize = 600 << 10;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (mode, data_idx) = match &args[..] {
        [_, mode, data_idx, ..] => (mode.as_str(), Path::new(data_idx)),
        _ => {
            eprintln!("usage: rose-stdb-import list|time|pack <data.idx> [out dir]");
            std::process::exit(2);
        }
    };
    let read: &'static Mutex<BTreeMap<PathBuf, Vec<u8>>> = Box::leak(Box::default());
    let root = data_idx.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let vfs = VirtualFilesystem::new(vec![
        Box::new(RecordingDevice { inner: VfsIndex::load(data_idx).expect("load data.idx"), read }),
        Box::new(RecordingDevice { inner: HostFilesystemDevice::new(root), read }),
    ]);
    let started = Instant::now();
    let zmo_data = rose_game_data::load_game_data(&vfs).expect("load game data");
    let mut files = std::mem::take(&mut *read.lock().unwrap());
    if mode == "pack" || mode == "time" {
        let path = std::env::var_os("ROSE_OVERRIDES")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(rose_game_overrides::OVERRIDES_FILE));
        let overrides = rose_game_overrides::Overrides::load(&path).expect("read the overrides file");
        let cells = overrides.patch_files(&mut files).expect("apply the overrides");
        eprintln!("overrides: {} changed cells from {}", cells, path.display());

        // overrides/motions.toml, next to the overrides file unless ROSE_MOTIONS names it.
        let path = std::env::var_os("ROSE_MOTIONS").map(PathBuf::from).unwrap_or_else(|| {
            path.parent().unwrap_or(Path::new("")).join(Path::new(rose_game_overrides::MOTIONS_FILE).file_name().unwrap())
        });
        match std::fs::read(&path) {
            Ok(data) => {
                let text = std::str::from_utf8(&data).expect("motions file is UTF-8");
                let timings = rose_game_overrides::MotionTimings::parse(text).expect("read the motions file");
                eprintln!("motions: {} motion timings from {}", timings.motion.len(), path.display());
                files.insert(PathBuf::from(rose_game_overrides::MOTIONS_GAME_FILE), data);
                // Fail here rather than on the server when a motion isn't in the game.
                let mut memory = rose_file_readers::MemoryFilesystemDevice::default();
                for (path, data) in &files {
                    memory.insert(&path.to_string_lossy(), data.clone());
                }
                if let Err(e) = rose_game_data::load_game_data(&VirtualFilesystem::new(vec![Box::new(memory)])) {
                    eprintln!("motions: {e:#}");
                    std::process::exit(1);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("motions: no {}, the ZMO timings stay", path.display())
            }
            Err(e) => panic!("read {}: {e}", path.display()),
        }
    }
    let total: usize = files.values().map(|d| d.len()).sum();
    eprintln!(
        "game data: {} files, {:.1} MB, loaded in {} ms",
        files.len(),
        total as f64 / 1e6,
        started.elapsed().as_millis()
    );

    match mode {
        "time" => {
            let mut memory = rose_file_readers::MemoryFilesystemDevice::default();
            for (path, data) in &files {
                memory.insert(&path.to_string_lossy(), data.clone());
            }
            let vfs = VirtualFilesystem::new(vec![Box::new(memory)]);
            let started = Instant::now();
            let game_data = rose_game_data::load_game_data(&vfs).expect("load from memory");
            eprintln!("from memory: {} ms", started.elapsed().as_millis());
            print_motion_changes(&zmo_data, &game_data);
            let started = Instant::now();
            let strings = rose_data_irose::get_string_database(&vfs, 1).unwrap();
            rose_data_irose::get_zone_database(&vfs, strings).unwrap();
            eprintln!("zones from memory: {} ms", started.elapsed().as_millis());
        }
        "list" => {
            let mut by_dir: BTreeMap<String, (usize, usize)> = BTreeMap::new();
            for (path, data) in &files {
                let dir = path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
                let entry = by_dir.entry(dir).or_default();
                entry.0 += 1;
                entry.1 += data.len();
            }
            for (dir, (count, bytes)) in by_dir {
                println!("{:>5} files {:>9} bytes  {}", count, bytes, dir);
            }
        }
        "pack" => {
            let out_dir = Path::new(args.get(3).expect("out dir"));
            std::fs::create_dir_all(out_dir).unwrap();
            let engine = base64::engine::general_purpose::STANDARD;
            let mut batch = Vec::new();
            let mut batch_bytes = 0;
            let mut batches = 0;
            let flush = |batch: &mut Vec<serde_json::Value>, batches: &mut usize| {
                if batch.is_empty() {
                    return;
                }
                *batches += 1;
                let name = out_dir.join(format!("upload-{:03}.json", batches));
                std::fs::write(name, json!([std::mem::take(batch)]).to_string()).unwrap();
            };
            for (path, data) in &files {
                if batch_bytes + data.len() > MAX_CALL_BYTES {
                    flush(&mut batch, &mut batches);
                    batch_bytes = 0;
                }
                batch_bytes += data.len();
                batch.push(json!({ "path": path.to_string_lossy(), "data": engine.encode(data) }));
            }
            flush(&mut batch, &mut batches);
            std::fs::write(
                out_dir.join("manifest.json"),
                json!({ "files": files.len(), "bytes": total, "uploads": batches }).to_string(),
            )
            .unwrap();
            eprintln!("packed into {} uploads in {}", batches, out_dir.display());
        }
        _ => {
            eprintln!("unknown mode {mode}");
            std::process::exit(2);
        }
    }
}

/// Every motion whose length or hits differ between the ZMO files and the uploaded game
/// files (with `overrides/motions.toml`), then the sword swings the server times hits by.
fn print_motion_changes(zmo: &rose_game_data::GameData, uploaded: &rose_game_data::GameData) {
    use rose_data::MotionFileData;
    let ms = |d: Option<std::time::Duration>| d.map_or("-".to_string(), |d| format!("{} ms", d.as_millis()));
    let (mut changed, mut total) = (0, 0);
    let mut compare = |a: &MotionFileData, b: &MotionFileData| {
        total += 1;
        if (a.duration, a.total_attack_frames, a.first_attack_time) != (b.duration, b.total_attack_frames, b.first_attack_time) {
            changed += 1;
            println!(
                "changed {}: {} ms {} hits first {} -> {} ms {} hits first {}",
                a.path.path().display(),
                a.duration.as_millis(),
                a.total_attack_frames,
                ms(a.first_attack_time),
                b.duration.as_millis(),
                b.total_attack_frames,
                ms(b.first_attack_time)
            );
        }
    };
    for gender in 0..2 {
        for (a, b) in zmo.motions.motion_files(gender).zip(uploaded.motions.motion_files(gender)) {
            compare(a, b);
        }
    }
    for (a, b) in zmo.npcs.iter().zip(uploaded.npcs.iter()) {
        for ((_, a), (_, b)) in a.motion_data.iter().zip(b.motion_data.iter()) {
            compare(a, b);
        }
    }
    println!("motions: {changed} of {total} (player motions, and each NPC's) differ from the ZMO files");
    use rose_data::CharacterMotionAction::{Attack, Attack2, Attack3};
    for (gender, name) in [(0, "male"), (1, "female")] {
        let times: Vec<String> = [Attack, Attack2, Attack3]
            .into_iter()
            .map(|action| ms(uploaded.motions.get_character_action_motion(action, 1, gender).and_then(|m| m.first_attack_time)))
            .collect();
        println!("{name} one-handed sword hits: {}", times.join(", "));
    }
    for id in [1u16, 2, 3] {
        let Some(npc_id) = rose_data::NpcId::new(id) else { continue };
        let attack = uploaded.npcs.get_npc_action_motion(npc_id, rose_data::NpcMotionAction::Attack);
        let name = uploaded.npcs.get_npc(npc_id).map_or("?", |n| n.name);
        println!(
            "monster {id} {name}: attack {} ms, hit at {}",
            attack.map_or(0, |m| m.duration.as_millis()),
            ms(attack.and_then(|m| m.first_attack_time))
        );
    }
}
