//! Zones as Godot scenes (step 5 of the Godot-native assets plan).
//!
//! A zone with a folder in `res://zones/` (`20_birth_island/`) loads from its scenes instead
//! of the ROSE map files. `zone.tscn` holds one `block_X_Y.tscn` per 160 m block and the
//! zone's event points (start and revive spots). The ground is the zone's Terrain3D node
//! (its regions in `terrain/`), drawn by `shaders/rose_terrain3d.gdshader` so it looks as
//! before; `RoseTerrainLook` beside it gives that shader the block lightmaps. A block scene holds:
//! - its water planes (`RoseWater`);
//! - its buildings, decorations, event objects and warp gates, each an instance of a model
//!   scene in `assets/rose/` (`RoseZoneObject`) with the lightmap of that placement;
//! - markers for monster spawns, NPCs, ambient sounds and effects, which show a label and
//!   their range (and the NPC or monster) only in the editor.
//!
//! `--import-zone=<id>` writes the scenes once from the ROSE files. After that the scenes
//! are the source and are committed; the model scenes, tile textures and lightmaps they use
//! are made by `--import-rose` like every other converted asset.

use std::collections::{BTreeMap, BTreeSet, HashSet};

use godot::{
    classes::{
        INode,
        base_material_3d::{Flags, ShadingMode, Transparency},
        file_access::ModeFlags,
        mesh::PrimitiveType,
        node::InternalMode,
        packed_scene::GenEditState,
        DirAccess, Engine, FileAccess, INode3D, ImmediateMesh, Label3D, Mesh, MeshInstance3D, Node, Node3D, PackedScene,
        ResourceLoader, ResourceSaver, ShaderMaterial, StandardMaterial3D, Texture2D, Texture2DArray,
    },
    global::Error,
    prelude::*,
};
use rose_file_readers::{
    HimFile, IfoFile, IfoMonsterSpawn, LitFile, TilFile, ZonFile, ZscEffectType, ZscFile,
};

use crate::{
    assets, data, material, texture,
    zone::{self, Ground, ObjectSpawner},
};

pub const ZONES: &str = "res://zones/";
/// Bumped when the model scenes or texture arrays made for zone scenes change.
pub const ZONE_IMPORT_VERSION: i64 = 1;

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
/// not committed; `--import-rose` makes them). Without them the zone loads from the ROSE files.
pub fn zone_scene(zone_id: u16, zone_dir: &str) -> Option<Gd<PackedScene>> {
    let path = format!("{}/zone.tscn", zone_folder(zone_id)?);
    if !ResourceLoader::singleton().exists(&path) {
        return None;
    }
    let first_tile = data::read_file::<ZonFile>(&format!("{zone_dir}/{}.ZON", zone_dir.rsplit('/').next().unwrap_or("")))
        .and_then(|zon| zone::tile_texture_paths(&zon).into_iter().next());
    if first_tile.is_some_and(|path| assets::load_texture(&path).is_none()) {
        godot_warn!("rose: {path} needs the converted assets; run --import-rose. Loading the zone from the client data.");
        return None;
    }
    godot::tools::try_load::<PackedScene>(&path).ok()
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
            for child in object.get_children().iter_shared() {
                if let Ok(part) = child.clone().try_cast::<MeshInstance3D>() {
                    let Some(mesh) = part.get_mesh() else { continue };
                    spawner.parts += 1;
                    if let Some(material) = part.get_material_override() {
                        materials.insert(material.instance_id());
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
        } else if let Ok(effect) = node.clone().try_cast::<RoseEffectSpot>() {
            let transform = to_zone * effect.get_global_transform();
            spawner.effect_spots.push((transform, effect.bind().effect.to_string(), false));
        } else if let Ok(sound) = node.clone().try_cast::<RoseSoundSpot>() {
            let position = (to_zone * sound.get_global_transform()).origin;
            let sound = sound.bind();
            spawner.ambient_sounds.push((position, sound.sound.to_string(), sound.range));
        }
    });
    materials.len()
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

/// Goes with the zone's Terrain3D node (its child): gives the ROSE terrain shader the block
/// lightmaps, points Terrain3D at the camera, and says which ROSE texture each Terrain3D
/// texture is (for footstep sounds).
#[derive(GodotClass)]
#[class(tool, base=Node, init)]
pub struct RoseTerrainLook {
    base: Base<Node>,
    /// The blocks with ground, as x, y pairs.
    #[export]
    pub blocks: PackedInt32Array,
    /// Each block's lightmap, in the order of `blocks`.
    #[export]
    pub lightmaps: VarArray,
    /// The ROSE (ZON) texture number of each Terrain3D texture, then of each extra texture.
    #[export]
    pub texture_numbers: PackedInt32Array,
    /// Ground textures past Terrain3D's 32 (texture 32 onwards), for zones that use more.
    #[export]
    pub extra_textures: VarArray,
    lightmap_array: Option<Gd<Texture2DArray>>,
    extra_array: Option<Gd<Texture2DArray>>,
    lightmap_index: Option<Gd<Texture2D>>,
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
        // The material can be remade (the editor does on shader changes): give the new one the lightmaps.
        let Ok(material) = terrain.get("material").try_to::<Gd<godot::classes::Object>>() else { return };
        let Ok(rid) = material.clone().call("get_material_rid", &[]).try_to::<Rid>() else { return };
        if !rid.is_valid() || self.applied_to == Some(rid) {
            return;
        }
        let (array, index) = self.lightmap_textures();
        let mut server = godot::classes::RenderingServer::singleton();
        // The server takes textures by their RIDs.
        server.material_set_param(rid, "rose_lightmaps", &array.get_rid().to_variant());
        server.material_set_param(rid, "rose_lightmap_index", &index.get_rid().to_variant());
        if !self.extra_textures.is_empty() {
            let extra = self.extra_array.get_or_insert_with(|| {
                texture::texture_array(self.extra_textures.iter_shared().map(|t| t.try_to::<Gd<Texture2D>>().ok().and_then(|t| t.get_image())).collect())
            });
            server.material_set_param(rid, "rose_extra_tiles", &extra.get_rid().to_variant());
        }
        self.applied_to = Some(rid);
    }
}

