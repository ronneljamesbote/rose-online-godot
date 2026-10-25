//! Zones as Godot scenes (step 5 of the Godot-native assets plan).
//!
//! A zone with a folder in `res://zones/` (`20_birth_island/`) loads from its scenes instead
//! of the ROSE map files. `zone.tscn` is the whole zone: its root is a `RoseZoneScene`, with
//! one `RoseBlock` node per 160 m block and the zone's event points (start and revive spots).
//! The ground is the zone's Terrain3D node (its regions in `terrain/`), drawn by
//! `shaders/rose_terrain3d.gdshader` so it looks as before, with the colour the ROSE
//! lightmaps gave it in Terrain3D's colour map; `RoseTerrainLook` beside it says which ROSE
//! texture each ground texture is. Everything is lit by the sun (`sun.rs`), with real-time
//! shadows. Zones converted before 2026-10-18 kept each block in its own `block_X_Y.tscn`;
//! `--merge-zone-blocks` moves them into `zone.tscn`, and scenes from before real-time
//! shadows still hold the old baked lightmaps, which `--strip-lightmaps` takes out. A block holds:
//! - its water planes (`RoseWater`);
//! - its buildings, decorations, event objects and warp gates, each an instance of a model
//!   scene in `assets/rose/` (`RoseZoneObject`);
//! - markers for monster spawns, NPCs, ambient sounds and effects, which show a label and
//!   their range (and the NPC or monster) only in the editor.
//!
//! `--import-zone=<id>` writes the scenes once from the ROSE files. After that the scenes
//! are the source and are committed; the model scenes and tile textures they use are made by
//! `--import-rose` like every other converted asset.

use std::collections::{BTreeMap, BTreeSet, HashSet};

use godot::{
    classes::{
        INode,
        base_material_3d::{Flags, ShadingMode, Transparency},
        file_access::ModeFlags,
        mesh::PrimitiveType,
        node::InternalMode,
        packed_scene::GenEditState,
        geometry_instance_3d::ShadowCastingSetting,
        AudioStream, DirAccess, Engine, FileAccess, GeometryInstance3D, INode3D, ImmediateMesh, Label3D, Mesh, MeshInstance3D, Node, Node3D,
        PackedScene, ResourceLoader, ResourceSaver, ShaderMaterial, StandardMaterial3D, Texture2D, Texture2DArray,
    },
    global::Error,
    prelude::*,
};
use rose_file_readers::{HimFile, IfoFile, IfoMonsterSpawn, TilFile, ZonFile, ZscEffectType, ZscFile};

use crate::{
    assets, data, data_resources::{self, RoseMonster, RoseNpc, RoseWarpGate, RoseZoneSettings}, material, sun::{self, RoseSun}, texture,
    dialog_blocks::RoseDialog,
    zone::{self, Ground, ObjectSpawner},
};

pub const ZONES: &str = "res://zones/";
/// Bumped when the model scenes or texture arrays made for zone scenes change.
pub const ZONE_IMPORT_VERSION: i64 = 2;

/// The model lists a zone's objects come from, with the zsc index `zone.rs` uses for them.
const LISTS: [(&str, usize); 4] = [("cnst", 0), ("deco", 1), ("event", 2), ("warp", 3)];

// ---------------------------------------------------------------------------------------
// Finding and loading a zone's scenes

/// The folder of a zone's scenes (`res://zones/20_birth_island`), if it has one.
pub fn zone_folder(zone_id: u16) -> Option<String> {
    zone_folders().into_iter().find(|(id, _)| *id == zone_id).map(|(_, folder)| folder)
}

/// Every zone with a folder in `res://zones/`: its id (the number the folder name starts with) and folder.
pub fn zone_folders() -> Vec<(u16, String)> {
    let Some(dir) = DirAccess::open(ZONES) else { return Vec::new() };
    dir.get_directories()
        .as_slice()
        .iter()
        .filter_map(|name| {
            let name = name.to_string();
            let id = name.split('_').next()?.parse::<u16>().ok()?;
            Some((id, format!("{ZONES}{name}")))
        })
        .collect()
}


/// The zone's scene, when it has one and the converted assets it uses are there (they are
/// not committed; `--import-rose` makes them). Without them the zone loads from the ROSE
/// files: the error is `None` for a zone with no scene, or why its scene was not used.
pub fn zone_scene(zone_id: u16, zone_dir: &str) -> Result<Gd<PackedScene>, Option<&'static str>> {
    let path = format!("{}/zone.tscn", zone_folder(zone_id).ok_or(None)?);
    if !ResourceLoader::singleton().exists(&path) {
        return Err(None);
    }
    let first_tile = data::read_file::<ZonFile>(&format!("{zone_dir}/{}.ZON", zone_dir.rsplit('/').next().unwrap_or("")))
        .and_then(|zon| zone::tile_texture_paths(&zon).into_iter().next());
    if first_tile.is_some_and(|path| assets::load_texture(&path).is_none()) {
        godot_warn!("rose: {path} needs the converted assets; run --import-rose. Loading the zone from the client data.");
        return Err(Some("assets missing"));
    }
    godot::tools::try_load::<PackedScene>(&path).map_err(|_| Some("scene did not load"))
}

fn walk(node: &Gd<Node>, visit: &mut impl FnMut(&Gd<Node>)) {
    visit(node);
    for child in node.get_children().iter_shared() {
        walk(&child, visit);
    }
}

/// Reads what the game needs from a loaded zone scene (in the tree): ground heights and
/// footstep tiles, collision triangles, warp gates, effects and ambient sounds. Returns
/// the number of materials drawn.
pub fn gather(map: &Gd<Node>, to_zone: Transform3D, spawner: &mut ObjectSpawner, grounds: &mut [Option<Ground>]) -> usize {
    let mut materials: HashSet<InstanceId> = HashSet::new();
    walk(map, &mut |node| {
        if let Ok(look) = node.clone().try_cast::<RoseTerrainLook>() {
            if let Some(terrain) = look.get_parent() {
                spawner.blocks += terrain_grounds(&terrain, &look, grounds);
            }
        } else if let Ok(object) = node.clone().try_cast::<RoseZoneObject>() {
            spawner.objects += 1;
            let object_transform = to_zone * object.get_global_transform();
            let is_warp = object.bind().list == "warp";
            let warp_id = object.bind().warp_id;
            let mut warp_box: Option<Aabb> = None;
            let mut casters = Vec::new();
            let mut size: Option<Aabb> = None;
            for child in object.get_children().iter_shared() {
                if let Ok(part) = child.clone().try_cast::<MeshInstance3D>() {
                    let Some(mesh) = part.get_mesh() else { continue };
                    spawner.parts += 1;
                    if let Some(material) = part.get_material_override() {
                        materials.insert(material.instance_id());
                    }
                    let part_box = part.get_transform() * mesh.get_aabb();
                    size = Some(size.map_or(part_box, |s| s.merge(part_box)));
                    if part.get_cast_shadows_setting() != ShadowCastingSetting::OFF {
                        casters.push(part.clone().upcast::<GeometryInstance3D>());
                    }
                    let to_zone_part = object_transform * part.get_transform();
                    let walls = part.get_meta_ex("walls").default(&false.to_variant()).done().try_to::<bool>().unwrap_or(false);
                    let floors = part.get_meta_ex("floors").default(&false.to_variant()).done().try_to::<bool>().unwrap_or(false);
                    if walls || floors {
                        let faces: Vec<Vector3> = mesh.get_faces().as_slice().iter().map(|v| to_zone_part * *v).collect();
                        if walls {
                            spawner.walls.extend_from_slice(&faces);
                        }
                        if floors {
                            spawner.floors.extend(faces);
                        }
                    }
                    if is_warp {
                        let aabb = to_zone_part * mesh.get_aabb();
                        warp_box = Some(warp_box.map_or(aabb, |b| b.merge(aabb)));
                    }
                } else if let Ok(effect) = child.clone().try_cast::<Node3D>() {
                    if let Ok(path) = effect.get_meta_ex("effect").default(&Variant::nil()).done().try_to::<GString>() {
                        let night = effect.get_meta_ex("night_only").default(&false.to_variant()).done().try_to::<bool>().unwrap_or(false);
                        spawner.effect_spots.push((object_transform * effect.get_transform(), path.to_string(), night));
                    }
                }
            }
            if let Some(aabb) = warp_box {
                spawner.warps.push((warp_id.clamp(0, u16::MAX as i32) as u16, aabb));
            }
            if let Some(size) = size {
                spawner.add_casters(casters, object_transform, size);
            }
        } else if let Ok(effect) = node.clone().try_cast::<RoseEffectSpot>() {
            let transform = to_zone * effect.get_global_transform();
            let path = effect.bind().scene.as_ref().map(|s| s.get_path().to_string()).unwrap_or_default();
            if !path.is_empty() {
                spawner.effect_spots.push((transform, path, false));
            }
        } else if let Ok(sound) = node.clone().try_cast::<RoseSoundSpot>() {
            let position = (to_zone * sound.get_global_transform()).origin;
            let sound = sound.bind();
            if let Some(stream) = sound.stream.as_ref() {
                spawner.ambient_sounds.push((position, stream.get_path().to_string(), sound.range));
            }
        }
    });
    materials.len()
}

// ---------------------------------------------------------------------------------------
// Zone and block nodes

/// Metres per side of a ROSE block.
pub const BLOCK_SIZE: f32 = 160.0;

/// The north-west corner of block (x, y) on Godot's x and z axes.
fn block_origin(block_x: i32, block_y: i32) -> (f32, f32) {
    (block_x as f32 * BLOCK_SIZE, (block_y - 65) as f32 * BLOCK_SIZE)
}

/// The root of a ROSE zone scene (`zone.tscn`): one whole zone.
///
/// Its children are one `RoseBlock` per 160 m block, the `Terrain` node (the ground, with
/// `RoseTerrainLook` under it) and `Events` (the zone's named spots, `RoseZoneEvent`). The
/// game loads this scene instead of the ROSE map files. Spawns, NPCs and named spots go to
/// the server through `--export-data`, which writes `export/zones/<id>-<name>.toml`. The
/// zone's name, music, sky, time of day and rules are its `settings`.
#[derive(GodotClass)]
#[class(tool, base=Node3D, init)]
pub struct RoseZoneScene {
    base: Base<Node3D>,
    /// The zone's number: the id of its `RoseZoneSettings`, the same number the folder name
    /// starts with (`20` for `20_birth_island`). The ROSE Zone Tools plugin uses it for the
    /// `/tp` command it shows under the mouse. Don't change it.
    #[export]
    pub zone_id: i32,
    /// The zone's settings (name, music, sky, time of day, PvP ...): the `RoseZoneSettings`
    /// in `res://data/zones/` with this zone's number. Click it to edit them; they are saved
    /// in that resource, not in the scene.
    #[var(get = get_settings, no_set, usage_flags = [EDITOR], hint = RESOURCE_TYPE, hint_string = "RoseZoneSettings")]
    settings: PhantomVar<Option<Gd<RoseZoneSettings>>>,
    /// Editor only: the time of day the zone is lit for while you edit it, with the sun and
    /// shadows of the zone's sky at that time, as in the game. Not saved in the scene.
    #[var(get = get_preview_time, set = set_preview_time, usage_flags = [EDITOR], hint = ENUM, hint_string = "Morning,Day,Evening,Night")]
    preview_time: PhantomVar<i32>,
    time: i32,
    sun: Option<Gd<RoseSun>>,
}

#[godot_api]
impl INode3D for RoseZoneScene {
    fn enter_tree(&mut self) {
        // The editor takes inactive scene tabs out of the tree: light the zone again on return.
        if is_editor() {
            self.light_preview();
        }
    }
}

#[godot_api]
impl RoseZoneScene {
    #[func]
    fn get_settings(&self) -> Option<Gd<RoseZoneSettings>> {
        data_resources::find(data_resources::ZONES_DIR, self.zone_id as i64).and_then(|(r, _)| r.try_cast::<RoseZoneSettings>().ok())
    }

    #[func]
    fn get_preview_time(&self) -> i32 {
        self.time
    }

    #[func]
    fn set_preview_time(&mut self, time: i32) {
        self.time = time.clamp(0, 3);
        if is_editor() && self.base().is_inside_tree() {
            self.light_preview();
        }
    }
}

impl RoseZoneScene {
    /// The editor's preview sun (not saved: an internal child), lit for `time`.
    fn light_preview(&mut self) {
        let mut sun = match self.sun.as_ref().filter(|s| s.is_instance_valid()) {
            Some(sun) => sun.clone(),
            None => {
                let mut sun = RoseSun::new_alloc();
                sun.set_name("PreviewSun");
                self.base_mut().add_child_ex(&sun).internal(InternalMode::BACK).done();
                sun.bind_mut().set_quality("medium".into());
                self.sun = Some(sun.clone());
                sun
            }
        };
        let settings = self.get_settings().map(|s| s.upcast::<godot::classes::Resource>());
        let number = |key: &str| settings.as_ref().and_then(|s| s.get(key).try_to::<i64>().ok()).unwrap_or(0).max(0) as u32;
        let times = sun::DayTimes { day_cycle: number("day_cycle"), morning: number("morning_time"), day: number("day_time"), evening: number("evening_time"), night: number("night_time") };
        let sky = settings
            .as_ref()
            .and_then(|s| s.get("sky").try_to::<Gd<godot::classes::Resource>>().ok())
            .and_then(|sky| data_resources::sky_data(&sky));
        // A moment well inside each state, so its own colours show.
        let (start, end) = match self.time {
            0 => (times.morning, times.day),
            1 => (times.day, times.evening),
            2 => (times.evening, times.night),
            _ => (times.night, times.morning + times.day_cycle),
        };
        let at = start as f32 + (end.saturating_sub(start)) as f32 * if self.time == 0 { 0.7 } else if self.time == 2 { 0.3 } else { 0.5 };
        let lighting = sun::lighting(sky.as_ref(), &times, at.floor() as i64, at.fract());
        sun.bind_mut().apply(lighting);
    }
}

