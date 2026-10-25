//! Builds a whole zone (terrain, water, objects, sky) as Godot nodes.
//! Ported from rose-offline-client's zone_loader.rs.

use std::{collections::HashMap, path::Path, time::Instant};

use godot::{
    classes::{
        mesh::{ArrayCustomFormat, ArrayFormat, ArrayType, PrimitiveType},
        geometry_instance_3d::ShadowCastingSetting,
        ArrayMesh, CollisionShape3D, ConcavePolygonShape3D, GeometryInstance3D, Image, ImageTexture, Mesh, MeshInstance3D, Node3D,
        Shader, ShaderMaterial, StaticBody3D,
    },
    prelude::*,
};
use rose_data::ZoneId;
use rose_file_readers::{HimFile, IfoFile, IfoObject, LitFile, TilFile, ZonFile, ZonTileRotation, ZscCollisionFlags, ZscEffectType, ZscFile};

use crate::{data, effect::RoseEffect, material, mesh, sun, texture};

const BLOCK_SIZE: f32 = 160.0;
/// Zone objects are stored relative to the centre of the 64x64 block grid.
const OBJECT_OFFSET: f32 = 5200.0;
/// Physics layer of zone objects that block walking (walls, trees, rocks).
pub const LAYER_WALLS: u32 = 1;
/// Physics layer of zone object surfaces characters can stand on (bridges, stairs, floors).
pub const LAYER_FLOORS: u32 = 2;
/// Model lists every zone shares: event objects (zsc index 2) and warp gates (3).
pub const EVENT_ZSC: &str = "3DDATA/SPECIAL/EVENT_OBJECT.ZSC";
pub const SPECIAL_ZSC: &str = "3DDATA/SPECIAL/LIST_DECO_SPECIAL.ZSC";
pub const TERRAIN_SHADER: &str = "res://shaders/rose_terrain.gdshader";
pub const WATER_SHADER: &str = "res://shaders/rose_water.gdshader";

/// The zone's terrain tile textures, in ZON order.
pub fn tile_texture_paths(zon: &ZonFile) -> Vec<String> {
    zon.tile_textures.iter().take_while(|path| *path != "end").cloned().collect()
}

/// The 25 frames of the animated water.
pub fn water_texture_paths() -> Vec<String> {
    (1..=25).map(|i| format!("3DDATA/JUNON/WATER/OCEAN01_{i:02}.DDS")).collect()
}

pub fn block_lightmap_path(zone_dir: &str, block_x: usize, block_y: usize) -> String {
    format!("{zone_dir}/{block_x}_{block_y}/{block_x}_{block_y}_PLANELIGHTINGMAP.DDS")
}

/// Cells per side of a block's ground tint (2.5 m each, one per ground point).
pub const TINT_CELLS: usize = 64;

/// How much of a lightmap's darkness the tint keeps: down to 85 % where the light has the
/// block's usual colour (a plain shadow, which the sun casts now), down to 40 % where its
/// colour differs from it (water, tinted spots).
const TINT_FLOOR: f32 = 0.85;
const TINT_FLOOR_COLOURED: f32 = 0.4;

/// A light colour (ROSE's x2 lightmap value) without most of its brightness: kept down to
/// `floor`, then scaled so its brightest channel is at most 1.
pub fn tint_colour(light: [f32; 3], floor: f32) -> [f32; 3] {
    let brightness = (light[0] * 0.299 + light[1] * 0.587 + light[2] * 0.114).max(0.05);
    let colour = light.map(|v| v / brightness * brightness.clamp(floor, 1.0));
    let brightest = colour.iter().copied().fold(1.0f32, f32::max);
    colour.map(|v| v / brightest)
}

/// The colour a block's ROSE ground lightmap gave the ground, without its shadows: the
/// lightmap averaged over 2.5 m cells, its brightness mostly taken out (see `TINT_FLOOR`)
/// and scaled to fit 0..1. Birth Island's blue sea floor and Zant's canal are a blue
/// lightmap on sand. `TINT_CELLS` x `TINT_CELLS` colours, row by row from the north-west
/// corner, in the ROSE (gamma) colour space; None without a lightmap.
pub fn ground_tint(zone_dir: &str, block_x: usize, block_y: usize) -> Option<Vec<Color>> {
    let mut image = texture::load_rose_image(&block_lightmap_path(zone_dir, block_x, block_y))?;
    if image.is_compressed() {
        image.decompress();
    }
    image.convert(godot::classes::image::Format::RGB8);
    let (width, height) = (image.get_width().max(1) as usize, image.get_height().max(1) as usize);
    let bytes = image.get_data();
    let bytes = bytes.as_slice();
    // The block's usual light colour, brightness taken out.
    let usual = {
        let mut sum = [0.0f64; 3];
        for pixel in bytes.chunks_exact(3) {
            for (total, value) in sum.iter_mut().zip(pixel) {
                *total += *value as f64;
            }
        }
        let sum = sum.map(|v| v as f32);
        let brightness = (sum[0] * 0.299 + sum[1] * 0.587 + sum[2] * 0.114).max(1.0);
        sum.map(|v| v / brightness)
    };
    let mut tint = Vec::with_capacity(TINT_CELLS * TINT_CELLS);
    for cy in 0..TINT_CELLS {
        for cx in 0..TINT_CELLS {
            let (x0, x1) = (cx * width / TINT_CELLS, ((cx + 1) * width / TINT_CELLS).max(cx * width / TINT_CELLS + 1));
            let (y0, y1) = (cy * height / TINT_CELLS, ((cy + 1) * height / TINT_CELLS).max(cy * height / TINT_CELLS + 1));
            let mut sum = [0.0f32; 3];
            for y in y0..y1.min(height) {
                for x in x0..x1.min(width) {
                    let i = (y * width + x) * 3;
                    for (c, total) in sum.iter_mut().enumerate() {
                        *total += bytes.get(i + c).copied().unwrap_or(128) as f32;
                    }
                }
            }
            let count = ((y1.min(height) - y0) * (x1.min(width) - x0)).max(1) as f32;
            // ROSE draws the ground times twice the lightmap.
            let light = sum.map(|v| v / count / 255.0 * 2.0);
            let brightness = (light[0] * 0.299 + light[1] * 0.587 + light[2] * 0.114).max(0.05);
            let difference = (0..3).map(|c| (light[c] / brightness - usual[c]).abs()).fold(0.0f32, f32::max);
            let coloured = (difference * 2.5).clamp(0.0, 1.0);
            let colour = tint_colour(light, TINT_FLOOR + (TINT_FLOOR_COLOURED - TINT_FLOOR) * coloured);
            tint.push(Color::from_rgb(colour[0], colour[1], colour[2]));
        }
    }
    Some(tint)
}

