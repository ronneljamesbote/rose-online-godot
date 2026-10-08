//! Effects (EFT files): particle systems (PTL) and animated meshes, as one RoseEffect node
//! that simulates itself and frees itself when everything in it has finished.
//! Ported from rose-offline-client's effect_loader.rs, particle_sequence_system.rs,
//! mesh_animation.rs, transform_animation.rs and effect_system.rs.

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ops::RangeInclusive,
    rc::Rc,
};

use godot::{
    classes::{
        image::Format, multi_mesh::TransformFormat, GeometryInstance3D, INode3D, Image, ImageTexture, MeshInstance3D,
        MultiMesh, MultiMeshInstance3D, Node3D, QuadMesh, Shader, ShaderMaterial,
    },
    prelude::*,
};
use rose_file_readers::{EftFile, PtlFile, PtlKeyframeData, PtlUpdateCoords, ZmoChannel, ZmoFile};

use crate::{data, mesh, texture};

/// ROSE's particle timestep runs 4.8 times faster than real time.
const PARTICLE_TIME_SCALE: f32 = 4.8;

thread_local! {
    static RNG: Cell<u64> = Cell::new(0x9E37_79B9_7F4A_7C15 ^ std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).map_or(1, |d| d.as_nanos() as u64));
    static SHADERS: RefCell<HashMap<ShaderKey, Gd<Shader>>> = RefCell::new(HashMap::new());
    static PARTICLE_MATERIALS: RefCell<HashMap<(ShaderKey, String, u32, u32, u32), Gd<ShaderMaterial>>> = RefCell::new(HashMap::new());
    static QUAD: RefCell<Option<Gd<QuadMesh>>> = const { RefCell::new(None) };
    static MESH_ANIMATIONS: RefCell<HashMap<String, Option<Rc<MeshAnimationData>>>> = RefCell::new(HashMap::new());
}

fn random() -> f32 {
    RNG.with(|state| {
        let mut x = state.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        state.set(x);
        (x >> 40) as f32 / (1u64 << 24) as f32
    })
}

/// A random value in the range, written like the original engine (so it still works when
/// the range is backwards).
fn pick(range: &RangeInclusive<f32>) -> f32 {
    let (min, max) = (*range.start(), *range.end());
    if min == max { min } else { random() * (max - min).abs() + min }
}

fn rose_rotation(yaw: f32, pitch: f32, roll: f32) -> Basis {
    Basis::from_quaternion(
        Quaternion::from_axis_angle(Vector3::UP, yaw.to_radians())
            * Quaternion::from_axis_angle(Vector3::RIGHT, pitch.to_radians())
            * Quaternion::from_axis_angle(Vector3::BACK, roll.to_radians()),
    )
}

// ---------------------------------------------------------------------------------------
// Blending and shaders

/// How a ROSE blend (source factor, destination factor, operation) is drawn in Godot.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Blend {
    Opaque,
    Mix,
    Add,
    AddOne,
    AddSquared,
    Sub,
    SubOne,
    Screen,
    Multiply,
}

