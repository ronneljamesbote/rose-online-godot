//! Zone pages: the minimap with monster spawns, NPCs and warp gates on it.

use std::{collections::BTreeSet, path::Path};

use rose_data::{WarpGateId, ZoneData};
use rose_file_readers::{IfoFile, IfoReadOptions, VfsPath};
use serde_yaml::Value;

use crate::{
    images,
    page::{map, metres, one_line, row, seq, slug, strs, Page, Pages},
    Ctx,
};

/// Metres per minimap pixel, and the map block size in metres (godot/scripts/ui/hud/minimap.gd).
const METRES_PER_PIXEL: f32 = 2.5;
const BLOCK: f32 = 160.0;

struct Warp {
    x: f32,
    y: f32,
    to_zone: u16,
}

fn warps(cx: &Ctx, zone: &ZoneData) -> Vec<Warp> {
    let Some(file) = cx.zone_stb.as_ref().and_then(|s| s.try_get(zone.id.get() as usize, 1)) else { return Vec::new() };
    let zon = VfsPath::from(file);
    let Some(dir) = zon.path().parent() else { return Vec::new() };
    let block = zone.grid_size * zone.grid_per_patch * 16.0;
    let offset = 32.0 * block + block / 2.0;
    let options = IfoReadOptions {
        skip_monster_spawns: true,
        skip_npcs: true,
        skip_animated_objects: true,
        skip_collision_objects: true,
        skip_event_objects: true,
        skip_cnst_objects: true,
        skip_deco_objects: true,
        skip_effect_objects: true,
        skip_sound_objects: true,
        skip_water_planes: true,
        skip_warp_objects: false,
    };
    let mut out = Vec::new();
    for by in 0..64u32 {
        for bx in 0..64u32 {
            let path = dir.join(format!("{bx}_{by}.IFO"));
            if !cx.vfs.exists(path.as_path()) {
                continue;
            }
            let Ok(ifo) = cx.vfs.read_file_with::<IfoFile, _>(path, &options) else { continue };
            for w in &ifo.warps {
                let Some(gate) = cx.game.warp_gates.get_warp_gate(WarpGateId::new(w.warp_id)) else { continue };
                out.push(Warp { x: w.position.x + offset, y: w.position.y + offset, to_zone: gate.target_zone.get() });
            }
        }
    }
    out
}

fn planet(n: u32) -> &'static str {
    match n {
        1 => "Junon",
        2 => "Luna",
        3 => "Eldeon",
        4 => "Orlo",
        _ => "",
    }
}