/// The tint at a point of the block (metres from its north-west corner), between the cell centres.
pub fn tint_at(tint: &[Color], x: f32, z: f32) -> Color {
    let cell = 160.0 / TINT_CELLS as f32;
    let last = (TINT_CELLS - 1) as f32;
    let (u, v) = ((x / cell - 0.5).clamp(0.0, last), (z / cell - 0.5).clamp(0.0, last));
    let (i, j) = (u.floor() as usize, v.floor() as usize);
    let (i1, j1) = ((i + 1).min(TINT_CELLS - 1), (j + 1).min(TINT_CELLS - 1));
    let (fu, fv) = (u.fract(), v.fract());
    let at = |i: usize, j: usize| tint.get(j * TINT_CELLS + i).copied().unwrap_or(Color::WHITE);
    let top = at(i, j).lerp(at(i1, j), fu as f64);
    let bottom = at(i, j1).lerp(at(i1, j1), fu as f64);
    top.lerp(bottom, fv as f64)
}

/// A model part's lightmap, read once from the client data (RGB8).
pub struct LightmapImage {
    width: usize,
    height: usize,
    rgb: Vec<u8>,
}

fn load_lightmap_image(path: &str) -> Option<LightmapImage> {
    let mut image = texture::load_rose_image(path)?;
    if image.is_compressed() {
        image.decompress();
    }
    image.clear_mipmaps();
    image.convert(godot::classes::image::Format::RGB8);
    let (width, height) = (image.get_width() as usize, image.get_height() as usize);
    (width > 0 && height > 0).then(|| LightmapImage { width, height, rgb: image.get_data().to_vec() })
}

/// The colour a model part's old lightmap gave it, without its shadows: the lightmap at the
/// part's vertices (its second UVs, in cell `cell` of a lightmap holding `per_row` x `per_row`
/// parts), averaged, with most of its brightness taken out like the ground's. What ROSE's
/// twice-the-lightmap did to a mid-bright texture is clipped first, so parts the lightmap
/// washed out to near white stay light. White for a part in deep shadow (no colour to read)
/// or without second UVs.
fn part_tint(lightmap: &LightmapImage, mesh: &Gd<Mesh>, per_row: u32, cell: u32) -> Color {
    let per_row = per_row.max(1);
    let (offset_x, offset_y, scale) = ((cell % per_row) as f32, (cell / per_row) as f32, 1.0 / per_row as f32);
    let mut sum = [0.0f32; 3];
    let mut count = 0.0f32;
    for surface in 0..mesh.get_surface_count() {
        let arrays = mesh.surface_get_arrays(surface);
        let Ok(uv2) = arrays.at(ArrayType::TEX_UV2.ord() as usize).try_to::<PackedVector2Array>() else { continue };
        for uv in uv2.as_slice() {
            let u = ((uv.x + offset_x) * scale).rem_euclid(1.0);
            let v = ((uv.y + offset_y) * scale).rem_euclid(1.0);
            let x = ((u * lightmap.width as f32) as usize).min(lightmap.width - 1);
            let y = ((v * lightmap.height as f32) as usize).min(lightmap.height - 1);
            let i = (y * lightmap.width + x) * 3;
            for (c, total) in sum.iter_mut().enumerate() {
                *total += lightmap.rgb.get(i + c).copied().unwrap_or(128) as f32 / 255.0 * 2.0;
            }
            count += 1.0;
        }
    }
    let light = sum.map(|v| v / count.max(1.0));
    if count == 0.0 || light[0] * 0.299 + light[1] * 0.587 + light[2] * 0.114 < 0.15 {
        return Color::WHITE;
    }
    let clipped = light.map(|v| (v * 0.7).min(1.0) / 0.7);
    let [r, g, b] = tint_colour(clipped, TINT_FLOOR);
    // Three decimals are plenty for a colour, and keep the scene lines short.
    let round = |v: f32| (v * 1000.0).round() / 1000.0;
    Color::from_rgb(round(r), round(g), round(b))
}

/// The colours placed buildings and decorations had from their ROSE lightmaps (the warm light
/// on walls and rocks), read from the client data: the LIT files say which lightmap and cell
/// each part used. Each file is read once.
#[derive(Default)]
pub struct PartTints {
    lits: HashMap<String, Option<LitFile>>,
    lightmaps: HashMap<String, Option<LightmapImage>>,
}

impl PartTints {
    /// One colour per part number of a placed model (building: `zsc_index` 0, decoration: 1;
    /// `ifo_index` its place in the block's IFO list), from `parts` (part number, mesh). White
    /// for a part without a lightmap; empty when no part has a colour.
    pub fn object(&mut self, zone_dir: &str, block_x: usize, block_y: usize, zsc_index: usize, ifo_index: usize, parts: &[(usize, Gd<Mesh>)]) -> PackedColorArray {
        let lit_name = match zsc_index {
            0 => "BUILDINGLIGHTMAPDATA.LIT",
            1 => "OBJECTLIGHTMAPDATA.LIT",
            _ => return PackedColorArray::new(),
        };
        let lightmap_dir = format!("{zone_dir}/{block_x}_{block_y}/LIGHTMAP/");
        let lit_path = format!("{lightmap_dir}{lit_name}");
        let Some(lit) = self.lits.entry(lit_path.clone()).or_insert_with(|| data::read_file::<LitFile>(&lit_path)).as_ref() else {
            return PackedColorArray::new();
        };
        let Some(lit_object) = lit.objects.iter().find(|o| o.id as usize == ifo_index + 1) else { return PackedColorArray::new() };
        let mut tints = vec![Color::WHITE; parts.iter().map(|(n, _)| n + 1).max().unwrap_or(0)];
        for (part_index, mesh) in parts {
            let lit_part = lit_object.parts.iter().find(|p| p.object_part_index as usize == *part_index).or_else(|| lit_object.parts.get(*part_index));
            let Some(lit_part) = lit_part else { continue };
            let path = format!("{lightmap_dir}{}", lit_part.filename);
            let Some(lightmap) = self.lightmaps.entry(path.clone()).or_insert_with(|| load_lightmap_image(&path)).as_ref() else { continue };
            tints[*part_index] = part_tint(lightmap, mesh, lit_part.parts_per_row, lit_part.part_index);
        }
        if tints.iter().all(|c| *c == Color::WHITE) {
            return PackedColorArray::new();
        }
        tints.into_iter().collect()
    }
}