/// One 160 m x 160 m block of a zone (one IFO/HIM file pair in iROSE).
///
/// Holds the block's water (`Water`), placed models (`Buildings`, `Decorations`, `Events`,
/// `Warps`) and markers (`Spawns`, `Npcs`, `Sounds`, `Effects`). They sit at their world
/// positions; the block node itself stays at the origin. In the editor it draws its outline
/// and name on the ground, and the 10 m ground tiles when the ROSE Zone Tools toggle is on.
/// A warning sign in the Scene panel means something stands outside its square, or its name
/// and numbers don't match.
#[derive(GodotClass)]
#[class(tool, base=Node3D, init)]
pub struct RoseBlock {
    base: Base<Node3D>,
    /// Which block this is, counting east (the first number of the ROSE map file names,
    /// `33` in `33_31.IFO`). Block x covers Godot x from `160 * block_x` to 160 m further.
    /// The game and the server export go by this number, not by the node name.
    #[export]
    pub block_x: i32,
    /// Which block this is, counting south (the second number of the ROSE map file names,
    /// `31` in `33_31.IFO`). Block 65 is at Godot z 0; each block lower is 160 m further
    /// north (negative z).
    #[export]
    pub block_y: i32,
    #[init(val = true)]
    show_outline: bool,
    show_tiles: bool,
}

#[godot_api]
impl INode3D for RoseBlock {
    fn ready(&mut self) {
        if is_editor() {
            // After the Terrain node (which comes after the blocks) has loaded its heights.
            self.base_mut().call_deferred("redraw_overlay", &[]);
        }
    }

    fn get_configuration_warnings(&self) -> PackedStringArray {
        self.warnings()
    }
}

#[godot_api]
impl RoseBlock {
    /// What the editor shows as this block's warnings (a ⚠ in the Scene panel).
    #[func]
    pub fn warnings(&self) -> PackedStringArray {
        let mut warnings = PackedStringArray::new();
        let name = self.base().get_name().to_string();
        if name != format!("Block_{}_{}", self.block_x, self.block_y) {
            warnings.push(&format!("Named {name} but set to block {}_{}. The game and the server export go by block_x and block_y.", self.block_x, self.block_y));
        }
        let outside = self.things_outside();
        if !outside.is_empty() {
            let shown = outside.iter().take(5).cloned().collect::<Vec<_>>().join(", ");
            let more = if outside.len() > 5 { format!(" and {} more", outside.len() - 5) } else { String::new() };
            warnings.push(&format!(
                "{} {} outside this block's 160 m square: {shown}{more}. They work in game, but belong in the block they stand in (drag them there in the Scene panel).",
                outside.len(),
                if outside.len() == 1 { "model or marker stands" } else { "models or markers stand" }
            ));
        }
        if let Some(look) = self.terrain_look() {
            let listed = look.bind().blocks.as_slice().chunks(2).any(|pair| pair == [self.block_x, self.block_y]);
            if !listed {
                warnings.push("This block has no ground: it is not in Terrain/RoseTerrainLook's blocks, so the game reads no heights here.");
            }
        }
        warnings
    }

    /// The block's centre on the ground, in the zone scene's space.
    #[func]
    pub fn center(&self) -> Vector3 {
        let (x0, z0) = block_origin(self.block_x, self.block_y);
        let (x, z) = (x0 + BLOCK_SIZE / 2.0, z0 + BLOCK_SIZE / 2.0);
        let y = self.terrain_data().and_then(|data| ground_height(&data, x, z)).unwrap_or(0.0);
        Vector3::new(x, y, z)
    }

    /// What the editor draws on the ground: the outline and name, the 10 m tiles.
    #[func]
    fn set_overlay(&mut self, outline: bool, tiles: bool) {
        if (outline, tiles) != (self.show_outline, self.show_tiles) {
            self.show_outline = outline;
            self.show_tiles = tiles;
            self.redraw_overlay();
        }
    }

    /// Draws the outline again, following the ground as it is now.
    #[func]
    fn redraw_overlay(&mut self) {
        if let Some(old) = self.base().get_node_or_null("EditorOverlay") {
            old.free();
        }
        if !is_editor() || !(self.show_outline || self.show_tiles) {
            return;
        }
        let data = self.terrain_data();
        let height = |x: f32, z: f32| data.as_ref().and_then(|d| ground_height(d, x, z)).unwrap_or(0.0) + 0.3;
        let (x0, z0) = block_origin(self.block_x, self.block_y);
        let step = VERTEX_SPACING;
        let samples = (BLOCK_SIZE / step) as i32;
        // A line from one edge of the block to the other, along x (or z), at `at` metres in.
        let line = |lines: &mut ImmediateMesh, along_x: bool, at: f32| {
            for i in 0..samples {
                for j in [i, i + 1] {
                    let (x, z) = if along_x { (x0 + j as f32 * step, z0 + at) } else { (x0 + at, z0 + j as f32 * step) };
                    lines.surface_add_vertex(Vector3::new(x, height(x, z), z));
                }
            }
        };

        let mut holder = Node3D::new_alloc();
        holder.set_name("EditorOverlay");
        // The lines are in zone space; undo this node's own placement.
        holder.set_transform(self.base().get_transform().affine_inverse());
        let mut lines = ImmediateMesh::new_gd();
        if self.show_tiles {
            lines.surface_begin_ex(PrimitiveType::LINES).material(&overlay_material(Color::from_rgba(1.0, 1.0, 1.0, 0.35))).done();
            for tile in 1..16 {
                let at = tile as f32 * 10.0;
                line(&mut lines, true, at);
                line(&mut lines, false, at);
            }
            lines.surface_end();
        }
        if self.show_outline {
            lines.surface_begin_ex(PrimitiveType::LINES).material(&overlay_material(Color::from_rgb(1.0, 0.8, 0.2))).done();
            for at in [0.0, BLOCK_SIZE] {
                line(&mut lines, true, at);
                line(&mut lines, false, at);
            }
            lines.surface_end();
        }
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_mesh(&lines.upcast::<Mesh>());
        mesh.set_cast_shadows_setting(godot::classes::geometry_instance_3d::ShadowCastingSetting::OFF);
        holder.add_child(&mesh);

        if self.show_outline {
            let mut text = Label3D::new_alloc();
            text.set_text(&format!("Block {}_{}", self.block_x, self.block_y));
            text.set_billboard_mode(godot::classes::base_material_3d::BillboardMode::ENABLED);
            text.set_draw_flag(godot::classes::label_3d::DrawFlags::FIXED_SIZE, true);
            text.set_pixel_size(0.0015);
            text.set_font_size(28);
            text.set_outline_size(8);
            text.set_modulate(Color::from_rgb(1.0, 0.85, 0.3));
            let (x, z) = (x0 + BLOCK_SIZE / 2.0, z0 + BLOCK_SIZE / 2.0);
            text.set_position(Vector3::new(x, height(x, z) + 8.0, z));
            holder.add_child(&text);
        }
        self.base_mut().add_child_ex(&holder).internal(InternalMode::BACK).done();
    }
}

impl RoseBlock {
    /// The zone's Terrain3D node, a sibling of the blocks.
    fn terrain(&self) -> Option<Gd<Node>> {
        let parent = self.base().get_parent()?;
        parent.get_node_or_null("Terrain").filter(|t| t.is_class("Terrain3D"))
    }

    fn terrain_data(&self) -> Option<Gd<godot::classes::Object>> {
        self.terrain()?.get("data").try_to::<Gd<godot::classes::Object>>().ok()
    }

    fn terrain_look(&self) -> Option<Gd<RoseTerrainLook>> {
        let terrain = self.terrain()?;
        terrain.get_children().iter_shared().find_map(|c| c.try_cast::<RoseTerrainLook>().ok())
    }

    /// The names of the models and markers that stand outside the block (half a metre of slack).
    fn things_outside(&self) -> Vec<String> {
        let (x0, z0) = block_origin(self.block_x, self.block_y);
        let inside = |p: Vector3| (x0 - 0.5..=x0 + BLOCK_SIZE + 0.5).contains(&p.x) && (z0 - 0.5..=z0 + BLOCK_SIZE + 0.5).contains(&p.z);
        let block_transform = self.base().get_transform();
        let mut outside = Vec::new();
        let mut visit = |node: &Gd<Node>, transform: Transform3D| {
            let thing = node.clone().try_cast::<RoseZoneObject>().is_ok()
                || node.clone().try_cast::<RoseSpawnSpot>().is_ok()
                || node.clone().try_cast::<RoseNpcSpot>().is_ok()
                || node.clone().try_cast::<RoseSoundSpot>().is_ok()
                || node.clone().try_cast::<RoseEffectSpot>().is_ok();
            if thing && !inside(transform.origin) {
                outside.push(node.get_name().to_string());
            }
        };
        // The groups (Buildings, Npcs, ...) and what is in them.
        for group in self.base().get_children().iter_shared() {
            let group_transform = block_transform * transform_of(&group);
            visit(&group, group_transform);
            for child in group.get_children().iter_shared() {
                visit(&child, group_transform * transform_of(&child));
            }
        }
        outside
    }
}

/// The ground height at (x, z) from Terrain3D's data, if there is ground there.
fn ground_height(data: &Gd<godot::classes::Object>, x: f32, z: f32) -> Option<f32> {
    let height = data.clone().call("get_height", &[Vector3::new(x, 0.0, z).to_variant()]).try_to::<f32>().ok()?;
    height.is_finite().then_some(height)
}

fn overlay_material(color: Color) -> Gd<StandardMaterial3D> {
    let mut material = StandardMaterial3D::new_gd();
    material.set_shading_mode(ShadingMode::UNSHADED);
    material.set_albedo(color);
    material.set_transparency(Transparency::ALPHA);
    material
}

/// The block (x, y) a zone-scene child is, from its `RoseBlock` properties, or for scenes
/// from before `RoseBlock`, its name (`Block_31_32`).
fn block_of(node: &Gd<Node>) -> Option<(i32, i32)> {
    if let Ok(block) = node.clone().try_cast::<RoseBlock>() {
        let b = block.bind();
        return Some((b.block_x, b.block_y));
    }
    let name = node.get_name().to_string();
    let (x, y) = name.strip_prefix("Block_")?.split_once('_')?;
    Some((x.parse().ok()?, y.parse().ok()?))
}

// ---------------------------------------------------------------------------------------
// Ground and water

/// Terrain3D settings for ROSE zones: 1 region = 512 x 512 vertices 2.5 m apart (8 x 8 ROSE
/// blocks), which keeps every ROSE zone inside Terrain3D's 32 x 32 regions around the origin
/// without moving the world.
pub const REGION_SIZE: i64 = 512;
/// Terrain3D's ground mesh rings: 5 reach about 1.9 km around the camera, past its 1 km
/// far plane. The default 7 (7.7 km) draws triangles hundreds of metres wide that software
/// Vulkan (lavapipe) crashes on.
const MESH_LODS: i64 = 5;
pub const VERTEX_SPACING: f32 = 2.5;
/// Vertices per ROSE block side (160 m / 2.5 m); block (x, y) starts at vertex (64 x, 64 (y - 65)).
const BLOCK_VERTICES: i64 = 64;
const TERRAIN_SHADER3D: &str = "res://shaders/rose_terrain3d.gdshader";

/// A Terrain3D control map value: texture layer 1 as the base, layer 2 as the overlay drawn
/// by its alpha times `blend`, and the ROSE rotation code of layer 2 in the angle bits.
/// Terrain3D holds 32 textures; a zone with more keeps the rest in `RoseTerrainLook`'s
/// `extra_textures`, and texture 32 + n is n with the base or overlay extra bit set (two of
/// the control bits Terrain3D leaves unused).
fn control_bits(base: u32, overlay: u32, rotation: u32, blend: u32) -> u32 {
    let extra = |id: u32, bit: u32| if id >= TERRAIN3D_TEXTURES as u32 { bit } else { 0 };
    (base & 0x1F) << 27 | (overlay & 0x1F) << 22 | (blend & 0xFF) << 14 | (rotation & 0xF) << 10 | extra(base, BASE_EXTRA_BIT) | extra(overlay, OVERLAY_EXTRA_BIT)
}
const HOLE_BIT: u32 = 1 << 2;
const BASE_EXTRA_BIT: u32 = 1 << 3;
const OVERLAY_EXTRA_BIT: u32 = 1 << 4;
/// How many ground textures Terrain3D takes; `MAX_GROUND_TEXTURES` with the extra bit.
const TERRAIN3D_TEXTURES: usize = 32;
const MAX_GROUND_TEXTURES: usize = 64;

/// The texture number (0-63) of a control value's overlay.
fn overlay_id(control: u32) -> usize {
    (control >> 22 & 0x1F) as usize + if control & OVERLAY_EXTRA_BIT != 0 { TERRAIN3D_TEXTURES } else { 0 }
}

