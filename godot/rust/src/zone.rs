//! Builds a whole zone (terrain, water, objects, sky) as Godot nodes.
//! Ported from rose-offline-client's zone_loader.rs.

use std::{collections::HashMap, path::Path, time::Instant};

use godot::{
    classes::{
        mesh::{ArrayCustomFormat, ArrayFormat, ArrayType, PrimitiveType},
        ArrayMesh, MeshInstance3D, Node3D, Shader, ShaderMaterial, Texture2D,
    },
    prelude::*,
};
use rose_data::{SkyboxState, ZoneId};
use rose_file_readers::{HimFile, IfoFile, IfoObject, LitFile, LitObject, TilFile, ZonFile, ZonTileRotation, ZscFile};

use crate::{data, material, mesh, texture};

const BLOCK_SIZE: f32 = 160.0;
/// Zone objects are stored relative to the centre of the 64x64 block grid.
const OBJECT_OFFSET: f32 = 5200.0;

#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct RoseZone {
    base: Base<Node3D>,
    heights: Vec<Option<HimFile>>,
    zone_id: u16,
    stats: VarDictionary,
}

fn rose_quat(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
    let q = Quaternion::new(x, z, -y, w);
    if q.length_squared() > 0.0 { q.normalized() } else { Quaternion::IDENTITY }
}

fn rose_transform(position: Vector3, rotation: Quaternion, scale: Vector3) -> Transform3D {
    Transform3D::new(Basis::from_quaternion(rotation) * Basis::from_scale(scale), position)
}

fn ifo_transform(object: &IfoObject) -> Transform3D {
    rose_transform(
        Vector3::new(object.position.x, object.position.z, -object.position.y) / 100.0
            + Vector3::new(OBJECT_OFFSET, 0.0, -OBJECT_OFFSET),
        rose_quat(object.rotation.x, object.rotation.y, object.rotation.z, object.rotation.w),
        Vector3::new(object.scale.x, object.scale.z, object.scale.y),
    )
}

fn load_shader(path: &str) -> Gd<Shader> {
    godot::tools::load::<Shader>(path)
}

/// Shared per-zone state while spawning objects.
struct ObjectSpawner {
    materials: HashMap<(usize, u16, String), Gd<ShaderMaterial>>,
    lightmaps: HashMap<String, Option<Gd<Texture2D>>>,
    parts: usize,
}

impl ObjectSpawner {
    fn lightmap(&mut self, path: String) -> Option<Gd<Texture2D>> {
        self.lightmaps.entry(path.clone()).or_insert_with(|| texture::load_texture(&path)).clone()
    }

    #[allow(clippy::too_many_arguments)]
    fn spawn(
        &mut self,
        parent: &mut Gd<Node3D>,
        zsc: &ZscFile,
        zsc_index: usize,
        lightmap_dir: &str,
        lit_object: Option<&LitObject>,
        object_instance: &IfoObject,
        zsc_object_id: usize,
        name: &str,
    ) {
        let Some(object) = zsc.objects.get(zsc_object_id) else { return };
        let mut object_node = Node3D::new_alloc();
        object_node.set_name(name);
        object_node.set_transform(ifo_transform(object_instance));

        for (part_index, part) in object.parts.iter().enumerate() {
            let Some(mesh_path) = zsc.meshes.get(part.mesh_id as usize) else { continue };
            let Some(part_mesh) = mesh::load_mesh(&mesh_path.path().to_string_lossy(), false) else { continue };
            let Some(zsc_material) = zsc.materials.get(part.material_id as usize) else { continue };

            let lit_part = lit_object.and_then(|lit_object| {
                lit_object
                    .parts
                    .iter()
                    .find(|lit_part| lit_part.object_part_index as usize == part_index)
                    .or_else(|| lit_object.parts.get(part_index))
            });
            let lightmap_path = lit_part.map(|lit_part| format!("{lightmap_dir}{}", lit_part.filename));
            let lightmap = lightmap_path.clone().and_then(|path| self.lightmap(path));
            let cache_key = (zsc_index, part.material_id, if lightmap.is_some() { lightmap_path.unwrap() } else { String::new() });
            let part_material = self
                .materials
                .entry(cache_key)
                .or_insert_with(|| material::object_material(zsc_material, lightmap.clone()))
                .clone();

            let mut instance = MeshInstance3D::new_alloc();
            instance.set_mesh(&part_mesh);
            instance.set_material_override(&part_material);
            instance.set_transform(rose_transform(
                Vector3::new(part.position.x, part.position.z, -part.position.y) / 100.0,
                rose_quat(part.rotation.x, part.rotation.y, part.rotation.z, part.rotation.w),
                Vector3::new(part.scale.x, part.scale.z, part.scale.y),
            ));
            if let (Some(lit_part), true) = (lit_part, lightmap.is_some()) {
                let per_row = lit_part.parts_per_row.max(1);
                instance.set_instance_shader_parameter(
                    "lightmap_uv_offset",
                    &Vector2::new((lit_part.part_index % per_row) as f32, (lit_part.part_index / per_row) as f32).to_variant(),
                );
                instance.set_instance_shader_parameter("lightmap_uv_scale", &(1.0 / per_row as f32).to_variant());
            }
            object_node.add_child(&instance);
            self.parts += 1;
        }

        parent.add_child(&object_node);
    }
}