/// Sets the colour of a model part (`rose_tint` in `rose_object.gdshaderinc`, white = none).
/// A per-instance value: the part keeps sharing its material.
pub fn set_part_tint(part: &mut Gd<GeometryInstance3D>, colour: Color) {
    part.set_instance_shader_parameter("rose_tint", &Vector3::new(colour.r, colour.g, colour.b).to_variant());
}

/// An IFO water plane's corners in zone space.
pub fn water_corners(start: &rose_file_readers::types::Vec3<f32>, end: &rose_file_readers::types::Vec3<f32>) -> (Vector3, Vector3) {
    (rose_position(start.x, start.z, start.y), rose_position(end.x, end.z, start.y))
}

#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct RoseZone {
    base: Base<Node3D>,
    /// Heights and footstep tiles per block (64 x 64, most empty).
    grounds: Vec<Option<Ground>>,
    /// Looping ambient sounds (IFO sound objects): position, file, range in metres.
    ambient_sounds: Vec<(Vector3, String, f32)>,
    /// Effects placed in the zone (IFO effect objects, ZSC object effects): transform in
    /// zone space, EFT file, and whether it only shows at night.
    effect_spots: Vec<(Transform3D, String, bool)>,
    zone_id: u16,
    stats: VarDictionary,
    /// Warp gates: WARP.STB id and the box their model fills, in zone space.
    warps: Vec<(u16, Aabb)>,
    /// Build from the ROSE map files even when the zone has scenes (for comparing the two).
    #[var]
    files_only: bool,
    /// What casts shadows, switched by `update_shadows`.
    shadow_casters: Vec<ShadowCaster>,
    /// The block (x, z in 160 m steps) `update_shadows` last set the shadows for.
    shadow_block: Option<(i32, i32)>,
}

/// How far from the player's block small decorations still cast shadows, in metres.
const SMALL_SHADOW_REACH: f32 = 40.0;

/// One block's ground, for height and footstep lookups.
pub struct Ground {
    /// Height samples per side (65 for a ROSE block).
    pub size: i32,
    /// Heights in metres, row by row from the block's north-west corner.
    pub heights: Vec<f32>,
    /// Each 10 m tile's texture number (16 x 16), which picks the footstep sound.
    pub tile_textures: Vec<i32>,
}

impl Ground {
    pub fn height(&self, x: i32, y: i32) -> f32 {
        let size = self.size.max(1);
        self.heights.get((y.clamp(0, size - 1) * size + x.clamp(0, size - 1)) as usize).copied().unwrap_or(0.0)
    }

    fn from_files(zon: &ZonFile, him: &HimFile, til: Option<&TilFile>) -> Ground {
        Ground {
            size: him.width as i32,
            heights: (0..him.height as i32)
                .flat_map(|y| (0..him.width as i32).map(move |x| (x, y)))
                .map(|(x, y)| him.get_clamped(x, y) / 100.0)
                .collect(),
            tile_textures: (0..16usize)
                .flat_map(|y| (0..16usize).map(move |x| (x, y)))
                .map(|(x, y)| {
                    let tile = til.map(|til| til.get_clamped(x, y) as usize).unwrap_or(0);
                    zon.tiles.get(tile).map_or(0, |t| (t.layer2 + t.offset2) as i32)
                })
                .collect(),
        }
    }
}

/// A ZON tile's two texture layers and rotation, as the terrain shader takes them.
pub fn tile_layers(zon: &ZonFile, til: Option<&TilFile>, tile_x: usize, tile_y: usize) -> [u8; 3] {
    let tile_index = til.map(|til| til.get_clamped(tile_x, tile_y) as usize).unwrap_or(0);
    let tile = &zon.tiles[tile_index.min(zon.tiles.len() - 1)];
    let rotation: u8 = match tile.rotation {
        ZonTileRotation::FlipHorizontal => 2,
        ZonTileRotation::FlipVertical => 3,
        ZonTileRotation::Flip => 4,
        ZonTileRotation::Clockwise90 => 5,
        ZonTileRotation::CounterClockwise90 => 6,
        _ => 0,
    };
    [(tile.layer1 + tile.offset1).min(255) as u8, (tile.layer2 + tile.offset2).min(255) as u8, rotation]
}

/// Where a block's ground sits: its north-west corner in zone space.
pub fn block_origin(block_x: usize, block_y: usize) -> Vector3 {
    Vector3::new(BLOCK_SIZE * block_x as f32, 0.0, -BLOCK_SIZE * (65.0 - block_y as f32))
}

/// A ROSE map position (centimetres, z up, relative to the zone's centre) in zone space.
pub fn rose_position(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3::new(OBJECT_OFFSET + x / 100.0, z / 100.0, -(OBJECT_OFFSET + y / 100.0))
}

pub fn rose_quat(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
    let q = Quaternion::new(x, z, -y, w);
    if q.length_squared() > 0.0 { q.normalized() } else { Quaternion::IDENTITY }
}

pub fn rose_transform(position: Vector3, rotation: Quaternion, scale: Vector3) -> Transform3D {
    Transform3D::new(Basis::from_quaternion(rotation) * Basis::from_scale(scale), position)
}

pub fn ifo_transform(object: &IfoObject) -> Transform3D {
    rose_transform(
        Vector3::new(object.position.x, object.position.z, -object.position.y) / 100.0
            + Vector3::new(OBJECT_OFFSET, 0.0, -OBJECT_OFFSET),
        rose_quat(object.rotation.x, object.rotation.y, object.rotation.z, object.rotation.w),
        Vector3::new(object.scale.x, object.scale.z, object.scale.y),
    )
}

pub fn load_shader(path: &str) -> Gd<Shader> {
    godot::tools::load::<Shader>(path)
}

