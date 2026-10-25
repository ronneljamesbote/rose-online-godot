//! Sky and interface: `skies.toml` (one `[[sky]]` per sky, iROSE's LIST_SKY.STB: the dome
//! around a zone and its light at each time of day) and `interface.toml` (the `[interface]`
//! table: the loading and character creation pictures and the mouse cursors). The keys are
//! the property names of the Godot resources (`RoseSky`, `RoseInterface`) they are exported
//! from. A zone names its sky by id (its `sky`, -1 = none).
//!
//! Art is a path: a Godot file (`res://assets/rose/3ddata/sky/day_sky.mesh`, made by the
//! import, or one of our own in `res://assets/ours/`) or, without the import, the client
//! file (`3DDATA\SKY\DAY_SKY.ZMS`). Only the game draws these: the server doesn't read them.

use std::collections::BTreeSet;

use anyhow::bail;
use enum_map::enum_map;
use glam::Vec4;
use rose_data::{SkyboxData, SkyboxDatabase, SkyboxId, SkyboxState};
use serde::{Deserialize, Serialize};

use crate::Format;

pub const SKIES_FILE: &str = "skies";
pub const INTERFACE_FILE: &str = "interface";

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

/// A colour as the export holds it: red, green, blue from 0 to 255.
pub type Rgb = [u8; 3];

fn to_rgb(colour: Vec4) -> Rgb {
    let channel = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    [channel(colour.x), channel(colour.y), channel(colour.z)]
}

fn from_rgb(rgb: Rgb) -> Vec4 {
    Vec4::new(rgb[0] as f32 / 255.0, rgb[1] as f32 / 255.0, rgb[2] as f32 / 255.0, 1.0)
}

/// One sky.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SkyExport {
    /// The number (iROSE's LIST_SKY.STB row, from 0). Zones name it.
    pub id: u16,
    /// For finding it only (the zones under it).
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    /// The dome's mesh.
    #[serde(skip_serializing_if = "is_default")]
    pub mesh: String,
    /// The dome's picture by day, and by night (the game blends them through the evening and
    /// morning).
    #[serde(skip_serializing_if = "is_default")]
    pub texture_day: String,
    #[serde(skip_serializing_if = "is_default")]
    pub texture_night: String,
    /// The light on the ground and buildings at each time of day.
    pub map_ambient_morning: Rgb,
    pub map_ambient_day: Rgb,
    pub map_ambient_evening: Rgb,
    pub map_ambient_night: Rgb,
    /// The light on characters and monsters from all around.
    pub character_ambient_morning: Rgb,
    pub character_ambient_day: Rgb,
    pub character_ambient_evening: Rgb,
    pub character_ambient_night: Rgb,
    /// The light on characters and monsters from the sun's side.
    pub character_diffuse_morning: Rgb,
    pub character_diffuse_day: Rgb,
    pub character_diffuse_evening: Rgb,
    pub character_diffuse_night: Rgb,
    /// The sun's colour on the ground and buildings.
    pub sun_color_morning: Rgb,
    pub sun_color_day: Rgb,
    pub sun_color_evening: Rgb,
    pub sun_color_night: Rgb,
    /// How strongly the sun lights what faces it (1 = as bright as the ground light).
    pub sun_strength_morning: f32,
    pub sun_strength_day: f32,
    pub sun_strength_evening: f32,
    pub sun_strength_night: f32,
    /// How much of the ground light the sun gives: 0 = no shadows, 0.4 = shade is 60 % as bright.
    pub shadow_darkness_morning: f32,
    pub shadow_darkness_day: f32,
    pub shadow_darkness_evening: f32,
    pub shadow_darkness_night: f32,
}