/// The ROSE look of the zone's ground. Sits under the zone's Terrain3D node.
///
/// Points Terrain3D at the game camera, and says which ROSE texture each Terrain3D texture
/// is, so the game knows the footstep sound of each tile. The game also reads the ground
/// heights of the blocks listed here. Made by `--import-zone`; normally left alone. The
/// ground's colour (blue sea floors, tinted areas) is Terrain3D's colour map: paint it with
/// Terrain3D's colour brush.
#[derive(GodotClass)]
#[class(tool, base=Node, init)]
pub struct RoseTerrainLook {
    base: Base<Node>,
    /// The blocks that have ground, as pairs of numbers: block x, block y, block x, block y,
    /// ... (so `33, 31, 34, 31` is blocks 33_31 and 34_31). The game reads ground heights and
    /// footstep tiles only for these blocks. A new block needs its pair added here.
    #[export]
    pub blocks: PackedInt32Array,
    /// For each Terrain3D ground texture (0, 1, 2, ...), then each texture in
    /// `extra_textures`: its texture number in the zone's ROSE ZON file. Picks the footstep
    /// sound of each tile.
    #[export]
    pub texture_numbers: PackedInt32Array,
    /// Ground textures past Terrain3D's 32, for zones that use more (up to 64). The first
    /// one here is ground texture 32.
    #[export]
    pub extra_textures: VarArray,
    extra_array: Option<Gd<Texture2DArray>>,
    applied_to: Option<Rid>,
}

#[godot_api]
impl INode for RoseTerrainLook {
    fn process(&mut self, _delta: f64) {
        let Some(mut terrain) = self.base().get_parent().and_then(|p| p.try_cast::<Node3D>().ok()) else { return };
        if !terrain.is_class("Terrain3D") {
            return;
        }
        // Terrain3D follows one camera; ours is made after the zone and can change.
        if !is_editor() {
            let camera = self.base().get_viewport().and_then(|v| v.get_camera_3d());
            if let Some(camera) = camera {
                let current = terrain.call("get_camera", &[]).try_to::<Gd<godot::classes::Camera3D>>().ok();
                if current.as_ref() != Some(&camera) {
                    terrain.call("set_camera", &[camera.to_variant()]);
                }
            }
        }
        // The material can be remade (the editor does on shader changes): give the new one the extra textures.
        if self.extra_textures.is_empty() {
            return;
        }
        let Ok(material) = terrain.get("material").try_to::<Gd<godot::classes::Object>>() else { return };
        let Ok(rid) = material.clone().call("get_material_rid", &[]).try_to::<Rid>() else { return };
        if !rid.is_valid() || self.applied_to == Some(rid) {
            return;
        }
        let mut server = godot::classes::RenderingServer::singleton();
        {
            let extra = self.extra_array.get_or_insert_with(|| {
                texture::texture_array(self.extra_textures.iter_shared().map(|t| t.try_to::<Gd<Texture2D>>().ok().and_then(|t| t.get_image())).collect())
            });
            server.material_set_param(rid, "rose_extra_tiles", &extra.get_rid().to_variant());
        }
        self.applied_to = Some(rid);
    }
}