/// Shared per-zone state while spawning objects.
pub struct ObjectSpawner {
    materials: HashMap<(usize, u16), Gd<ShaderMaterial>>,
    pub parts: usize,
    /// Collision triangles in zone space, three vertices each (zone_loader.rs's collision groups).
    pub walls: Vec<Vector3>,
    pub floors: Vec<Vector3>,
    pub warps: Vec<(u16, Aabb)>,
    pub effect_spots: Vec<(Transform3D, String, bool)>,
    pub ambient_sounds: Vec<(Vector3, String, f32)>,
    pub blocks: usize,
    pub objects: usize,
    /// What casts shadows, for the block rule (`RoseZone::update_shadows`).
    pub shadow_casters: Vec<ShadowCaster>,
    /// The part colours from the client's lightmaps (old zone loader only).
    pub tints: PartTints,
}

/// A model part or ground block that casts a shadow, and where it stands.
pub struct ShadowCaster {
    pub node: Gd<GeometryInstance3D>,
    /// Its place in zone space (the model's origin).
    pub position: Vector3,
    /// Small decorations (grass, flowers, small rocks) cast shadows only near the player.
    pub small: bool,
    pub on: bool,
}

/// Models smaller than this (metres, the longest side) count as small for shadows.
pub const SMALL_MODEL: f32 = 4.0;

impl ObjectSpawner {
    /// Adds the model parts in `parts` (one placed model at `transform`) to the shadow casters.
    pub fn add_casters(&mut self, parts: Vec<Gd<GeometryInstance3D>>, transform: Transform3D, size: Aabb) {
        let longest = (transform * size).size;
        let small = longest.x.max(longest.y).max(longest.z) < SMALL_MODEL;
        for node in parts {
            self.shadow_casters.push(ShadowCaster { node, position: transform.origin, small, on: true });
        }
    }

    #[allow(clippy::too_many_arguments)]
    /// `place` is the zone folder, block x, block y and the model's place in the block's IFO
    /// list, for its part colours.
    fn spawn(&mut self, parent: &mut Gd<Node3D>, zsc: &ZscFile, zsc_index: usize, object_instance: &IfoObject, zsc_object_id: usize, name: &str, place: (&str, usize, usize, usize)) {
        let Some(object) = zsc.objects.get(zsc_object_id) else { return };
        let mut object_node = Node3D::new_alloc();
        object_node.set_name(name);
        let object_transform = ifo_transform(object_instance);
        object_node.set_transform(object_transform);
        let mut casters = Vec::new();
        let mut size: Option<Aabb> = None;
        let mut tinted: Vec<(usize, Gd<Mesh>, Gd<GeometryInstance3D>)> = Vec::new();

        for (part_index, part) in object.parts.iter().enumerate() {
            let Some(mesh_path) = zsc.meshes.get(part.mesh_id as usize) else { continue };
            let Some(part_mesh) = mesh::load_mesh(&mesh_path.path().to_string_lossy(), false) else { continue };
            let Some(zsc_material) = zsc.materials.get(part.material_id as usize) else { continue };

            let part_material = self
                .materials
                .entry((zsc_index, part.material_id))
                .or_insert_with(|| material::object_material(zsc_material, material::Light::Map))
                .clone();

            let mut instance = MeshInstance3D::new_alloc();
            instance.set_mesh(&part_mesh);
            instance.set_material_override(&part_material);
            let part_transform = part_transform(&part.position, &part.rotation, &part.scale);
            instance.set_transform(part_transform);
            let part_box = part_transform * part_mesh.get_aabb();
            size = Some(size.map_or(part_box, |s| s.merge(part_box)));
            if material::casts_shadow(&part_material) {
                casters.push(instance.clone().upcast::<GeometryInstance3D>());
            } else {
                instance.set_cast_shadows_setting(ShadowCastingSetting::OFF);
            }

            let (blocks, walkable) = part_collision(zsc_index, part);
            if blocks || walkable {
                {
                    let to_zone = object_transform * part_transform;
                    let faces: Vec<Vector3> =
                        part_mesh.clone().upcast::<Mesh>().get_faces().as_slice().iter().map(|v| to_zone * *v).collect();
                    if blocks {
                        self.walls.extend_from_slice(&faces);
                    }
                    if walkable {
                        self.floors.extend(faces);
                    }
                }
            }
            if zsc_index == 3 {
                let to_zone = object_transform * part_transform;
                let aabb = to_zone * part_mesh.get_aabb();
                match self.warps.last_mut() {
                    Some((id, last)) if *id == object_instance.warp_id && part_index > 0 => *last = last.merge(aabb),
                    _ => self.warps.push((object_instance.warp_id, aabb)),
                }
            }
            tinted.push((part_index, part_mesh.clone().upcast(), instance.clone().upcast()));
            object_node.add_child(&instance);
            self.parts += 1;
        }
        let (zone_dir, block_x, block_y, ifo_index) = place;
        let meshes: Vec<(usize, Gd<Mesh>)> = tinted.iter().map(|(i, mesh, _)| (*i, mesh.clone())).collect();
        let tints = self.tints.object(zone_dir, block_x, block_y, zsc_index, ifo_index, &meshes);
        for (part_index, _, mut part) in tinted {
            if let Some(colour) = tints.get(part_index) {
                set_part_tint(&mut part, colour);
            }
        }

        for effect in object.effects.iter() {
            let Some(path) = zsc.effects.get(effect.effect_id as usize) else { continue };
            let effect_transform = part_transform(&effect.position, &effect.rotation, &effect.scale);
            let night_only = matches!(effect.effect_type, ZscEffectType::DayNight);
            self.effect_spots.push((object_transform * effect_transform, path.path().to_string_lossy().to_string(), night_only));
        }
        if let Some(size) = size {
            self.add_casters(casters, object_transform, size);
        }

        parent.add_child(&object_node);
    }
}

/// Whether a model part blocks walking (walls) and can be stood on (floors). Event objects
/// (zsc index 2) can be stood on but don't block; warps (3) do neither.
pub fn part_collision(zsc_index: usize, part: &rose_file_readers::ZscObjectPart) -> (bool, bool) {
    if part.collision_shape.is_none() || zsc_index == 3 {
        return (false, false);
    }
    let blocks = zsc_index != 2 && !part.collision_flags.contains(ZscCollisionFlags::HEIGHT_ONLY);
    let walkable = !part.collision_flags.contains(ZscCollisionFlags::NOT_MOVEABLE);
    (blocks, walkable)
}