impl SkyExport {
    pub fn from_data(sky: &SkyboxData) -> Self {
        let c = |colours: &enum_map::EnumMap<SkyboxState, Vec4>, state| to_rgb(colours[state]);
        use SkyboxState::*;
        Self {
            id: sky.id.get(),
            name: String::new(),
            mesh: sky.mesh.clone(),
            texture_day: sky.texture_day.clone(),
            texture_night: sky.texture_night.clone(),
            map_ambient_morning: c(&sky.map_ambient_color, Morning),
            map_ambient_day: c(&sky.map_ambient_color, Day),
            map_ambient_evening: c(&sky.map_ambient_color, Evening),
            map_ambient_night: c(&sky.map_ambient_color, Night),
            character_ambient_morning: c(&sky.character_ambient_color, Morning),
            character_ambient_day: c(&sky.character_ambient_color, Day),
            character_ambient_evening: c(&sky.character_ambient_color, Evening),
            character_ambient_night: c(&sky.character_ambient_color, Night),
            character_diffuse_morning: c(&sky.character_diffuse_color, Morning),
            character_diffuse_day: c(&sky.character_diffuse_color, Day),
            character_diffuse_evening: c(&sky.character_diffuse_color, Evening),
            character_diffuse_night: c(&sky.character_diffuse_color, Night),
            sun_color_morning: c(&sky.sun_color, Morning),
            sun_color_day: c(&sky.sun_color, Day),
            sun_color_evening: c(&sky.sun_color, Evening),
            sun_color_night: c(&sky.sun_color, Night),
            sun_strength_morning: sky.sun_strength[Morning],
            sun_strength_day: sky.sun_strength[Day],
            sun_strength_evening: sky.sun_strength[Evening],
            sun_strength_night: sky.sun_strength[Night],
            shadow_darkness_morning: sky.shadow_darkness[Morning],
            shadow_darkness_day: sky.shadow_darkness[Day],
            shadow_darkness_evening: sky.shadow_darkness[Evening],
            shadow_darkness_night: sky.shadow_darkness[Night],
        }
    }

    pub fn to_data(&self) -> SkyboxData {
        SkyboxData {
            id: SkyboxId::new(self.id),
            mesh: self.mesh.clone(),
            texture_day: self.texture_day.clone(),
            texture_night: self.texture_night.clone(),
            map_ambient_color: enum_map! {
                SkyboxState::Morning => from_rgb(self.map_ambient_morning),
                SkyboxState::Day => from_rgb(self.map_ambient_day),
                SkyboxState::Evening => from_rgb(self.map_ambient_evening),
                SkyboxState::Night => from_rgb(self.map_ambient_night),
            },
            character_ambient_color: enum_map! {
                SkyboxState::Morning => from_rgb(self.character_ambient_morning),
                SkyboxState::Day => from_rgb(self.character_ambient_day),
                SkyboxState::Evening => from_rgb(self.character_ambient_evening),
                SkyboxState::Night => from_rgb(self.character_ambient_night),
            },
            character_diffuse_color: enum_map! {
                SkyboxState::Morning => from_rgb(self.character_diffuse_morning),
                SkyboxState::Day => from_rgb(self.character_diffuse_day),
                SkyboxState::Evening => from_rgb(self.character_diffuse_evening),
                SkyboxState::Night => from_rgb(self.character_diffuse_night),
            },
            sun_color: enum_map! {
                SkyboxState::Morning => from_rgb(self.sun_color_morning),
                SkyboxState::Day => from_rgb(self.sun_color_day),
                SkyboxState::Evening => from_rgb(self.sun_color_evening),
                SkyboxState::Night => from_rgb(self.sun_color_night),
            },
            sun_strength: enum_map! {
                SkyboxState::Morning => self.sun_strength_morning,
                SkyboxState::Day => self.sun_strength_day,
                SkyboxState::Evening => self.sun_strength_evening,
                SkyboxState::Night => self.sun_strength_night,
            },
            shadow_darkness: enum_map! {
                SkyboxState::Morning => self.shadow_darkness_morning,
                SkyboxState::Day => self.shadow_darkness_day,
                SkyboxState::Evening => self.shadow_darkness_evening,
                SkyboxState::Night => self.shadow_darkness_night,
            },
        }
    }
}