/// Factors: 1 Zero, 2 One, 3 SrcColor, 4 InvSrcColor, 5 SrcAlpha, 6 InvSrcAlpha, 7 DstAlpha,
/// 8 InvDstAlpha, 9 DstColor, 10 InvDstColor. Operations: 1 Add, 2 Sub, 3 RevSub, 4 Min, 5 Max.
/// With `alpha_is_one` the source alpha is taken as 1 first (meshes drawn without alpha).
fn resolve_blend(src: u32, dst: u32, op: u32, alpha_is_one: bool) -> Blend {
    let (src, dst) = if alpha_is_one {
        (match src { 5 | 11 => 2, 6 => 1, s => s }, match dst { 5 => 2, 6 => 1, d => d })
    } else {
        (src, dst)
    };
    match (src, dst, op) {
        (2 | 3, _, 3) => Blend::SubOne,
        (_, _, 3) => Blend::Sub,
        (2, 1, _) => Blend::Opaque,
        (2, 2, _) => Blend::AddOne,
        (3, 2, _) => Blend::AddSquared,
        (_, 2, _) => Blend::Add,
        (_, 4, _) => Blend::Screen,
        (1, 3, _) | (9, 1, _) | (1, 9, _) => Blend::Multiply,
        _ => Blend::Mix,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ShaderKey {
    particle: bool,
    mesh_animation: bool,
    blend: Blend,
    mask: bool,
    two_sided: bool,
    z_write: bool,
    z_test: bool,
}

fn shader_code(key: &ShaderKey) -> String {
    let mut modes = vec!["unshaded", "fog_disabled", "shadows_disabled"];
    modes.push(if key.two_sided || key.particle { "cull_disabled" } else { "cull_back" });
    modes.push(match (key.blend, key.z_write && !key.particle) {
        (_, false) => "depth_draw_never",
        (Blend::Opaque, true) => "depth_draw_opaque",
        _ => "depth_draw_always",
    });
    if !key.z_test {
        modes.push("depth_test_disabled");
    }
    match key.blend {
        Blend::Opaque => {}
        Blend::Mix => modes.push("blend_mix"),
        Blend::Add | Blend::AddOne | Blend::AddSquared => modes.push("blend_add"),
        Blend::Sub | Blend::SubOne => modes.push("blend_sub"),
        Blend::Screen => modes.push("blend_premul_alpha"),
        Blend::Multiply => modes.push("blend_mul"),
    }
    let mut code = format!("shader_type spatial;\nrender_mode {};\n", modes.join(", "));
    let mut define = |name: &str, on: bool| {
        if on {
            code.push_str(&format!("#define {name}\n"));
        }
    };
    define("PARTICLE", key.particle);
    define("MESH_ANIMATION", key.mesh_animation);
    define("COLOR_OPAQUE", key.blend == Blend::Opaque && !key.mask);
    define("COLOR_MASK", key.blend == Blend::Opaque && key.mask);
    define("COLOR_ONE", matches!(key.blend, Blend::AddOne | Blend::SubOne));
    define("COLOR_SQUARED", key.blend == Blend::AddSquared);
    define("COLOR_SCREEN", key.blend == Blend::Screen);
    code.push_str("#include \"res://shaders/rose_effect.gdshaderinc\"\n");
    code
}

fn shader(key: ShaderKey) -> Gd<Shader> {
    SHADERS.with_borrow_mut(|cache| {
        cache
            .entry(key)
            .or_insert_with(|| {
                let mut shader = Shader::new_gd();
                shader.set_code(&shader_code(&key));
                shader
            })
            .clone()
    })
}

fn quad() -> Gd<QuadMesh> {
    QUAD.with_borrow_mut(|quad| {
        quad.get_or_insert_with(|| {
            let mut mesh = QuadMesh::new_gd();
            mesh.set_size(Vector2::new(2.0, 2.0));
            mesh
        })
        .clone()
    })
}

// ---------------------------------------------------------------------------------------
// Animation timing (animation_state.rs)

struct Clock {
    fps: f64,
    frames: usize,
    max_loops: Option<usize>,
    delay: f32,
    started: bool,
    elapsed: f64,
    completed: bool,
    current: usize,
    next: usize,
    fract: f32,
}

impl Clock {
    fn new(fps: usize, frames: usize, repeat_count: u32, delay: f32) -> Self {
        Self {
            fps: fps.max(1) as f64,
            frames: frames.max(1),
            max_loops: if repeat_count == 0 { None } else { Some(repeat_count as usize) },
            delay,
            started: false,
            elapsed: 0.0,
            completed: false,
            current: 0,
            next: 0,
            fract: 0.0,
        }
    }

    /// Advances by `delta` seconds; false while it waits for its start delay.
    fn advance(&mut self, delta: f32) -> bool {
        if self.completed {
            return true;
        }
        if self.delay > 0.0 {
            self.delay -= delta;
            if self.delay > 0.0 {
                return false;
            }
        }
        if self.started {
            self.elapsed += delta as f64;
        }
        self.started = true;
        let frame = self.elapsed * self.fps;
        let loops = frame as usize / self.frames;
        if self.max_loops.is_some_and(|max| loops >= max) {
            self.completed = true;
            self.fract = 0.0;
            self.current = self.frames - 1;
            self.next = self.current;
        } else {
            self.fract = frame.fract() as f32;
            self.current = frame as usize % self.frames;
            let last_loop = self.max_loops.is_some_and(|max| loops + 1 >= max);
            self.next = if self.current + 1 == self.frames && last_loop { self.current } else { (self.current + 1) % self.frames };
        }
        true
    }
}

/// Position, rotation and scale keys of a ZMO's first channels (an effect's own motion).
struct TransformAnimation {
    clock: Clock,
    translation: Vec<Vector3>,
    rotation: Vec<Quaternion>,
    scale: Vec<f32>,
}

impl TransformAnimation {
    fn load(path: &str, repeat_count: u32) -> Option<Self> {
        let zmo = data::read_file::<ZmoFile>(path)?;
        let mut animation = Self {
            clock: Clock::new(zmo.fps, zmo.num_frames, repeat_count, 0.0),
            translation: Vec::new(),
            rotation: Vec::new(),
            scale: Vec::new(),
        };
        for (_, channel) in zmo.channels.iter() {
            match channel {
                ZmoChannel::Position(keys) if animation.translation.is_empty() => {
                    animation.translation = keys.iter().map(|p| Vector3::new(p.x, p.z, -p.y) / 100.0).collect();
                }
                ZmoChannel::Rotation(keys) if animation.rotation.is_empty() => {
                    animation.rotation = keys.iter().map(|r| Quaternion::new(r.x, r.z, -r.y, r.w).normalized()).collect();
                }
                ZmoChannel::Scale(keys) if animation.scale.is_empty() => animation.scale = keys.clone(),
                _ => {}
            }
        }
        Some(animation)
    }

    fn apply(&mut self, delta: f32, node: &mut Gd<Node3D>) {
        if self.clock.completed || !self.clock.advance(delta) {
            return;
        }
        let (a, b, t) = (self.clock.current, self.clock.next, self.clock.fract);
        if let (Some(p0), Some(p1)) = (self.translation.get(a), self.translation.get(b)) {
            node.set_position(p0.lerp(*p1, t));
        }
        if let (Some(r0), Some(r1)) = (self.rotation.get(a), self.rotation.get(b)) {
            node.set_quaternion(r0.slerp(*r1, t));
        }
        if let (Some(s0), Some(s1)) = (self.scale.get(a), self.scale.get(b)) {
            node.set_scale(Vector3::splat(s0 + (s1 - s0) * t));
        }
    }
}

// ---------------------------------------------------------------------------------------
// Particles (particle_sequence.rs, particle_sequence_system.rs)

/// Indices of the animated particle values: size x/y, colour rgba, velocity xyz,
/// texture atlas index and rotation (degrees).
const SIZE_X: usize = 0;
const SIZE_Y: usize = 1;
const RED: usize = 2;
const VELOCITY_X: usize = 6;
const TEXTURE: usize = 9;
const ROTATION: usize = 10;
const VALUES: usize = 11;

struct Keyframe {
    start_time: f32,
    fade: bool,
    next_fade: Option<usize>,
    data: PtlKeyframeData,
}

/// The particle values a keyframe sets, with the range each is picked from.
fn keyframe_values(data: &PtlKeyframeData) -> Vec<(usize, &RangeInclusive<f32>)> {
    match data {
        PtlKeyframeData::SizeXY(x, y) => vec![(SIZE_X, x), (SIZE_Y, y)],
        PtlKeyframeData::Timer(_) => vec![],
        PtlKeyframeData::Red(v) => vec![(RED, v)],
        PtlKeyframeData::Green(v) => vec![(RED + 1, v)],
        PtlKeyframeData::Blue(v) => vec![(RED + 2, v)],
        PtlKeyframeData::Alpha(v) => vec![(RED + 3, v)],
        PtlKeyframeData::ColourRGBA(r, g, b, a) => vec![(RED, r), (RED + 1, g), (RED + 2, b), (RED + 3, a)],
        PtlKeyframeData::VelocityX(v) => vec![(VELOCITY_X, v)],
        PtlKeyframeData::VelocityY(v) => vec![(VELOCITY_X + 1, v)],
        PtlKeyframeData::VelocityZ(v) => vec![(VELOCITY_X + 2, v)],
        PtlKeyframeData::VelocityXYZ(x, y, z) => vec![(VELOCITY_X, x), (VELOCITY_X + 1, y), (VELOCITY_X + 2, z)],
        PtlKeyframeData::Texture(v) => vec![(TEXTURE, v)],
        PtlKeyframeData::Rotation(v) => vec![(ROTATION, v)],
    }
}

struct Particle {
    age: f32,
    life: f32,
    keyframe_timer: f32,
    next_keyframe: usize,
    gravity_local: Vector3,
    /// Emitted in world space: the emitter's rotation (in ROSE axes) at that moment.
    world_direction: Option<Quaternion>,
    /// ROSE axes, centimetres.
    position: Vector3,
    values: [f32; VALUES],
    steps: [f32; VALUES],
}

struct Sequence {
    emit_rate: RangeInclusive<f32>,
    life: RangeInclusive<f32>,
    emit_radius: [RangeInclusive<f32>; 3],
    gravity: [RangeInclusive<f32>; 3],
    keyframes: Vec<Keyframe>,
    update_coords: PtlUpdateCoords,
    num_loops: i32,
    num_particles: usize,
    start_delay: f32,
    emit_counter: f32,
    num_emitted: usize,
    finished: bool,
    particles: Vec<Particle>,
    /// The emitter: its global transform places new particles.
    anchor: Gd<Node3D>,
    animation: Option<TransformAnimation>,
    instance: Gd<MultiMeshInstance3D>,
    multimesh: Gd<MultiMesh>,
}

impl Sequence {
    fn done(&self) -> bool {
        (self.finished && self.particles.is_empty()) || self.animation.as_ref().is_some_and(|a| a.clock.completed)
    }

    fn apply_keyframes(&mut self, index: usize) {
        let Self { keyframes, particles, .. } = self;
        let particle = &mut particles[index];
        let mut k = particle.next_keyframe;
        while k < keyframes.len() && keyframes[k].start_time <= particle.keyframe_timer {
            let keyframe = &keyframes[k];
            if let PtlKeyframeData::Timer(range) = &keyframe.data {
                // Jump the timer; the keyframes it skips to apply next frame.
                particle.keyframe_timer = pick(range);
                particle.next_keyframe = keyframes.iter().filter(|k| k.start_time <= particle.keyframe_timer).count();
                return;
            }
            let values = keyframe_values(&keyframe.data);
            if !keyframe.fade {
                for (i, range) in values.iter() {
                    particle.values[*i] = pick(range);
                }
            }
            if let Some(next) = keyframe.next_fade.map(|n| &keyframes[n]) {
                let dt = next.start_time - keyframe.start_time;
                if dt > 0.0 {
                    for ((i, _), (_, next_range)) in values.iter().zip(keyframe_values(&next.data).iter()) {
                        particle.steps[*i] = (pick(next_range) - particle.values[*i]) / dt;
                    }
                }
            }
            k += 1;
            particle.next_keyframe = k;
        }
    }

    fn update(&mut self, delta: f32) {
        if self.start_delay > 0.0 {
            self.start_delay -= delta;
            if self.start_delay > 0.0 {
                return;
            }
            self.start_delay = 0.0;
        }
        if let Some(animation) = self.animation.as_mut() {
            animation.apply(delta, &mut self.anchor);
        }
        let world = matches!(self.update_coords, PtlUpdateCoords::World);
        let timestep = PARTICLE_TIME_SCALE * delta;

        for index in 0..self.particles.len() {
            let particle = &mut self.particles[index];
            particle.age += timestep;
            if particle.age >= particle.life {
                continue;
            }
            particle.keyframe_timer += timestep;
            let velocity = Vector3::new(particle.values[VELOCITY_X], particle.values[VELOCITY_X + 1], particle.values[VELOCITY_X + 2]);
            particle.position += match particle.world_direction {
                Some(direction) => direction * velocity,
                None => velocity,
            } * timestep;
            for i in 0..VALUES {
                particle.values[i] += particle.steps[i] * timestep;
            }
            while particle.values[ROTATION] > 360.0 {
                particle.values[ROTATION] -= 360.0;
            }
            let gravity = if world {
                particle.gravity_local * PARTICLE_TIME_SCALE
            } else {
                Vector3::new(pick(&self.gravity[0]), pick(&self.gravity[1]), pick(&self.gravity[2]))
            };
            let particle = &mut self.particles[index];
            for axis in 0..3 {
                particle.values[VELOCITY_X + axis] += [gravity.x, gravity.y, gravity.z][axis] * delta;
            }
            self.apply_keyframes(index);
        }
        self.particles.retain(|p| p.age < p.life);

        let global = self.anchor.get_global_transform();
        if !self.finished {
            self.emit_counter += delta * pick(&self.emit_rate);
            if self.num_loops > 0 {
                let limit = self.num_loops as usize * self.num_particles;
                let remaining = limit.saturating_sub(self.num_emitted);
                if self.emit_counter as usize >= remaining {
                    self.finished = true;
                    self.emit_counter = remaining as f32 + 0.1;
                }
            }
            while self.emit_counter > 1.0 && self.particles.len() < self.num_particles {
                let mut position = Vector3::new(pick(&self.emit_radius[0]), pick(&self.emit_radius[1]), pick(&self.emit_radius[2]));
                let mut gravity_local = Vector3::ZERO;
                let mut world_direction = None;
                if world {
                    let r = global.basis.get_quaternion();
                    let rotation = Quaternion::new(r.x, -r.z, r.y, r.w);
                    world_direction = Some(rotation);
                    let gravity = Vector3::new(pick(&self.gravity[0]), pick(&self.gravity[1]), pick(&self.gravity[2]));
                    gravity_local = rotation.inverse() * gravity;
                    position = rotation * position;
                    position += Vector3::new(global.origin.x, -global.origin.z, global.origin.y) * 100.0;
                }
                let mut values = [0.0; VALUES];
                values[SIZE_X] = 10.0;
                values[SIZE_Y] = 10.0;
                values[RED..RED + 4].copy_from_slice(&[1.0; 4]);
                self.particles.push(Particle {
                    age: 0.0,
                    life: pick(&self.life),
                    keyframe_timer: 0.0,
                    next_keyframe: 0,
                    gravity_local,
                    world_direction,
                    position,
                    values,
                    steps: [0.0; VALUES],
                });
                let index = self.particles.len() - 1;
                self.apply_keyframes(index);
                self.num_emitted += 1;
                self.emit_counter -= 1.0;
            }
        }
        self.draw(global);
    }

    fn draw(&mut self, global: Transform3D) {
        let to_world = match self.update_coords {
            PtlUpdateCoords::World => Transform3D::IDENTITY,
            PtlUpdateCoords::LocalPosition => Transform3D::new(Basis::IDENTITY, global.origin),
            PtlUpdateCoords::Local => global,
        };
        let count = self.particles.len().min(self.num_particles);
        let mut buffer = vec![0.0f32; self.num_particles.max(1) * 20];
        let mut bounds: Option<Aabb> = None;
        for (i, particle) in self.particles.iter().take(count).enumerate() {
            let p = particle.position;
            let at = to_world * (Vector3::new(p.x, p.z, -p.y) / 100.0);
            let v = &particle.values;
            let size = Vector2::new(v[SIZE_X], v[SIZE_Y]) / 100.0;
            let o = i * 20;
            buffer[o..o + 12].copy_from_slice(&[1.0, 0.0, 0.0, at.x, 0.0, 1.0, 0.0, at.y, 0.0, 0.0, 1.0, at.z]);
            buffer[o + 12..o + 16].copy_from_slice(&v[RED..RED + 4]);
            buffer[o + 16..o + 20].copy_from_slice(&[size.x, size.y, v[ROTATION].to_radians(), v[TEXTURE]]);
            let reach = size.x.abs().max(size.y.abs()) * 1.5;
            let around = Aabb::new(at - Vector3::splat(reach), Vector3::splat(reach * 2.0));
            bounds = Some(bounds.map_or(around, |b| b.merge(around)));
        }
        self.multimesh.set_buffer(&PackedFloat32Array::from(buffer.as_slice()));
        self.multimesh.set_visible_instance_count(count as i32);
        if let Some(bounds) = bounds {
            self.instance.set_custom_aabb(bounds);
        }
    }
}

// ---------------------------------------------------------------------------------------
// Meshes (mesh_animation.rs, zmo_asset_loader.rs's animation texture)

struct MeshAnimationData {
    texture: Gd<ImageTexture>,
    fps: usize,
    frames: usize,
    flags: i32,
    alphas: Vec<f32>,
}

/// Bakes a vertex animation ZMO into a float texture: one row per vertex, one column per
/// frame of position + uv.x, then (with normals or uvs) one per frame of normal + uv.y.
fn mesh_animation(path: &str) -> Option<Rc<MeshAnimationData>> {
    let key = path.to_ascii_uppercase();
    if let Some(cached) = MESH_ANIMATIONS.with_borrow(|cache| cache.get(&key).cloned()) {
        return cached;
    }
    let baked = data::read_file::<ZmoFile>(path).and_then(|zmo| {
        let mut vertices = 0;
        let mut flags = 0;
        for (vertex, channel) in zmo.channels.iter() {
            vertices = vertices.max(*vertex as usize + 1);
            flags |= match channel {
                ZmoChannel::Position(_) => 1,
                ZmoChannel::Normal(_) => 2,
                ZmoChannel::UV1(_) => 4,
                ZmoChannel::Alpha(_) => 8,
                _ => 0,
            };
        }
        let frames = zmo.num_frames.max(1);
        let stride = if flags & 6 != 0 { frames * 2 } else { frames };
        let mut pixels = vec![0.0f32; vertices.max(1) * stride * 4];
        let mut alphas = Vec::new();
        for (vertex, channel) in zmo.channels.iter() {
            let row = *vertex as usize * stride * 4;
            match channel {
                ZmoChannel::Position(values) => {
                    for (x, p) in values.iter().enumerate().take(frames) {
                        pixels[row + x * 4..row + x * 4 + 3].copy_from_slice(&[p.x / 100.0, p.z / 100.0, -p.y / 100.0]);
                    }
                }
                ZmoChannel::Normal(values) => {
                    for (x, n) in values.iter().enumerate().take(frames) {
                        let o = row + (frames + x) * 4;
                        pixels[o..o + 3].copy_from_slice(&[n.x, n.z, -n.y]);
                    }
                }
                ZmoChannel::UV1(values) => {
                    for (x, uv) in values.iter().enumerate().take(frames) {
                        pixels[row + x * 4 + 3] = uv.x;
                        pixels[row + (frames + x) * 4 + 3] = uv.y;
                    }
                }
                ZmoChannel::Alpha(values) => alphas = values.clone(),
                _ => {}
            }
        }
        let bytes: Vec<u8> = pixels.iter().flat_map(|f| f.to_le_bytes()).collect();
        let image = Image::create_from_data(stride as i32, vertices.max(1) as i32, false, Format::RGBAF, &PackedByteArray::from(bytes.as_slice()))?;
        let texture = ImageTexture::create_from_image(&image)?;
        Some(Rc::new(MeshAnimationData { texture, fps: zmo.fps, frames, flags, alphas }))
    });
    MESH_ANIMATIONS.with_borrow_mut(|cache| cache.insert(key, baked.clone()));
    baked
}

struct EffectMesh {
    node: Gd<Node3D>,
    material: Gd<ShaderMaterial>,
    animation: Option<(Clock, Rc<MeshAnimationData>)>,
    motion: Option<TransformAnimation>,
}

impl EffectMesh {
    fn done(&self) -> bool {
        self.animation.as_ref().is_none_or(|(clock, _)| clock.completed)
    }

    fn update(&mut self, delta: f32) {
        if let Some(motion) = self.motion.as_mut() {
            motion.apply(delta, &mut self.node);
        }
        let Some((clock, data)) = self.animation.as_mut() else { return };
        if clock.completed {
            return;
        }
        if !clock.advance(delta) {
            return;
        }
        self.node.set_visible(true);
        let m = &mut self.material;
        m.set_shader_parameter("current_frame", &(clock.current as i32).to_variant());
        m.set_shader_parameter("next_frame", &(clock.next as i32).to_variant());
        m.set_shader_parameter("next_weight", &clock.fract.to_variant());
        if let (Some(a), Some(b)) = (data.alphas.get(clock.current), data.alphas.get(clock.next)) {
            m.set_shader_parameter("animation_alpha", &(a * (1.0 - clock.fract) + b * clock.fract).to_variant());
        }
    }
}

// ---------------------------------------------------------------------------------------
// The node

/// One EFT effect. Add it to the scene, call `load`, place it; it plays and frees itself
/// once every particle system and mesh animation in it has finished (effects that loop
/// forever, like torches in a zone, never do).
#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct RoseEffect {
    base: Base<Node3D>,
    sequences: Vec<Sequence>,
    meshes: Vec<EffectMesh>,
    loaded: bool,
}

#[godot_api]
impl INode3D for RoseEffect {
    fn process(&mut self, delta: f64) {
        if !self.loaded || !self.base().is_visible_in_tree() {
            return;
        }
        let delta = delta as f32;
        for sequence in self.sequences.iter_mut() {
            sequence.update(delta);
        }
        for mesh in self.meshes.iter_mut() {
            mesh.update(delta);
        }
        let parts = self.sequences.len() + self.meshes.len();
        let done = self.sequences.iter().filter(|s| s.done()).count() + self.meshes.iter().filter(|m| m.done()).count();
        if parts == 0 || done == parts {
            self.loaded = false;
            self.base_mut().queue_free();
        }
    }
}

#[godot_api]
impl RoseEffect {
    /// Loads an EFT file by path (e.g. 3DDATA/EFFECT/LEVELUP_01.EFT). Returns false if it
    /// is missing or empty.
    #[func]
    fn load(&mut self, path: GString) -> bool {
        self.load_path(&path.to_string())
    }

    /// Loads an effect by its FILE_EFFECT.STB row.
    #[func]
    fn load_id(&mut self, file_id: i32) -> bool {
        match effect_file(file_id) {
            Some(path) => self.load_path(&path),
            None => false,
        }
    }

    /// Whether `load` found anything to show.
    #[func]
    fn is_loaded(&self) -> bool {
        self.loaded
    }
}

impl RoseEffect {
    pub fn load_path(&mut self, path: &str) -> bool {
        let Some(eft) = data::read_file::<EftFile>(path) else { return false };

        for eft_particle in eft.particles.iter() {
            let Some(ptl) = data::read_file::<PtlFile>(&eft_particle.particle_file.path().to_string_lossy()) else { continue };
            let mut holder = Node3D::new_alloc();
            holder.set_position(Vector3::new(eft_particle.position.x, eft_particle.position.z, -eft_particle.position.y) / 100.0);
            holder.set_basis(rose_rotation(eft_particle.yaw, eft_particle.pitch, eft_particle.roll));
            self.base_mut().add_child(&holder);

            for sequence in ptl.sequences {
                let mut anchor = Node3D::new_alloc();
                holder.add_child(&anchor);
                let key = ShaderKey {
                    particle: true,
                    mesh_animation: false,
                    blend: resolve_blend(sequence.src_blend_mode, sequence.dst_blend_mode, sequence.blend_op, false),
                    mask: false,
                    two_sided: true,
                    z_write: false,
                    z_test: true,
                };
                let texture_path = sequence.texture_path.path().to_string_lossy().to_string();
                let billboard = match sequence.align_type { 1 => 1, 2 => 2, _ => 0 };
                let (cols, rows) = (sequence.texture_atlas_cols.max(1), sequence.texture_atlas_rows.max(1));
                let material = PARTICLE_MATERIALS.with_borrow_mut(|cache| {
                    cache
                        .entry((key, texture_path.to_ascii_uppercase(), billboard, cols, rows))
                        .or_insert_with(|| {
                            let mut material = ShaderMaterial::new_gd();
                            material.set_shader(&shader(key));
                            if let Some(texture) = texture::load_texture(&texture_path) {
                                material.set_shader_parameter("base_texture", &texture.to_variant());
                            }
                            material.set_shader_parameter("billboard", &(billboard as i32).to_variant());
                            material.set_shader_parameter("atlas_size", &Vector2::new(cols as f32, rows as f32).to_variant());
                            material
                        })
                        .clone()
                });
                let num_particles = sequence.num_particles.max(0) as usize;
                let mut multimesh = MultiMesh::new_gd();
                multimesh.set_transform_format(TransformFormat::TRANSFORM_3D);
                multimesh.set_use_colors(true);
                multimesh.set_use_custom_data(true);
                multimesh.set_mesh(&quad());
                multimesh.set_instance_count(num_particles.max(1) as i32);
                multimesh.set_visible_instance_count(0);
                let mut instance = MultiMeshInstance3D::new_alloc();
                instance.set_multimesh(&multimesh);
                instance.set_as_top_level(true);
                instance.set_material_override(&material);
                instance.upcast_mut::<GeometryInstance3D>().set_cast_shadows_setting(godot::classes::geometry_instance_3d::ShadowCastingSetting::OFF);
                anchor.add_child(&instance);

                let mut keyframes: Vec<Keyframe> = sequence
                    .keyframes
                    .into_iter()
                    .map(|k| Keyframe { start_time: pick(&k.start_time), fade: k.fade, next_fade: None, data: k.data })
                    .collect();
                keyframes.sort_by(|a, b| a.start_time.total_cmp(&b.start_time));
                for i in 0..keyframes.len() {
                    keyframes[i].next_fade = (i + 1..keyframes.len()).find(|&j| {
                        keyframes[j].fade && std::mem::discriminant(&keyframes[i].data) == std::mem::discriminant(&keyframes[j].data)
                    });
                }
                let animation = eft_particle
                    .animation_file
                    .as_ref()
                    .and_then(|path| TransformAnimation::load(&path.path().to_string_lossy(), eft_particle.animation_repeat_count));
                self.sequences.push(Sequence {
                    emit_rate: sequence.emit_rate,
                    life: sequence.life,
                    emit_radius: [sequence.emit_radius_x, sequence.emit_radius_y, sequence.emit_radius_z],
                    gravity: [sequence.gravity_x, sequence.gravity_y, sequence.gravity_z],
                    keyframes,
                    update_coords: sequence.update_coords,
                    num_loops: sequence.num_loops,
                    num_particles,
                    start_delay: eft_particle.start_delay as f32 / 1000.0,
                    emit_counter: 0.0,
                    num_emitted: 0,
                    finished: false,
                    particles: Vec::with_capacity(num_particles),
                    anchor,
                    animation,
                    instance,
                    multimesh,
                });
            }
        }

        for eft_mesh in eft.meshes.iter() {
            let Some(effect_mesh) = mesh::load_mesh(&eft_mesh.mesh_file.path().to_string_lossy(), false) else { continue };
            let mut holder = Node3D::new_alloc();
            holder.set_position(Vector3::new(eft_mesh.position.x, eft_mesh.position.z, -eft_mesh.position.y) / 100.0);
            holder.set_basis(rose_rotation(eft_mesh.yaw, eft_mesh.pitch, eft_mesh.roll));
            self.base_mut().add_child(&holder);

            let animation = eft_mesh.mesh_animation_file.as_ref().and_then(|path| mesh_animation(&path.path().to_string_lossy()));
            let blended = eft_mesh.alpha_enabled || !eft_mesh.depth_write_enabled;
            let key = ShaderKey {
                particle: false,
                mesh_animation: animation.is_some(),
                blend: resolve_blend(eft_mesh.src_blend_factor, eft_mesh.dst_blend_factor, eft_mesh.blend_op, !blended),
                mask: !blended && eft_mesh.alpha_test_enabled,
                two_sided: eft_mesh.two_sided,
                z_write: eft_mesh.depth_write_enabled,
                z_test: eft_mesh.depth_test_enabled,
            };
            let mut material = ShaderMaterial::new_gd();
            material.set_shader(&shader(key));
            if let Some(texture) = texture::load_texture(&eft_mesh.mesh_texture_file.path().to_string_lossy()) {
                material.set_shader_parameter("base_texture", &texture.to_variant());
            }
            let mut instance = MeshInstance3D::new_alloc();
            instance.set_mesh(&effect_mesh);
            instance.set_material_override(&material);
            instance.upcast_mut::<GeometryInstance3D>().set_cast_shadows_setting(godot::classes::geometry_instance_3d::ShadowCastingSetting::OFF);
            let animation = animation.map(|data| {
                material.set_shader_parameter("animation_texture", &data.texture.to_variant());
                material.set_shader_parameter("animation_flags", &data.flags.to_variant());
                material.set_shader_parameter("animation_frames", &(data.frames as i32).to_variant());
                // Vertex animation moves the mesh beyond its rest bounds.
                instance.set_extra_cull_margin(20.0);
                let clock = Clock::new(data.fps, data.frames, eft_mesh.repeat_count, eft_mesh.start_delay as f32 / 1000.0);
                (clock, data)
            });
            // A mesh waiting for its start delay is not drawn yet.
            if animation.as_ref().is_some_and(|(clock, _)| clock.delay > 0.0) {
                instance.set_visible(false);
            }
            holder.add_child(&instance);
            let motion = eft_mesh
                .animation_file
                .as_ref()
                .and_then(|path| TransformAnimation::load(&path.path().to_string_lossy(), eft_mesh.animation_repeat_count));
            self.meshes.push(EffectMesh { node: instance.upcast(), material, animation, motion });
        }

        self.loaded = !self.sequences.is_empty() || !self.meshes.is_empty();
        self.loaded
    }
}

/// An effect file path by FILE_EFFECT.STB row.
pub fn effect_file(file_id: i32) -> Option<String> {
    let id = rose_data::EffectFileId::new(u16::try_from(file_id).ok()?)?;
    Some(data::get()?.effects.get_effect_file(id)?.path().to_string_lossy().to_string())
}

/// Drops the cached shaders, materials and animation textures while the engine runs.
pub fn clear_cache() {
    SHADERS.with_borrow_mut(|cache| cache.clear());
    PARTICLE_MATERIALS.with_borrow_mut(|cache| cache.clear());
    QUAD.with_borrow_mut(|quad| *quad = None);
    MESH_ANIMATIONS.with_borrow_mut(|cache| cache.clear());
}

// ---------------------------------------------------------------------------------------
// Which effect each game event shows (animation_effect_system.rs, hit_event_system.rs,
// client_entity_event_system.rs)

use rose_data::{EffectBulletMoveType, EffectData, EffectId, ItemClass, NpcId, SkillId, SkillType};

fn file_path(id: Option<rose_data::EffectFileId>) -> GString {
    id.and_then(|id| data::get()?.effects.get_effect_file(id))
        .map_or(GString::new(), |path| GString::from(path.path().to_string_lossy().as_ref()))
}

fn effect_data(id: Option<EffectId>) -> Option<&'static EffectData> {
    data::get()?.effects.get_effect(id?)
}

