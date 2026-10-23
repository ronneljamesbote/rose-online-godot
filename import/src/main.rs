//! Lists the client files the server's game databases read, or packs them for upload.
//!
//! Usage:
//!   rose-stdb-import list <data.idx>       files per folder
//!   rose-stdb-import time <data.idx>       how long the databases take to build from memory
//!   rose-stdb-import pack <data.idx> <dir> upload batches for server/upload-game-data.sh
//!   rose-stdb-import compare <data.idx>    every value the data export changes from iROSE
//!                                          (items, monsters, NPCs, shop tabs, drop tables,
//!                                          skills, status effects, job classes, zones, warp
//!                                          gates, recipes, new characters)
//!
//! `pack` writes upload-NNN.json files, each the JSON argument list of the module's
//! `upload_game_files` reducer (well under the server's request size limit), plus manifest.json.
//! It sends what the Godot editor exported (see export/README.md), from the folder named by
//! ROSE_EXPORT, else export/ in the current folder: the game data in data/ (or ROSE_DATA),
//! read in place of the iROSE tables it replaces, which are then neither read nor sent; the
//! motion timings in motions.toml (or ROSE_MOTIONS); the zone files in zones/ (or
//! ROSE_ZONES).

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
            eprintln!("usage: rose-stdb-import list|time|pack|compare <data.idx> [out dir]");
            std::process::exit(2);
        }
    };
    let export_dir = std::env::var_os("ROSE_EXPORT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(rose_data_export::EXPORT_DIR));
    // A file or folder of the export (by its path in the repository), unless `variable` names it.
    let in_export = |variable: &str, path: &str| {
        std::env::var_os(variable)
            .map(PathBuf::from)
            .unwrap_or_else(|| export_dir.join(Path::new(path).strip_prefix(rose_data_export::EXPORT_DIR).unwrap()))
    };
    let data_dir = in_export("ROSE_DATA", rose_data_export::DATA_DIR);
    let export = read_export(&data_dir);
    let read: &'static Mutex<BTreeMap<PathBuf, Vec<u8>>> = Box::leak(Box::default());
    let root = data_idx.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let mut export_device = rose_file_readers::MemoryFilesystemDevice::default();
    for (game_path, data) in &export {
        export_device.insert(&game_path.to_string_lossy(), data.clone());
    }
    let vfs = VirtualFilesystem::new(vec![
        Box::new(export_device),
        Box::new(RecordingDevice { inner: VfsIndex::load(data_idx).expect("load data.idx"), read }),
        Box::new(RecordingDevice { inner: HostFilesystemDevice::new(root), read }),
    ]);
    if mode == "compare" {
        compare(&vfs, &data_dir, export.is_empty());
        return;
    }
    let started = Instant::now();
    let zmo_data = rose_game_data::load_game_data(&vfs).expect("load game data");
    let mut files = std::mem::take(&mut *read.lock().unwrap());
    if export.is_empty() {
        eprintln!("data: no export in {}, the items come from the iROSE tables", data_dir.display());
    } else {
        eprintln!("data: {} files from {}", export.len(), data_dir.display());
    }
    files.extend(export);
    if mode == "pack" || mode == "time" {
        let motions_path = in_export("ROSE_MOTIONS", rose_data_export::MOTIONS_FILE);
        match std::fs::read(&motions_path) {
            Ok(data) => {
                let text = std::str::from_utf8(&data).expect("motions file is UTF-8");
                let timings = rose_data_export::MotionTimings::parse(text).expect("read the motions file");
                eprintln!("motions: {} motion timings from {}", timings.motion.len(), motions_path.display());
                files.insert(PathBuf::from(rose_data_export::MOTIONS_GAME_FILE), data);
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                eprintln!("motions: no {}, the ZMO timings stay", motions_path.display())
            }
            Err(e) => panic!("read {}: {e}", motions_path.display()),
        }

        // export/zones/*.toml (exported from the Godot zone scenes).
        let zones_dir = in_export("ROSE_ZONES", rose_data_export::ZONES_DIR);
        let layouts = rose_data_export::ZoneLayout::load_dir(&zones_dir).expect("read the zone files");
        for (name, layout) in &layouts {
            eprintln!(
                "zones: zone {} from {name}: {} spawn points, {} NPCs, {} named spots",
                layout.zone,
                layout.spawn.len(),
                layout.npc.len(),
                layout.event.len()
            );
            let data = std::fs::read(zones_dir.join(name)).unwrap();
            files.insert(PathBuf::from(rose_data_export::zone_game_file(layout.zone)), data);
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
            eprintln!("export: {e:#}");
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

/// The export (`items/*.toml`, `monsters.toml` ... under `dir`), checked and turned into the
/// JSON files the server reads (by their path among the game files). Empty when there is no
/// export.
fn read_export(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let read = |file: String| {
        let path = dir.join(file);
        match std::fs::read_to_string(&path) {
            Ok(text) => Ok(Some(text)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(anyhow::Error::new(e).context(path.display().to_string())),
        }
    };
    let fail = |e: anyhow::Error| -> ! {
        eprintln!("data: {e:#}");
        std::process::exit(1);
    };
    let items = rose_data_export::ItemsExport::from_files(|name| read(rose_data_export::items_file(name)))
        .unwrap_or_else(|e| fail(e));
    let toml = rose_data_export::Format::Toml;
    let npcs = rose_data_export::NpcsExport::from_files_as(toml, |name| read(rose_data_export::data_file_as(name, toml)))
        .unwrap_or_else(|e| fail(e));
    let skills = rose_data_export::SkillsExport::from_files_as(toml, |name| read(rose_data_export::data_file_as(name, toml)))
        .unwrap_or_else(|e| fail(e));
    let world = rose_data_export::WorldExport::from_files_as(toml, |name| read(rose_data_export::data_file_as(name, toml)))
        .unwrap_or_else(|e| fail(e));
    let mut files = BTreeMap::new();
    let format = rose_data_export::Format::Json;
    let mut add = |path: String, text: String| {
        let game_path = format!("{}/{}", rose_data_export::DATA_GAME_DIR, path);
        files.insert(PathBuf::from(game_path.to_uppercase()), text.into_bytes());
    };
    if let Some(items) = items {
        for (name, text) in items.to_files_as(format).expect("write the items as JSON") {
            add(rose_data_export::items_file_as(name, format), text);
        }
    }
    if let Some(npcs) = npcs {
        npcs.check().unwrap_or_else(|e| fail(e));
        for (name, text) in npcs.to_files_as(format).expect("write the monsters and NPCs as JSON") {
            add(rose_data_export::data_file_as(name, format), text);
        }
    }
    if let Some(skills) = skills {
        skills.check().unwrap_or_else(|e| fail(e));
        for (name, text) in skills.to_files_as(format).expect("write the skills as JSON") {
            add(rose_data_export::data_file_as(name, format), text);
        }
    }
    if let Some(world) = world {
        world.check().unwrap_or_else(|e| fail(e));
        for (name, text) in world.to_files_as(format).expect("write the world as JSON") {
            add(rose_data_export::data_file_as(name, format), text);
        }
    }
    files
}

/// The same-data check: loads the items, monsters, NPCs, shop tabs, drop tables, skills,
/// status effects, job classes, zone settings, warp gates, crafting recipes and new
/// characters from the iROSE tables and from the export and prints every value that differs.
fn compare(vfs: &VirtualFilesystem, data_dir: &Path, no_export: bool) {
    if no_export {
        eprintln!("compare: no export in {}", data_dir.display());
        std::process::exit(1);
    }
    let strings = rose_data_irose::get_string_database(vfs, 1).expect("string database");
    let started = Instant::now();
    let irose = rose_data_irose::get_item_database(vfs, strings.clone()).expect("iROSE items");
    let irose_ms = started.elapsed().as_millis();
    let started = Instant::now();
    let export_items = rose_game_data::load_items(vfs, strings.clone()).expect("exported items");
    let export_ms = started.elapsed().as_millis();
    let count = rose_data_export::ItemsExport::from_database(&export_items).count();
    let mut differences = rose_data_export::compare_item_databases(&irose, &export_items);
    println!(
        "items: {count} items and {} grades; iROSE tables {irose_ms} ms, export {export_ms} ms",
        export_items.item_grades().len(),
    );

    let options = rose_data::NpcDatabaseOptions { load_frame_data: false };
    let started = Instant::now();
    let irose_npcs = rose_data_irose::get_npc_database(vfs, strings.clone(), &options).expect("iROSE NPCs");
    let irose_rows = rose_game_irose::data::read_drop_rows(vfs, rose_data_export::npcs::DROP_ROW_COLUMNS)
        .expect("ITEM_DROP.STB");
    let irose_ms = started.elapsed().as_millis();
    let started = Instant::now();
    let (npcs, rows) = rose_game_data::load_npcs(vfs, strings.clone(), &options).expect("exported monsters and NPCs");
    let export_ms = started.elapsed().as_millis();
    match rows {
        None => println!("monsters and NPCs: no export (monsters.toml ...), skipped"),
        Some(rows) => {
            let export = rose_game_data::read_npcs_export(vfs).unwrap().unwrap();
            differences.extend(rose_data_export::compare_npc_databases(&irose_npcs, &npcs));
            differences.extend(rose_data_export::compare_drop_rows(&irose_rows, &rows));
            println!(
                "monsters and NPCs: {} monsters, {} NPCs, {} shop tabs, {} drop tables; iROSE tables {irose_ms} ms, export {export_ms} ms",
                export.monsters.len(),
                export.npcs.len(),
                export.shop_tabs.len(),
                export.drop_tables.len(),
            );
        }
    }

    let started = Instant::now();
    let irose_skills = rose_data_irose::get_skill_database(vfs, strings.clone()).expect("iROSE skills");
    let irose_effects = rose_data_irose::get_status_effect_database(vfs, strings.clone()).expect("iROSE status effects");
    let irose_classes = rose_data_irose::get_job_class_database(vfs, strings.clone()).expect("iROSE job classes");
    let irose_ms = started.elapsed().as_millis();
    match rose_game_data::read_skills_export(vfs).expect("read the skill export") {
        None => println!("skills: no export (skills.toml ...), skipped"),
        Some(export) => {
            let started = Instant::now();
            let (skills, effects, classes) = rose_game_data::load_skills(vfs, strings.clone()).expect("exported skills");
            let export_ms = started.elapsed().as_millis();
            rose_game_data::check_summons(&skills, &npcs).expect("summoned monsters");
            differences.extend(rose_data_export::compare_skill_databases(
                (&irose_skills, &irose_effects, &irose_classes),
                (&skills, &effects, &classes),
            ));
            println!(
                "skills: {} skills, {} status effects, {} job classes; iROSE tables {irose_ms} ms, export {export_ms} ms",
                export.skills.len(),
                export.status_effects.len(),
                export.job_classes.len(),
            );
        }
    }

    let started = Instant::now();
    let irose_world = rose_game_data::load_world_irose(vfs, &strings).expect("iROSE world");
    let irose_ms = started.elapsed().as_millis();
    match rose_game_data::read_world_export(vfs).expect("read the world export") {
        None => println!("world: no export (zones.toml ...), skipped"),
        Some(export) => {
            let started = Instant::now();
            let world = rose_game_data::world_from_export(&export).expect("exported world");
            let export_ms = started.elapsed().as_millis();
            world.check_items(&export_items).expect("the items the world names");
            differences.extend(rose_game_data::compare_worlds(&irose_world, &world));
            println!(
                "world: {} zones, {} warp gates, {} recipes, {} new characters; iROSE tables {irose_ms} ms, export {export_ms} ms",
                export.zones.len(),
                export.warp_gates.len(),
                export.craft_recipes.len(),
                export.new_characters.len(),
            );
        }
    }
    for line in &differences {
        println!("{line}");
    }
    println!("{} values differ", differences.len());
}

/// For each zone the uploaded files change (`export/zones/`), how far its spawn points,
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
/// files (with `export/motions.toml`), then the sword swings the server times hits by.
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