/// The mouse cursors, in the order the resource lists them: (name, the client's file in its
/// RES folder, where it clicks at normal size). The original cursors all click at the
/// top-left corner; ours click at the hands' fingertips and the speech bubble's tail.
pub const CURSORS: [(&str, &str, [i32; 2]); 9] = [
    ("default", "RES/DEFAULT.CUR", [0, 0]),
    ("attack", "RES/ATTACK.CUR", [0, 0]),
    ("talk", "RES/NPC.CUR", [3, 29]),
    ("player", "RES/USER.CUR", [1, 1]),
    ("pick_up", "RES/ITEM_PICK.CUR", [1, 12]),
    ("drag", "RES/ITEM_INVEN.CUR", [2, 4]),
    ("dragging", "RES/ITEM_INVEM.CUR", [3, 2]),
    ("repair", "RES/REPAIR.CUR", [0, 0]),
    ("camera", "RES/WHEEL.CUR", [16, 16]),
];

/// The client's loading picture (also the backdrop of character creation).
pub const LOADING_PICTURE: &str = "3DDATA/CONTROL/RES/LOADING.DDS";

/// The interface's pictures and mouse cursors. Each cursor is a picture (`cursor_<name>`) and
/// the pixel it clicks at, at normal size (`cursor_<name>_click`, [x, y] from the top left).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct InterfaceSettingsExport {
    /// The picture behind the loading screen.
    #[serde(skip_serializing_if = "is_default")]
    pub loading_picture: String,
    /// The picture behind character creation.
    #[serde(skip_serializing_if = "is_default")]
    pub character_create_picture: String,
    #[serde(skip_serializing_if = "is_default")]
    pub cursor_default: String,
    pub cursor_default_click: [i32; 2],
    #[serde(skip_serializing_if = "is_default")]
    pub cursor_attack: String,
    pub cursor_attack_click: [i32; 2],
    #[serde(skip_serializing_if = "is_default")]
    pub cursor_talk: String,
    pub cursor_talk_click: [i32; 2],
    #[serde(skip_serializing_if = "is_default")]
    pub cursor_player: String,
    pub cursor_player_click: [i32; 2],
    #[serde(skip_serializing_if = "is_default")]
    pub cursor_pick_up: String,
    pub cursor_pick_up_click: [i32; 2],
    #[serde(skip_serializing_if = "is_default")]
    pub cursor_drag: String,
    pub cursor_drag_click: [i32; 2],
    #[serde(skip_serializing_if = "is_default")]
    pub cursor_dragging: String,
    pub cursor_dragging_click: [i32; 2],
    #[serde(skip_serializing_if = "is_default")]
    pub cursor_repair: String,
    pub cursor_repair_click: [i32; 2],
    #[serde(skip_serializing_if = "is_default")]
    pub cursor_camera: String,
    pub cursor_camera_click: [i32; 2],
}

impl Default for InterfaceSettingsExport {
    /// The client's own: its loading picture and its cursor files.
    fn default() -> Self {
        let mut out = Self {
            loading_picture: LOADING_PICTURE.into(),
            character_create_picture: LOADING_PICTURE.into(),
            cursor_default: String::new(),
            cursor_default_click: [0, 0],
            cursor_attack: String::new(),
            cursor_attack_click: [0, 0],
            cursor_talk: String::new(),
            cursor_talk_click: [0, 0],
            cursor_player: String::new(),
            cursor_player_click: [0, 0],
            cursor_pick_up: String::new(),
            cursor_pick_up_click: [0, 0],
            cursor_drag: String::new(),
            cursor_drag_click: [0, 0],
            cursor_dragging: String::new(),
            cursor_dragging_click: [0, 0],
            cursor_repair: String::new(),
            cursor_repair_click: [0, 0],
            cursor_camera: String::new(),
            cursor_camera_click: [0, 0],
        };
        for (name, file, click) in CURSORS {
            if let Some((picture, at)) = out.cursor_mut(name) {
                *picture = file.into();
                *at = click;
            }
        }
        out
    }
}