/// A model part's place in its model, and an effect's.
pub fn part_transform(position: &rose_file_readers::types::Vec3<f32>, rotation: &rose_file_readers::types::Vec4<f32>, scale: &rose_file_readers::types::Vec3<f32>) -> Transform3D {
    rose_transform(
        Vector3::new(position.x, position.z, -position.y) / 100.0,
        rose_quat(rotation.x, rotation.y, rotation.z, rotation.w),
        Vector3::new(scale.x, scale.z, scale.y),
    )
}

/// One static body holding every triangle of a collision layer.
fn collision_body(name: &str, layer: u32, faces: &[Vector3]) -> Gd<StaticBody3D> {
    let mut shape = ConcavePolygonShape3D::new_gd();
    shape.set_faces(&PackedVector3Array::from(faces));
    // Index buffers are flipped for Godot's winding, so let rays hit both sides.
    shape.set_backface_collision_enabled(true);
    let mut collision = CollisionShape3D::new_alloc();
    collision.set_shape(&shape);
    let mut body = StaticBody3D::new_alloc();
    body.set_name(name);
    body.set_collision_layer(1 << (layer - 1));
    body.set_collision_mask(0);
    body.add_child(&collision);
    body
}

#[godot_api]
impl RoseZone {
    /// Loads and spawns a zone. Returns false if the zone or the game data is missing.
    #[func]
    fn load_zone(&mut self, zone_id: i32) -> bool {
        let Some(game_data) = data::get() else {
            godot_error!("rose: call RoseData.open() first");
            return false;
        };
        let Some(zone_entry) = ZoneId::new(zone_id as u16).and_then(|id| game_data.zone_list.get_zone(id)) else {
            godot_error!("rose: unknown zone {zone_id}");
            return false;
        };
        let started = Instant::now();

        let zon_path = zone_entry.zon_file_path.path().to_string_lossy().to_string();
        let zone_dir = Path::new(&zon_path).parent().map(|p| p.to_string_lossy().replace('\\', "/")).unwrap_or_default();
        self.grounds = (0..64 * 64).map(|_| None).collect();
        self.ambient_sounds.clear();
        let mut spawner = ObjectSpawner {
            materials: HashMap::new(),
            parts: 0,
            walls: Vec::new(),
            floors: Vec::new(),
            warps: Vec::new(),
            effect_spots: Vec::new(),
            ambient_sounds: Vec::new(),
            blocks: 0,
            objects: 0,
            shadow_casters: Vec::new(),
            tints: PartTints::default(),
        };

        // A zone with scenes in res://zones/ loads from them; any other from the ROSE files.
        let scene = if self.files_only {
            crate::zone_scene::zone_folder(zone_id as u16).map_or(Err(None), |_| Err(Some("--zone-files")))
        } else {
            crate::zone_scene::zone_scene(zone_id as u16, &zone_dir)
        };
        let fallback = scene.as_ref().err().copied().flatten().unwrap_or("");
        let from_scene = match scene {
            Ok(scene) => {
                let Some(map) = scene.instantiate() else {
                    godot_error!("rose: could not open the scene of zone {zone_id}");
                    return false;
                };
                self.base_mut().add_child(&map);
                let to_zone = self.base().get_global_transform().affine_inverse();
                let materials = crate::zone_scene::gather(&map, to_zone, &mut spawner, &mut self.grounds);
                spawner.materials.clear();
                Some(materials)
            }
            Err(_) => {
                let Some(zon) = data::read_file::<ZonFile>(&zon_path) else { return false };
                self.build_from_files(zone_entry, &zon, &zone_dir, &mut spawner);
                None
            }
        };

        for (name, layer, faces) in [("Walls", LAYER_WALLS, &spawner.walls), ("Floors", LAYER_FLOORS, &spawner.floors)] {
            self.base_mut().add_child(&collision_body(name, layer, faces));
        }

        if let Some(sky) = zone_entry.skybox_id.and_then(|id| game_data.skybox.get_skybox_data(id)) {
            if let Some(sky_mesh) = mesh::load_mesh(&sky.mesh, false) {
                let mut sky_material = ShaderMaterial::new_gd();
                sky_material.set_shader(&load_shader("res://shaders/rose_sky.gdshader"));
                if let Some(day) = texture::load_texture(&sky.texture_day) {
                    sky_material.set_shader_parameter("texture_day", &day.to_variant());
                }
                if let Some(night) = texture::load_texture(&sky.texture_night) {
                    sky_material.set_shader_parameter("texture_night", &night.to_variant());
                }
                let mut sky_node = MeshInstance3D::new_alloc();
                sky_node.set_name("Sky");
                sky_node.set_mesh(&sky_mesh);
                sky_node.set_material_override(&sky_material);
                sky_node.set_scale(Vector3::splat(10.0));
                // The sky follows the camera in the shader, so it must never be culled.
                sky_node.set_custom_aabb(Aabb::new(Vector3::splat(-1.0e6), Vector3::splat(2.0e6)));
                sky_node.set_cast_shadows_setting(ShadowCastingSetting::OFF);
                self.base_mut().add_child(&sky_node);
            }
        }

        self.zone_id = zone_id as u16;
        self.warps = std::mem::take(&mut spawner.warps);
        self.effect_spots = std::mem::take(&mut spawner.effect_spots);
        self.ambient_sounds = std::mem::take(&mut spawner.ambient_sounds);
        self.shadow_casters = std::mem::take(&mut spawner.shadow_casters);
        self.shadow_block = None;
        // Zone effects (torches, fountains, glows). Night-only ones start hidden.
        let mut effects = 0;
        for (name, night) in [("Effects", false), ("NightEffects", true)] {
            let mut group = Node3D::new_alloc();
            group.set_name(name);
            group.set_visible(!night);
            for (transform, path, night_only) in self.effect_spots.iter() {
                if *night_only != night {
                    continue;
                }
                let mut effect = RoseEffect::new_alloc();
                if effect.bind_mut().load_path(path) {
                    effect.set_transform(*transform);
                    group.add_child(&effect);
                    effects += 1;
                } else {
                    effect.free();
                }
            }
            self.base_mut().add_child(&group);
        }
        let mut stats = VarDictionary::new();
        stats.set("source", if from_scene.is_some() { "scene" } else { "client data" });
        // Why a zone that has a scene loaded from the client data instead ("" when it didn't).
        stats.set("fallback", fallback);
        stats.set("blocks", spawner.blocks as i64);
        stats.set("objects", spawner.objects as i64);
        stats.set("mesh_parts", spawner.parts as i64);
        stats.set("materials", from_scene.unwrap_or(spawner.materials.len()) as i64);
        stats.set("wall_triangles", (spawner.walls.len() / 3) as i64);
        stats.set("floor_triangles", (spawner.floors.len() / 3) as i64);
        stats.set("effects", effects as i64);
        stats.set("ambient_sounds", self.ambient_sounds.len() as i64);
        stats.set("load_ms", started.elapsed().as_millis() as i64);
        self.stats = stats;
        true
    }

