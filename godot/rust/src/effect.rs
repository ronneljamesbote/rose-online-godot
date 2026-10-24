//! Effects as Godot scenes: a `RoseEffect` root with `RoseEffectParticles` (a particle
//! system, `RoseParticleSystem`, iROSE's PTL file) and `RoseEffectMesh` (an animated mesh)
//! children, which simulates itself and frees itself when everything in it has finished.
//! `--import-rose` writes one scene per EFT file (`res://assets/rose/3ddata/effect/*.scn`)
//! and one particle system per PTL file (`*.particles.res`); without the import the same
//! scene is built from the client's files when an effect is first shown.
//! Ported from rose-offline-client's effect_loader.rs, particle_sequence_system.rs,
//! mesh_animation.rs, transform_animation.rs and effect_system.rs.

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    ops::RangeInclusive,
};

use godot::{
    classes::{
        animation::TrackType, image::Format, multi_mesh::TransformFormat, node::InternalMode, Animation, ArrayMesh,
        Engine, GeometryInstance3D, INode3D, IResource, Image, ImageTexture, Mesh, MeshInstance3D, MultiMesh,
        MultiMeshInstance3D, Node3D, PackedScene, QuadMesh, Resource, ResourceLoader, Shader, ShaderMaterial, Texture2D,
    },
    global::Error,
    prelude::*,
};
use rose_file_readers::{EftFile, PtlFile, PtlKeyframeData, PtlSequence, PtlUpdateCoords, ZmoChannel, ZmoFile};

use crate::{assets, data, mesh, texture};

/// ROSE's particle timestep runs 4.8 times faster than real time.
const PARTICLE_TIME_SCALE: f32 = 4.8;

thread_local! {
    static RNG: Cell<u64> = Cell::new(0x9E37_79B9_7F4A_7C15 ^ std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).map_or(1, |d| d.as_nanos() as u64));
    static SHADERS: RefCell<HashMap<ShaderKey, Gd<Shader>>> = RefCell::new(HashMap::new());
    static PARTICLE_MATERIALS: RefCell<HashMap<(ShaderKey, i64, u32, u32, u32), Gd<ShaderMaterial>>> = RefCell::new(HashMap::new());
    static QUAD: RefCell<Option<Gd<QuadMesh>>> = const { RefCell::new(None) };
    /// What the client's files became when there is no imported copy, by client path.
    static SCENES: RefCell<HashMap<String, Option<Gd<PackedScene>>>> = RefCell::new(HashMap::new());
    static PARTICLE_SYSTEMS: RefCell<HashMap<String, Option<Gd<RoseParticleSystem>>>> = RefCell::new(HashMap::new());
    static MOTIONS: RefCell<HashMap<String, Option<Gd<Animation>>>> = RefCell::new(HashMap::new());
    static MESH_ANIMATIONS: RefCell<HashMap<String, Option<Gd<RoseMeshAnimation>>>> = RefCell::new(HashMap::new());
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

/// Position, rotation and scale keys of an effect's own motion (a part's `motion`).
struct TransformAnimation {
    clock: Clock,
    translation: Vec<Vector3>,
    rotation: Vec<Quaternion>,
    scale: Vec<f32>,
}

impl TransformAnimation {
    /// The keys of `motion`'s position, rotation and scale tracks, one key per frame.
    fn from_motion(motion: &Gd<Animation>, repeat_count: u32) -> Option<Self> {
        let step = motion.get_step();
        let fps = if step > 0.0 { (1.0 / step).round() as usize } else { 30 };
        let mut animation = Self { clock: Clock::new(fps, 1, repeat_count, 0.0), translation: Vec::new(), rotation: Vec::new(), scale: Vec::new() };
        let mut frames = 0;
        for track in 0..motion.get_track_count() {
            let keys = motion.track_get_key_count(track);
            let values = (0..keys).map(|k| motion.track_get_key_value(track, k));
            match motion.track_get_type(track) {
                TrackType::POSITION_3D if animation.translation.is_empty() => animation.translation = values.filter_map(|v| v.try_to::<Vector3>().ok()).collect(),
                TrackType::ROTATION_3D if animation.rotation.is_empty() => animation.rotation = values.filter_map(|v| v.try_to::<Quaternion>().ok()).collect(),
                TrackType::SCALE_3D if animation.scale.is_empty() => animation.scale = values.filter_map(|v| v.try_to::<Vector3>().ok()).map(|v| v.x).collect(),
                _ => continue,
            }
            frames = frames.max(keys as usize);
        }
        let frames = frames.max((motion.get_length() * fps as f64).round() as usize);
        if frames == 0 {
            return None;
        }
        animation.clock = Clock::new(fps, frames, repeat_count, 0.0);
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
// The resources an effect scene links: particle systems and baked mesh animations

/// How a ROSE blend factor is named in the Inspector (the numbers the files store).
const BLEND_FACTORS: &str = "Zero:1,One:2,Source colour:3,One minus source colour:4,Source alpha:5,One minus source alpha:6,Destination alpha:7,One minus destination alpha:8,Destination colour:9,One minus destination colour:10,Source alpha saturate:11";
const BLEND_OPS: &str = "Add:1,Subtract:2,Reverse subtract:3,Min:4,Max:5";

/// Gives the blend fields of the effect classes their dropdowns.
fn blend_hint(property: &mut godot::register::info::PropertyInfo) {
    let name = property.property_name.to_string();
    let options = if name.ends_with("_blend") {
        BLEND_FACTORS
    } else if name == "blend_op" {
        BLEND_OPS
    } else {
        return;
    };
    property.hint_info.hint = godot::register::info::PropertyHint::ENUM;
    property.hint_info.hint_string = options.into();
}

/// A particle system: what an effect's `RoseEffectParticles` emits (iROSE's PTL file). Each
/// sequence is one emitter with its own texture. Several effects can share one system, so a
/// change here shows in all of them; to change it for one effect only, make it unique
/// (right-click the field) first. The imported ones are
/// `res://assets/rose/3ddata/effect/particles/*.particles.res`.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseParticleSystem {
    /// The emitters, drawn together.
    #[export]
    sequences: Array<Option<Gd<RoseParticleSequence>>>,
    base: Base<Resource>,
}

/// One emitter of a particle system: how many particles it sends out, where, how long they
/// live, and how they change over their life (the keyframes). Distances are in centimetres
/// and times in ROSE particle time, which runs 4.8 times faster than real seconds, as in the
/// PTL files. A value given as a range (x to y) is picked at random in it for each particle.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseParticleSequence {
    /// For finding it only.
    #[export]
    name: GString,
    /// The particles' picture. With an atlas (columns and rows above 1) a particle shows one
    /// cell of it, picked by the Texture keyframes.
    #[export_group(name = "Look")]
    #[export]
    texture: Option<Gd<Texture2D>>,
    /// Cells across the texture.
    #[export(range = (1.0, 16.0, 1.0, or_greater))]
    #[init(val = 1)]
    atlas_columns: i64,
    /// Cells down the texture.
    #[export(range = (1.0, 16.0, 1.0, or_greater))]
    #[init(val = 1)]
    atlas_rows: i64,
    /// How a particle faces: the camera, lying flat, or upright turning about the vertical
    /// axis only.
    #[export(enum = (FaceCamera = 0, Flat = 1, Upright = 2))]
    facing: i64,
    /// The source blend factor (with the destination factor and operation: how the particles
    /// mix with what is behind them; Source alpha + One is the usual glow).
    #[export]
    #[init(val = 5)]
    source_blend: i64,
    /// The destination blend factor.
    #[export]
    #[init(val = 6)]
    destination_blend: i64,
    /// The blend operation.
    #[export]
    #[init(val = 1)]
    blend_op: i64,

