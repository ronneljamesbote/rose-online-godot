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
//! overrides/game-data.toml in the current folder, then adds motions.toml (ROSE_MOTIONS) and
//! the zone files in zones/ (ROSE_ZONES) from beside it (see overrides/README.md).

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
        let next_to_overrides = |name: &str| {
            path.parent().unwrap_or(Path::new("")).join(Path::new(name).file_name().unwrap())
        };
        let motions_path = std::env::var_os("ROSE_MOTIONS")
            .map(PathBuf::from)
            .unwrap_or_else(|| next_to_overrides(rose_game_overrides::MOTIONS_FILE));
        match std::fs::read(&motions_path) {
            Ok(data) => {
                let text = std::str::from_utf8(&data).expect("motions file is UTF-8");
                let timings = rose_game_overrides::MotionTimings::parse(text).expect("read the motions file");
                eprintln!("motions: {} motion timings from {}", timings.motion.len(), motions_path.display());
                files.insert(PathBuf::from(rose_game_overrides::MOTIONS_GAME_FILE), data);
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("motions: no {}, the ZMO timings stay", motions_path.display())
            }
            Err(e) => panic!("read {}: {e}", motions_path.display()),
        }

        // overrides/zones/*.toml (exported from the Godot zone scenes), next to the overrides
        // file unless ROSE_ZONES names the folder.
        let zones_dir = std::env::var_os("ROSE_ZONES")
            .map(PathBuf::from)
            .unwrap_or_else(|| next_to_overrides(rose_game_overrides::ZONES_DIR));
        let layouts = rose_game_overrides::ZoneLayout::load_dir(&zones_dir).expect("read the zone files");
        for (name, layout) in &layouts {
            eprintln!(
                "zones: zone {} from {name}: {} spawn points, {} NPCs, {} named spots",
                layout.zone,
                layout.spawn.len(),
                layout.npc.len(),
                layout.event.len()
            );
            let data = std::fs::read(zones_dir.join(name)).unwrap();
            files.insert(PathBuf::from(rose_game_overrides::zone_game_file(layout.zone)), data);
        }
        if layouts.is_empty() {
            eprintln!("zones: no files in {}, every zone keeps its IFO and ZON data", zones_dir.display());
        }

        // Fail here rather than on the server when a motion or a zone file doesn't fit the game.
        let mut memory = rose_file_readers::MemoryFilesystemDevice::default();
        for (path, data) in &files {
            memory.insert(&path.to_string_lossy(), data.clone());
        }
        if let Err(e) = rose_game_data::load_game_data(&VirtualFilesystem::new(vec![Box::new(memory)])) {
            eprintln!("overrides: {e:#}");
            std::process::exit(1);
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
            print_zone_changes(&zmo_data, &game_data);
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

/// For each zone the uploaded files change (`overrides/zones/`), how far its spawn points,
/// NPCs and named spots are from the ones in the IFO and ZON files.
fn print_zone_changes(files: &rose_game_data::GameData, uploaded: &rose_game_data::GameData) {
    for (a, b) in files.zones.iter().zip(uploaded.zones.iter()) {
        let same_spawns = a.monster_spawns.len() == b.monster_spawns.len()
            && a.monster_spawns.iter().zip(&b.monster_spawns).all(|(a, b)| {
                (a.interval, a.limit_count, a.range, a.tactic_points) == (b.interval, b.limit_count, b.range, b.tactic_points)
                    && a.basic_spawns == b.basic_spawns
                    && a.tactic_spawns == b.tactic_spawns
            });
        let same_npcs = a.npcs.len() == b.npcs.len()
            && a.npcs.iter().zip(&b.npcs).all(|(a, b)| a.npc_id == b.npc_id && a.conversation.get() == b.conversation.get());
        let moved = |a: glam::Vec3, b: glam::Vec3| a.distance(b);
        let spawn_moved = a.monster_spawns.iter().zip(&b.monster_spawns).map(|(a, b)| moved(a.position, b.position));
        let npc_moved = a.npcs.iter().zip(&b.npcs).map(|(a, b)| moved(a.position, b.position));
        let turned = a.npcs.iter().zip(&b.npcs).map(|(a, b)| (a.direction - b.direction).abs()).fold(0.0, f32::max);
        let event_moved = a.event_positions.iter().map(|(name, p)| b.event_positions.get(name).map_or(f32::INFINITY, |q| moved(*p, *q)));
        let most = spawn_moved.chain(npc_moved).chain(event_moved).fold(0.0, f32::max);
        if same_spawns && same_npcs && most == 0.0 && turned == 0.0 && a.event_positions.len() == b.event_positions.len() {
            continue;
        }
        println!(
            "zone {} {}: spawns {} -> {}{}, NPCs {} -> {}{}, named spots {} -> {}, moved at most {:.2} cm, turned at most {:.2} degrees, start ({:.0}, {:.0}) -> ({:.0}, {:.0})",
            a.id.get(),
            a.name,
            a.monster_spawns.len(),
            b.monster_spawns.len(),
            if same_spawns { " (same monsters and timing)" } else { " (monsters or timing differ)" },
            a.npcs.len(),
            b.npcs.len(),
            if same_npcs { " (same NPCs)" } else { " (NPCs differ)" },
            a.event_positions.len(),
            b.event_positions.len(),
            most,
            turned,
            a.start_position.x,
            a.start_position.y,
            b.start_position.x,
            b.start_position.y,
        );
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
