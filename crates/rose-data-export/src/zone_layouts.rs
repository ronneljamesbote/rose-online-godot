//! `export/zones/<id>-<name>.toml`: what the server needs from a zone that is edited as
//! Godot scenes (`godot/zones/<id>_<name>/`): its monster spawn points, town NPCs and named
//! event spots (start, revive and warp targets). Exported from the scenes by
//! `--export-data`; the server's game data uses them in place of the zone's IFO and ZON
//! files, so moving a spawn marker in the Godot editor moves the spawn on the server too.
//!
//! Positions are in metres in the server's frame: `x` east, `y` north, `z` up.

use std::path::Path;

use anyhow::Context;
use serde::Deserialize;

/// The folder in the repository (and in the export folder on a server).
pub const ZONES_DIR: &str = "export/zones";

/// Where the upload puts a zone's file among the game files the server reads.
pub fn zone_game_file(zone: u16) -> String {
    format!("EXPORT/ZONES/{zone}.TOML")
}

#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ZoneLayout {
    /// The zone number (its `RoseZoneSettings` id).
    pub zone: u16,
    #[serde(default)]
    pub event: Vec<ZoneEvent>,
    #[serde(default)]
    pub spawn: Vec<ZoneSpawn>,
    #[serde(default)]
    pub npc: Vec<ZoneNpc>,
}

/// A named spot (ZON event position): the zone's settings name the start and revive spots,
/// the warp gates their targets.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ZoneEvent {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// A monster spawn point.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ZoneSpawn {
    /// The scene node it came from, written as a comment.
    #[serde(skip)]
    pub node: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    /// Seconds between spawn checks.
    pub interval: u32,
    /// Monsters alive at most.
    pub limit: u32,
    /// Metres around the point that monsters appear in.
    pub range: u32,
    pub tactic_points: u32,
    /// `[monster, count]` pairs.
    #[serde(default)]
    pub basic: Vec<[u32; 2]>,
    #[serde(default)]
    pub tactic: Vec<[u32; 2]>,
}

/// A town NPC.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ZoneNpc {
    #[serde(skip)]
    pub node: String,
    /// The NPC number (its `RoseNpc` id).
    pub npc: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    /// Degrees it faces, around the up axis.
    pub direction: f32,
    /// Its conversation (quest script) file.
    pub conversation: String,
}

/// Metres with centimetre precision, as written to the file.
fn metres(value: f32) -> String {
    let text = format!("{value:.2}");
    if text == "-0.00" { "0.00".into() } else { text }
}

impl ZoneLayout {
    pub fn parse(text: &str) -> Result<Self, anyhow::Error> {
        let layout: Self = toml::from_str(text)?;
        if layout.zone == 0 {
            anyhow::bail!("zone = 0");
        }
        if let Some(bad) = layout.spawn.iter().find(|s| s.limit == 0 || s.interval == 0) {
            anyhow::bail!("the spawn at {}, {} has limit or interval 0", bad.x, bad.y);
        }
        Ok(layout)
    }

    pub fn load(path: &Path) -> Result<Self, anyhow::Error> {
        let text = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        Self::parse(&text).with_context(|| format!("reading {}", path.display()))
    }