impl InterfaceSettingsExport {
    /// A cursor by name (one of `CURSORS`): its picture and click point.
    pub fn cursor(&self, name: &str) -> Option<(&str, [i32; 2])> {
        Some(match name {
            "default" => (&self.cursor_default, self.cursor_default_click),
            "attack" => (&self.cursor_attack, self.cursor_attack_click),
            "talk" => (&self.cursor_talk, self.cursor_talk_click),
            "player" => (&self.cursor_player, self.cursor_player_click),
            "pick_up" => (&self.cursor_pick_up, self.cursor_pick_up_click),
            "drag" => (&self.cursor_drag, self.cursor_drag_click),
            "dragging" => (&self.cursor_dragging, self.cursor_dragging_click),
            "repair" => (&self.cursor_repair, self.cursor_repair_click),
            "camera" => (&self.cursor_camera, self.cursor_camera_click),
            _ => return None,
        })
        .map(|(picture, click)| (picture.as_str(), click))
    }

    pub fn cursor_mut(&mut self, name: &str) -> Option<(&mut String, &mut [i32; 2])> {
        Some(match name {
            "default" => (&mut self.cursor_default, &mut self.cursor_default_click),
            "attack" => (&mut self.cursor_attack, &mut self.cursor_attack_click),
            "talk" => (&mut self.cursor_talk, &mut self.cursor_talk_click),
            "player" => (&mut self.cursor_player, &mut self.cursor_player_click),
            "pick_up" => (&mut self.cursor_pick_up, &mut self.cursor_pick_up_click),
            "drag" => (&mut self.cursor_drag, &mut self.cursor_drag_click),
            "dragging" => (&mut self.cursor_dragging, &mut self.cursor_dragging_click),
            "repair" => (&mut self.cursor_repair, &mut self.cursor_repair_click),
            "camera" => (&mut self.cursor_camera, &mut self.cursor_camera_click),
            _ => return None,
        })
    }

    /// Every picture path with what it is for (`loading_picture`, `cursor_attack` ...).
    pub fn pictures(&self) -> Vec<(String, &str)> {
        let mut out = vec![
            ("loading_picture".to_string(), self.loading_picture.as_str()),
            ("character_create_picture".to_string(), self.character_create_picture.as_str()),
        ];
        for (name, _, _) in CURSORS {
            if let Some((picture, _)) = self.cursor(name) {
                out.push((format!("cursor_{name}"), picture));
            }
        }
        out
    }