fn find_lit<'a>(lit: Option<&'a LitFile>, ifo_object_id: usize) -> Option<&'a LitObject> {
    lit.and_then(|lit| lit.objects.iter().find(|lit_object| lit_object.id as usize == ifo_object_id + 1))
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
        let Some(zon) = data::read_file::<ZonFile>(&zon_path) else { return false };
        let zsc_cnst = data::read_file::<ZscFile>(&zone_entry.zsc_cnst_path.path().to_string_lossy());
        let zsc_deco = data::read_file::<ZscFile>(&zone_entry.zsc_deco_path.path().to_string_lossy());
        let zsc_event = data::read_file::<ZscFile>("3DDATA/SPECIAL/EVENT_OBJECT.ZSC");
        let zsc_special = data::read_file::<ZscFile>("3DDATA/SPECIAL/LIST_DECO_SPECIAL.ZSC");
        let zone_dir = Path::new(&zon_path).parent().map(|p| p.to_string_lossy().replace('\\', "/")).unwrap_or_default();

        // Terrain tile textures: one array for the whole zone, indexed per vertex.
        let tile_paths: Vec<String> = zon.tile_textures.iter().take_while(|path| *path != "end").cloned().collect();
        let tile_array = texture::load_texture_array(&tile_paths);
        let terrain_shader = load_shader("res://shaders/rose_terrain.gdshader");
        let water_shader = load_shader("res://shaders/rose_water.gdshader");
        let water_textures = texture::load_texture_array(
            &(1..=25).map(|i| format!("3DDATA/JUNON/WATER/OCEAN01_{i:02}.DDS")).collect::<Vec<_>>(),
        );
        let mut water_material = ShaderMaterial::new_gd();
        water_material.set_shader(&water_shader);
        water_material.set_shader_parameter("water_textures", &water_textures.to_variant());

        self.heights = (0..64 * 64).map(|_| None).collect();
        let mut spawner = ObjectSpawner { materials: HashMap::new(), lightmaps: HashMap::new(), parts: 0 };
        let mut blocks = 0;
        let mut objects = 0;

        for block_y in 0..64usize {
            for block_x in 0..64usize {
                let Some(him) = data::read_file::<HimFile>(&format!("{zone_dir}/{block_x}_{block_y}.HIM")) else { continue };
                let til = data::read_file::<TilFile>(&format!("{zone_dir}/{block_x}_{block_y}.TIL"));
                let ifo = data::read_file::<IfoFile>(&format!("{zone_dir}/{block_x}_{block_y}.IFO"));
                let lightmap_dir = format!("{zone_dir}/{block_x}_{block_y}/LIGHTMAP/");
                let lit_cnst = data::read_file::<LitFile>(&format!("{lightmap_dir}BUILDINGLIGHTMAPDATA.LIT"));
                let lit_deco = data::read_file::<LitFile>(&format!("{lightmap_dir}OBJECTLIGHTMAPDATA.LIT"));

                let mut block_node = Node3D::new_alloc();
                block_node.set_name(&format!("Block_{block_x}_{block_y}"));

                let mut terrain_material = ShaderMaterial::new_gd();
                terrain_material.set_shader(&terrain_shader);
                terrain_material.set_shader_parameter("tile_textures", &tile_array.to_variant());
                if let Some(lightmap) = texture::load_texture(&format!(
                    "{zone_dir}/{block_x}_{block_y}/{block_x}_{block_y}_PLANELIGHTINGMAP.DDS"
                )) {
                    terrain_material.set_shader_parameter("lightmap_texture", &lightmap.to_variant());
                }
                let mut terrain = MeshInstance3D::new_alloc();
                terrain.set_name("Terrain");
                terrain.set_mesh(&build_terrain_mesh(&zon, &him, til.as_ref()));
                terrain.set_material_override(&terrain_material);
                terrain.set_position(Vector3::new(
                    BLOCK_SIZE * block_x as f32,
                    0.0,
                    -BLOCK_SIZE * (65.0 - block_y as f32),
                ));
                block_node.add_child(&terrain);
                blocks += 1;

                if let Some(ifo) = ifo.as_ref() {
                    for (start, end) in ifo.water_planes.iter() {
                        let mut water = MeshInstance3D::new_alloc();
                        water.set_name("Water");
                        water.set_mesh(&build_water_mesh(
                            ifo.water_size,
                            Vector3::new(start.x, start.y, start.z),
                            Vector3::new(end.x, end.y, end.z),
                        ));
                        water.set_material_override(&water_material);
                        block_node.add_child(&water);
                    }

                    let mut spawn_list = |zsc: Option<&ZscFile>, zsc_index: usize, list: &[(usize, &IfoObject, usize, Option<&LitObject>)], kind: &str| {
                        let Some(zsc) = zsc else { return };
                        for &(ifo_id, object, zsc_id, lit) in list {
                            spawner.spawn(&mut block_node, zsc, zsc_index, &lightmap_dir, lit, object, zsc_id, &format!("{kind}_{ifo_id}"));
                            objects += 1;
                        }
                    };

                    let cnst: Vec<_> = ifo.cnst_objects.iter().enumerate()
                        .map(|(i, o)| (i, o, o.object_id as usize, find_lit(lit_cnst.as_ref(), i))).collect();
                    spawn_list(zsc_cnst.as_ref(), 0, &cnst, "Cnst");
                    let deco: Vec<_> = ifo.deco_objects.iter().enumerate()
                        .map(|(i, o)| (i, o, o.object_id as usize, find_lit(lit_deco.as_ref(), i))).collect();
                    spawn_list(zsc_deco.as_ref(), 1, &deco, "Deco");
                    let events: Vec<_> = ifo.event_objects.iter().enumerate()
                        .map(|(i, o)| (i, &o.object, o.object.object_id as usize, None)).collect();
                    spawn_list(zsc_event.as_ref(), 2, &events, "Event");
                    let warps: Vec<_> = ifo.warps.iter().enumerate().map(|(i, o)| (i, o, 1, None)).collect();
                    spawn_list(zsc_special.as_ref(), 3, &warps, "Warp");
                }

                self.heights[block_x + block_y * 64] = Some(him);
                self.base_mut().add_child(&block_node);
            }
        }

        if let Some(sky) = zone_entry.skybox_id.and_then(|id| game_data.skybox.get_skybox_data(id)) {
            if let Some(sky_mesh) = mesh::load_mesh(&sky.mesh.path().to_string_lossy(), false) {
                let mut sky_material = ShaderMaterial::new_gd();
                sky_material.set_shader(&load_shader("res://shaders/rose_sky.gdshader"));
                if let Some(day) = texture::load_texture(&sky.texture_day.path().to_string_lossy()) {
                    sky_material.set_shader_parameter("texture_day", &day.to_variant());
                }
                if let Some(night) = texture::load_texture(&sky.texture_night.path().to_string_lossy()) {
                    sky_material.set_shader_parameter("texture_night", &night.to_variant());
                }
                let mut sky_node = MeshInstance3D::new_alloc();
                sky_node.set_name("Sky");
                sky_node.set_mesh(&sky_mesh);
                sky_node.set_material_override(&sky_material);
                sky_node.set_scale(Vector3::splat(10.0));
                // The sky follows the camera in the shader, so it must never be culled.
                sky_node.set_custom_aabb(Aabb::new(Vector3::splat(-1.0e6), Vector3::splat(2.0e6)));
                self.base_mut().add_child(&sky_node);
            }
        }

        self.zone_id = zone_id as u16;
        let mut stats = VarDictionary::new();
        stats.set("blocks", blocks);
        stats.set("objects", objects);
        stats.set("mesh_parts", spawner.parts as i64);
        stats.set("materials", spawner.materials.len() as i64);
        stats.set("load_ms", started.elapsed().as_millis() as i64);
        self.stats = stats;
        true
    }

    #[func]
    fn get_stats(&self) -> VarDictionary {
        self.stats.clone()
    }

    /// Terrain height in metres at Godot world position (x, z). Mirrors
    /// ZoneLoaderAsset::get_terrain_height, which works in ROSE centimetres.
    #[func]
    fn get_terrain_height(&self, x: f32, z: f32) -> f32 {
        let (x, y) = (x * 100.0, -z * 100.0);
        let block_x = x / (BLOCK_SIZE * 100.0);
        let block_y = 65.0 - (y / (BLOCK_SIZE * 100.0));
        let index = block_x.clamp(0.0, 63.0) as usize + block_y.clamp(0.0, 63.0) as usize * 64;
        let Some(him) = self.heights.get(index).and_then(|h| h.as_ref()) else { return 0.0 };

        let tile_x = (him.width - 1) as f32 * block_x.fract();
        let tile_y = (him.height - 1) as f32 * block_y.fract();
        let (ix, iy) = (tile_x as i32, tile_y as i32);
        let (wx, wy) = (tile_x.fract(), tile_y.fract());
        let h0 = him.get_clamped(ix, iy) * (1.0 - wx) + him.get_clamped(ix + 1, iy) * wx;
        let h1 = him.get_clamped(ix, iy + 1) * (1.0 - wx) + him.get_clamped(ix + 1, iy + 1) * wx;
        (h0 * (1.0 - wy) + h1 * wy) / 100.0
    }

    /// Length of the zone's day in world ticks (10 s each).
    #[func]
    fn get_day_cycle(&self) -> i64 {
        data::get()
            .and_then(|game_data| ZoneId::new(self.zone_id).and_then(|id| game_data.zone_list.get_zone(id)))
            .map_or(0, |zone| zone.day_cycle as i64)
    }

    /// Zone lighting at a time of day, ported from zone_time_system.rs. `day_time` is in world
    /// ticks (0..day_cycle) and `partial_tick` is the fraction of the current tick.
    #[func]
    fn get_lighting_at(&self, day_time: i64, partial_tick: f32) -> VarDictionary {
        let mut out = VarDictionary::new();
        let Some(game_data) = data::get() else { return out };
        let Some(zone) = ZoneId::new(self.zone_id).and_then(|id| game_data.zone_list.get_zone(id)) else { return out };
        let sky = zone.skybox_id.and_then(|id| game_data.skybox.get_skybox_data(id));

        let day_time = (day_time.max(0) as u32) % zone.day_cycle.max(1);
        let (state, ticks, length) = if day_time >= zone.night_time || day_time < zone.morning_time {
            let length = zone.morning_time + (zone.day_cycle - zone.night_time);
            let ticks = if day_time >= zone.night_time { day_time - zone.night_time } else { day_time + zone.day_cycle - zone.night_time };
            (SkyboxState::Night, ticks, length)
        } else if day_time >= zone.evening_time {
            (SkyboxState::Evening, day_time - zone.evening_time, zone.night_time - zone.evening_time)
        } else if day_time >= zone.day_time {
            (SkyboxState::Day, day_time - zone.day_time, zone.evening_time - zone.day_time)
        } else {
            (SkyboxState::Morning, day_time - zone.morning_time, zone.day_time - zone.morning_time)
        };
        let percent = ((ticks as f32 + partial_tick) / length.max(1) as f32).clamp(0.0, 1.0);

        // Each state blends from the previous state into itself over its first half and into
        // the next over its second half, except day and night, which hold.
        let fog = |s: SkyboxState| match s {
            SkyboxState::Morning => (100.0 / 255.0, 0.0022),
            SkyboxState::Day => (200.0 / 255.0, 0.0018),
            SkyboxState::Evening => (100.0 / 255.0, 0.0022),
            SkyboxState::Night => (10.0 / 255.0, 0.0020),
        };
        let (from, to, t) = match state {
            SkyboxState::Morning if percent < 0.5 => (SkyboxState::Night, SkyboxState::Morning, percent * 2.0),
            SkyboxState::Morning => (SkyboxState::Morning, SkyboxState::Day, (percent - 0.5) * 2.0),
            SkyboxState::Evening if percent < 0.5 => (SkyboxState::Day, SkyboxState::Evening, percent * 2.0),
            SkyboxState::Evening => (SkyboxState::Evening, SkyboxState::Night, (percent - 0.5) * 2.0),
            s => (s, s, 0.0),
        };
        let mix3 = |a: Vector3, b: Vector3| a + (b - a) * t;
        let (map_ambient, char_ambient, char_diffuse) = if let Some(sky) = sky {
            macro_rules! v {
                ($c:expr) => {{ let c = $c; Vector3::new(c.x, c.y, c.z) }};
            }
            (
                mix3(v!(sky.map_ambient_color[from]), v!(sky.map_ambient_color[to])),
                mix3(v!(sky.character_ambient_color[from]), v!(sky.character_ambient_color[to])),
                mix3(v!(sky.character_diffuse_color[from]), v!(sky.character_diffuse_color[to])),
            )
        } else {
            (Vector3::ONE, Vector3::ONE, Vector3::ONE)
        };
        let ((fog_from, density_from), (fog_to, density_to)) = (fog(from), fog(to));
        let day_weight = match state {
            SkyboxState::Morning => percent,
            SkyboxState::Day => 1.0,
            SkyboxState::Evening => 1.0 - percent,
            SkyboxState::Night => 0.0,
        };

        out.set("state", format!("{state:?}").to_lowercase());
        out.set("state_percent", percent);
        out.set("day_weight", day_weight);
        out.set("map_ambient", map_ambient);
        out.set("character_ambient", char_ambient);
        out.set("character_diffuse", char_diffuse);
        // default_light_transform().back() in zone_lighting.rs
        out.set("light_direction", Vector3::new(0.612_372_4, 0.707_106_8, -0.353_553_4));
        out.set("fog_color", Vector3::splat(fog_from + (fog_to - fog_from) * t));
        out.set("fog_density", density_from + (density_to - density_from) * t);
        out
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

fn build_terrain_mesh(zon: &ZonFile, him: &HimFile, til: Option<&TilFile>) -> Gd<ArrayMesh> {
    let mut positions = Vec::with_capacity(16 * 16 * 25);
    let mut normals = Vec::with_capacity(16 * 16 * 25);
    let mut uv_lightmap = Vec::with_capacity(16 * 16 * 25);
    let mut uv_tile = Vec::with_capacity(16 * 16 * 25);
    let mut tile_info: Vec<u8> = Vec::with_capacity(16 * 16 * 25 * 4);
    let mut indices: Vec<u16> = Vec::with_capacity(16 * 16 * 16 * 6);

    for tile_x in 0..16usize {
        for tile_y in 0..16usize {
            let tile_index = til.map(|til| til.get_clamped(tile_x, tile_y) as usize).unwrap_or(0);
            let tile = &zon.tiles[tile_index.min(zon.tiles.len() - 1)];
            let layer1 = (tile.layer1 + tile.offset1).min(255) as u8;
            let layer2 = (tile.layer2 + tile.offset2).min(255) as u8;
            let rotation: u8 = match tile.rotation {
                ZonTileRotation::FlipHorizontal => 2,
                ZonTileRotation::FlipVertical => 3,
                ZonTileRotation::Flip => 4,
                ZonTileRotation::Clockwise90 => 5,
                ZonTileRotation::CounterClockwise90 => 6,
                _ => 0,
            };
            let base = positions.len() as u16;
            let offset_x = tile_x as f32 * 10.0;
            let offset_y = tile_y as f32 * 10.0;

            for y in 0..5i32 {
                for x in 0..5i32 {
                    let hx = x + tile_x as i32 * 4;
                    let hy = y + tile_y as i32 * 4;
                    let height = him.get_clamped(hx, hy) / 100.0;
                    let l = him.get_clamped(hx - 1, hy) / 100.0;
                    let r = him.get_clamped(hx + 1, hy) / 100.0;
                    let t = him.get_clamped(hx, hy - 1) / 100.0;
                    let b = him.get_clamped(hx, hy + 1) / 100.0;
                    positions.push(Vector3::new(offset_x + x as f32 * 2.5, height, offset_y + y as f32 * 2.5));
                    normals.push(Vector3::new((l - r) / 2.0, 1.0, (t - b) / 2.0).normalized());
                    uv_tile.push(Vector2::new(x as f32 / 4.0, y as f32 / 4.0));
                    uv_lightmap.push(Vector2::new(
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
    arrays.set(ArrayType::TEX_UV.ord() as usize, &PackedVector2Array::from(uv_lightmap.as_slice()).to_variant());
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

fn build_water_mesh(water_size: f32, plane_start: Vector3, plane_end: Vector3) -> Gd<ArrayMesh> {
    let start = Vector3::new(OBJECT_OFFSET + plane_start.x / 100.0, plane_start.y / 100.0, -(OBJECT_OFFSET + plane_start.z / 100.0));
    let end = Vector3::new(OBJECT_OFFSET + plane_end.x / 100.0, plane_start.y / 100.0, -(OBJECT_OFFSET + plane_end.z / 100.0));
    let uv_x = (end.x - start.x) / (water_size / 100.0);
    let uv_y = (end.z - start.z) / (water_size / 100.0);

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