/// Reads one map of a Terrain3D region as 32-bit values (heights as floats, control maps as bits).
fn region_map(data: &Gd<godot::classes::Object>, location: Vector2i, map: &str) -> Option<Vec<u32>> {
    let region = data.clone().call("get_region", &[location.to_variant()]).try_to::<Gd<godot::classes::Object>>().ok()?;
    let image = region.get(map).try_to::<Gd<godot::classes::Image>>().ok()?;
    let bytes = image.get_data();
    Some(bytes.as_slice().chunks_exact(4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect())
}

/// Heights and footstep tiles of every block from the zone's Terrain3D node.
fn terrain_grounds(terrain: &Gd<Node>, look: &Gd<RoseTerrainLook>, grounds: &mut [Option<Ground>]) -> usize {
    let Ok(data) = terrain.get("data").try_to::<Gd<godot::classes::Object>>() else { return 0 };
    let region_size = terrain.get("region_size").try_to::<i64>().unwrap_or(REGION_SIZE);
    /// A region's height and control map pixels, if it has a region.
    type RegionMaps = Option<(Vec<u32>, Vec<u32>)>;
    let mut maps: BTreeMap<(i64, i64), RegionMaps> = BTreeMap::new();
    let look = look.bind();
    let numbers = look.texture_numbers.as_slice();
    let mut sample = |gx: i64, gz: i64| -> Option<(f32, u32)> {
        let (rx, rz) = (gx.div_euclid(region_size), gz.div_euclid(region_size));
        let entry = maps.entry((rx, rz)).or_insert_with(|| {
            let location = Vector2i::new(rx as i32, rz as i32);
            Some((region_map(&data, location, "height_map")?, region_map(&data, location, "control_map")?))
        });
        let (heights, controls) = entry.as_ref()?;
        let i = (gz.rem_euclid(region_size) * region_size + gx.rem_euclid(region_size)) as usize;
        Some((f32::from_bits(*heights.get(i)?), *controls.get(i)?))
    };
    let mut blocks = 0;
    for pair in look.blocks.as_slice().chunks(2) {
        let [block_x, block_y] = pair else { continue };
        let (gx0, gz0) = (BLOCK_VERTICES * *block_x as i64, BLOCK_VERTICES * (*block_y as i64 - 65));
        let size = BLOCK_VERTICES as i32 + 1;
        let heights: Vec<f32> = (0..size as i64)
            .flat_map(|j| (0..size as i64).map(move |i| (i, j)))
            .map(|(i, j)| sample(gx0 + i, gz0 + j).map_or(0.0, |(h, _)| h))
            .collect();
        let tile_textures = (0..16i64)
            .flat_map(|ty| (0..16i64).map(move |tx| (tx, ty)))
            .map(|(tx, ty)| {
                let control = sample(gx0 + tx * 4 + 1, gz0 + ty * 4 + 1).map_or(0, |(_, c)| c);
                numbers.get(overlay_id(control)).copied().unwrap_or(0)
            })
            .collect();
        if (0..64).contains(block_x) && (0..64).contains(block_y) {
            grounds[(*block_x + *block_y * 64) as usize] = Some(Ground { size, heights, tile_textures });
            blocks += 1;
        }
    }
    blocks
}

thread_local! {
    static WATER_TEXTURES: std::cell::RefCell<Option<Gd<Texture2DArray>>> = const { std::cell::RefCell::new(None) };
}

/// The 25 water frames as one array (made once).
fn water_textures() -> Gd<Texture2DArray> {
    WATER_TEXTURES.with_borrow_mut(|cached| cached.get_or_insert_with(|| texture::load_texture_array(&zone::water_texture_paths())).clone())
}

/// A flat, animated water surface (an IFO water plane). Drawn by the game only; the server
/// doesn't know about water.
///
/// The rectangle runs from `start` to `end`, at the height of `start`. Moving or rotating
/// the node doesn't move the water: change the corners.
#[derive(GodotClass)]
#[class(tool, base=Node3D, init)]
pub struct RoseWater {
    base: Base<Node3D>,
    /// One corner of the water, in the zone scene's space (Godot x, height, z, in metres).
    /// The whole surface lies at this corner's height.
    #[export]
    pub start: Vector3,
    /// The opposite corner, in the zone scene's space. Its height is not used.
    #[export]
    pub end: Vector3,
    /// Metres of water per repeat of the animated water texture (20 in every zone converted so
    /// far). Smaller makes the ripples smaller.
    #[export]
    #[init(val = 10.0)]
    pub tile_size: f32,
    mesh: Option<Gd<MeshInstance3D>>,
}

thread_local! {
    static WATER_MATERIAL: std::cell::RefCell<Option<Gd<ShaderMaterial>>> = const { std::cell::RefCell::new(None) };
}

#[godot_api]
impl INode3D for RoseWater {
    fn ready(&mut self) {
        if let Some(old) = self.mesh.take() {
            if old.is_instance_valid() {
                old.free();
            }
        }
        let material = WATER_MATERIAL.with_borrow_mut(|cached| {
            cached
                .get_or_insert_with(|| {
                    let mut material = ShaderMaterial::new_gd();
                    material.set_shader(&zone::load_shader(zone::WATER_SHADER));
                    material.set_shader_parameter("water_textures", &water_textures().to_variant());
                    material
                })
                .clone()
        });
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name("Mesh");
        mesh.set_mesh(&zone::build_water_mesh(self.start, self.end, self.tile_size.max(0.01)));
        mesh.set_material_override(&material);
        mesh.set_cast_shadows_setting(ShadowCastingSetting::OFF);
        // The corners are in zone space; undo this node's own placement.
        let inverse = self.base().get_transform().affine_inverse();
        mesh.set_transform(inverse);
        self.base_mut().add_child_ex(&mesh).internal(InternalMode::FRONT).done();
        self.mesh = Some(mesh);
    }
}

pub fn clear_cache() {
    WATER_MATERIAL.with_borrow_mut(|cached| *cached = None);
    WATER_TEXTURES.with_borrow_mut(|cached| *cached = None);
}

// ---------------------------------------------------------------------------------------
// Placed models

/// A placed model: a building, decoration, event object or warp gate.
///
/// Each one is an instance of a model scene in `assets/rose/` (one per model of a ROSE ZSC
/// model list), so move, turn, scale, copy or delete it like any scene. Its parts are the
/// `PartN` mesh children; `EffectN` children are the model's own effects (smoke, fire). Parts
/// marked in the model block the player (walls) or can be stood on (floors). Solid parts cast
/// real-time shadows wherever the model stands (see-through ones don't). Drawn and collided
/// by the game only; the server doesn't read models.
#[derive(GodotClass)]
#[class(tool, base=Node3D, init)]
pub struct RoseZoneObject {
    base: Base<Node3D>,
    /// Which ROSE model list the model comes from: `cnst` (buildings), `deco` (decorations),
    /// `event` (event objects) or `warp` (warp gates). Set by the model scene; don't change it.
    #[export]
    pub list: GString,
    /// Warp gates only: where the gate takes you. Drag a `RoseWarpGate` in from
    /// `res://data/warp_gates/` (it names the zone and named spot you land on). Walking into
    /// the gate asks the server to warp you through it. Unused for other models.
    #[var(get = get_warp_gate, set = set_warp_gate, usage_flags = [EDITOR], hint = RESOURCE_TYPE, hint_string = "RoseWarpGate")]
    warp_gate: PhantomVar<Option<Gd<RoseWarpGate>>>,
    /// Warp gates only: the number of the gate's `RoseWarpGate`, as the scene stores it
    /// (`warp_gate` shows it as the resource).
    #[export]
    pub warp_id: i32,
    /// Event objects only: the event number from the IFO file. Kept from iROSE; the game
    /// doesn't use it yet.
    #[export]
    pub event_id: i32,
    /// Event objects only: the quest trigger name from the IFO file. Kept from iROSE; the
    /// game doesn't use it yet.
    #[export]
    pub quest_trigger: GString,
    /// Event objects only: the script function name from the IFO file. Kept from iROSE; the
    /// game doesn't use it yet.
    #[export]
    pub script_function: GString,
}

#[godot_api]
impl RoseZoneObject {
    #[func]
    fn get_warp_gate(&self) -> Option<Gd<RoseWarpGate>> {
        data_resources::find(data_resources::WARP_GATES_DIR, self.warp_id as i64).and_then(|(r, _)| r.try_cast::<RoseWarpGate>().ok())
    }

    #[func]
    fn set_warp_gate(&mut self, gate: Option<Gd<RoseWarpGate>>) {
        match gate {
            Some(gate) => self.warp_id = gate.upcast::<godot::classes::Resource>().get("id").try_to::<i64>().unwrap_or(0) as i32,
            // Cleared in the Inspector: keep a number that has no resource.
            None if self.get_warp_gate().is_none() => {}
            None => self.warp_id = 0,
        }
    }
}

// ---------------------------------------------------------------------------------------
// Markers

/// The editor-only look of a marker: a label, a ring of `radius` metres and a post.
fn draw_marker(node: &mut Gd<Node3D>, label: &str, color: Color, radius: f32, model: Option<String>) {
    if let Some(old) = node.get_node_or_null("EditorMarker") {
        old.free();
    }
    let mut holder = Node3D::new_alloc();
    holder.set_name("EditorMarker");

    let mut material = StandardMaterial3D::new_gd();
    material.set_shading_mode(ShadingMode::UNSHADED);
    material.set_albedo(color);
    material.set_flag(Flags::DISABLE_DEPTH_TEST, true);
    material.set_transparency(Transparency::ALPHA);
    let mut lines = ImmediateMesh::new_gd();
    lines.surface_begin_ex(PrimitiveType::LINES).material(&material).done();
    lines.surface_add_vertex(Vector3::ZERO);
    lines.surface_add_vertex(Vector3::new(0.0, 2.5, 0.0));
    if radius > 0.0 {
        let steps = 48;
        for i in 0..steps {
            for j in [i, i + 1] {
                let angle = j as f32 / steps as f32 * std::f32::consts::TAU;
                lines.surface_add_vertex(Vector3::new(angle.cos() * radius, 0.2, angle.sin() * radius));
            }
        }
    }
    lines.surface_end();
    let mut ring = MeshInstance3D::new_alloc();
    ring.set_mesh(&lines.upcast::<Mesh>());
    holder.add_child(&ring);

    let mut text = Label3D::new_alloc();
    text.set_text(label);
    text.set_billboard_mode(godot::classes::base_material_3d::BillboardMode::ENABLED);
    text.set_draw_flag(godot::classes::label_3d::DrawFlags::DISABLE_DEPTH_TEST, true);
    text.set_modulate(color);
    text.set_outline_size(6);
    // Same size on screen at any distance, so markers stay readable zoomed out.
    text.set_draw_flag(godot::classes::label_3d::DrawFlags::FIXED_SIZE, true);
    text.set_pixel_size(0.0012);
    text.set_font_size(24);
    text.set_position(Vector3::new(0.0, 3.0, 0.0));
    holder.add_child(&text);

    if let Some(path) = model {
        if let Some(scene) = godot::tools::try_load::<PackedScene>(&path).ok().and_then(|s| s.instantiate()) {
            holder.add_child(&scene);
        }
    }
    node.add_child_ex(&holder).internal(InternalMode::BACK).done();
}

fn is_editor() -> bool {
    Engine::singleton().is_editor_hint()
}

/// A marker node: the class docs and fields given (each field with its own `#[export]`), and
/// an editor look redrawn when what it shows changed (checked each editor frame).
macro_rules! marker_class {
    ($(#[$($cattr:tt)*])* $name:ident { $($(#[$($fattr:tt)*])* $field:ident : $ty:ty $(= $init:expr)?),* $(,)? }, |$this:ident| $look:expr) => {
        $(#[$($cattr)*])*
        #[derive(GodotClass)]
        #[class(tool, base=Node3D, init)]
        pub struct $name {
            base: Base<Node3D>,
            $(
                $(#[$($fattr)*])*
                $(#[init(val = $init)])?
                pub $field: $ty,
            )*
            shown: String,
        }

        #[godot_api]
        impl INode3D for $name {
            fn ready(&mut self) {
                self.base_mut().set_process(is_editor());
            }

            fn on_set(&mut self, property: StringName, value: Variant) -> bool {
                self.set_old_property(&property.to_string(), &value)
            }

            fn process(&mut self, _delta: f64) {
                let $this = &*self;
                let (label, color, radius, model): (String, Color, f32, Option<String>) = $look;
                let key = format!("{label}|{radius}|{model:?}");
                if key != self.shown {
                    self.shown = key;
                    let mut node = self.to_gd().upcast::<Node3D>();
                    draw_marker(&mut node, &label, color, radius, model);
                }
            }
        }
    };
}

/// Properties a marker had before (in scenes saved then), read into today's fields.
trait OldProperties {
    fn set_old_property(&mut self, _name: &str, _value: &Variant) -> bool {
        false
    }
}

fn npc_model(npc_id: i32) -> Option<String> {
    (npc_id > 0).then(|| assets::find_stem(&crate::character::npc_scene_stem(npc_id as u16), &["tscn"])).flatten()
}

/// A monster's or NPC's name for the editor labels, from its resource (the number when
/// there is none).
fn character_name(dir: &str, id: i32) -> String {
    match data_resources::find_name(dir, id as i64) {
        Some(name) if !name.trim().is_empty() => format!("{} {id}", name.trim()),
        _ => id.to_string(),
    }
}

fn spawn_label(monsters: &PackedInt32Array, counts: &PackedInt32Array) -> String {
    let counts = counts.as_slice();
    monsters
        .as_slice()
        .iter()
        .enumerate()
        .map(|(i, id)| format!("{}x {}", counts.get(i).copied().unwrap_or(0), character_name(data_resources::MONSTERS_DIR, *id)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The resources of the numbers in `ids` (the Inspector's view of a marker's monsters).
fn monsters_of(ids: &PackedInt32Array) -> Array<Option<Gd<RoseMonster>>> {
    ids.as_slice()
        .iter()
        .map(|id| data_resources::find(data_resources::MONSTERS_DIR, *id as i64).and_then(|(r, _)| r.try_cast::<RoseMonster>().ok()))
        .collect()
}

/// The numbers of `monsters` (set in the Inspector). An empty place keeps the number it had
/// when that number has no resource (so a monster not made in Godot isn't lost), else 0.
fn monster_ids(monsters: &Array<Option<Gd<RoseMonster>>>, old: &PackedInt32Array) -> PackedInt32Array {
    monsters
        .iter_shared()
        .enumerate()
        .map(|(i, monster)| match monster {
            Some(monster) => monster.upcast::<godot::classes::Resource>().get("id").try_to::<i64>().unwrap_or(0) as i32,
            None => old
                .get(i)
                .filter(|id| data_resources::find_name(data_resources::MONSTERS_DIR, *id as i64).is_none())
                .unwrap_or(0),
        })
        .collect()
}

marker_class!(
    /// A monster spawn point (an IFO spawn point). The server keeps monsters alive around it.
    ///
    /// Every `interval` seconds the server checks the point. If fewer than `limit` of its
    /// monsters are alive, it adds some from its lists, within `range` metres. Which entries
    /// spawn depends on the point's "tactics value", which grows each check while players keep
    /// killing its monsters: at first it brings basic monsters, and when it is fought hard
    /// also the tactic monsters (reinforcements, often stronger ones). Only the node's
    /// position matters; its rotation is not used. The full rules and the table of what
    /// spawns when are on the wiki page "Monster spawns" (`wiki/rules/monster-spawns.md`).
    /// After a change, `--export-data` gives it to the server.
    RoseSpawnSpot {
        /// The basic monsters: drag `RoseMonster` resources in from `res://data/monsters/`.
        /// Up to 5 entries are used, and the order matters: each spawn check brings entry 1,
        /// 2, 3, 4 or 5 depending on how hard the point is fought, so entry 1 comes most often
        /// and entries 4 and 5 later. The same monster can be listed more than once. Goes with
        /// `basic_counts`.
        #[export_group(name = "Basic monsters")]
        #[var(get = get_basic_monsters, set = set_basic_monsters, usage_flags = [EDITOR])]
        basic_monsters: PhantomVar<Array<Option<Gd<RoseMonster>>>>,
        /// How many of each basic monster come at once when its entry spawns, one number per
        /// entry of `basic_monsters`, in the same order (so `2, 1` with monsters `1, 8` is two
        /// of monster 1 and one of monster 8). Some checks bring 1 or 2 fewer; 0 means that
        /// entry brings none.
        #[export]
        basic_counts: PackedInt32Array,
        /// The tactic monsters ("reinforcements"): drag `RoseMonster` resources in. They only
        /// come when the point has been fought hard for a while (a high tactics value). Up to
        /// 2 entries are used: entry 1 comes first, entry 2 joins it at the highest tactics.
        /// Goes with `tactic_counts`.
        #[export_group(name = "Tactic monsters")]
        #[var(get = get_tactic_monsters, set = set_tactic_monsters, usage_flags = [EDITOR])]
        tactic_monsters: PhantomVar<Array<Option<Gd<RoseMonster>>>>,
        /// How many of each tactic monster come at once, one number per entry of
        /// `tactic_monsters`, in the same order. Some checks bring one fewer or one more.
        #[export]
        tactic_counts: PackedInt32Array,
        /// The basic monsters' numbers, as the scene stores them (`basic_monsters` shows them
        /// as resources).
        #[export(storage)]
        basic_monster_ids: PackedInt32Array,
        /// The tactic monsters' numbers, as the scene stores them.
        #[export(storage)]
        tactic_monster_ids: PackedInt32Array,
        #[export_group(name = "Spawning")]
        #[export(range = (1.0, 600.0, 1.0, or_greater, suffix = "s"))]
        interval: i32 = 60,
        /// How many of this point's monsters may be alive at once. A check with this many or
        /// more alive spawns nothing (one check can add a few at once, so the count can go a
        /// little over). At least 1 (the export refuses 0).
        #[export(range = (1.0, 50.0, 1.0, or_greater))]
        limit: i32 = 5,
        /// Monsters appear at a random spot within this many metres of the point (the red
        /// circle in the editor). That spot becomes the monster's home, which it wanders
        /// around and walks back to after a chase.
        #[export(range = (0.0, 100.0, 1.0, or_greater, suffix = "m"))]
        range: i32 = 10,
        /// How slowly the point turns to its tactic monsters: the tactics value is divided by
        /// this number, so a smaller number brings tactic monsters sooner and more often, and
        /// a bigger one later. 100 for most points; 60 to 90 on some. At least 1.
        #[export(range = (1.0, 200.0, 1.0, or_greater))]
        tactic_points: i32 = 100,
    }, |this| (
        format!("Spawn: {}\nevery {} s, max {}", spawn_label(&this.basic_monster_ids, &this.basic_counts), this.interval, this.limit),
        Color::from_rgb(1.0, 0.35, 0.3),
        this.range as f32,
        this.basic_monster_ids.as_slice().first().and_then(|id| npc_model(*id)),
    )
);

#[godot_api]
impl RoseSpawnSpot {
    #[func]
    fn get_basic_monsters(&self) -> Array<Option<Gd<RoseMonster>>> {
        monsters_of(&self.basic_monster_ids)
    }

    #[func]
    fn set_basic_monsters(&mut self, monsters: Array<Option<Gd<RoseMonster>>>) {
        self.basic_monster_ids = monster_ids(&monsters, &self.basic_monster_ids);
    }

    #[func]
    fn get_tactic_monsters(&self) -> Array<Option<Gd<RoseMonster>>> {
        monsters_of(&self.tactic_monster_ids)
    }

    #[func]
    fn set_tactic_monsters(&mut self, monsters: Array<Option<Gd<RoseMonster>>>) {
        self.tactic_monster_ids = monster_ids(&monsters, &self.tactic_monster_ids);
    }
}

impl OldProperties for RoseSpawnSpot {
    /// Scenes saved before the markers took resources stored the numbers under the names
    /// the resource lists have now. Only a number list is such an old value: the Inspector
    /// sets the resource list under the same name, and that must reach its setter.
    fn set_old_property(&mut self, name: &str, value: &Variant) -> bool {
        if value.get_type() != VariantType::PACKED_INT32_ARRAY {
            return false;
        }
        let Ok(ids) = value.try_to::<PackedInt32Array>() else { return false };
        match name {
            "basic_monsters" => self.basic_monster_ids = ids,
            "tactic_monsters" => self.tactic_monster_ids = ids,
            _ => return false,
        }
        true
    }
}

marker_class!(
    /// A town NPC standing here (an IFO NPC), facing the node's forward direction (turn the
    /// node to turn the NPC). The server places the NPC from this marker; `--export-data`
    /// gives it a change.
    RoseNpcSpot {
        /// Which NPC: drag a `RoseNpc` resource in from `res://data/npcs/`. Sets the NPC's
        /// name, model, shop and AI. The editor shows its model on the marker.
        #[var(get = get_npc, set = set_npc, usage_flags = [EDITOR], hint = RESOURCE_TYPE, hint_string = "RoseNpc")]
        npc: PhantomVar<Option<Gd<RoseNpc>>>,
        /// The NPC's number, as the scene stores it (`npc` shows it as the resource).
        #[export(storage)]
        npc_id: i32,
        /// The AI script number from the IFO file. Kept from iROSE but not used: the NPC's
        /// AI comes from its LIST_NPC.STB row.
        #[export]
        ai: i32,
        /// What the NPC says and offers when clicked: drag a `RoseDialog` in from
        /// `res://data/dialogs/`. Sets `quest_file` to its key.
        #[var(get = get_dialog, set = set_dialog, usage_flags = [EDITOR], hint = RESOURCE_TYPE, hint_string = "RoseDialog")]
        dialog: PhantomVar<Option<Gd<RoseDialog>>>,
        /// The NPC's dialog by its key (iROSE's CON file name, such as `EM20-001.con`), as
        /// the scene stores it (`dialog` shows it as the resource). Empty: it says nothing.
        #[export]
        quest_file: GString,
    }, |this| (
        format!("NPC {}\n{}", character_name(data_resources::NPCS_DIR, this.npc_id), this.quest_file),
        Color::from_rgb(0.4, 0.8, 1.0),
        0.0,
        npc_model(this.npc_id),
    )
);

#[godot_api]
impl RoseNpcSpot {
    #[func]
    fn get_npc(&self) -> Option<Gd<RoseNpc>> {
        data_resources::find(data_resources::NPCS_DIR, self.npc_id as i64).and_then(|(r, _)| r.try_cast::<RoseNpc>().ok())
    }

    #[func]
    fn set_npc(&mut self, npc: Option<Gd<RoseNpc>>) {
        match npc {
            Some(npc) => self.npc_id = npc.upcast::<godot::classes::Resource>().get("id").try_to::<i64>().unwrap_or(0) as i32,
            // Cleared in the Inspector: keep a number that has no resource (not made in Godot).
            None if self.get_npc().is_none() => {}
            None => self.npc_id = 0,
        }
    }

    #[func]
    fn get_dialog(&self) -> Option<Gd<RoseDialog>> {
        let path = data_resources::find_dialog(&self.quest_file.to_string())?;
        godot::classes::ResourceLoader::singleton().load(&path)?.try_cast::<RoseDialog>().ok()
    }

    #[func]
    fn set_dialog(&mut self, dialog: Option<Gd<RoseDialog>>) {
        match dialog {
            Some(dialog) => self.quest_file = dialog.upcast::<godot::classes::Resource>().get("key").to::<GString>(),
            // Cleared in the Inspector: keep a key that has no resource.
            None if self.get_dialog().is_none() => {}
            None => self.quest_file = GString::new(),
        }
    }
}

impl OldProperties for RoseNpcSpot {
    /// Scenes saved before the markers took resources stored the number as `npc`.
    fn set_old_property(&mut self, name: &str, value: &Variant) -> bool {
        match (name, value.try_to::<i64>()) {
            ("npc", Ok(id)) => {
                self.npc_id = id as i32;
                true
            }
            _ => false,
        }
    }
}

marker_class!(
    /// A looping ambient sound played here (an IFO sound), such as running water or birds.
    /// Played by the game only; the server doesn't know about sounds.
    RoseSoundSpot {
        /// The sound: an audio file, such as the imported
        /// `res://assets/rose/sound/terrain/water_002.res` or one of our own in
        /// `res://assets/ours/`. The Inspector plays it (the arrow next to it).
        #[export]
        stream: Option<Gd<AudioStream>>,
        /// Metres around the spot where the sound plays at full volume (the green circle in
        /// the editor). It fades out further away and is silent past 4 times this (at least
        /// 20 m).
        #[export(range = (0.0, 50.0, 0.1, or_greater, suffix = "m"))]
        range: f32 = 10.0,
        /// Seconds between plays, from the IFO file. Kept from iROSE but not used: the game
        /// loops the sound without a pause.
        #[export(range = (0.0, 600.0, 0.1, or_greater, suffix = "s"))]
        interval: f32,
    }, |this| (format!("Sound\n{}", this.stream.as_ref().map(|s| s.get_path().to_string()).unwrap_or_default().rsplit('/').next().unwrap_or("")), Color::from_rgb(0.6, 1.0, 0.5), this.range, None)
);

marker_class!(
    /// An effect playing here all the time (an IFO effect), such as fire, smoke or sparkles.
    /// Turn and scale the node to turn and scale the effect. Played by the game only.
    RoseEffectSpot {
        /// The effect scene: drag one in, such as the imported
        /// `res://assets/rose/3ddata/effect/bandy_01.scn` or a scene of our own in
        /// `res://assets/ours/`.
        #[export]
        scene: Option<Gd<PackedScene>>,
    }, |this| (format!("Effect\n{}", this.scene.as_ref().map(|s| s.get_path().to_string()).unwrap_or_default().rsplit('/').next().unwrap_or("")), Color::from_rgb(1.0, 0.85, 0.3), 0.0, None)
);

marker_class!(
    /// A named spot of the zone (a ZON event position). The zone's `RoseZoneSettings` names
    /// its start spot (the default place to arrive in the zone) and its revive spots (fallen
    /// players get up at the nearest one), and a `RoseWarpGate` names the spot it lands you
    /// on. Several spots can share a name. The server reads them through `--export-data`.
    RoseZoneEvent {
        /// The spot's name, as the zone settings and warp gates refer to it: usually `start` for the
        /// start spot and `restore` for revive spots; warp targets have names such as
        /// `WARP-JD01-JG01`. Renaming a spot breaks whatever pointed at the old name.
        #[export]
        event_name: GString,
    }, |this| (format!("Event: {}", this.event_name), Color::from_rgb(0.9, 0.5, 1.0), 1.0, None)
);

/// The imported audio file of a sound file in the client data (`SOUND/TERRAIN/WIND_001.WAV`),
/// None when it isn't imported.
fn sound_stream(client_path: &str) -> Option<Gd<AudioStream>> {
    let path = data_resources::audio_link(client_path)?;
    godot::tools::try_load::<AudioStream>(&path).ok()
}

impl OldProperties for RoseSoundSpot {
    /// Scenes saved before the spots linked audio files stored the client file as `sound`.
    fn set_old_property(&mut self, name: &str, value: &Variant) -> bool {
        match (name, value.try_to::<GString>()) {
            ("sound", Ok(path)) => {
                self.stream = sound_stream(&path.to_string());
                true
            }
            _ => false,
        }
    }
}
impl OldProperties for RoseEffectSpot {
    /// Scenes saved before the spots linked effect scenes stored the client file as `effect`.
    fn set_old_property(&mut self, name: &str, value: &Variant) -> bool {
        match (name, value.try_to::<GString>()) {
            ("effect", Ok(path)) => {
                self.scene = crate::effect::effect_scene(&path.to_string());
                true
            }
            _ => false,
        }
    }
}
impl OldProperties for RoseZoneEvent {}

// ---------------------------------------------------------------------------------------
// Import

#[derive(Default)]
pub struct ZoneImport {
    pub saved: i64,
    pub kept: i64,
    pub failed: i64,
    pub bytes: u64,
    /// The files written or kept so far, as zones share some (the ground textures).
    done: HashSet<String>,
}

impl ZoneImport {
    fn save(&mut self, redo: bool, stem: &str, extension: &str, make: impl FnOnce() -> Option<Gd<godot::classes::Resource>>) {
        let target = format!("{}{stem}.{extension}", assets::ROSE);
        if !self.done.insert(target.clone()) {
            return;
        }
        let disk = godot::classes::ProjectSettings::singleton().globalize_path(&target).to_string();
        if !redo && std::path::Path::new(&disk).is_file() {
            self.kept += 1;
        } else if make().is_some_and(|resource| assets::save(resource, &target)) {
            self.saved += 1;
        } else {
            godot_warn!("rose: could not make {target}");
            self.failed += 1;
            return;
        }
        self.bytes += std::fs::metadata(&disk).map_or(0, |m| m.len());
    }
}

struct ZoneFiles {
    name: String,
    zon: ZonFile,
    zone_dir: String,
    /// The four model lists, in `LISTS` order.
    lists: Vec<(String, Option<ZscFile>)>,
}

fn zone_files(zone_id: u16) -> Option<ZoneFiles> {
    let game_data = data::get()?;
    let entry = rose_data::ZoneId::new(zone_id).and_then(|id| game_data.zone_list.get_zone(id))?;
    let zon_path = entry.zon_file_path.path().to_string_lossy().to_string();
    let zon = data::read_file::<ZonFile>(&zon_path)?;
    let zone_dir = std::path::Path::new(&zon_path).parent().map(|p| p.to_string_lossy().replace('\\', "/")).unwrap_or_default();
    let list_paths = [
        entry.zsc_cnst_path.path().to_string_lossy().to_string(),
        entry.zsc_deco_path.path().to_string_lossy().to_string(),
        zone::EVENT_ZSC.to_string(),
        zone::SPECIAL_ZSC.to_string(),
    ];
    let lists = list_paths.into_iter().map(|path| {
        let zsc = data::read_file::<ZscFile>(&path);
        (path, zsc)
    }).collect();
    Some(ZoneFiles { name: entry.name.to_string(), zon, zone_dir, lists })
}

fn object_stem(list_path: &str, object_id: usize) -> String {
    format!("{}/{object_id}", assets::stem(list_path))
}

/// The model scene of one ZSC object: a `RoseZoneObject` with a `PartN` mesh per part (its
/// collision in the `walls`/`floors` meta) and an `EffectN` node per effect.
fn object_scene(zsc: &ZscFile, zsc_index: usize, list: &str, object_id: usize) -> Option<Gd<godot::classes::Resource>> {
    let object = zsc.objects.get(object_id)?;
    let mut root = RoseZoneObject::new_alloc();
    root.set_name(&format!("{}{object_id}", list[..1].to_uppercase() + &list[1..]));
    root.bind_mut().list = list.into();
    let mut materials: BTreeMap<u16, Gd<ShaderMaterial>> = BTreeMap::new();
    let mut parts = 0;
    for (part_index, part) in object.parts.iter().enumerate() {
        let Some(mesh_path) = zsc.meshes.get(part.mesh_id as usize) else { continue };
        let Some(zsc_material) = zsc.materials.get(part.material_id as usize) else { continue };
        let mesh_path = mesh_path.path().to_string_lossy().to_string();
        let Some(mesh) = assets::load_mesh(&mesh_path) else {
            godot_warn!("rose: {mesh_path} is not converted; run --import-rose first");
            continue;
        };
        let mesh = crate::mesh::rigid(&mesh);
        let material = materials.entry(part.material_id).or_insert_with(|| material::object_material(zsc_material, material::Light::Map)).clone();
        let mut instance = MeshInstance3D::new_alloc();
        instance.set_name(&format!("Part{part_index}"));
        instance.set_mesh(&mesh);
        instance.set_material_override(&material);
        if !material::casts_shadow(&material) {
            instance.set_cast_shadows_setting(ShadowCastingSetting::OFF);
        }
        instance.set_transform(zone::part_transform(&part.position, &part.rotation, &part.scale));
        let (walls, floors) = zone::part_collision(zsc_index, part);
        if walls {
            instance.set_meta("walls", &true.to_variant());
        }
        if floors {
            instance.set_meta("floors", &true.to_variant());
        }
        root.add_child(&instance);
        instance.set_owner(&root);
        parts += 1;
    }
    for (i, effect) in object.effects.iter().enumerate() {
        let Some(path) = zsc.effects.get(effect.effect_id as usize) else { continue };
        let mut node = Node3D::new_alloc();
        node.set_name(&format!("Effect{i}"));
        node.set_transform(zone::part_transform(&effect.position, &effect.rotation, &effect.scale));
        node.set_meta("effect", &path.path().to_string_lossy().to_string().to_variant());
        if matches!(effect.effect_type, ZscEffectType::DayNight) {
            node.set_meta("night_only", &true.to_variant());
        }
        root.add_child(&node);
        node.set_owner(&root);
    }
    let mut scene = PackedScene::new_gd();
    let packed = parts > 0 && scene.pack(&root) == Error::OK;
    root.free();
    packed.then(|| scene.upcast())
}

/// Makes the converted assets a zone's scenes use: a model scene for every model in the
/// zone's own lists and for the event objects and warp gate it places, and its ground
/// textures in the form Terrain3D takes.
pub fn import_zone_assets(zone_id: u16, redo: bool, out: &mut ZoneImport) {
    let Some(files) = zone_files(zone_id) else {
        out.failed += 1;
        return;
    };
    for ((list, zsc_index), (path, zsc)) in LISTS.iter().zip(files.lists.iter()) {
        let Some(zsc) = zsc else { continue };
        // Every model of the zone's own lists (so any can be placed); the shared event and
        // warp lists only as far as the zone uses them.
        let ids: Vec<usize> = if *zsc_index < 2 { (0..zsc.objects.len()).collect() } else { used_objects(&files, *zsc_index) };
        for object_id in ids {
            if zsc.objects.get(object_id).is_none_or(|o| o.parts.is_empty()) {
                continue;
            }
            out.save(redo, &object_stem(path, object_id), "tscn", || object_scene(zsc, *zsc_index, list, object_id));
        }
    }
    let tile_paths = zone::tile_texture_paths(&files.zon);
    for number in used_tile_numbers(&files) {
        let Some(path) = tile_paths.get(number) else { continue };
        out.save(redo, &terrain_texture_stem(&files, number), "res", || terrain_texture(path));
    }
}

fn blocks(files: &ZoneFiles) -> Vec<(usize, usize)> {
    let mut blocks = Vec::new();
    for block_y in 0..64usize {
        for block_x in 0..64usize {
            if data::get().is_some_and(|g| g.vfs.exists(format!("{}/{block_x}_{block_y}.HIM", files.zone_dir).as_str())) {
                blocks.push((block_x, block_y));
            }
        }
    }
    blocks
}

fn used_objects(files: &ZoneFiles, zsc_index: usize) -> Vec<usize> {
    let mut ids = BTreeSet::new();
    for (block_x, block_y) in blocks(files) {
        let Some(ifo) = data::read_file::<IfoFile>(&format!("{}/{block_x}_{block_y}.IFO", files.zone_dir)) else { continue };
        match zsc_index {
            2 => ids.extend(ifo.event_objects.iter().map(|o| o.object.object_id as usize)),
            3 if !ifo.warps.is_empty() => {
                ids.insert(1);
            }
            _ => {}
        }
    }
    ids.into_iter().collect()
}

/// Every ROSE (ZON) ground texture the zone's tiles use: texture N of the zone's ground is
/// the Nth of these. In number order; past 32 textures, the 32 used on most tiles come
/// first so that Terrain3D's paint tools hold them, and the rest become extra textures.
fn used_tile_numbers(files: &ZoneFiles) -> Vec<usize> {
    let mut tiles: BTreeMap<usize, usize> = BTreeMap::new();
    for (block_x, block_y) in blocks(files) {
        let til = data::read_file::<TilFile>(&format!("{}/{block_x}_{block_y}.TIL", files.zone_dir));
        for y in 0..16 {
            for x in 0..16 {
                let [layer1, layer2, _] = zone::tile_layers(&files.zon, til.as_ref(), x, y);
                *tiles.entry(layer1 as usize).or_default() += 1;
                if layer2 != layer1 {
                    *tiles.entry(layer2 as usize).or_default() += 1;
                }
            }
        }
    }
    let mut used: Vec<usize> = tiles.keys().copied().collect();
    if used.len() > TERRAIN3D_TEXTURES {
        let mut by_use = used.clone();
        by_use.sort_by_key(|n| std::cmp::Reverse(tiles[n]));
        let (mut first, mut rest) = (by_use[..TERRAIN3D_TEXTURES].to_vec(), by_use[TERRAIN3D_TEXTURES..].to_vec());
        first.sort();
        rest.sort();
        used = first.into_iter().chain(rest).collect();
    }
    used
}

/// Where the Terrain3D copy of the zone's ground texture `number` goes. It is named after the
/// ROSE tile file (`terrain_textures/3ddata/terrain/tiles/.../t001_01`), so zones that use
/// the same tile share one copy.
fn terrain_texture_stem(files: &ZoneFiles, number: usize) -> String {
    let tile = zone::tile_texture_paths(&files.zon).get(number).cloned().unwrap_or_default();
    format!("terrain_textures/{}", assets::stem(&tile))
}

/// A ground texture as Terrain3D takes it: RGBA8, 256 x 256 (every ROSE tile's size), with mipmaps.
fn terrain_texture(path: &str) -> Option<Gd<godot::classes::Resource>> {
    let mut image = texture::load_image(path)?;
    if image.is_compressed() {
        image.decompress();
    }
    image.convert(godot::classes::image::Format::RGBA8);
    if image.get_width() != 256 || image.get_height() != 256 {
        image.resize(256, 256);
    }
    image.generate_mipmaps();
    godot::classes::ImageTexture::create_from_image(&image).map(|t| t.upcast())
}

/// A region's heights, control values, and which vertices belong to a block.
type RegionData = (Vec<f32>, Vec<u32>, Vec<bool>);

/// Terrain3D's colour map value for no colour: white, and its default roughness (0.5).
const NO_TINT: [u8; 4] = [255, 255, 255, 128];

/// Collects the blocks' heights and tiles into Terrain3D regions.
#[derive(Default)]
struct GroundBuilder {
    regions: BTreeMap<(i64, i64), RegionData>,
    /// Colour map pixels (RGBA8) of the regions that have a tinted block.
    tints: BTreeMap<(i64, i64), Vec<[u8; 4]>>,
}

/// The colour map pixel of a block's ground tint at vertex (i, j) of the block.
fn tint_pixel(tint: &[Color], i: i64, j: i64) -> [u8; 4] {
    let colour = zone::tint_at(tint, i as f32 * VERTEX_SPACING, j as f32 * VERTEX_SPACING);
    let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    [byte(colour.r), byte(colour.g), byte(colour.b), NO_TINT[3]]
}

/// Paints a block's ground tint (`zone::ground_tint`): `set` takes each vertex (gx, gz) of
/// the block, its far edge included, and its colour map pixel.
fn paint_tint(block_x: usize, block_y: usize, tint: &[Color], mut set: impl FnMut(i64, i64, [u8; 4])) {
    let (gx0, gz0) = (BLOCK_VERTICES * block_x as i64, BLOCK_VERTICES * (block_y as i64 - 65));
    for j in 0..=BLOCK_VERTICES {
        for i in 0..=BLOCK_VERTICES {
            set(gx0 + i, gz0 + j, tint_pixel(tint, i, j));
        }
    }
}

impl GroundBuilder {
    fn add_tint(&mut self, block_x: usize, block_y: usize, tint: &[Color]) {
        let size = REGION_SIZE;
        let tints = &mut self.tints;
        paint_tint(block_x, block_y, tint, |gx, gz, pixel| {
            let map = tints.entry((gx.div_euclid(size), gz.div_euclid(size))).or_insert_with(|| vec![NO_TINT; (size * size) as usize]);
            map[(gz.rem_euclid(size) * size + gx.rem_euclid(size)) as usize] = pixel;
        });
    }

    fn vertex(&mut self, gx: i64, gz: i64) -> (&mut f32, &mut u32, &mut bool) {
        let size = REGION_SIZE;
        let area = (size * size) as usize;
        let (heights, controls, used) = self
            .regions
            .entry((gx.div_euclid(size), gz.div_euclid(size)))
            .or_insert_with(|| (vec![0.0; area], vec![HOLE_BIT; area], vec![false; area]));
        let i = (gz.rem_euclid(size) * size + gx.rem_euclid(size)) as usize;
        (&mut heights[i], &mut controls[i], &mut used[i])
    }

    /// A block's 65 x 65 heights (its far edge is the next block's first row) and each
    /// vertex's tile textures.
    fn add_block(&mut self, block_x: usize, block_y: usize, him: &HimFile, tile_control: impl Fn(usize, usize) -> u32) {
        let (gx0, gz0) = (BLOCK_VERTICES * block_x as i64, BLOCK_VERTICES * (block_y as i64 - 65));
        for j in 0..=BLOCK_VERTICES {
            for i in 0..=BLOCK_VERTICES {
                let edge = i == BLOCK_VERTICES || j == BLOCK_VERTICES;
                let (height, control, used) = self.vertex(gx0 + i, gz0 + j);
                // A neighbour's own first row wins over this block's edge.
                if edge && *used {
                    continue;
                }
                *height = him.get_clamped(i as i32, j as i32) / 100.0;
                let (tile_x, tile_y) = ((i / 4).min(15) as usize, (j / 4).min(15) as usize);
                *control = tile_control(tile_x, tile_y);
                *used = !edge || *used;
            }
        }
    }

    /// A Terrain3D node with these regions, saved in `directory`, the ROSE terrain shader
    /// and the zone's textures.
    fn build(self, directory: &str, files: &ZoneFiles, numbers: &[usize]) -> Result<Gd<Node>, String> {
        let make = |class: &str| -> Result<Gd<godot::classes::Object>, String> {
            godot::classes::ClassDb::singleton()
                .instantiate(class)
                .try_to::<Gd<godot::classes::Object>>()
                .map_err(|_| format!("{class} is missing; is the Terrain3D addon in addons/terrain_3d?"))
        };
        let mut terrain = make("Terrain3D")?.try_cast::<Node3D>().map_err(|_| "Terrain3D is not a node".to_string())?;
        terrain.set_name("Terrain");
        terrain.set("vertex_spacing", &VERTEX_SPACING.to_variant());
        // No physics: the game takes ground heights from the data, and layer 1 is our walls.
        terrain.set("collision_mode", &0.to_variant());
        terrain.set("mesh_lods", &MESH_LODS.to_variant());

        // Into the tree, so Terrain3D sets up its data, then add and save the regions.
        let mut root = Engine::singleton()
            .get_main_loop()
            .and_then(|l| l.try_cast::<godot::classes::SceneTree>().ok())
            .map(|t| t.get_root())
            .ok_or("no scene tree")?;
        root.add_child(&terrain);
        terrain.call("change_region_size", &[REGION_SIZE.to_variant()]);
        let mut data = terrain.get("data").try_to::<Gd<godot::classes::Object>>().map_err(|_| "Terrain3D has no data".to_string())?;
        for ((rx, rz), (heights, controls, _)) in self.regions {
            let image = |values: Vec<u8>| {
                godot::classes::Image::create_from_data(REGION_SIZE as i32, REGION_SIZE as i32, false, godot::classes::image::Format::RF, &PackedByteArray::from(values.as_slice()))
            };
            let height_map = image(heights.iter().flat_map(|h| h.to_le_bytes()).collect()).ok_or("height map")?;
            let control_map = image(controls.iter().flat_map(|c| c.to_le_bytes()).collect()).ok_or("control map")?;
            let mut region = make("Terrain3DRegion")?;
            region.set("region_size", &REGION_SIZE.to_variant());
            region.set("vertex_spacing", &VERTEX_SPACING.to_variant());
            region.set("location", &Vector2i::new(rx as i32, rz as i32).to_variant());
            region.call("set_height_map", &[height_map.to_variant()]);
            region.call("set_control_map", &[control_map.to_variant()]);
            if let Some(tint) = self.tints.get(&(rx, rz)) {
                region.call("set_color_map", &[color_map(tint).ok_or("color map")?.to_variant()]);
            }
            region.call("sanitize_maps", &[]);
            region.call("calc_height_range", &[]);
            region.set("modified", &true.to_variant());
            data.call("add_region", &[region.to_variant(), false.to_variant()]);
        }
        let _ = std::fs::remove_dir_all(godot::classes::ProjectSettings::singleton().globalize_path(directory).to_string());
        DirAccess::make_dir_recursive_absolute(directory);
        data.call("save_directory", &[directory.to_variant()]);
        root.remove_child(&terrain);
        terrain.set("data_directory", &directory.to_variant());

        // The ROSE look, and the ground textures for the editor's paint tools.
        let mut material = make("Terrain3DMaterial")?;
        material.set("world_background", &0.to_variant());
        material.set("shader_override", &godot::tools::load::<godot::classes::Shader>(TERRAIN_SHADER3D).to_variant());
        material.set("shader_override_enabled", &true.to_variant());
        terrain.set("material", &material.to_variant());
        let mut assets = make("Terrain3DAssets")?;
        for (id, number) in numbers.iter().take(TERRAIN3D_TEXTURES).enumerate() {
            let Some(path) = zone::tile_texture_paths(&files.zon).get(*number).cloned() else { continue };
            let Some(texture) = assets::find_stem(&terrain_texture_stem(files, *number), &["res"]).and_then(|p| godot::tools::try_load::<Texture2D>(&p).ok()) else {
                return Err(format!("{path} is not converted for Terrain3D; run --import-rose"));
            };
            let mut asset = make("Terrain3DTextureAsset")?;
            let name = path.rsplit(['/', '\\']).next().unwrap_or("").to_string();
            asset.set("name", &name.to_variant());
            asset.set("id", &(id as i64).to_variant());
            asset.set("albedo_texture", &texture.to_variant());
            assets.call("set_texture", &[(id as i64).to_variant(), asset.to_variant()]);
        }
        // Saved in its own file, which lets Terrain3D free the editor's copies of the textures in game.
        let assets_path = format!("{directory}_assets.tres");
        if !save_resource(assets.clone().cast::<godot::classes::Resource>(), &assets_path) {
            return Err(format!("could not save {assets_path}"));
        }
        let assets = godot::tools::try_load::<godot::classes::Resource>(&assets_path).map_err(|_| format!("could not open {assets_path}"))?;
        terrain.set("assets", &assets.to_variant());
        Ok(terrain.upcast())
    }
}

/// A Terrain3D colour map from its RGBA8 pixels (with mipmaps, which Terrain3D draws it with).
fn color_map(pixels: &[[u8; 4]]) -> Option<Gd<godot::classes::Image>> {
    let bytes: Vec<u8> = pixels.iter().flatten().copied().collect();
    let size = (pixels.len() as f64).sqrt() as i32;
    let mut image = godot::classes::Image::create_from_data(size, size, false, godot::classes::image::Format::RGBA8, &PackedByteArray::from(bytes.as_slice()))?;
    if image.is_empty() {
        return None;
    }
    image.generate_mipmaps();
    Some(image)
}

fn slug(name: &str) -> String {
    let slug: String = name.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    slug.split('_').filter(|s| !s.is_empty()).collect::<Vec<_>>().join("_")
}

/// Saves `root` (owning its nodes) as a text scene. References to converted assets keep
/// only their paths, since `assets/rose/` is remade on every machine with new ids.
fn save_scene(root: Gd<Node>, path: &str) -> bool {
    let mut scene = PackedScene::new_gd();
    let packed = scene.pack(&root) == Error::OK;
    root.free();
    packed && save_resource(scene.upcast(), path)
}

/// Saves a text resource, without the ids of the converted assets it names.
fn save_resource(resource: Gd<godot::classes::Resource>, path: &str) -> bool {
    let error = ResourceSaver::singleton().save_ex(&resource).path(path).done();
    if error != Error::OK {
        godot_warn!("rose: could not save {path}: {error:?}");
        return false;
    }
    let Some(file) = FileAccess::open(path, ModeFlags::READ) else { return false };
    let text = file.get_as_text().to_string();
    drop(file);
    let stripped: Vec<String> = text
        .lines()
        .map(|line| {
            if line.starts_with("[ext_resource") {
                if let Some(start) = line.find(" uid=\"") {
                    if let Some(len) = line[start + 6..].find('"') {
                        return format!("{}{}", &line[..start], &line[start + 6 + len + 1..]);
                    }
                }
            }
            line.to_string()
        })
        .collect();
    let Some(mut file) = FileAccess::open(path, ModeFlags::WRITE) else { return false };
    file.store_string(&(stripped.join("\n") + "\n"));
    true
}

fn instance(path: &str) -> Option<Gd<Node>> {
    godot::tools::try_load::<PackedScene>(path).ok()?.instantiate_ex().edit_state(GenEditState::INSTANCE).done()
}

fn group(root: &mut Gd<Node3D>, name: &str) -> Gd<Node3D> {
    let mut node = Node3D::new_alloc();
    node.set_name(name);
    root.add_child(&node);
    node.set_owner(&*root);
    node
}

fn add_owned(parent: &mut Gd<Node3D>, owner: &Gd<Node3D>, node: Gd<Node>) {
    parent.add_child(&node);
    node.clone().set_owner(owner);
}

/// Every node under `owner` that it owns (what a scene saved from it would hold).
fn owned_by(owner: &Gd<Node>) -> Vec<Gd<Node>> {
    let mut owned = Vec::new();
    for child in owner.get_children().iter_shared() {
        walk(&child, &mut |node| {
            if node.get_owner().as_ref() == Some(owner) {
                owned.push(node.clone());
            }
        });
    }
    owned
}

/// The `block_X_Y.tscn` files in a zone folder: zones converted before 2026-10-18 had one per block.
fn block_files(folder: &str) -> Vec<String> {
    let Some(dir) = DirAccess::open(folder) else { return Vec::new() };
    let mut files: Vec<String> = dir
        .get_files()
        .as_slice()
        .iter()
        .map(|f| f.to_string())
        .filter(|f| f.starts_with("block_") && f.ends_with(".tscn"))
        .map(|f| format!("{folder}/{f}"))
        .collect();
    files.sort();
    files
}

/// Moves the blocks of a zone scene from their own `block_X_Y.tscn` files into `zone.tscn` as
/// `RoseBlock` nodes, makes the root a `RoseZoneScene`, and deletes the block files. Nothing
/// in the blocks changes: same nodes, names, positions and edits made in `zone.tscn`.
/// Returns (folder, blocks merged); 0 when the zone is already one scene.
pub fn merge_zone_blocks(zone_id: u16) -> Result<(String, usize), String> {
    let folder = zone_folder(zone_id).ok_or(format!("zone {zone_id} has no folder in {ZONES}"))?;
    let path = format!("{folder}/zone.tscn");
    let files = block_files(&folder);
    // Every scene and texture the blocks use must load, or the nodes made from it would be lost.
    let mut missing = BTreeSet::new();
    for file in std::iter::once(&path).chain(files.iter()) {
        let text = FileAccess::get_file_as_string(file.as_str()).to_string();
        for line in text.lines().filter(|l| l.starts_with("[ext_resource")) {
            let Some(start) = line.find(" path=\"") else { continue };
            let rest = &line[start + 7..];
            let used = &rest[..rest.find('"').unwrap_or(rest.len())];
            if !ResourceLoader::singleton().exists(used) {
                missing.insert(used.to_string());
            }
        }
    }
    if let Some(first) = missing.iter().next() {
        return Err(format!("{path} uses {} files that are not there, such as {first}; run --import-rose first", missing.len()));
    }
    let scene = godot::tools::try_load::<PackedScene>(&path).map_err(|_| format!("could not open {path}"))?;
    let old_root = scene.instantiate_ex().edit_state(GenEditState::MAIN).done().ok_or(format!("could not open {path}"))?;

    let mut root = RoseZoneScene::new_alloc();
    root.set_name(&old_root.get_name());
    let meta_id = old_root.get_meta_ex("zone_id").default(&Variant::nil()).done().try_to::<i64>().ok();
    root.bind_mut().zone_id = meta_id.map_or(zone_id as i32, |id| id as i32);
    if let Ok(old) = old_root.clone().try_cast::<Node3D>() {
        root.set_transform(old.get_transform());
    }
    let mut root = root.upcast::<Node>();

    let mut merged = 0;
    for child in old_root.get_children().iter_shared() {
        let owned = owned_by(&old_root).into_iter().filter(|n| n == &child || child.is_ancestor_of(n)).collect::<Vec<_>>();
        let mut child = child;
        let instanced_block = !child.get_scene_file_path().is_empty() && block_of(&child).is_some();
        if instanced_block {
            // The block's own nodes, as the zone scene shows them (with any edits made there).
            let (block_x, block_y) = block_of(&child).unwrap_or_default();
            let mut block = RoseBlock::new_alloc();
            block.set_name(&child.get_name());
            {
                let mut b = block.bind_mut();
                b.block_x = block_x;
                b.block_y = block_y;
            }
            if let Ok(old) = child.clone().try_cast::<Node3D>() {
                block.set_transform(old.get_transform());
                block.set_visible(old.is_visible());
            }
            let inner = owned_by(&child);
            for mut node in inner.iter().cloned() {
                node.set_owner(Gd::null_arg());
            }
            for grandchild in child.get_children().iter_shared() {
                child.remove_child(&grandchild);
                block.add_child(&grandchild);
            }
            old_root.clone().remove_child(&child);
            child.free();
            let block = block.upcast::<Node>();
            root.add_child(&block);
            block.clone().set_owner(&root);
            for mut node in inner {
                node.set_owner(&root);
            }
            merged += 1;
        } else {
            for mut node in owned.iter().cloned() {
                node.set_owner(Gd::null_arg());
            }
            old_root.clone().remove_child(&child);
            root.add_child(&child);
            for mut node in owned {
                node.set_owner(&root);
            }
        }
    }
    old_root.free();
    if merged == 0 && files.is_empty() && meta_id.is_none() {
        root.free();
        return Ok((folder, 0));
    }
    if !save_scene(root, &path) {
        return Err(format!("could not save {path}"));
    }
    for file in &files {
        DirAccess::remove_absolute(file.as_str());
    }
    Ok((folder, merged))
}

/// Takes the baked ROSE lightmaps out of a zone scene made before real-time shadows: the
/// `lightmaps` and `lightmap_cells` of its models and ground, and the textures only they
/// used. First it paints the colour those lightmaps gave the ground into Terrain3D's colour
/// map (`zone::ground_tint`, from the client's lightmaps). Nothing else in the scene changes:
/// same nodes, names and positions. Returns (folder, lines taken out, blocks coloured); 0
/// lines when the scene has no lightmaps (it is left alone).
pub fn strip_lightmaps(zone_id: u16) -> Result<(String, usize, usize), String> {
    let folder = zone_folder(zone_id).ok_or(format!("zone {zone_id} has no folder in {ZONES}"))?;
    let path = format!("{folder}/zone.tscn");
    let text = FileAccess::get_file_as_string(path.as_str()).to_string();
    if text.is_empty() {
        return Err(format!("could not read {path}"));
    }
    let is_lightmap_line = |line: &str| line.starts_with("lightmaps = ") || line.starts_with("lightmap_cells = ");
    let removed = text.lines().filter(|line| is_lightmap_line(line)).count();
    if removed == 0 {
        return Ok((folder, 0, 0));
    }
    let kept: Vec<&str> = text.lines().filter(|line| !is_lightmap_line(line)).collect();
    // The textures nothing links any more.
    let mut used = HashSet::new();
    for line in kept.iter().filter(|l| !l.starts_with("[ext_resource")) {
        let mut rest = *line;
        while let Some(at) = rest.find("ExtResource(\"") {
            rest = &rest[at + 13..];
            if let Some(end) = rest.find('"') {
                used.insert(rest[..end].to_string());
                rest = &rest[end..];
            }
        }
    }
    let id_of = |line: &str| line.find(" id=\"").and_then(|at| line[at + 5..].find('"').map(|end| line[at + 5..at + 5 + end].to_string()));
    let kept: Vec<&str> = kept
        .into_iter()
        .filter(|line| !(line.starts_with("[ext_resource type=\"Texture2D\"") && id_of(line).is_some_and(|id| !used.contains(&id))))
        .collect();

    // The ground's colour, into the regions the scene's Terrain3D node loads.
    let value_of = |key: &str| kept.iter().find_map(|line| line.strip_prefix(key)).map(|v| v.trim().to_string());
    let directory = value_of("data_directory = ").map(|v| v.trim_matches('"').to_string()).ok_or(format!("{path} has no Terrain3D data_directory"))?;
    let blocks: Vec<i32> = value_of("blocks = PackedInt32Array(")
        .map(|v| v.trim_end_matches(')').split(',').filter_map(|n| n.trim().parse().ok()).collect())
        .unwrap_or_default();
    let files = zone_files(zone_id).ok_or(format!("zone {zone_id} is not in the zone list; is --data-idx right?"))?;
    let coloured = paint_ground_tints(&directory, &files.zone_dir, &blocks)?;

    let Some(mut file) = FileAccess::open(path.as_str(), ModeFlags::WRITE) else { return Err(format!("could not write {path}")) };
    file.store_string(&(kept.join("\n") + "\n"));
    Ok((folder, removed, coloured))
}

/// Paints the ground tint of `blocks` (pairs of block x, block y) into the colour maps of
/// the Terrain3D regions saved in `directory`. Returns how many blocks had a lightmap.
fn paint_ground_tints(directory: &str, zone_dir: &str, blocks: &[i32]) -> Result<usize, String> {
    let mut terrain = godot::classes::ClassDb::singleton()
        .instantiate("Terrain3D")
        .try_to::<Gd<Node3D>>()
        .map_err(|_| "Terrain3D is missing; is the Terrain3D addon in addons/terrain_3d?".to_string())?;
    terrain.set("vertex_spacing", &VERTEX_SPACING.to_variant());
    terrain.set("data_directory", &directory.to_variant());
    let mut root = Engine::singleton()
        .get_main_loop()
        .and_then(|l| l.try_cast::<godot::classes::SceneTree>().ok())
        .map(|t| t.get_root())
        .ok_or("no scene tree")?;
    root.add_child(&terrain);
    let result = (|| {
        let mut data = terrain.get("data").try_to::<Gd<godot::classes::Object>>().map_err(|_| "Terrain3D has no data".to_string())?;
        let region_size = terrain.get("region_size").try_to::<i64>().unwrap_or(REGION_SIZE);
        let mut maps: BTreeMap<(i64, i64), Vec<[u8; 4]>> = BTreeMap::new();
        let mut coloured = 0;
        for pair in blocks.chunks(2) {
            let [block_x, block_y] = pair else { continue };
            let Some(tint) = zone::ground_tint(zone_dir, *block_x as usize, *block_y as usize) else { continue };
            coloured += 1;
            paint_tint(*block_x as usize, *block_y as usize, &tint, |gx, gz, pixel| {
                let location = (gx.div_euclid(region_size), gz.div_euclid(region_size));
                let map = maps.entry(location).or_insert_with(|| {
                    // The region's colour map as it is (Terrain3D's default where there is none).
                    let region = data.call("get_region", &[Vector2i::new(location.0 as i32, location.1 as i32).to_variant()]);
                    let image = region.try_to::<Gd<godot::classes::Object>>().ok().and_then(|r| r.get("color_map").try_to::<Gd<godot::classes::Image>>().ok());
                    match image {
                        Some(mut image) if image.get_width() as i64 == region_size => {
                            image.clear_mipmaps();
                            image.convert(godot::classes::image::Format::RGBA8);
                            image.get_data().as_slice().chunks_exact(4).map(|c| [c[0], c[1], c[2], c[3]]).collect()
                        }
                        _ => vec![NO_TINT; (region_size * region_size) as usize],
                    }
                });
                if let Some(p) = map.get_mut((gz.rem_euclid(region_size) * region_size + gx.rem_euclid(region_size)) as usize) {
                    *p = pixel;
                }
            });
        }
        for ((rx, rz), pixels) in maps {
            let Ok(mut region) = data.call("get_region", &[Vector2i::new(rx as i32, rz as i32).to_variant()]).try_to::<Gd<godot::classes::Object>>() else {
                continue;
            };
            region.call("set_color_map", &[color_map(&pixels).ok_or("color map")?.to_variant()]);
            region.set("modified", &true.to_variant());
        }
        data.call("save_directory", &[directory.to_variant()]);
        Ok(coloured)
    })();
    root.remove_child(&terrain);
    terrain.free();
    result
}

/// A model placed in a block: its number in the list and the IFO entry.
type Placement<'a> = (usize, &'a rose_file_readers::IfoObject);

fn spawn_lists(spawns: &[IfoMonsterSpawn]) -> (PackedInt32Array, PackedInt32Array) {
    (spawns.iter().map(|s| s.id as i32).collect(), spawns.iter().map(|s| s.count as i32).collect())
}

/// Writes a zone's scene (`res://zones/<id>_<name>/zone.tscn`, a `RoseBlock` per block) and
/// its ground (`terrain/`) from the ROSE files. Refuses to replace scenes already there unless `force`.
/// Returns (folder, blocks, objects, markers).
pub fn write_zone_scenes(zone_id: u16, force: bool) -> Result<(String, usize, usize, usize), String> {
    let files = zone_files(zone_id).ok_or(format!("zone {zone_id} is not in the zone list or its ZON file is missing"))?;
    if let Some(existing) = zone_folder(zone_id) {
        if !force {
            return Err(format!("{existing} already holds this zone's scenes, which may have edits; use --import-zone={zone_id},force to replace them"));
        }
    }
    let folder = zone_folder(zone_id).unwrap_or_else(|| format!("{ZONES}{zone_id}_{}", slug(&files.name)));
    DirAccess::make_dir_recursive_absolute(&folder);
    // Block files of an older import would be left behind unused.
    for file in block_files(&folder) {
        DirAccess::remove_absolute(file.as_str());
    }

    let mut assets_made = ZoneImport::default();
    import_zone_assets(zone_id, false, &mut assets_made);
    let numbers = used_tile_numbers(&files);
    if numbers.len() > MAX_GROUND_TEXTURES {
        return Err(format!("the zone uses {} ground textures; zone scenes take {MAX_GROUND_TEXTURES}", numbers.len()));
    }
    let mut ground = GroundBuilder::default();
    let mut look = RoseTerrainLook::new_alloc();
    look.set_name("RoseTerrainLook");
    look.bind_mut().texture_numbers = numbers.iter().map(|n| *n as i32).collect();
    for number in numbers.iter().skip(TERRAIN3D_TEXTURES) {
        let texture = assets::find_stem(&terrain_texture_stem(&files, *number), &["res"]).and_then(|p| godot::tools::try_load::<Texture2D>(&p).ok());
        let Some(texture) = texture else {
            return Err(format!("ground texture {number} is not converted for Terrain3D; run --import-rose"));
        };
        look.bind_mut().extra_textures.push(&texture.to_variant());
    }

    let (mut object_count, mut marker_count) = (0, 0);
    let mut zone_root = RoseZoneScene::new_alloc();
    // "Adventurer's Plain" -> AdventurersPlain
    zone_root.set_name(&slug(&files.name.replace('\'', "")).split('_').map(|w| w[..1].to_uppercase() + &w[1..]).collect::<String>());
    zone_root.bind_mut().zone_id = zone_id as i32;
    let mut zone_root = zone_root.upcast::<Node3D>();
    let block_list = blocks(&files);
    for &(block_x, block_y) in &block_list {
        let dir = &files.zone_dir;
        let Some(him) = data::read_file::<HimFile>(&format!("{dir}/{block_x}_{block_y}.HIM")) else { continue };
        let til = data::read_file::<TilFile>(&format!("{dir}/{block_x}_{block_y}.TIL"));
        let ifo = data::read_file::<IfoFile>(&format!("{dir}/{block_x}_{block_y}.IFO"));

        let mut block = RoseBlock::new_alloc();
        block.set_name(&format!("Block_{block_x}_{block_y}"));
        {
            let mut b = block.bind_mut();
            b.block_x = block_x as i32;
            b.block_y = block_y as i32;
        }
        let mut root = block.upcast::<Node3D>();

        ground.add_block(block_x, block_y, &him, |x, y| {
            let [layer1, layer2, rotation] = zone::tile_layers(&files.zon, til.as_ref(), x, y);
            let id = |number: u8| numbers.iter().position(|n| *n == number as usize).unwrap_or(0) as u32;
            control_bits(id(layer1), id(layer2), rotation as u32, 255)
        });
        if let Some(tint) = zone::ground_tint(dir, block_x, block_y) {
            ground.add_tint(block_x, block_y, &tint);
        }
        {
            let mut l = look.bind_mut();
            l.blocks.push(block_x as i32);
            l.blocks.push(block_y as i32);
        }

        if let Some(ifo) = ifo.as_ref() {
            for (i, (start, end)) in ifo.water_planes.iter().enumerate() {
                let mut water = RoseWater::new_alloc();
                water.set_name(&format!("Water{i}"));
                let (start, end) = zone::water_corners(start, end);
                {
                    let mut w = water.bind_mut();
                    w.start = start;
                    w.end = end;
                    w.tile_size = ifo.water_size / 100.0;
                }
                let owner = root.clone();
                add_owned(&mut root, &owner, water.upcast());
            }

            // Buildings, decorations, event objects and warps, as instances of the model scenes.
            let placements: [(&str, usize, Vec<Placement>); 4] = [
                ("Buildings", 0, ifo.cnst_objects.iter().map(|o| (o.object_id as usize, o)).collect()),
                ("Decorations", 1, ifo.deco_objects.iter().map(|o| (o.object_id as usize, o)).collect()),
                ("EventObjects", 2, ifo.event_objects.iter().map(|o| (o.object.object_id as usize, &o.object)).collect()),
                ("Warps", 3, ifo.warps.iter().map(|o| (1, o)).collect()),
            ];
            let names = ["Cnst", "Deco", "Event", "Warp"];
            for (group_name, zsc_index, list) in placements {
                if list.is_empty() {
                    continue;
                }
                let (list_path, Some(_)) = (&files.lists[zsc_index].0, files.lists[zsc_index].1.as_ref()) else { continue };
                let mut parent = group(&mut root, group_name);
                for (ifo_index, (object_id, object)) in list.into_iter().enumerate() {
                    let Some(path) = assets::find_stem(&object_stem(list_path, object_id), &["tscn"]) else { continue };
                    let Some(node) = instance(&path) else { continue };
                    let Ok(mut placed) = node.clone().try_cast::<RoseZoneObject>() else {
                        node.free();
                        continue;
                    };
                    placed.set_name(&format!("{}_{ifo_index}", names[zsc_index]));
                    placed.set_transform(zone::ifo_transform(object));
                    {
                        let mut p = placed.bind_mut();
                        p.warp_id = object.warp_id as i32;
                        p.event_id = object.event_id as i32;
                    }
                    if zsc_index == 2 {
                        if let Some(event) = ifo.event_objects.get(ifo_index) {
                            let mut p = placed.bind_mut();
                            p.quest_trigger = event.quest_trigger_name.as_str().into();
                            p.script_function = event.script_function_name.as_str().into();
                        }
                    }
                    add_owned(&mut parent, &root, placed.upcast());
                    object_count += 1;
                }
            }

            if !ifo.effect_objects.is_empty() {
                let mut parent = group(&mut root, "Effects");
                for (i, effect) in ifo.effect_objects.iter().enumerate() {
                    let mut spot = RoseEffectSpot::new_alloc();
                    spot.set_name(&format!("Effect_{i}"));
                    spot.set_transform(zone::ifo_transform(&effect.object));
                    spot.bind_mut().scene = crate::effect::effect_scene(&effect.effect_path.path().to_string_lossy());
                    add_owned(&mut parent, &root, spot.upcast());
                    marker_count += 1;
                }
            }
            if !ifo.sound_objects.is_empty() {
                let mut parent = group(&mut root, "Sounds");
                for (i, sound) in ifo.sound_objects.iter().enumerate() {
                    let mut spot = RoseSoundSpot::new_alloc();
                    spot.set_name(&format!("Sound_{i}"));
                    spot.set_transform(zone::ifo_transform(&sound.object));
                    {
                        let mut s = spot.bind_mut();
                        s.stream = sound_stream(&sound.sound_path.path().to_string_lossy());
                        s.range = sound.range as f32 / 10.0;
                        s.interval = sound.interval.as_secs_f32();
                    }
                    add_owned(&mut parent, &root, spot.upcast());
                    marker_count += 1;
                }
            }
            if !ifo.monster_spawns.is_empty() {
                let mut parent = group(&mut root, "Spawns");
                for (i, point) in ifo.monster_spawns.iter().enumerate() {
                    let mut spot = RoseSpawnSpot::new_alloc();
                    spot.set_name(&format!("Spawn_{i}"));
                    spot.set_transform(zone::ifo_transform(&point.object));
                    {
                        let mut s = spot.bind_mut();
                        (s.basic_monster_ids, s.basic_counts) = spawn_lists(&point.basic_spawns);
                        (s.tactic_monster_ids, s.tactic_counts) = spawn_lists(&point.tactic_spawns);
                        s.interval = point.interval as i32;
                        s.limit = point.limit_count as i32;
                        s.range = point.range as i32;
                        s.tactic_points = point.tactic_points as i32;
                    }
                    add_owned(&mut parent, &root, spot.upcast());
                    marker_count += 1;
                }
            }
            if !ifo.npcs.is_empty() {
                let mut parent = group(&mut root, "Npcs");
                for (i, npc) in ifo.npcs.iter().enumerate() {
                    let mut spot = RoseNpcSpot::new_alloc();
                    spot.set_name(&format!("Npc_{i}"));
                    spot.set_transform(zone::ifo_transform(&npc.object));
                    {
                        let mut s = spot.bind_mut();
                        s.npc_id = npc.object.object_id as i32;
                        s.ai = npc.ai_id as i32;
                        s.quest_file = npc.quest_file_name.as_str().into();
                    }
                    add_owned(&mut parent, &root, spot.upcast());
                    marker_count += 1;
                }
            }
        }

        // Into the zone scene: what the block owned, the zone root now owns.
        let block = root.upcast::<Node>();
        let owned = owned_by(&block);
        let zone_root_clone = zone_root.clone();
        add_owned(&mut zone_root, &zone_root_clone, block);
        for mut node in owned {
            node.set_owner(&zone_root_clone);
        }
    }

    // The ground: a Terrain3D node with its regions saved in terrain/.
    let terrain = ground.build(&format!("{folder}/terrain"), &files, &numbers)?;
    let owner = zone_root.clone();
    add_owned(&mut zone_root, &owner, terrain.clone().upcast());
    let mut terrain_node = terrain.cast::<Node3D>();
    add_owned(&mut terrain_node, &owner, look.upcast());

    let mut events = group(&mut zone_root, "Events");
    for (i, (name, position)) in files.zon.event_positions.iter().enumerate() {
        let mut spot = RoseZoneEvent::new_alloc();
        let clean: String = name.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == ' ').collect();
        spot.set_name(&format!("Event_{i}_{}", clean.trim().replace(' ', "_")));
        spot.set_position(zone::rose_position(position.x, position.z, position.y));
        spot.bind_mut().event_name = name.as_str().into();
        let owner = zone_root.clone();
        add_owned(&mut events, &owner, spot.upcast());
        marker_count += 1;
    }
    let zone_path = format!("{folder}/zone.tscn");
    if !save_scene(zone_root.upcast(), &zone_path) {
        return Err(format!("could not save {zone_path}"));
    }
    Ok((folder, block_list.len(), object_count, marker_count))
}

// ---------------------------------------------------------------------------------------
// Export for the server

/// What the server needs from a zone's scenes (`export/zones/<id>-<name>.toml`): its
/// spawn points, town NPCs and named spots, with positions in metres in the server's frame
/// (x east, y north, z up). Blocks come in the order the server reads IFO files (north to
/// south rows, west to east), the markers of a block in scene order.
pub fn zone_layout(zone_id: u16) -> Result<rose_data_export::ZoneLayout, String> {
    let folder = zone_folder(zone_id).ok_or(format!("zone {zone_id} has no folder in {ZONES}"))?;
    let path = format!("{folder}/zone.tscn");
    let scene = ResourceLoader::singleton()
        .load(&path)
        .and_then(|r| r.try_cast::<PackedScene>().ok())
        .ok_or(format!("could not open {path}"))?;
    let root = scene.instantiate().ok_or(format!("could not open {path}"))?;
    let mut layout = rose_data_export::ZoneLayout { zone: zone_id, ..Default::default() };
    let mut blocks: Vec<((i32, i32), Gd<Node>)> = Vec::new();
    for child in root.get_children().iter_shared() {
        match block_of(&child) {
            Some((x, y)) => blocks.push(((y, x), child)),
            None => collect_markers(&child, transform_of(&child), &child.get_name().to_string(), &mut layout),
        }
    }
    blocks.sort_by_key(|(key, _)| *key);
    for (_, block) in &blocks {
        collect_markers(block, transform_of(block), &block.get_name().to_string(), &mut layout);
    }
    root.free();
    Ok(layout)
}

fn transform_of(node: &Gd<Node>) -> Transform3D {
    node.clone().try_cast::<Node3D>().map_or(Transform3D::IDENTITY, |n| n.get_transform())
}

/// Metres in the server's frame from a Godot position.
fn server_metres(at: Vector3) -> (f32, f32, f32) {
    (at.x, -at.z, at.y)
}

fn pairs(monsters: &PackedInt32Array, counts: &PackedInt32Array) -> Vec<[u32; 2]> {
    let counts = counts.as_slice();
    monsters.as_slice().iter().enumerate().map(|(i, m)| [*m as u32, counts.get(i).copied().unwrap_or(0).max(0) as u32]).collect()
}

fn collect_markers(node: &Gd<Node>, transform: Transform3D, path: &str, layout: &mut rose_data_export::ZoneLayout) {
    if let Ok(spot) = node.clone().try_cast::<RoseSpawnSpot>() {
        let (x, y, z) = server_metres(transform.origin);
        let s = spot.bind();
        layout.spawn.push(rose_data_export::ZoneSpawn {
            node: path.to_string(),
            x,
            y,
            z,
            interval: s.interval.max(0) as u32,
            limit: s.limit.max(0) as u32,
            range: s.range.max(0) as u32,
            tactic_points: s.tactic_points.max(0) as u32,
            basic: pairs(&s.basic_monster_ids, &s.basic_counts),
            tactic: pairs(&s.tactic_monster_ids, &s.tactic_counts),
        });
    } else if let Ok(spot) = node.clone().try_cast::<RoseNpcSpot>() {
        let (x, y, z) = server_metres(transform.origin);
        // Back to the ROSE rotation (see zone::rose_quat), then the angle the server takes.
        let q = transform.basis.get_quaternion();
        let direction = glam::Quat::from_xyzw(q.x, -q.z, q.y, q.w).to_euler(glam::EulerRot::XYZ).2.to_degrees();
        let s = spot.bind();
        layout.npc.push(rose_data_export::ZoneNpc {
            node: path.to_string(),
            npc: s.npc_id.max(0) as u32,
            x,
            y,
            z,
            direction,
            conversation: s.quest_file.to_string(),
        });
    } else if let Ok(spot) = node.clone().try_cast::<RoseZoneEvent>() {
        let (x, y, z) = server_metres(transform.origin);
        layout.event.push(rose_data_export::ZoneEvent { name: spot.bind().event_name.to_string(), x, y, z });
    }
    for child in node.get_children().iter_shared() {
        let child_path = format!("{path}/{}", child.get_name());
        collect_markers(&child, transform * transform_of(&child), &child_path, layout);
    }
}

/// The export file of a zone folder: `res://zones/20_birth_island` -> `20-birth-island.toml`.
pub fn layout_file_name(folder: &str) -> String {
    format!("{}.toml", folder.rsplit('/').next().unwrap_or(folder).replace('_', "-"))
}

/// The export of a zone as text, with the zone's name and the monster and NPC names (by id,
/// from the resources being exported) in the comments.
pub fn layout_text(
    layout: &rose_data_export::ZoneLayout,
    zone_name: &str,
    names: &std::collections::HashMap<u32, String>,
) -> String {
    let name = |id: u32| names.get(&id).filter(|n| !n.is_empty()).cloned().unwrap_or_else(|| format!("#{id}"));
    let zone_name = if zone_name.is_empty() { format!("Zone {}", layout.zone) } else { zone_name.to_string() };
    layout.to_toml(&zone_name, name)
}