/// A projectile or hit effect as GDScript reads it: bullet (EFT path, "" for none), speed
/// (m/s), move ("linear", "parabola" or "immediate"), hit and hit_critical (EFT paths),
/// hit_sound (AudioStream or null).
fn effect_dict(effect: &EffectData) -> VarDictionary {
    let mut d = VarDictionary::new();
    d.set("bullet", &file_path(effect.bullet_effect));
    d.set("speed", effect.bullet_speed / 100.0);
    d.set(
        "move",
        match effect.bullet_move_type {
            Some(EffectBulletMoveType::Parabola) => "parabola",
            Some(EffectBulletMoveType::Immediate) => "immediate",
            _ => "linear",
        },
    );
    d.set("hit", &file_path(effect.hit_effect_normal));
    d.set("hit_critical", &file_path(effect.hit_effect_critical));
    match crate::audio::sound(effect.hit_sound_id) {
        Some(stream) => d.set("hit_sound", &stream),
        None => d.set("hit_sound", &Variant::nil()),
    }
    d
}

fn npc_data(npc_id: i32) -> Option<&'static rose_data::NpcData> {
    data::get()?.npcs.get_npc(NpcId::new(u16::try_from(npc_id).ok()?)?)
}

/// Effect lookups for GDScript. Paths are "" and dictionaries empty when there is none.
#[derive(GodotClass)]
#[class(base=RefCounted, init)]
pub struct RoseFx {
    base: Base<RefCounted>,
}