pub fn write(cx: &Ctx, pages: &mut Pages, dds_dir: &Path) -> anyhow::Result<()> {
    let game = cx.game;
    let mut list = Page::new("zones", "list");
    list.set("title", "Zones");
    list.set("columns", strs(["planet", "monster_levels", "pvp"].map(String::from)));
    list.line("# Zones");
    pages.add(list);

    for zone in game.zones.iter() {
        let id = zone.id.get();
        let (path, name) = &cx.zone_pages[&id];
        let mut p = Page::new(path.clone(), "zone");
        p.set("id", id as i64);
        p.set("name", name.as_str());
        p.set("status", "in-game");
        p.set_some("planet", planet(zone.planet));
        p.set("pvp", match zone.pvp_state {
            0 => "no",
            1 => "yes, except your clan",
            2 => "yes, except your party",
            3 => "yes, everyone",
            11 => "clan war zone",
            _ => "other",
        });
        p.set_some("vehicles", match zone.vehicle_use_flags {
            1 => "no carts",
            2 => "no castle gear",
            3 => "no carts or castle gear",
            _ => "",
        });

        let mut monsters = BTreeSet::new();
        for sp in &zone.monster_spawns {
            for (n, _) in sp.basic_spawns.iter().chain(sp.tactic_spawns.iter()) {
                monsters.insert(n.get());
            }
        }
        let levels: Vec<i32> = monsters
            .iter()
            .filter_map(|m| rose_data::NpcId::new(*m).and_then(|n| game.npcs.get_npc(n)))
            .map(|n| n.level)
            .collect();
        if let (Some(lo), Some(hi)) = (levels.iter().min(), levels.iter().max()) {
            p.set("monster_levels", if lo == hi { lo.to_string() } else { format!("{lo}–{hi}") });
        }

        // The minimap and where its edges are in the world.
        let stb = cx.zone_stb.as_ref();
        let minimap = stb.and_then(|s| s.try_get(id as usize, 8)).filter(|s| !s.is_empty());
        let sx = stb.and_then(|s| s.try_get_int(id as usize, 9)).unwrap_or(0) as f32;
        let sy = stb.and_then(|s| s.try_get_int(id as usize, 10)).unwrap_or(0) as f32;
        if let Some(minimap) = minimap {
            let dds = format!("maps/{id}.dds");
            if let Some((w, h)) = images::export_dds(cx.vfs, minimap, &dds_dir.join(&dds)) {
                let image = format!("maps/{id}-{}.jpg", slug(name));
                images::queue(dds_dir, &dds, &image, "jpg");
                let left = (sx - 1.0) * BLOCK;
                let top = BLOCK * (66.0 - sy);
                p.set(
                    "map",
                    map(vec![
                        ("image", image.into()),
                        ("width", (w as i64).into()),
                        ("height", (h as i64).into()),
                        ("left", (left as i64).into()),
                        ("top", (top as i64).into()),
                        ("right", ((left + w as f32 * METRES_PER_PIXEL) as i64).into()),
                        ("bottom", ((top - h as f32 * METRES_PER_PIXEL) as i64).into()),
                    ]),
                );
            }
        }

        if !zone.monster_spawns.is_empty() {
            p.set(
                "monster_spawns",
                seq(zone.monster_spawns.iter().map(|sp| {
                    let names = |list: &Vec<(rose_data::NpcId, usize)>| {
                        list.iter().map(|(n, c)| format!("{} ×{c}", cx.npc_link(n.get()))).collect::<Vec<_>>().join(", ")
                    };
                    let tactic = names(&sp.tactic_spawns);
                    row([
                        ("monsters", names(&sp.basic_spawns).into()),
                        ("reinforcements", if tactic.is_empty() { Value::Null } else { tactic.into() }),
                        ("x", metres(sp.position.x)),
                        ("y", metres(sp.position.y)),
                        ("radius", (sp.range as i64).into()),
                        ("max_alive", (sp.limit_count as i64).into()),
                        ("respawn_seconds", (sp.interval as i64).into()),
                    ])
                })),
            );
        }
        if !zone.npcs.is_empty() {
            p.set(
                "npcs",
                seq(zone.npcs.iter().map(|n| {
                    row([("npc", cx.npc_link(n.npc_id.get()).into()), ("x", metres(n.position.x)), ("y", metres(n.position.y))])
                })),
            );
        }
        let w = warps(cx, zone);
        if !w.is_empty() {
            p.set(
                "warp_gates",
                seq(w.iter().map(|w| row([("to", cx.zone_link(w.to_zone).into()), ("x", metres(w.x)), ("y", metres(w.y))]))),
            );
        }
        let drops = cx.drop_row(id as usize);
        if !drops.is_empty() {
            p.set("zone_drops", strs(drops.iter().map(|(i, _)| cx.item_link(*i))));
        }
        p.set(
            "source",
            row([
                ("data", format!("LIST_ZONE.STB row {id}, the zone's IFO files, ITEM_DROP.STB row {id}").into()),
                ("code", strs(["module/src/world.rs".into(), "crates/rose-data-irose/src/zone_database.rs".into()])),
            ]),
        );
        p.line(format!("# {name}"));
        let desc = one_line(zone.description);
        if !desc.is_empty() {
            p.line("");
            p.line(desc);
        }
        p.line("");
        p.line("Positions are in metres. Each spawn point keeps up to \"max alive\" monsters; the");
        p.line("reinforcements show up once a point has been refilled many times (see");
        p.line("[[rules/monster-spawns|Monster spawns]]). Zone drops can come from any monster here (see");
        p.line("[[rules/drops|Drops]]).");
        pages.add(p);
    }
    Ok(())
}