    #[func]
    fn get_stats(&self) -> VarDictionary {
        self.stats.clone()
    }

    /// Warp gates in this zone: id (WARP.STB row) and the box (position, size) their model fills.
    #[func]
    fn get_warps(&self) -> VarArray {
        let mut out = VarArray::new();
        for (id, aabb) in self.warps.iter() {
            let mut d = VarDictionary::new();
            d.set("id", *id as i64);
            d.set("position", aabb.position);
            d.set("size", aabb.size);
            out.push(&d.to_variant());
        }
        out
    }

    /// Terrain height in metres at Godot world position (x, z). Mirrors
    /// ZoneLoaderAsset::get_terrain_height, which works in ROSE centimetres.
    #[func]
    fn get_terrain_height(&self, x: f32, z: f32) -> f32 {
        let (x, y) = (x * 100.0, -z * 100.0);
        let block_x = x / (BLOCK_SIZE * 100.0);
        let block_y = 65.0 - (y / (BLOCK_SIZE * 100.0));
        let index = block_x.clamp(0.0, 63.0) as usize + block_y.clamp(0.0, 63.0) as usize * 64;
        let Some(ground) = self.grounds.get(index).and_then(|g| g.as_ref()) else { return 0.0 };

        let tile_x = (ground.size - 1) as f32 * block_x.fract();
        let tile_y = (ground.size - 1) as f32 * block_y.fract();
        let (ix, iy) = (tile_x as i32, tile_y as i32);
        let (wx, wy) = (tile_x.fract(), tile_y.fract());
        let h0 = ground.height(ix, iy) * (1.0 - wx) + ground.height(ix + 1, iy) * wx;
        let h1 = ground.height(ix, iy + 1) * (1.0 - wx) + ground.height(ix + 1, iy + 1) * wx;
        h0 * (1.0 - wy) + h1 * wy
    }

    /// The terrain texture number under Godot world position (x, z), which picks the
    /// footstep sound (ZoneLoaderAsset::get_tile_index).
    #[func]
    fn get_tile_index(&self, x: f32, z: f32) -> i32 {
        let (x, y) = (x * 100.0, -z * 100.0);
        let block_x = x / (BLOCK_SIZE * 100.0);
        let block_y = 65.0 - (y / (BLOCK_SIZE * 100.0));
        let index = block_x.clamp(0.0, 63.0) as usize + block_y.clamp(0.0, 63.0) as usize * 64;
        let Some(ground) = self.grounds.get(index).and_then(|g| g.as_ref()) else { return 0 };
        let tile_x = ((16.0 * block_x.fract()) as usize).min(15);
        let tile_y = ((16.0 * block_y.fract()) as usize).min(15);
        ground.tile_textures.get(tile_x + tile_y * 16).copied().unwrap_or(0)
    }

    /// Shows the zone's night-only effects (evening and night) or hides them.
    #[func]
    fn set_night_effects(&mut self, night: bool) {
        if let Some(mut group) = self.base().get_node_or_null("NightEffects").and_then(|n| n.try_cast::<Node3D>().ok()) {
            group.set_visible(night);
        }
    }

    /// Looping ambient sounds placed in the zone: position, path and range (metres).
    #[func]
    fn get_ambient_sounds(&self) -> VarArray {
        let mut out = VarArray::new();
        for (position, path, range) in self.ambient_sounds.iter() {
            let mut d = VarDictionary::new();
            d.set("position", *position);
            d.set("path", path.as_str());
            d.set("range", *range);
            out.push(&d.to_variant());
        }
        out
    }

    /// Length of the zone's day in world ticks (10 s each).
    #[func]
    fn get_day_cycle(&self) -> i64 {
        data::get()
            .and_then(|game_data| ZoneId::new(self.zone_id).and_then(|id| game_data.zone_list.get_zone(id)))
            .map_or(0, |zone| zone.day_cycle as i64)
    }

    /// Zone lighting at a time of day (see `sun::lighting`): what `RoseSun.apply` takes.
    /// `day_time` is in world ticks (0..day_cycle) and `partial_tick` is the fraction of the
    /// current tick.
    #[func]
    fn get_lighting_at(&self, day_time: i64, partial_tick: f32) -> VarDictionary {
        let Some(game_data) = data::get() else { return VarDictionary::new() };
        let Some(zone) = ZoneId::new(self.zone_id).and_then(|id| game_data.zone_list.get_zone(id)) else { return VarDictionary::new() };
        let sky = zone.skybox_id.and_then(|id| game_data.skybox.get_skybox_data(id));
        let times = sun::DayTimes { day_cycle: zone.day_cycle, morning: zone.morning_time, day: zone.day_time, evening: zone.evening_time, night: zone.night_time };
        sun::lighting(sky, &times, day_time, partial_tick)
    }

    /// Lets only what stands near `position` (zone space, usually the player) cast shadows:
    /// the models and ground of the player's block and the 8 around it, and of those the
    /// small decorations only within 40 m of the player's block. Does nothing until the
    /// player enters another 160 m block, so it costs nothing per frame.
    #[func]
    fn update_shadows(&mut self, position: Vector3) {
        let block = ((position.x / BLOCK_SIZE).floor() as i32, (position.z / BLOCK_SIZE).floor() as i32);
        if self.shadow_block == Some(block) {
            return;
        }
        self.shadow_block = Some(block);
        let (x0, z0) = (block.0 as f32 * BLOCK_SIZE, block.1 as f32 * BLOCK_SIZE);
        for caster in self.shadow_casters.iter_mut() {
            let p = caster.position;
            let near = ((p.x / BLOCK_SIZE).floor() as i32 - block.0).abs() <= 1 && ((p.z / BLOCK_SIZE).floor() as i32 - block.1).abs() <= 1;
            let on = near && (!caster.small || {
                let dx = (x0 - p.x).max(p.x - (x0 + BLOCK_SIZE)).max(0.0);
                let dz = (z0 - p.z).max(p.z - (z0 + BLOCK_SIZE)).max(0.0);
                dx * dx + dz * dz <= SMALL_SHADOW_REACH * SMALL_SHADOW_REACH
            });
            if on != caster.on && caster.node.is_instance_valid() {
                caster.on = on;
                caster.node.set_cast_shadows_setting(if on { ShadowCastingSetting::ON } else { ShadowCastingSetting::OFF });
            }
        }
    }