#[godot_api]
impl RoseFx {
    /// An effect file path by FILE_EFFECT.STB row.
    #[func]
    fn effect_file(file_id: i32) -> GString {
        effect_file(file_id).map_or(GString::new(), |p| GString::from(p.as_str()))
    }

    /// What a melee hit shows on its target: the weapon's effect, else the monster's
    /// bare-hand effect (weapon 0 for monsters).
    #[func]
    fn weapon_hit(weapon: i32, npc_id: i32) -> VarDictionary {
        let Some(game_data) = data::get() else { return VarDictionary::new() };
        let id = (weapon > 0)
            .then(|| game_data.items.get_weapon_item(weapon as usize))
            .flatten()
            .and_then(|w| w.effect_id)
            .or_else(|| npc_data(npc_id).and_then(|n| n.hand_hit_effect_id));
        effect_data(id).map_or(VarDictionary::new(), effect_dict)
    }

    /// The projectile a bow, crossbow, gun or launcher fires. Ammunition is not known for
    /// other players, so the first ammunition of the weapon's kind stands in for it.
    #[func]
    fn weapon_projectile(weapon: i32) -> VarDictionary {
        let Some(game_data) = data::get() else { return VarDictionary::new() };
        let Some(weapon) = game_data.items.get_weapon_item(weapon.max(0) as usize) else { return VarDictionary::new() };
        let ammo_class = match weapon.item_data.class {
            ItemClass::Bow | ItemClass::Crossbow => Some(ItemClass::Arrow),
            ItemClass::Gun | ItemClass::DualGuns => Some(ItemClass::Bullet),
            ItemClass::Launcher => Some(ItemClass::Shell),
            _ => None,
        };
        let ammo_effect = ammo_class.and_then(|class| {
            (1..1000)
                .filter_map(|id| game_data.items.get_material_item(id))
                .find(|m| m.item_data.class == class && m.bullet_effect_id.is_some())
                .and_then(|m| m.bullet_effect_id)
        });
        effect_data(ammo_effect.or(weapon.bullet_effect_id)).map_or(VarDictionary::new(), effect_dict)
    }