    pub fn pictures_mut(&mut self) -> Vec<&mut String> {
        let mut out = vec![&mut self.loading_picture, &mut self.character_create_picture];
        out.extend([
            &mut self.cursor_default,
            &mut self.cursor_attack,
            &mut self.cursor_talk,
            &mut self.cursor_player,
            &mut self.cursor_pick_up,
            &mut self.cursor_drag,
            &mut self.cursor_dragging,
            &mut self.cursor_repair,
            &mut self.cursor_camera,
        ]);
        out
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SkyFile {
    #[serde(default)]
    sky: Vec<SkyExport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct InterfaceFile {
    #[serde(default)]
    interface: InterfaceSettingsExport,
}

/// Every sky and the interface settings, as exported.
#[derive(Debug, Default)]
pub struct InterfaceExport {
    pub skies: Vec<SkyExport>,
    pub interface: InterfaceSettingsExport,
}

impl InterfaceExport {
    /// The converter's starting point: every sky of the database (paths as it has them) and
    /// the client's own pictures and cursors.
    pub fn from_database(skies: &SkyboxDatabase) -> Self {
        Self { skies: skies.iter().map(SkyExport::from_data).collect(), interface: InterfaceSettingsExport::default() }
    }

    /// The sky database the game reads (by row, as LIST_SKY.STB lays them out).
    pub fn to_database(&self) -> Result<SkyboxDatabase, anyhow::Error> {
        self.check()?;
        let mut skies: Vec<Option<SkyboxData>> = Vec::new();
        for sky in &self.skies {
            let at = sky.id as usize;
            if skies.len() <= at {
                skies.resize_with(at + 1, || None);
            }
            skies[at] = Some(sky.to_data());
        }
        Ok(SkyboxDatabase::new(skies))
    }

    /// Ids used twice, and skies without a mesh.
    pub fn check(&self) -> Result<(), anyhow::Error> {
        let mut ids = BTreeSet::new();
        for sky in &self.skies {
            if !ids.insert(sky.id) {
                bail!("{SKIES_FILE}: sky {} is there twice", sky.id);
            }
        }
        Ok(())
    }

    pub fn has_sky(&self, id: i32) -> bool {
        id < 0 || self.skies.iter().any(|s| s.id as i32 == id)
    }

    /// The export's files: (file name without extension, contents).
    pub fn to_files_as(&self, format: Format) -> Result<Vec<(&'static str, String)>, anyhow::Error> {
        let mut sky = self.skies.clone();
        sky.sort_by_key(|s| s.id);
        Ok(vec![
            (SKIES_FILE, format.write(SKIES_FILE, &SkyFile { sky })?),
            (INTERFACE_FILE, format.write(INTERFACE_FILE, &InterfaceFile { interface: self.interface.clone() })?),
        ])
    }

    /// Reads the export's files through `read` (file name without extension -> its text, or
    /// None when it isn't there). None when there is none of them; one missing is an error.
    pub fn from_files_as(
        format: Format,
        mut read: impl FnMut(&str) -> Result<Option<String>, anyhow::Error>,
    ) -> Result<Option<Self>, anyhow::Error> {
        let skies = read(SKIES_FILE)?;
        let interface = read(INTERFACE_FILE)?;
        match (skies, interface) {
            (None, None) => Ok(None),
            (Some(skies), Some(interface)) => Ok(Some(Self {
                skies: format.parse::<SkyFile>(SKIES_FILE, &skies)?.sky,
                interface: format.parse::<InterfaceFile>(INTERFACE_FILE, &interface)?.interface,
            })),
            (None, _) => bail!("the interface export has no {SKIES_FILE}.{}", format.extension()),
            (_, None) => bail!("the interface export has no {INTERFACE_FILE}.{}", format.extension()),
        }
    }
}

/// Every difference between two sky databases as the game draws them: each sky's mesh and
/// textures (by their art stem, so a client file and its imported copy are the same) and its
/// colours (as the export rounds them).
pub fn compare_sky_databases(a: &SkyboxDatabase, b: &SkyboxDatabase) -> Vec<String> {
    let text = |db: &SkyboxDatabase| {
        db.iter()
            .map(|sky| {
                let mut sky = SkyExport::from_data(sky);
                for path in [&mut sky.mesh, &mut sky.texture_day, &mut sky.texture_night] {
                    *path = rose_data::art::art_stem(path);
                }
                (sky.id, format!("{sky:#?}"))
            })
            .collect()
    };
    crate::world::compare_text("sky", text(a), text(b))
}

/// Every difference between two interface settings: the pictures by their art stem, the
/// click points as they are.
pub fn compare_interface(a: &InterfaceSettingsExport, b: &InterfaceSettingsExport) -> Vec<String> {
    let stems = |s: &InterfaceSettingsExport| {
        let mut s = s.clone();
        for path in s.pictures_mut() {
            *path = rose_data::art::art_stem(path);
        }
        std::collections::BTreeMap::from([(0u16, format!("{s:#?}"))])
    };
    crate::world::compare_text("interface", stems(a), stems(b))
}