    /// Start of the zone's day state, in world ticks.
    #[func]
    fn get_state_start(&self, state: GString) -> i64 {
        data::get()
            .and_then(|game_data| ZoneId::new(self.zone_id).and_then(|id| game_data.zone_list.get_zone(id)))
            .map_or(0, |zone| match state.to_string().as_str() {
                "morning" => zone.morning_time,
                "day" => zone.day_time,
                "evening" => zone.evening_time,
                _ => zone.night_time,
            } as i64)
    }
}

impl RoseZone {
    /// Builds the zone's blocks from the ROSE map files (ZON, HIM, TIL, IFO, and the ground
    /// lightmaps for the ground's colour).
    fn build_from_files(&mut self, zone_entry: &rose_data::ZoneListEntry, zon: &ZonFile, zone_dir: &str, spawner: &mut ObjectSpawner) {
        let zsc_cnst = data::read_file::<ZscFile>(&zone_entry.zsc_cnst_path.path().to_string_lossy());
        let zsc_deco = data::read_file::<ZscFile>(&zone_entry.zsc_deco_path.path().to_string_lossy());
        let zsc_event = data::read_file::<ZscFile>(EVENT_ZSC);
        let zsc_special = data::read_file::<ZscFile>(SPECIAL_ZSC);

        // Terrain tile textures: one array for the whole zone, indexed per vertex.
        let tile_array = texture::load_texture_array(&tile_texture_paths(zon));
        let terrain_shader = load_shader(TERRAIN_SHADER);
        let mut water_material = ShaderMaterial::new_gd();
        water_material.set_shader(&load_shader(WATER_SHADER));
        water_material.set_shader_parameter("water_textures", &texture::load_texture_array(&water_texture_paths()).to_variant());

        for block_y in 0..64usize {
            for block_x in 0..64usize {
                let Some(him) = data::read_file::<HimFile>(&format!("{zone_dir}/{block_x}_{block_y}.HIM")) else { continue };
                let til = data::read_file::<TilFile>(&format!("{zone_dir}/{block_x}_{block_y}.TIL"));
                let ifo = data::read_file::<IfoFile>(&format!("{zone_dir}/{block_x}_{block_y}.IFO"));

                let mut block_node = Node3D::new_alloc();
                block_node.set_name(&format!("Block_{block_x}_{block_y}"));

                let mut terrain_material = ShaderMaterial::new_gd();
                terrain_material.set_shader(&terrain_shader);
                terrain_material.set_shader_parameter("tile_textures", &tile_array.to_variant());
                if let Some(tint) = ground_tint(zone_dir, block_x, block_y) {
                    let bytes: Vec<u8> = tint.iter().flat_map(|c| [c.r, c.g, c.b].map(|v| (v * 255.0).round() as u8)).collect();
                    let size = TINT_CELLS as i32;
                    if let Some(texture) = Image::create_from_data(size, size, false, godot::classes::image::Format::RGB8, &PackedByteArray::from(bytes.as_slice()))
                        .and_then(|image| ImageTexture::create_from_image(&image))
                    {
                        terrain_material.set_shader_parameter("tint_texture", &texture.to_variant());
                    }
                }
                let ground = Ground::from_files(zon, &him, til.as_ref());
                let mut terrain = MeshInstance3D::new_alloc();
                terrain.set_name("Terrain");
                terrain.set_mesh(&build_terrain_mesh(&|x, y| ground.height(x, y), &|x, y| tile_layers(zon, til.as_ref(), x, y)));
                terrain.set_material_override(&terrain_material);
                terrain.set_position(block_origin(block_x, block_y));
                block_node.add_child(&terrain);
                let centre = block_origin(block_x, block_y) + Vector3::new(BLOCK_SIZE / 2.0, 0.0, BLOCK_SIZE / 2.0);
                spawner.shadow_casters.push(ShadowCaster { node: terrain.clone().upcast(), position: centre, small: false, on: true });
                spawner.blocks += 1;

                if let Some(ifo) = ifo.as_ref() {
                    for (start, end) in ifo.water_planes.iter() {
                        let mut water = MeshInstance3D::new_alloc();
                        water.set_name("Water");
                        let (start, end) = water_corners(start, end);
                        water.set_mesh(&build_water_mesh(start, end, ifo.water_size / 100.0));
                        water.set_material_override(&water_material);
                        water.set_cast_shadows_setting(ShadowCastingSetting::OFF);
                        block_node.add_child(&water);
                    }

                    let mut spawn_list = |zsc: Option<&ZscFile>, zsc_index: usize, list: &[(usize, &IfoObject, usize)], kind: &str| {
                        let Some(zsc) = zsc else { return };
                        for &(ifo_id, object, zsc_id) in list {
                            spawner.spawn(&mut block_node, zsc, zsc_index, object, zsc_id, &format!("{kind}_{ifo_id}"), (zone_dir, block_x, block_y, ifo_id));
                            spawner.objects += 1;
                        }
                    };

                    let cnst: Vec<_> = ifo.cnst_objects.iter().enumerate().map(|(i, o)| (i, o, o.object_id as usize)).collect();
                    spawn_list(zsc_cnst.as_ref(), 0, &cnst, "Cnst");
                    let deco: Vec<_> = ifo.deco_objects.iter().enumerate().map(|(i, o)| (i, o, o.object_id as usize)).collect();
                    spawn_list(zsc_deco.as_ref(), 1, &deco, "Deco");
                    let events: Vec<_> = ifo.event_objects.iter().enumerate().map(|(i, o)| (i, &o.object, o.object.object_id as usize)).collect();
                    spawn_list(zsc_event.as_ref(), 2, &events, "Event");
                    let warps: Vec<_> = ifo.warps.iter().enumerate().map(|(i, o)| (i, o, 1)).collect();
                    spawn_list(zsc_special.as_ref(), 3, &warps, "Warp");

                    for effect in ifo.effect_objects.iter() {
                        let path = effect.effect_path.path().to_string_lossy().to_string();
                        spawner.effect_spots.push((ifo_transform(&effect.object), path, false));
                    }
                    for sound in ifo.sound_objects.iter() {
                        let position = ifo_transform(&sound.object).origin;
                        let path = sound.sound_path.path().to_string_lossy().to_string();
                        spawner.ambient_sounds.push((position, path, sound.range as f32 / 10.0));
                    }
                }

                self.grounds[block_x + block_y * 64] = Some(ground);
                self.base_mut().add_child(&block_node);
            }
        }
    }
}

