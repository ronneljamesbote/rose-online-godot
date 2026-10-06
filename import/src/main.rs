//! Reads zone and monster data from a ROSE 129_129en install with rose-offline's
//! readers and writes the JSON arguments for the module's import reducers.
//!
//! Usage: rose-stdb-import <data.idx> <zone id> <out dir>

use std::{collections::BTreeSet, path::Path, sync::Arc};

use rose_data::{NpcDatabaseOptions, NpcMotionAction, ZoneId};
use rose_data_irose::{get_npc_database, get_string_database, get_zone_database};
use rose_file_readers::{HostFilesystemDevice, VfsIndex, VirtualFilesystem};
use serde_json::json;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [_, data_idx, zone_id, out_dir] = &args[..] else {
        eprintln!("usage: rose-stdb-import <data.idx> <zone id> <out dir>");
        std::process::exit(2);
    };
    let data_idx = Path::new(data_idx);
    let zone_id: u16 = zone_id.parse().expect("zone id");
    let out_dir = Path::new(out_dir);
    std::fs::create_dir_all(out_dir).unwrap();

    let vfs = VirtualFilesystem::new(vec![
        Box::new(VfsIndex::load(data_idx).expect("load data.idx")),
        Box::new(HostFilesystemDevice::new(
            data_idx.parent().map(|p| p.to_path_buf()).unwrap_or_default(),
        )),
    ]);
    let strings = get_string_database(&vfs, 1).expect("strings");
    let npcs = Arc::new(
        get_npc_database(&vfs, strings.clone(), &NpcDatabaseOptions { load_frame_data: true })
            .expect("npcs"),
    );
    let zones = get_zone_database(&vfs, strings).expect("zones");
    let zone = zones.get_zone(ZoneId::new(zone_id).unwrap()).expect("zone not found");

    let size_x = zone.num_sectors_x as f32 * zone.sector_size as f32;
    let size_y = zone.num_sectors_y as f32 * zone.sector_size as f32;
    let revive = zone.revive_positions.first().copied().unwrap_or(zone.start_position);
    let zone_json = json!({
        "zone_id": zone_id,
        "name": zone.name,
        "start_x": zone.start_position.x, "start_y": zone.start_position.y,
        "revive_x": revive.x, "revive_y": revive.y,
        "min_x": zone.sectors_base_position.x, "min_y": zone.sectors_base_position.y,
        "max_x": zone.sectors_base_position.x + size_x, "max_y": zone.sectors_base_position.y + size_y,
    });

    let mut spawns = Vec::new();
    let mut used_npcs = BTreeSet::new();
    let mut next_id = zone_id as u32 * 10_000;
    for point in &zone.monster_spawns {
        for (npc_id, count) in &point.basic_spawns {
            next_id += 1;
            used_npcs.insert(npc_id.get());
            spawns.push(json!({
                "spawn_id": next_id,
                "zone_id": zone_id,
                "x": point.position.x, "y": point.position.y,
                "radius": (point.range * 100) as f32,
                "npc_id": npc_id.get(),
                "max_alive": (*count as u32).min(point.limit_count.max(1)),
                "respawn_secs": point.interval.max(5),
                "next_spawn_at_us": 0,
            }));
        }
    }

    let mut npc_rows = Vec::new();
    for npc_id in &used_npcs {
        let npc = npcs.get_npc(rose_data::NpcId::new(*npc_id).unwrap()).unwrap();
        let attack_ms = npcs
            .get_npc_action_motion(npc.id, NpcMotionAction::Attack)
            .map_or(1000, |m| m.duration.as_millis() as i32)
            .max(300);
        npc_rows.push(json!({
            "npc_id": npc_id,
            "name": npc.name,
            "level": npc.level,
            "max_hp": npc.level * npc.health_points,
            "attack_power": npc.attack,
            "hit": npc.hit,
            "defence": npc.defence,
            "avoid": npc.avoid,
            "attack_speed": npc.attack_speed,
            "attack_motion_ms": attack_ms,
            "attack_range": npc.attack_range as f32,
            "walk_speed": npc.walk_speed as f32,
            "run_speed": npc.run_speed as f32,
            "aggro_range": 0.0,
        }));
    }

    std::fs::write(out_dir.join("npcs.json"), json!([npc_rows]).to_string()).unwrap();
    std::fs::write(out_dir.join("zone.json"), json!([zone_json, spawns]).to_string()).unwrap();
    println!(
        "zone {} {}: {} spawn entries, {} monster types -> {}",
        zone_id, zone.name, spawns.len(), npc_rows.len(), out_dir.display()
    );
    for row in &npc_rows {
        println!("  {} {} lv{} hp{} atk{} def{} spd{} motion{}ms range{}",
            row["npc_id"], row["name"], row["level"], row["max_hp"], row["attack_power"],
            row["defence"], row["attack_speed"], row["attack_motion_ms"], row["attack_range"]);
    }
}