impl RoseTerrainLook {
    fn lightmap_textures(&mut self) -> (Gd<Texture2DArray>, Gd<Texture2D>) {
        if let (Some(array), Some(index)) = (&self.lightmap_array, &self.lightmap_index) {
            return (array.clone(), index.clone());
        }
        let images = self.lightmaps.iter_shared().map(|t| t.try_to::<Gd<Texture2D>>().ok().and_then(|t| t.get_image())).collect();
        let array = texture::texture_array(images);
        let mut grid = vec![0u8; 64 * 64];
        for (i, pair) in self.blocks.as_slice().chunks(2).enumerate() {
            if let [x, y] = pair {
                if (0..64).contains(x) && (0..64).contains(y) && i < 255 {
                    grid[(*x + *y * 64) as usize] = i as u8 + 1;
                }
            }
        }
        let image = godot::classes::Image::create_from_data(64, 64, false, godot::classes::image::Format::R8, &PackedByteArray::from(grid.as_slice()))
            .expect("64 x 64 R8 image");
        let index = godot::classes::ImageTexture::create_from_image(&image).expect("lightmap index texture").upcast::<Texture2D>();
        self.lightmap_array = Some(array.clone());
        self.lightmap_index = Some(index.clone());
        (array, index)
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

/// A water plane: its two corners in zone space (it lies at the first one's height) and
/// how often its animated texture repeats.
#[derive(GodotClass)]
#[class(tool, base=Node3D, init)]
pub struct RoseWater {
    base: Base<Node3D>,
    #[export]
    pub start: Vector3,
    #[export]
    pub end: Vector3,
    /// Metres per repeat of the texture.
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

/// A building, decoration, event object or warp gate: the root of a model scene in
/// `assets/rose/` (one per model in a ROSE model list), placed in a block scene. Its parts
/// are the `PartN` mesh children; `EffectN` children are the model's own effects.
#[derive(GodotClass)]
#[class(tool, base=Node3D, init)]
pub struct RoseZoneObject {
    base: Base<Node3D>,
    /// The model list: "cnst" (buildings), "deco" (decorations), "event" or "warp".
    #[export]
    pub list: GString,
    /// The lightmap of each part at this spot (by part number; empty for none).
    #[export]
    pub lightmaps: VarArray,
    /// Where each part's lightmap sits in its texture: (column, row, 1 / parts per row).
    #[export]
    pub lightmap_cells: PackedVector3Array,
    /// The WARP.STB row of a warp gate.
    #[export]
    pub warp_id: i32,
    /// An event object's event number, quest trigger and script function.
    #[export]
    pub event_id: i32,
    #[export]
    pub quest_trigger: GString,
    #[export]
    pub script_function: GString,
}

#[godot_api]
impl INode3D for RoseZoneObject {
    fn ready(&mut self) {
        let cells = self.lightmap_cells.as_slice().to_vec();
        for (i, lightmap) in self.lightmaps.iter_shared().enumerate() {
            let Ok(lightmap) = lightmap.try_to::<Gd<Texture2D>>() else { continue };
            let Some(mut part) = self.base().get_node_or_null(&format!("Part{i}")).and_then(|n| n.try_cast::<MeshInstance3D>().ok()) else {
                continue;
            };
            let Some(base_material) = part.get_material_override().and_then(|m| m.try_cast::<ShaderMaterial>().ok()) else { continue };
            part.set_material_override(&material::with_lightmap(&base_material, &lightmap));
            let cell = cells.get(i).copied().unwrap_or(Vector3::new(0.0, 0.0, 1.0));
            part.set_instance_shader_parameter("lightmap_uv_offset", &Vector2::new(cell.x, cell.y).to_variant());
            part.set_instance_shader_parameter("lightmap_uv_scale", &cell.z.to_variant());
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

/// Redraws a marker's editor look when what it shows changed (checked each editor frame).
macro_rules! marker_class {
    ($name:ident, $doc:literal, { $($(#[$fattr:meta])* $field:ident : $ty:ty $(= $init:expr)?),* $(,)? }, |$this:ident| $look:expr) => {
        #[doc = $doc]
        #[derive(GodotClass)]
        #[class(tool, base=Node3D, init)]
        pub struct $name {
            base: Base<Node3D>,
            $(
                $(#[$fattr])*
                #[export]
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

fn npc_model(npc_id: i32) -> Option<String> {
    (npc_id > 0).then(|| assets::find_stem(&crate::character::npc_scene_stem(npc_id as u16), &["tscn"])).flatten()
}

fn spawn_label(monsters: &PackedInt32Array, counts: &PackedInt32Array) -> String {
    let counts = counts.as_slice();
    monsters.as_slice().iter().enumerate().map(|(i, id)| format!("{}x {id}", counts.get(i).copied().unwrap_or(0))).collect::<Vec<_>>().join(", ")
}

marker_class!(RoseSpawnSpot, "A monster spawn point (an IFO spawn): which monsters, how many, how often and how far around it.", {
    /// Monster numbers (LIST_NPC rows) and how many of each, for the basic spawns and the tactic spawns.
    basic_monsters: PackedInt32Array,
    basic_counts: PackedInt32Array,
    tactic_monsters: PackedInt32Array,
    tactic_counts: PackedInt32Array,
    /// Seconds between spawn checks.
    interval: i32 = 60,
    /// Monsters alive at most.
    limit: i32 = 5,
    /// Metres around the point that monsters appear in.
    range: i32 = 10,
    tactic_points: i32 = 100,
}, |this| (
    format!("Spawn: {}\nevery {} s, max {}", spawn_label(&this.basic_monsters, &this.basic_counts), this.interval, this.limit),
    Color::from_rgb(1.0, 0.35, 0.3),
    this.range as f32,
    this.basic_monsters.as_slice().first().and_then(|id| npc_model(*id)),
));

marker_class!(RoseNpcSpot, "An NPC standing here, facing the node's forward direction.", {
    /// The NPC number (LIST_NPC row).
    npc: i32,
    /// Its AI script number.
    ai: i32,
    /// Its conversation file.
    quest_file: GString,
}, |this| (format!("NPC {}\n{}", this.npc, this.quest_file), Color::from_rgb(0.4, 0.8, 1.0), 0.0, npc_model(this.npc)));

marker_class!(RoseSoundSpot, "A looping ambient sound and the metres it carries.", {
    sound: GString,
    range: f32 = 10.0,
    /// Seconds between plays.
    interval: f32,
}, |this| (format!("Sound\n{}", this.sound.to_string().rsplit(['/', '\\']).next().unwrap_or("")), Color::from_rgb(0.6, 1.0, 0.5), this.range, None));

marker_class!(RoseEffectSpot, "An effect (EFT file) playing here.", {
    effect: GString,
}, |this| (format!("Effect\n{}", this.effect.to_string().rsplit(['/', '\\']).next().unwrap_or("")), Color::from_rgb(1.0, 0.85, 0.3), 0.0, None));

marker_class!(RoseZoneEvent, "A named spot of the zone (ZON event position), such as the start or revive point.", {
    event_name: GString,
}, |this| (format!("Event: {}", this.event_name), Color::from_rgb(0.9, 0.5, 1.0), 1.0, None));

// ---------------------------------------------------------------------------------------
// Import

#[derive(Default)]
pub struct ZoneImport {
    pub saved: i64,
    pub kept: i64,
    pub failed: i64,
    pub bytes: u64,
}

impl ZoneImport {
    fn save(&mut self, redo: bool, stem: &str, extension: &str, make: impl FnOnce() -> Option<Gd<godot::classes::Resource>>) {
        let target = format!("{}{stem}.{extension}", assets::ROSE);
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
        let material = materials.entry(part.material_id).or_insert_with(|| material::object_material(zsc_material, None)).clone();
        let mut instance = MeshInstance3D::new_alloc();
        instance.set_name(&format!("Part{part_index}"));
        instance.set_mesh(&mesh);
        instance.set_material_override(&material);
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
/// textures in the form Terrain3D takes. Lightmaps are converted by the texture import.
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

fn terrain_texture_stem(files: &ZoneFiles, number: usize) -> String {
    format!("{}/terrain_textures/{number}", assets::stem(&files.zone_dir))
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

/// Collects the blocks' heights and tiles into Terrain3D regions.
#[derive(Default)]
struct GroundBuilder {
    regions: BTreeMap<(i64, i64), RegionData>,
}

impl GroundBuilder {
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
            .and_then(|t| t.get_root())
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

/// A model placed in a block: its number in the list, the IFO entry and its lightmaps.
type Placement<'a> = (usize, &'a rose_file_readers::IfoObject, Option<&'a rose_file_readers::LitObject>);

fn spawn_lists(spawns: &[IfoMonsterSpawn]) -> (PackedInt32Array, PackedInt32Array) {
    (spawns.iter().map(|s| s.id as i32).collect(), spawns.iter().map(|s| s.count as i32).collect())
}

/// Writes a zone's scenes (`res://zones/<id>_<name>/zone.tscn` and a `block_X_Y.tscn` per
/// block) from the ROSE files. Refuses to replace scenes already there unless `force`.
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
    let mut zone_root = Node3D::new_alloc();
    // "Adventurer's Plain" -> AdventurersPlain
    zone_root.set_name(&slug(&files.name.replace('\'', "")).split('_').map(|w| w[..1].to_uppercase() + &w[1..]).collect::<String>());
    zone_root.set_meta("zone_id", &(zone_id as i64).to_variant());
    let block_list = blocks(&files);
    for &(block_x, block_y) in &block_list {
        let dir = &files.zone_dir;
        let Some(him) = data::read_file::<HimFile>(&format!("{dir}/{block_x}_{block_y}.HIM")) else { continue };
        let til = data::read_file::<TilFile>(&format!("{dir}/{block_x}_{block_y}.TIL"));
        let ifo = data::read_file::<IfoFile>(&format!("{dir}/{block_x}_{block_y}.IFO"));
        let lightmap_dir = format!("{dir}/{block_x}_{block_y}/LIGHTMAP/");
        let lit_cnst = data::read_file::<LitFile>(&format!("{lightmap_dir}BUILDINGLIGHTMAPDATA.LIT"));
        let lit_deco = data::read_file::<LitFile>(&format!("{lightmap_dir}OBJECTLIGHTMAPDATA.LIT"));

        let mut root = Node3D::new_alloc();
        root.set_name(&format!("Block_{block_x}_{block_y}"));

        ground.add_block(block_x, block_y, &him, |x, y| {
            let [layer1, layer2, rotation] = zone::tile_layers(&files.zon, til.as_ref(), x, y);
            let id = |number: u8| numbers.iter().position(|n| *n == number as usize).unwrap_or(0) as u32;
            control_bits(id(layer1), id(layer2), rotation as u32, 255)
        });
        {
            let mut l = look.bind_mut();
            l.blocks.push(block_x as i32);
            l.blocks.push(block_y as i32);
            let lightmap = texture::load_texture(&zone::block_lightmap_path(dir, block_x, block_y)).filter(|t| t.get_path().begins_with("res://"));
            l.lightmaps.push(&lightmap.map_or(Variant::nil(), |t| t.to_variant()));
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
                ("Buildings", 0, ifo.cnst_objects.iter().enumerate().map(|(i, o)| (o.object_id as usize, o, zone::find_lit(lit_cnst.as_ref(), i))).collect()),
                ("Decorations", 1, ifo.deco_objects.iter().enumerate().map(|(i, o)| (o.object_id as usize, o, zone::find_lit(lit_deco.as_ref(), i))).collect()),
                ("EventObjects", 2, ifo.event_objects.iter().map(|o| (o.object.object_id as usize, &o.object, None)).collect()),
                ("Warps", 3, ifo.warps.iter().map(|o| (1, o, None)).collect()),
            ];
            let names = ["Cnst", "Deco", "Event", "Warp"];
            for (group_name, zsc_index, list) in placements {
                if list.is_empty() {
                    continue;
                }
                let (list_path, Some(zsc)) = (&files.lists[zsc_index].0, files.lists[zsc_index].1.as_ref()) else { continue };
                let mut parent = group(&mut root, group_name);
                for (ifo_index, (object_id, object, lit)) in list.into_iter().enumerate() {
                    let Some(path) = assets::find_stem(&object_stem(list_path, object_id), &["tscn"]) else { continue };
                    let Some(node) = instance(&path) else { continue };
                    let Ok(mut placed) = node.clone().try_cast::<RoseZoneObject>() else {
                        node.free();
                        continue;
                    };
                    placed.set_name(&format!("{}_{ifo_index}", names[zsc_index]));
                    placed.set_transform(zone::ifo_transform(object));
                    let mut lightmaps = VarArray::new();
                    let mut cells = PackedVector3Array::new();
                    if let (Some(lit_object), Some(zsc_object)) = (lit, zsc.objects.get(object_id)) {
                        for part_index in 0..zsc_object.parts.len() {
                            let lit_part = lit_object
                                .parts
                                .iter()
                                .find(|p| p.object_part_index as usize == part_index)
                                .or_else(|| lit_object.parts.get(part_index));
                            let texture = lit_part
                                .and_then(|p| texture::load_texture(&format!("{lightmap_dir}{}", p.filename)))
                                .filter(|t| t.get_path().begins_with("res://"));
                            let cell = lit_part.map_or(Vector3::new(0.0, 0.0, 1.0), |p| {
                                let per_row = p.parts_per_row.max(1);
                                Vector3::new((p.part_index % per_row) as f32, (p.part_index / per_row) as f32, 1.0 / per_row as f32)
                            });
                            lightmaps.push(&texture.map_or(Variant::nil(), |t| t.to_variant()));
                            cells.push(cell);
                        }
                    }
                    {
                        let mut p = placed.bind_mut();
                        p.lightmaps = lightmaps;
                        p.lightmap_cells = cells;
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
                    spot.bind_mut().effect = effect.effect_path.path().to_string_lossy().to_string().as_str().into();
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
                        s.sound = sound.sound_path.path().to_string_lossy().to_string().as_str().into();
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
                        (s.basic_monsters, s.basic_counts) = spawn_lists(&point.basic_spawns);
                        (s.tactic_monsters, s.tactic_counts) = spawn_lists(&point.tactic_spawns);
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
                        s.npc = npc.object.object_id as i32;
                        s.ai = npc.ai_id as i32;
                        s.quest_file = npc.quest_file_name.as_str().into();
                    }
                    add_owned(&mut parent, &root, spot.upcast());
                    marker_count += 1;
                }
            }
        }

        let block_path = format!("{folder}/block_{block_x}_{block_y}.tscn");
        if !save_scene(root.upcast(), &block_path) {
            return Err(format!("could not save {block_path}"));
        }
        let Some(block) = instance(&block_path) else { return Err(format!("could not open {block_path}")) };
        let zone_root_clone = zone_root.clone();
        add_owned(&mut zone_root, &zone_root_clone, block);
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

/// What the server needs from a zone's scenes (`overrides/zones/<id>-<name>.toml`): its
/// spawn points, town NPCs and named spots, with positions in metres in the server's frame
/// (x east, y north, z up). Blocks come in the order the server reads IFO files (north to
/// south rows, west to east), the markers of a block in scene order.
pub fn zone_layout(zone_id: u16) -> Result<rose_game_overrides::ZoneLayout, String> {
    let folder = zone_folder(zone_id).ok_or(format!("zone {zone_id} has no folder in {ZONES}"))?;
    let path = format!("{folder}/zone.tscn");
    let scene = ResourceLoader::singleton()
        .load(&path)
        .and_then(|r| r.try_cast::<PackedScene>().ok())
        .ok_or(format!("could not open {path}"))?;
    let root = scene.instantiate().ok_or(format!("could not open {path}"))?;
    let mut layout = rose_game_overrides::ZoneLayout { zone: zone_id, ..Default::default() };
    let mut blocks: Vec<((i32, i32), Gd<Node>)> = Vec::new();
    for child in root.get_children().iter_shared() {
        let name = child.get_name().to_string();
        let block = name.strip_prefix("Block_").and_then(|xy| xy.split_once('_')).and_then(|(x, y)| Some((y.parse().ok()?, x.parse().ok()?)));
        match block {
            Some(key) => blocks.push((key, child)),
            None => collect_markers(&child, transform_of(&child), &name, &mut layout),
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

fn collect_markers(node: &Gd<Node>, transform: Transform3D, path: &str, layout: &mut rose_game_overrides::ZoneLayout) {
    if let Ok(spot) = node.clone().try_cast::<RoseSpawnSpot>() {
        let (x, y, z) = server_metres(transform.origin);
        let s = spot.bind();
        layout.spawn.push(rose_game_overrides::ZoneSpawn {
            node: path.to_string(),
            x,
            y,
            z,
            interval: s.interval.max(0) as u32,
            limit: s.limit.max(0) as u32,
            range: s.range.max(0) as u32,
            tactic_points: s.tactic_points.max(0) as u32,
            basic: pairs(&s.basic_monsters, &s.basic_counts),
            tactic: pairs(&s.tactic_monsters, &s.tactic_counts),
        });
    } else if let Ok(spot) = node.clone().try_cast::<RoseNpcSpot>() {
        let (x, y, z) = server_metres(transform.origin);
        // Back to the ROSE rotation (see zone::rose_quat), then the angle the server takes.
        let q = transform.basis.get_quaternion();
        let direction = glam::Quat::from_xyzw(q.x, -q.z, q.y, q.w).to_euler(glam::EulerRot::XYZ).2.to_degrees();
        let s = spot.bind();
        layout.npc.push(rose_game_overrides::ZoneNpc {
            node: path.to_string(),
            npc: s.npc.max(0) as u32,
            x,
            y,
            z,
            direction,
            conversation: s.quest_file.to_string(),
        });
    } else if let Ok(spot) = node.clone().try_cast::<RoseZoneEvent>() {
        let (x, y, z) = server_metres(transform.origin);
        layout.event.push(rose_game_overrides::ZoneEvent { name: spot.bind().event_name.to_string(), x, y, z });
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

/// The export of a zone as text, with monster and NPC names in the comments.
pub fn layout_text(layout: &rose_game_overrides::ZoneLayout) -> String {
    let game = data::get();
    let name = |id: u32| {
        game.as_ref()
            .and_then(|g| rose_data::NpcId::new(id as u16).and_then(|n| g.npcs.get_npc(n)).map(|n| n.name.to_string()))
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| format!("#{id}"))
    };
    let zone_name = game
        .and_then(|g| rose_data::ZoneId::new(layout.zone).and_then(|id| g.zone_list.get_zone(id)).map(|z| z.name.to_string()))
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| format!("Zone {}", layout.zone));
    layout.to_toml(&zone_name, name)
}