/// A block's terrain mesh: 16 x 16 tiles of 4 x 4 quads, 2.5 m apart. `height` gives a
/// height sample in metres (65 x 65, clamped at the edges) and `tile` a tile's
/// [layer 1, layer 2, rotation] (see `tile_layers`).
pub fn build_terrain_mesh(height: &dyn Fn(i32, i32) -> f32, tile: &dyn Fn(usize, usize) -> [u8; 3]) -> Gd<ArrayMesh> {
    let mut positions = Vec::with_capacity(16 * 16 * 25);
    let mut normals = Vec::with_capacity(16 * 16 * 25);
    let mut uv_block = Vec::with_capacity(16 * 16 * 25);
    let mut uv_tile = Vec::with_capacity(16 * 16 * 25);
    let mut tile_info: Vec<u8> = Vec::with_capacity(16 * 16 * 25 * 4);
    let mut indices: Vec<u16> = Vec::with_capacity(16 * 16 * 16 * 6);

    for tile_x in 0..16usize {
        for tile_y in 0..16usize {
            let [layer1, layer2, rotation] = tile(tile_x, tile_y);
            let base = positions.len() as u16;
            let offset_x = tile_x as f32 * 10.0;
            let offset_y = tile_y as f32 * 10.0;

            for y in 0..5i32 {
                for x in 0..5i32 {
                    let hx = x + tile_x as i32 * 4;
                    let hy = y + tile_y as i32 * 4;
                    let h = height(hx, hy);
                    let l = height(hx - 1, hy);
                    let r = height(hx + 1, hy);
                    let t = height(hx, hy - 1);
                    let b = height(hx, hy + 1);
                    positions.push(Vector3::new(offset_x + x as f32 * 2.5, h, offset_y + y as f32 * 2.5));
                    normals.push(Vector3::new((l - r) / 2.0, 1.0, (t - b) / 2.0).normalized());
                    uv_tile.push(Vector2::new(x as f32 / 4.0, y as f32 / 4.0));
                    uv_block.push(Vector2::new(
                        (tile_x as f32 * 4.0 + x as f32) / 64.0,
                        (tile_y as f32 * 4.0 + y as f32) / 64.0,
                    ));
                    tile_info.extend([layer1, layer2, rotation, 0]);
                }
            }

            for y in 0..4u16 {
                for x in 0..4u16 {
                    let start = base + y * 5 + x;
                    indices.extend([start, start + 5, start + 1, start + 1, start + 5, start + 6]);
                }
            }
        }
    }

    let mut arrays = VarArray::new();
    arrays.resize(ArrayType::MAX.ord() as usize, &Variant::nil());
    arrays.set(ArrayType::VERTEX.ord() as usize, &PackedVector3Array::from(positions.as_slice()).to_variant());
    arrays.set(ArrayType::NORMAL.ord() as usize, &PackedVector3Array::from(normals.as_slice()).to_variant());
    arrays.set(ArrayType::TEX_UV.ord() as usize, &PackedVector2Array::from(uv_block.as_slice()).to_variant());
    arrays.set(ArrayType::TEX_UV2.ord() as usize, &PackedVector2Array::from(uv_tile.as_slice()).to_variant());
    arrays.set(ArrayType::CUSTOM0.ord() as usize, &PackedByteArray::from(tile_info.as_slice()).to_variant());
    arrays.set(ArrayType::INDEX.ord() as usize, &mesh::flip_winding(&indices).to_variant());

    let format = ArrayFormat::from_ord(
        (ArrayCustomFormat::RGBA8_UNORM.ord() as u64) << ArrayFormat::CUSTOM0_SHIFT.ord(),
    );
    let mut terrain = ArrayMesh::new_gd();
    terrain
        .add_surface_from_arrays_ex(PrimitiveType::TRIANGLES, &arrays)
        .flags(format)
        .done();
    terrain
}

/// A water plane between two corners in zone space (at the first corner's height), its
/// texture repeating every `tile_size` metres.
pub fn build_water_mesh(start: Vector3, end: Vector3, tile_size: f32) -> Gd<ArrayMesh> {
    let end = Vector3::new(end.x, start.y, end.z);
    let uv_x = (end.x - start.x) / tile_size;
    let uv_y = (end.z - start.z) / tile_size;

    let positions = [
        Vector3::new(start.x, start.y, end.z),
        Vector3::new(start.x, start.y, start.z),
        Vector3::new(end.x, start.y, start.z),
        Vector3::new(end.x, start.y, end.z),
    ];
    let uvs = [Vector2::new(uv_x, uv_y), Vector2::new(uv_x, 0.0), Vector2::new(0.0, 0.0), Vector2::new(0.0, uv_y)];
    let mut arrays = VarArray::new();
    arrays.resize(ArrayType::MAX.ord() as usize, &Variant::nil());
    arrays.set(ArrayType::VERTEX.ord() as usize, &PackedVector3Array::from(positions.as_slice()).to_variant());
    arrays.set(ArrayType::NORMAL.ord() as usize, &PackedVector3Array::from([Vector3::UP; 4].as_slice()).to_variant());
    arrays.set(ArrayType::TEX_UV.ord() as usize, &PackedVector2Array::from(uvs.as_slice()).to_variant());
    arrays.set(ArrayType::INDEX.ord() as usize, &mesh::flip_winding(&[0, 2, 1, 0, 3, 2]).to_variant());
    let mut water = ArrayMesh::new_gd();
    water.add_surface_from_arrays(PrimitiveType::TRIANGLES, &arrays);
    water
}