    /// Every `*.toml` in a folder, by file name; a missing folder means none.
    pub fn load_dir(dir: &Path) -> Result<Vec<(String, Self)>, anyhow::Error> {
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e).with_context(|| format!("reading {}", dir.display())),
        };
        let mut files: Vec<_> = entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|e| e == "toml"))
            .collect();
        files.sort();
        files
            .into_iter()
            .map(|path| Ok((path.file_name().unwrap().to_string_lossy().to_string(), Self::load(&path)?)))
            .collect()
    }

    /// The file as the export writes it. `name` gives a monster or NPC's name for the
    /// comments.
    pub fn to_toml(&self, zone_name: &str, name: impl Fn(u32) -> String) -> String {
        let mut out = format!(
            "# {zone_name}: spawn points, town NPCs and named spots for the server.\n\
             # Exported from the Godot scenes in godot/zones/; don't edit by hand. Move the\n\
             # markers in the Godot editor and run\n\
             #   godot --headless --path godot -- --export-data\n\
             # Positions are metres (x east, y north, z up). See export/README.md.\n\
             zone = {}\n",
            self.zone
        );
        for event in &self.event {
            out.push_str(&format!(
                "\n[[event]]\nname = {:?}\nx = {}\ny = {}\nz = {}\n",
                event.name,
                metres(event.x),
                metres(event.y),
                metres(event.z)
            ));
        }
        let pairs = |list: &[[u32; 2]]| {
            let values: Vec<String> = list.iter().map(|[m, c]| format!("[{m}, {c}]")).collect();
            let names: Vec<String> = list.iter().map(|[m, c]| format!("{} x{c}", name(*m))).collect();
            format!("[{}]  # {}", values.join(", "), names.join(", "))
        };
        for spawn in &self.spawn {
            out.push_str(&format!(
                "\n[[spawn]]  # {}\nx = {}\ny = {}\nz = {}\ninterval = {}\nlimit = {}\nrange = {}\ntactic_points = {}\n",
                spawn.node,
                metres(spawn.x),
                metres(spawn.y),
                metres(spawn.z),
                spawn.interval,
                spawn.limit,
                spawn.range,
                spawn.tactic_points
            ));
            if !spawn.basic.is_empty() {
                out.push_str(&format!("basic = {}\n", pairs(&spawn.basic)));
            }
            if !spawn.tactic.is_empty() {
                out.push_str(&format!("tactic = {}\n", pairs(&spawn.tactic)));
            }
        }
        for npc in &self.npc {
            out.push_str(&format!(
                "\n[[npc]]  # {}\nnpc = {}  # {}\nx = {}\ny = {}\nz = {}\ndirection = {:.1}\nconversation = {:?}\n",
                npc.node,
                npc.npc,
                name(npc.npc),
                metres(npc.x),
                metres(npc.y),
                metres(npc.z),
                npc.direction,
                npc.conversation
            ));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        let layout = ZoneLayout {
            zone: 20,
            event: vec![ZoneEvent { name: "start".into(), x: 5300.5, y: 5200.25, z: 12.0 }],
            spawn: vec![ZoneSpawn {
                node: "Block_32_34/Spawns/Spawn_0".into(),
                x: 5348.0,
                y: 5294.0,
                z: 3.5,
                interval: 5,
                limit: 17,
                range: 40,
                tactic_points: 100,
                basic: vec![[1, 2], [8, 1]],
                tactic: vec![],
            }],
            npc: vec![ZoneNpc {
                node: "Block_32_34/Npcs/Npc_0".into(),
                npc: 1030,
                x: 5310.0,
                y: 5220.0,
                z: 10.0,
                direction: 90.0,
                conversation: "3DDATA/QUESTDATA/NPC.QSD".into(),
            }],
        };
        let text = layout.to_toml("Birth Island", |id| format!("Monster {id}"));
        assert!(text.contains("basic = [[1, 2], [8, 1]]  # Monster 1 x2, Monster 8 x1\n"));
        assert!(text.contains("[[spawn]]  # Block_32_34/Spawns/Spawn_0\n"));
        let mut parsed = ZoneLayout::parse(&text).unwrap();
        parsed.spawn[0].node = layout.spawn[0].node.clone();
        parsed.npc[0].node = layout.npc[0].node.clone();
        assert_eq!(parsed, layout);
    }

    #[test]
    fn rejects_unknown_keys_and_bad_spawns() {
        assert!(ZoneLayout::parse("zone = 20\nname = \"x\"\n").is_err());
        assert!(ZoneLayout::parse("zone = 0\n").is_err());
        let spawn = "zone = 20\n[[spawn]]\nx = 0\ny = 0\nz = 0\ninterval = 5\nlimit = 0\nrange = 1\ntactic_points = 1\n";
        assert!(ZoneLayout::parse(spawn).is_err());
    }
}