    /// The most particles alive at once.
    #[export_group(name = "Emitting")]
    #[export(range = (0.0, 1000.0, 1.0, or_greater))]
    #[init(val = 10)]
    particles: i64,
    /// Particles sent out per second (picked between x and y).
    #[export]
    emit_rate: Vector2,
    /// How many times the emitter sends out its `particles` before it stops (0 = forever).
    #[export]
    loops: i64,
    /// How long a particle lives (particle time).
    #[export]
    life: Vector2,
    /// Where a particle starts, sideways (x to y centimetres from the emitter).
    #[export]
    emit_radius_x: Vector2,
    /// Where a particle starts, forwards.
    #[export]
    emit_radius_y: Vector2,
    /// Where a particle starts, upwards.
    #[export]
    emit_radius_z: Vector2,
    /// Pull on the particles sideways.
    #[export]
    gravity_x: Vector2,
    /// Pull forwards.
    #[export]
    gravity_y: Vector2,
    /// Pull upwards (negative falls).
    #[export]
    gravity_z: Vector2,
    /// What the particles move with: World (they stay where they were sent out when the
    /// effect moves), LocalPosition (they follow the effect's position but not its turning),
    /// Local (they follow both).
    #[export(enum = (World = 0, LocalPosition = 1, Local = 2))]
    #[init(val = 2)]
    coordinates: i64,
    /// How the particles change over their life, in order of time.
    #[export_group(name = "Keyframes")]
    #[export]
    keyframes: Array<Option<Gd<RoseParticleKeyframe>>>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for RoseParticleSequence {
    fn on_validate_property(&self, property: &mut godot::register::info::PropertyInfo) {
        blend_hint(property);
    }
}

/// One change to a particle at a moment of its life: set a value (picked in the range), or,
/// with `fade`, move the value there gradually from the keyframe of the same kind before.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseParticleKeyframe {
    /// What it changes. Size: `value_1` is the width and `value_2` the height (centimetres).
    /// Red, Green, Blue, Alpha: 0 to 1. Colour: `value_1` to `value_4` are red, green, blue
    /// and alpha. Velocity: centimetres per particle time; VelocityXYZ uses `value_1` to
    /// `value_3`. Texture: the atlas cell. Rotation: degrees. Timer: jumps the particle's
    /// clock to `value_1` (to repeat keyframes).
    #[export(enum = (Size = 1, Timer = 2, Red = 3, Green = 4, Blue = 5, Alpha = 6, Colour = 7, VelocityX = 8, VelocityY = 9, VelocityZ = 10, VelocityXYZ = 11, Texture = 12, Rotation = 13))]
    #[init(val = 1)]
    kind: i64,
    /// When, in particle time from the particle's start (picked between x and y).
    #[export]
    start_time: Vector2,
    /// Move the value here gradually from the keyframe of the same kind before, instead of
    /// setting it at once.
    #[export]
    fade: bool,
    /// The first value (x to y).
    #[export]
    value_1: Vector2,
    /// The second value, for Size, Colour and VelocityXYZ.
    #[export]
    value_2: Vector2,
    /// The third value, for Colour and VelocityXYZ.
    #[export]
    value_3: Vector2,
    /// The fourth value, for Colour (alpha).
    #[export]
    value_4: Vector2,
    base: Base<Resource>,
}