    /// A skill's effects: type, bullet (a projectile dictionary or empty), bullet_bone,
    /// self_effect (EFT a self-bound skill shows on its caster), hit and hit_bone,
    /// dummy_hits (two EFT paths) and casting (four {path, bone}).
    #[func]
    fn skill(skill_id: i32) -> VarDictionary {
        let mut d = VarDictionary::new();
        let Some(skill) = u16::try_from(skill_id).ok().and_then(SkillId::new).and_then(|id| data::get()?.skills.get_skill(id)) else {
            return d;
        };
        let kind = match skill.skill_type {
            SkillType::SelfBound | SkillType::SelfBoundDuration | SkillType::SelfStateDuration | SkillType::SelfDamage => "self",
            SkillType::FireBullet => "bullet",
            SkillType::TargetBound | SkillType::TargetBoundDuration | SkillType::TargetStateDuration | SkillType::Resurrection => "target",
            _ => "other",
        };
        d.set("type", kind);
        let bullet = effect_data(skill.bullet_effect_id);
        d.set("bullet", &bullet.map_or(VarDictionary::new(), effect_dict));
        d.set("bullet_bone", skill.bullet_link_dummy_bone_id as i64);
        d.set("self_effect", &file_path(bullet.and_then(|b| b.bullet_effect)));
        d.set("hit", &file_path(skill.hit_effect_file_id));
        d.set("hit_bone", skill.hit_link_dummy_bone_id.map_or(-1, |b| b as i64));
        let mut dummies = PackedStringArray::new();
        for id in skill.hit_dummy_effect_file_id.iter() {
            dummies.push(&file_path(*id));
        }
        d.set("dummy_hits", &dummies);
        let mut casting = VarArray::new();
        for effect in skill.casting_effects.iter() {
            let mut c = VarDictionary::new();
            c.set("path", &file_path(effect.as_ref().map(|e| e.effect_file_id)));
            c.set("bone", effect.as_ref().and_then(|e| e.effect_dummy_bone_id).map_or(-1, |b| b as i64));
            casting.push(&c.to_variant());
        }
        d.set("casting", &casting);
        d
    }

    /// The effect a monster leaves when it dies.
    #[func]
    fn npc_die(npc_id: i32) -> GString {
        file_path(npc_data(npc_id).and_then(|n| n.die_effect_file_id))
    }
}