/// A mesh's vertex animation (an effect ZMO file), baked by the import into an image: one
/// row per vertex, one column per frame of position and uv. Not edited by hand; make a new
/// one by importing a ZMO.
#[derive(GodotClass)]
#[class(tool, base=Resource, init)]
pub struct RoseMeshAnimation {
    /// Frames per second.
    #[export]
    #[init(val = 30)]
    frames_per_second: i64,
    /// Frames in the animation.
    #[export]
    frames: i64,
    /// What it animates: 1 position, 2 normal, 4 uv, 8 alpha (added together).
    #[export]
    channels: i64,
    /// The transparency of each frame (with channel 8).
    #[export]
    alphas: PackedFloat32Array,
    /// The baked frames.
    #[export]
    image: Option<Gd<Image>>,
    texture: Option<Gd<ImageTexture>>,
    base: Base<Resource>,
}

impl RoseMeshAnimation {
    fn texture(&mut self) -> Option<Gd<ImageTexture>> {
        if self.texture.is_none() {
            self.texture = self.image.as_ref().and_then(ImageTexture::create_from_image);
        }
        self.texture.clone()
    }
}

/// Bakes a vertex animation ZMO into a float image: one row per vertex, one column per
/// frame of position + uv.x, then (with normals or uvs) one per frame of normal + uv.y.
fn bake_mesh_animation(zmo: &ZmoFile) -> Option<Gd<RoseMeshAnimation>> {
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
    let mut baked = RoseMeshAnimation::new_gd();
    {
        let mut b = baked.bind_mut();
        b.frames_per_second = zmo.fps as i64;
        b.frames = frames as i64;
        b.channels = flags as i64;
        b.alphas = PackedFloat32Array::from(alphas.as_slice());
        b.image = Some(image);
    }
    Some(baked)
}

/// An effect's own motion (a ZMO's first position, rotation and scale channels) as a Godot
/// animation with one key per frame, in Godot's axes and metres.
fn motion_from_zmo(zmo: &ZmoFile) -> Gd<Animation> {
    let mut motion = Animation::new_gd();
    let fps = zmo.fps.max(1) as f64;
    motion.set_step((1.0 / fps) as f32);
    motion.set_length(zmo.num_frames.max(1) as f64 / fps);
    let (mut position, mut rotation, mut scale) = (false, false, false);
    for (_, channel) in zmo.channels.iter() {
        match channel {
            ZmoChannel::Position(keys) if !position => {
                position = true;
                let track = motion.add_track(TrackType::POSITION_3D);
                motion.track_set_path(track, ".");
                for (i, p) in keys.iter().enumerate() {
                    motion.position_track_insert_key(track, i as f64 / fps, Vector3::new(p.x, p.z, -p.y) / 100.0);
                }
            }
            ZmoChannel::Rotation(keys) if !rotation => {
                rotation = true;
                let track = motion.add_track(TrackType::ROTATION_3D);
                motion.track_set_path(track, ".");
                for (i, r) in keys.iter().enumerate() {
                    motion.rotation_track_insert_key(track, i as f64 / fps, Quaternion::new(r.x, r.z, -r.y, r.w).normalized());
                }
            }
            ZmoChannel::Scale(keys) if !scale => {
                scale = true;
                let track = motion.add_track(TrackType::SCALE_3D);
                motion.track_set_path(track, ".");
                for (i, s) in keys.iter().enumerate() {
                    motion.scale_track_insert_key(track, i as f64 / fps, Vector3::splat(*s));
                }
            }
            _ => {}
        }
    }
    motion
}

fn range(v: Vector2) -> RangeInclusive<f32> {
    v.x..=v.y
}

fn vector(r: &RangeInclusive<f32>) -> Vector2 {
    Vector2::new(*r.start(), *r.end())
}

impl RoseParticleKeyframe {
    fn data(&self) -> Option<PtlKeyframeData> {
        let (a, b, c, d) = (range(self.value_1), range(self.value_2), range(self.value_3), range(self.value_4));
        Some(match self.kind {
            1 => PtlKeyframeData::SizeXY(a, b),
            2 => PtlKeyframeData::Timer(a),
            3 => PtlKeyframeData::Red(a),
            4 => PtlKeyframeData::Green(a),
            5 => PtlKeyframeData::Blue(a),
            6 => PtlKeyframeData::Alpha(a),
            7 => PtlKeyframeData::ColourRGBA(a, b, c, d),
            8 => PtlKeyframeData::VelocityX(a),
            9 => PtlKeyframeData::VelocityY(a),
            10 => PtlKeyframeData::VelocityZ(a),
            11 => PtlKeyframeData::VelocityXYZ(a, b, c),
            12 => PtlKeyframeData::Texture(a),
            13 => PtlKeyframeData::Rotation(a),
            _ => return None,
        })
    }

    fn from_ptl(start_time: &RangeInclusive<f32>, fade: bool, data: &PtlKeyframeData) -> Gd<Self> {
        let (kind, values): (i64, Vec<&RangeInclusive<f32>>) = match data {
            PtlKeyframeData::SizeXY(x, y) => (1, vec![x, y]),
            PtlKeyframeData::Timer(v) => (2, vec![v]),
            PtlKeyframeData::Red(v) => (3, vec![v]),
            PtlKeyframeData::Green(v) => (4, vec![v]),
            PtlKeyframeData::Blue(v) => (5, vec![v]),
            PtlKeyframeData::Alpha(v) => (6, vec![v]),
            PtlKeyframeData::ColourRGBA(r, g, b, a) => (7, vec![r, g, b, a]),
            PtlKeyframeData::VelocityX(v) => (8, vec![v]),
            PtlKeyframeData::VelocityY(v) => (9, vec![v]),
            PtlKeyframeData::VelocityZ(v) => (10, vec![v]),
            PtlKeyframeData::VelocityXYZ(x, y, z) => (11, vec![x, y, z]),
            PtlKeyframeData::Texture(v) => (12, vec![v]),
            PtlKeyframeData::Rotation(v) => (13, vec![v]),
        };
        let mut keyframe = Self::new_gd();
        {
            let mut k = keyframe.bind_mut();
            k.kind = kind;
            k.start_time = vector(start_time);
            k.fade = fade;
            let value = |i: usize| values.get(i).map_or(Vector2::ZERO, |r| vector(r));
            k.value_1 = value(0);
            k.value_2 = value(1);
            k.value_3 = value(2);
            k.value_4 = value(3);
        }
        keyframe
    }
}

impl RoseParticleSequence {
    fn from_ptl(sequence: &PtlSequence) -> Gd<Self> {
        let mut out = Self::new_gd();
        {
            let mut s = out.bind_mut();
            s.name = sequence.name.trim_matches('"').into();
            s.texture = texture::load_texture(&sequence.texture_path.path().to_string_lossy());
            s.atlas_columns = sequence.texture_atlas_cols as i64;
            s.atlas_rows = sequence.texture_atlas_rows as i64;
            s.facing = sequence.align_type as i64;
            s.source_blend = sequence.src_blend_mode as i64;
            s.destination_blend = sequence.dst_blend_mode as i64;
            s.blend_op = sequence.blend_op as i64;
            s.particles = sequence.num_particles as i64;
            s.emit_rate = vector(&sequence.emit_rate);
            s.loops = sequence.num_loops as i64;
            s.life = vector(&sequence.life);
            s.emit_radius_x = vector(&sequence.emit_radius_x);
            s.emit_radius_y = vector(&sequence.emit_radius_y);
            s.emit_radius_z = vector(&sequence.emit_radius_z);
            s.gravity_x = vector(&sequence.gravity_x);
            s.gravity_y = vector(&sequence.gravity_y);
            s.gravity_z = vector(&sequence.gravity_z);
            s.coordinates = match sequence.update_coords {
                PtlUpdateCoords::World => 0,
                PtlUpdateCoords::LocalPosition => 1,
                PtlUpdateCoords::Local => 2,
            };
            for keyframe in sequence.keyframes.iter() {
                s.keyframes.push(Some(&RoseParticleKeyframe::from_ptl(&keyframe.start_time, keyframe.fade, &keyframe.data)));
            }
        }
        out
    }
}

impl RoseParticleSystem {
    fn from_ptl(ptl: &PtlFile) -> Gd<Self> {
        let mut out = Self::new_gd();
        for sequence in ptl.sequences.iter() {
            out.bind_mut().sequences.push(Some(&RoseParticleSequence::from_ptl(sequence)));
        }
        out
    }
}

// ---------------------------------------------------------------------------------------
// The parts of an effect scene

/// A particle system placed in an effect (an EFT particle entry). Move and turn the node to
/// move and turn it; the particles start after `start_delay`.
#[derive(GodotClass)]
#[class(tool, base=Node3D, init)]
pub struct RoseEffectParticles {
    /// What it emits: drag a `RoseParticleSystem` in (the imported ones are
    /// `res://assets/rose/3ddata/effect/particles/*.particles.res`), or make a new one here.
    #[export]
    particle_system: Option<Gd<RoseParticleSystem>>,
    /// Milliseconds after the effect starts before it begins.
    #[export(range = (0.0, 10000.0, 1.0, or_greater, suffix = "ms"))]
    start_delay: i64,
    /// A motion the emitter follows (position, rotation and scale tracks on "."), empty = it
    /// stays where the node is. The imported effects keep their ZMO motions here.
    #[export]
    motion: Option<Gd<Animation>>,
    /// How many times the motion plays (0 = forever); the particle system stops with it.
    #[export]
    #[init(val = 1)]
    motion_repeat_count: i64,
    base: Base<Node3D>,
}

/// A mesh placed in an effect (an EFT mesh entry), optionally with a vertex animation and a
/// motion. Move and turn the node to move and turn it.
#[derive(GodotClass)]
#[class(tool, base=Node3D, init)]
pub struct RoseEffectMesh {
    /// The mesh (an imported `.mesh` or one of our own).
    #[export_group(name = "Look")]
    #[export]
    mesh: Option<Gd<Mesh>>,
    /// Its texture.
    #[export]
    texture: Option<Gd<Texture2D>>,
    /// Drawn with transparency (with `source_blend` ... deciding how).
    #[export]
    alpha_enabled: bool,
    /// Pixels whose alpha is low are not drawn (for an opaque mesh).
    #[export]
    alpha_test: bool,
    /// Both sides of each triangle are drawn.
    #[export]
    two_sided: bool,
    /// Hidden behind things in front of it.
    #[export]
    #[init(val = true)]
    depth_test: bool,
    /// Hides what is drawn behind it later.
    #[export]
    depth_write: bool,
    /// The source blend factor (how the mesh mixes with what is behind it).
    #[export]
    #[init(val = 5)]
    source_blend: i64,
    /// The destination blend factor.
    #[export]
    #[init(val = 6)]
    destination_blend: i64,
    /// The blend operation.
    #[export]
    #[init(val = 1)]
    blend_op: i64,
    /// The mesh's vertex animation (empty = it doesn't change shape; the mesh then shows for
    /// as long as the effect lasts).
    #[export_group(name = "Animation")]
    #[export]
    mesh_animation: Option<Gd<RoseMeshAnimation>>,
    /// How many times the vertex animation plays (0 = forever).
    #[export]
    repeat_count: i64,
    /// Milliseconds after the effect starts before the vertex animation begins (the mesh is
    /// hidden until then).
    #[export(range = (0.0, 10000.0, 1.0, or_greater, suffix = "ms"))]
    start_delay: i64,
    /// A motion the mesh follows (position, rotation and scale tracks on "."), empty = it
    /// stays where the node is.
    #[export]
    motion: Option<Gd<Animation>>,
    /// How many times the motion plays (0 = forever).
    #[export]
    #[init(val = 1)]
    motion_repeat_count: i64,
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for RoseEffectMesh {
    fn on_validate_property(&self, property: &mut godot::register::info::PropertyInfo) {
        blend_hint(property);
    }
}

/// Adds `node` to `parent` as an internal child: shown while playing, never saved with the
/// scene (the editor plays effect scenes too).
fn add_internal(parent: &mut Gd<Node3D>, node: &Gd<Node3D>) {
    parent.add_child_ex(node).internal(InternalMode::FRONT).done();
}

impl RoseEffectParticles {
    fn start(&self, sequences: &mut Vec<Sequence>) {
        let Some(system) = self.particle_system.as_ref() else { return };
        let mut holder = self.to_gd().upcast::<Node3D>();
        for sequence in system.bind().sequences.iter_shared().flatten() {
            let sequence = sequence.bind();
            let mut anchor = Node3D::new_alloc();
            add_internal(&mut holder, &anchor);
            let key = ShaderKey {
                particle: true,
                mesh_animation: false,
                blend: resolve_blend(sequence.source_blend as u32, sequence.destination_blend as u32, sequence.blend_op as u32, false),
                mask: false,
                two_sided: true,
                z_write: false,
                z_test: true,
            };
            let billboard = match sequence.facing { 1 => 1, 2 => 2, _ => 0 };
            let (cols, rows) = (sequence.atlas_columns.max(1) as u32, sequence.atlas_rows.max(1) as u32);
            let texture_id = sequence.texture.as_ref().map_or(0, |t| t.instance_id().to_i64());
            let material = PARTICLE_MATERIALS.with_borrow_mut(|cache| {
                cache
                    .entry((key, texture_id, billboard, cols, rows))
                    .or_insert_with(|| {
                        let mut material = ShaderMaterial::new_gd();
                        material.set_shader(&shader(key));
                        if let Some(texture) = sequence.texture.as_ref() {
                            material.set_shader_parameter("base_texture", &texture.to_variant());
                        }
                        material.set_shader_parameter("billboard", &(billboard as i32).to_variant());
                        material.set_shader_parameter("atlas_size", &Vector2::new(cols as f32, rows as f32).to_variant());
                        material
                    })
                    .clone()
            });
            let num_particles = sequence.particles.max(0) as usize;
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
            add_internal(&mut anchor, &instance.clone().upcast());

            let mut keyframes: Vec<Keyframe> = sequence
                .keyframes
                .iter_shared()
                .flatten()
                .filter_map(|k| {
                    let k = k.bind();
                    Some(Keyframe { start_time: pick(&range(k.start_time)), fade: k.fade, next_fade: None, data: k.data()? })
                })
                .collect();
            keyframes.sort_by(|a, b| a.start_time.total_cmp(&b.start_time));
            for i in 0..keyframes.len() {
                keyframes[i].next_fade = (i + 1..keyframes.len()).find(|&j| {
                    keyframes[j].fade && std::mem::discriminant(&keyframes[i].data) == std::mem::discriminant(&keyframes[j].data)
                });
            }
            let animation = self.motion.as_ref().and_then(|m| TransformAnimation::from_motion(m, self.motion_repeat_count.max(0) as u32));
            sequences.push(Sequence {
                emit_rate: range(sequence.emit_rate),
                life: range(sequence.life),
                emit_radius: [range(sequence.emit_radius_x), range(sequence.emit_radius_y), range(sequence.emit_radius_z)],
                gravity: [range(sequence.gravity_x), range(sequence.gravity_y), range(sequence.gravity_z)],
                keyframes,
                update_coords: match sequence.coordinates {
                    0 => PtlUpdateCoords::World,
                    1 => PtlUpdateCoords::LocalPosition,
                    _ => PtlUpdateCoords::Local,
                },
                num_loops: sequence.loops as i32,
                num_particles,
                start_delay: self.start_delay.max(0) as f32 / 1000.0,
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
}

impl RoseEffectMesh {
    fn start(&self) -> Option<EffectMesh> {
        let mesh = self.mesh.as_ref()?;
        let mesh = match mesh.clone().try_cast::<ArrayMesh>() {
            Ok(array) => mesh::rigid(&array).upcast::<Mesh>(),
            Err(other) => other,
        };
        let mut animation = self.mesh_animation.clone();
        let blended = self.alpha_enabled || !self.depth_write;
        let key = ShaderKey {
            particle: false,
            mesh_animation: animation.is_some(),
            blend: resolve_blend(self.source_blend as u32, self.destination_blend as u32, self.blend_op as u32, !blended),
            mask: !blended && self.alpha_test,
            two_sided: self.two_sided,
            z_write: self.depth_write,
            z_test: self.depth_test,
        };
        let mut material = ShaderMaterial::new_gd();
        material.set_shader(&shader(key));
        if let Some(texture) = self.texture.as_ref() {
            material.set_shader_parameter("base_texture", &texture.to_variant());
        }
        let mut instance = MeshInstance3D::new_alloc();
        instance.set_mesh(&mesh);
        instance.set_material_override(&material);
        instance.upcast_mut::<GeometryInstance3D>().set_cast_shadows_setting(godot::classes::geometry_instance_3d::ShadowCastingSetting::OFF);
        let animation = animation.as_mut().and_then(|data| {
            let texture = data.bind_mut().texture()?;
            let d = data.bind();
            material.set_shader_parameter("animation_texture", &texture.to_variant());
            material.set_shader_parameter("animation_flags", &(d.channels as i32).to_variant());
            material.set_shader_parameter("animation_frames", &(d.frames as i32).to_variant());
            // Vertex animation moves the mesh beyond its rest bounds.
            instance.set_extra_cull_margin(20.0);
            let clock = Clock::new(d.frames_per_second.max(1) as usize, d.frames.max(1) as usize, self.repeat_count.max(0) as u32, self.start_delay.max(0) as f32 / 1000.0);
            Some((clock, d.alphas.as_slice().to_vec()))
        });
        // A mesh waiting for its start delay is not drawn yet.
        if animation.as_ref().is_some_and(|(clock, _)| clock.delay > 0.0) {
            instance.set_visible(false);
        }
        let mut holder = self.to_gd().upcast::<Node3D>();
        add_internal(&mut holder, &instance.clone().upcast());
        let motion = self.motion.as_ref().and_then(|m| TransformAnimation::from_motion(m, self.motion_repeat_count.max(0) as u32));
        Some(EffectMesh { node: instance.upcast(), material, animation, motion })
    }
}

struct EffectMesh {
    node: Gd<Node3D>,
    material: Gd<ShaderMaterial>,
    /// The vertex animation's clock and the alpha of each frame.
    animation: Option<(Clock, Vec<f32>)>,
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
        let Some((clock, alphas)) = self.animation.as_mut() else { return };
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
        if let (Some(a), Some(b)) = (alphas.get(clock.current), alphas.get(clock.next)) {
            m.set_shader_parameter("animation_alpha", &(a * (1.0 - clock.fract) + b * clock.fract).to_variant());
        }
    }
}

// ---------------------------------------------------------------------------------------
// The node

/// Seconds the editor waits before playing a finished effect again, and the longest it lets
/// one play (effects that loop forever) before starting it over with any changes.
const EDITOR_PAUSE: f64 = 0.6;
const EDITOR_REPLAY_AFTER: f64 = 12.0;

/// An effect: the root of an effect scene, whose `RoseEffectParticles` and `RoseEffectMesh`
/// children are its parts. In the game it plays once it is added and frees itself when every
/// part has finished (effects that loop forever, like torches in a zone, never do). Opened in
/// the editor it plays over and over, so changes to its parts show the next time round.
/// The imported effects are `res://assets/rose/3ddata/effect/*.scn`; a scene of our own in
/// `res://assets/ours/` at the same path is shown instead, and the effect resources
/// (`RoseEffectFile`, `res://data/effect_files/`) link either.
#[derive(GodotClass)]
#[class(tool, base=Node3D, init)]
pub struct RoseEffect {
    base: Base<Node3D>,
    sequences: Vec<Sequence>,
    meshes: Vec<EffectMesh>,
    started: bool,
    loaded: bool,
    /// Editor only: seconds since the effect started, and since it finished.
    playing_for: f64,
    finished_for: f64,
}

#[godot_api]
impl INode3D for RoseEffect {
    fn ready(&mut self) {
        if !self.started {
            self.start();
        }
    }

    fn process(&mut self, delta: f64) {
        if !self.loaded || !self.base().is_visible_in_tree() {
            return;
        }
        let editor = Engine::singleton().is_editor_hint();
        let done = self.advance(delta as f32);
        if editor {
            self.playing_for += delta;
            if done {
                self.finished_for += delta;
            }
            if self.finished_for > EDITOR_PAUSE || self.playing_for > EDITOR_REPLAY_AFTER {
                self.start();
            }
        } else if done {
            self.loaded = false;
            self.base_mut().queue_free();
        }
    }
}

#[godot_api]
impl RoseEffect {
    /// Loads an effect by its path: an effect scene (`res://assets/rose/3ddata/effect/x.scn`,
    /// our own copy in `res://assets/ours/` first) or an EFT file in the client data
    /// (`3DDATA/EFFECT/LEVELUP_01.EFT`, shown from its imported scene when there is one).
    /// Returns false if it is missing or empty.
    #[func]
    fn load(&mut self, path: GString) -> bool {
        self.load_path(&path.to_string())
    }

    /// Loads an effect by its number (a `RoseEffectFile`'s id, iROSE's FILE_EFFECT.STB row).
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
        let Some(scene) = effect_scene(path) else { return false };
        let Some(mut instance) = scene.instantiate() else { return false };
        // The scene's parts become this node's.
        for mut child in instance.get_children().iter_shared() {
            child.set_owner(Gd::null_arg());
            instance.remove_child(&child);
            self.base_mut().add_child(&child);
        }
        instance.free();
        self.start();
        self.loaded
    }

    /// Starts (or starts over) every part.
    fn start(&mut self) {
        self.stop();
        self.started = true;
        let mut sequences = Vec::new();
        let mut meshes = Vec::new();
        for child in self.base().get_children().iter_shared() {
            if let Ok(particles) = child.clone().try_cast::<RoseEffectParticles>() {
                particles.bind().start(&mut sequences);
            } else if let Ok(part) = child.try_cast::<RoseEffectMesh>() {
                meshes.extend(part.bind().start());
            }
        }
        self.sequences = sequences;
        self.meshes = meshes;
        self.playing_for = 0.0;
        self.finished_for = 0.0;
        self.loaded = !self.sequences.is_empty() || !self.meshes.is_empty();
    }

    /// Removes what the parts drew.
    fn stop(&mut self) {
        for mut sequence in self.sequences.drain(..) {
            sequence.anchor.queue_free();
        }
        for mut mesh in self.meshes.drain(..) {
            mesh.node.queue_free();
        }
    }

    /// Plays every part on by `delta` seconds; true when all have finished.
    fn advance(&mut self, delta: f32) -> bool {
        for sequence in self.sequences.iter_mut() {
            sequence.update(delta);
        }
        for mesh in self.meshes.iter_mut() {
            mesh.update(delta);
        }
        let parts = self.sequences.len() + self.meshes.len();
        let done = self.sequences.iter().filter(|s| s.done()).count() + self.meshes.iter().filter(|m| m.done()).count();
        parts == 0 || done == parts
    }
}

// ---------------------------------------------------------------------------------------
// Effect scenes: imported, our own, or made from the client's files

/// Bumped when the imported effect scenes or particle systems change, so the next import
/// redoes them.
pub const EFFECT_IMPORT_VERSION: i64 = 1;

/// The extensions an effect scene may have in the asset folders (the imported `.scn`
/// first), and a particle system.
pub const SCENE_EXTENSIONS: &[&str] = &["scn", "tscn"];
pub const PARTICLE_EXTENSIONS: &[&str] = &["particles.res", "particles.tres"];

/// An effect file path by its number (FILE_EFFECT.STB row, now `RoseEffectFile`).
pub fn effect_file(file_id: i32) -> Option<String> {
    let id = rose_data::EffectFileId::new(u16::try_from(file_id).ok()?)?;
    Some(data::get()?.effects.get_effect_file(id)?.to_string())
}

/// The scene an effect path names: a Godot path (in the asset folders, our own copy first)
/// or a client EFT file (its imported scene, our own copy first); without one, a scene made
/// from the client's EFT file. None when there is nothing.
pub fn effect_scene(path: &str) -> Option<Gd<PackedScene>> {
    if path.trim().is_empty() {
        return None;
    }
    let inner = path.strip_prefix(assets::ROSE).or_else(|| path.strip_prefix(assets::OURS));
    if inner.is_none() && path.starts_with("res://") {
        let mut loader = ResourceLoader::singleton();
        return loader.exists(path).then(|| loader.load(path)).flatten().and_then(|r| r.try_cast::<PackedScene>().ok());
    }
    let stem = assets::stem(inner.unwrap_or(path));
    if let Some(scene) = assets::load_stem::<PackedScene>(&stem, SCENE_EXTENSIONS) {
        return Some(scene);
    }
    let client = format!("{stem}.eft");
    let key = client.to_ascii_uppercase();
    if let Some(cached) = SCENES.with_borrow(|cache| cache.get(&key).cloned()) {
        return cached;
    }
    let scene = data::read_file::<EftFile>(&client).and_then(|eft| pack(build_effect(&eft, &stem, false)));
    if scene.is_none() {
        godot_warn!("rose: could not load effect {path}");
    }
    SCENES.with_borrow_mut(|cache| cache.insert(key, scene.clone()));
    scene
}

/// A particle system by its client PTL path: our own or the imported one, else made from the
/// client's file (kept for the next effect).
fn particle_system(path: &str) -> Option<Gd<RoseParticleSystem>> {
    if let Some(system) = assets::load_stem::<RoseParticleSystem>(&assets::stem(path), PARTICLE_EXTENSIONS) {
        return Some(system);
    }
    let key = path.to_ascii_uppercase();
    if let Some(cached) = PARTICLE_SYSTEMS.with_borrow(|cache| cache.get(&key).cloned()) {
        return cached;
    }
    let system = data::read_file::<PtlFile>(path).map(|ptl| RoseParticleSystem::from_ptl(&ptl));
    PARTICLE_SYSTEMS.with_borrow_mut(|cache| cache.insert(key, system.clone()));
    system
}

fn cached<T: GodotClass>(
    cache: &'static std::thread::LocalKey<RefCell<HashMap<String, Option<Gd<T>>>>>,
    path: &str,
    make: impl FnOnce() -> Option<Gd<T>>,
) -> Option<Gd<T>> {
    let key = path.to_ascii_uppercase();
    if let Some(found) = cache.with_borrow(|c| c.get(&key).cloned()) {
        return found;
    }
    let made = make();
    cache.with_borrow_mut(|c| c.insert(key, made.clone()));
    made
}

/// The parts of an EFT file as an effect scene's root (`RoseEffect`, named after `stem`'s
/// file) with its part nodes, each owned by the root. `import` links the imported particle
/// systems (which must be there) and leaves out what isn't imported; otherwise what isn't
/// imported is made from the client's files.
pub fn build_effect(eft: &EftFile, stem: &str, import: bool) -> Gd<Node3D> {
    let mut root = RoseEffect::new_alloc().upcast::<Node3D>();
    root.set_name(stem.rsplit('/').next().unwrap_or("Effect"));
    let rose_transform = |position: &rose_file_readers::types::Vec3<f32>, yaw: f32, pitch: f32, roll: f32| {
        Transform3D::new(rose_rotation(yaw, pitch, roll), Vector3::new(position.x, position.z, -position.y) / 100.0)
    };
    let motion = |path: &Option<rose_file_readers::VfsPathBuf>| {
        let path = path.as_ref()?.path().to_string_lossy().to_string();
        cached(&MOTIONS, &path, || data::read_file::<ZmoFile>(&path).map(|zmo| motion_from_zmo(&zmo)))
    };
    for (i, entry) in eft.particles.iter().enumerate() {
        let ptl = entry.particle_file.path().to_string_lossy().to_string();
        let system = if import {
            assets::load_stem::<RoseParticleSystem>(&assets::stem(&ptl), PARTICLE_EXTENSIONS)
        } else {
            particle_system(&ptl)
        };
        let Some(system) = system else { continue };
        let mut part = RoseEffectParticles::new_alloc();
        part.set_name(&format!("Particles{i}"));
        part.set_transform(rose_transform(&entry.position, entry.yaw, entry.pitch, entry.roll));
        {
            let mut p = part.bind_mut();
            p.particle_system = Some(system);
            p.start_delay = entry.start_delay as i64;
            p.motion = motion(&entry.animation_file);
            p.motion_repeat_count = entry.animation_repeat_count as i64;
        }
        root.add_child(&part);
        part.set_owner(&root);
    }
    for (i, entry) in eft.meshes.iter().enumerate() {
        let mesh_path = entry.mesh_file.path().to_string_lossy().to_string();
        let mesh = assets::load_mesh(&mesh_path)
            .or_else(|| if import { None } else { mesh::load_mesh(&mesh_path, false) })
            .map(|m| m.upcast::<Mesh>());
        let Some(mesh) = mesh else { continue };
        let texture_path = entry.mesh_texture_file.path().to_string_lossy().to_string();
        let texture = if import { assets::load_texture(&texture_path) } else { texture::load_texture(&texture_path) };
        let mut part = RoseEffectMesh::new_alloc();
        part.set_name(&format!("Mesh{i}"));
        part.set_transform(rose_transform(&entry.position, entry.yaw, entry.pitch, entry.roll));
        {
            let mut p = part.bind_mut();
            p.mesh = Some(mesh);
            p.texture = texture;
            p.alpha_enabled = entry.alpha_enabled;
            p.alpha_test = entry.alpha_test_enabled;
            p.two_sided = entry.two_sided;
            p.depth_test = entry.depth_test_enabled;
            p.depth_write = entry.depth_write_enabled;
            p.source_blend = entry.src_blend_factor as i64;
            p.destination_blend = entry.dst_blend_factor as i64;
            p.blend_op = entry.blend_op as i64;
            p.mesh_animation = entry.mesh_animation_file.as_ref().and_then(|path| {
                let path = path.path().to_string_lossy().to_string();
                cached(&MESH_ANIMATIONS, &path, || data::read_file::<ZmoFile>(&path).and_then(|zmo| bake_mesh_animation(&zmo)))
            });
            p.repeat_count = entry.repeat_count as i64;
            p.start_delay = entry.start_delay as i64;
            p.motion = motion(&entry.animation_file);
            p.motion_repeat_count = entry.animation_repeat_count as i64;
        }
        root.add_child(&part);
        part.set_owner(&root);
    }
    root
}

/// Packs a built effect into a scene (and frees the nodes). None when it has no parts.
fn pack(root: Gd<Node3D>) -> Option<Gd<PackedScene>> {
    let parts = root.get_child_count();
    let mut scene = PackedScene::new_gd();
    let packed = parts > 0 && scene.pack(&root) == Error::OK;
    root.free();
    packed.then_some(scene)
}

/// Writes the imported particle system of a PTL file (`target`, a `.particles.res`).
pub fn import_particle_system(ptl_path: &str, target: &str) -> bool {
    match data::read_file::<PtlFile>(ptl_path) {
        Some(ptl) => assets::save(RoseParticleSystem::from_ptl(&ptl).upcast(), target),
        None => false,
    }
}

/// Writes the imported scene of an EFT file (`target`, a `.scn`), linking the imported
/// meshes, textures and particle systems. Whether it was saved; None when the file can't be
/// read or shows nothing (no scene is written).
pub fn import_effect(eft_path: &str, target: &str) -> Option<bool> {
    let Some(eft) = data::read_file::<EftFile>(eft_path) else {
        godot_warn!("rose: effect {eft_path} can't be read; left out");
        return None;
    };
    let Some(scene) = pack(build_effect(&eft, &assets::stem(eft_path), true)) else {
        godot_warn!("rose: effect {eft_path} has nothing to show; left out");
        return None;
    };
    Some(assets::save(scene.upcast::<Resource>(), target))
}

/// Drops the cached shaders, materials and effects made from the client's files while the
/// engine runs.
pub fn clear_cache() {
    SHADERS.with_borrow_mut(|cache| cache.clear());
    PARTICLE_MATERIALS.with_borrow_mut(|cache| cache.clear());
    QUAD.with_borrow_mut(|quad| *quad = None);
    SCENES.with_borrow_mut(|cache| cache.clear());
    PARTICLE_SYSTEMS.with_borrow_mut(|cache| cache.clear());
    MOTIONS.with_borrow_mut(|cache| cache.clear());
    MESH_ANIMATIONS.with_borrow_mut(|cache| cache.clear());
}

// ---------------------------------------------------------------------------------------
// Which effect each game event shows (animation_effect_system.rs, hit_event_system.rs,
// client_entity_event_system.rs)

use rose_data::{EffectBulletMoveType, EffectData, EffectId, ItemClass, NpcId, SkillId, SkillType};

fn file_path(id: Option<rose_data::EffectFileId>) -> GString {
    id.and_then(|id| data::get()?.effects.get_effect_file(id)).map_or(GString::new(), GString::from)
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
    /// An effect's path by its number (a `RoseEffectFile`'s id, FILE_EFFECT.STB row).
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
