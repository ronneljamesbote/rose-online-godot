//! The sun: one `DirectionalLight3D` (`RoseSun`) that stands still at the sky's sun height and
//! direction (the moon at night stands in the same place) and casts real-time shadows, and the zone light at a time of day, ported from
//! rose-offline-client's zone_time_system.rs. The ROSE shaders take the light as global
//! shader values (see `shaders/rose_common.gdshaderinc`).

use godot::{
    classes::{
        light_3d::Param,
        directional_light_3d::ShadowMode,
        rendering_server::ShadowQuality,
        DirectionalLight3D, IDirectionalLight3D, RenderingServer,
    },
    prelude::*,
};
use rose_data::{SkyboxData, SkyboxState};

/// A zone's day, in world ticks (10 s each): its length and when each state starts.
#[derive(Clone, Copy, Debug)]
pub struct DayTimes {
    pub day_cycle: u32,
    pub morning: u32,
    pub day: u32,
    pub evening: u32,
    pub night: u32,
}

/// Fog colour and density per state (zone_time_system.rs).
fn fog(state: SkyboxState) -> (f32, f32) {
    match state {
        SkyboxState::Morning => (100.0 / 255.0, 0.0022),
        SkyboxState::Day => (200.0 / 255.0, 0.0018),
        SkyboxState::Evening => (100.0 / 255.0, 0.0022),
        SkyboxState::Night => (10.0 / 255.0, 0.0020),
    }
}

/// Which way the sun (and the moon) is, from the ground: the same at every time of day, so
/// shadows never move. `height` is degrees above the horizon, `azimuth` the compass direction
/// seen from above (0 = north, which is -z, 90 = east, +x).
pub fn sun_direction(height: f32, azimuth: f32) -> Vector3 {
    let (height, azimuth) = (height.clamp(1.0, 90.0).to_radians(), azimuth.to_radians());
    Vector3::new(height.cos() * azimuth.sin(), height.sin(), -height.cos() * azimuth.cos())
}

/// The zone light at `day_time` (world ticks, 0..day_cycle) plus `partial_tick`, from the
/// zone's sky (white light and no sun without one): what `RoseSun.apply` takes.
pub fn lighting(sky: Option<&SkyboxData>, times: &DayTimes, day_time: i64, partial_tick: f32) -> VarDictionary {
    let mut out = VarDictionary::new();
    let cycle = times.day_cycle.max(1);
    let day_time = (day_time.max(0) as u32) % cycle;
    let (state, ticks, length) = if day_time >= times.night || day_time < times.morning {
        let length = times.morning + (cycle - times.night.min(cycle));
        let ticks = if day_time >= times.night { day_time - times.night } else { day_time + cycle - times.night.min(cycle) };
        (SkyboxState::Night, ticks, length)
    } else if day_time >= times.evening {
        (SkyboxState::Evening, day_time - times.evening, times.night - times.evening)
    } else if day_time >= times.day {
        (SkyboxState::Day, day_time - times.day, times.evening - times.day)
    } else {
        (SkyboxState::Morning, day_time - times.morning, times.day - times.morning)
    };
    let percent = ((ticks as f32 + partial_tick) / length.max(1) as f32).clamp(0.0, 1.0);

    // Each state blends from the previous state into itself over its first half and into
    // the next over its second half, except day and night, which hold.
    let (from, to, t) = match state {
        SkyboxState::Morning if percent < 0.5 => (SkyboxState::Night, SkyboxState::Morning, percent * 2.0),
        SkyboxState::Morning => (SkyboxState::Morning, SkyboxState::Day, (percent - 0.5) * 2.0),
        SkyboxState::Evening if percent < 0.5 => (SkyboxState::Day, SkyboxState::Evening, percent * 2.0),
        SkyboxState::Evening => (SkyboxState::Evening, SkyboxState::Night, (percent - 0.5) * 2.0),
        s => (s, s, 0.0),
    };
    let mix = |a: f32, b: f32| a + (b - a) * t;
    let mix3 = |a: Vector3, b: Vector3| a + (b - a) * t;
    let colour = |map: &enum_map::EnumMap<SkyboxState, glam::Vec4>| {
        let v = |s: SkyboxState| Vector3::new(map[s].x, map[s].y, map[s].z);
        mix3(v(from), v(to))
    };
    let (map_ambient, char_ambient, char_diffuse, sun_color, sun_strength, darkness) = match sky {
        Some(sky) => (
            colour(&sky.map_ambient_color),
            colour(&sky.character_ambient_color),
            colour(&sky.character_diffuse_color),
            colour(&sky.sun_color),
            mix(sky.sun_strength[from], sky.sun_strength[to]),
            mix(sky.shadow_darkness[from], sky.shadow_darkness[to]),
        ),
        None => (Vector3::ONE, Vector3::ONE, Vector3::ONE, Vector3::ONE, 1.0, 0.0),
    };
    let ((fog_from, density_from), (fog_to, density_to)) = (fog(from), fog(to));
    let day_weight = match state {
        SkyboxState::Morning => percent,
        SkyboxState::Day => 1.0,
        SkyboxState::Evening => 1.0 - percent,
        SkyboxState::Night => 0.0,
    };
    let sun_direction = match sky {
        Some(sky) => sun_direction(sky.sun_height, sky.sun_azimuth),
        None => sun_direction(SkyboxData::DEFAULT_SUN_HEIGHT, SkyboxData::DEFAULT_SUN_AZIMUTH),
    };

    out.set("state", format!("{state:?}").to_lowercase());
    out.set("state_percent", percent);
    out.set("day_weight", day_weight);
    out.set("map_ambient", map_ambient);
    out.set("character_ambient", char_ambient);
    out.set("character_diffuse", char_diffuse);
    out.set("sun_direction", sun_direction);
    out.set("sun_color", sun_color);
    // Flat ground gets about as much sun from a sky with a low sun as at ROSE's 45 degrees;
    // what faces a low sun gets a little more.
    let height_boost = (std::f32::consts::FRAC_1_SQRT_2 / sun_direction.y.max(0.01)).clamp(1.0, 1.4);
    out.set("sun_strength", sun_strength * height_boost);
    out.set("shadow_darkness", darkness);
    out.set("fog_color", Vector3::splat(fog_from + (fog_to - fog_from) * t));
    out.set("fog_density", density_from + (density_to - density_from) * t);
    out
}

/// Shadow settings (Options > Graphics): splits, distance in metres, shadow map size, blur.
fn quality(name: &str) -> Option<(ShadowMode, f32, i32, f32, ShadowQuality)> {
    match name {
        "low" => Some((ShadowMode::PARALLEL_2_SPLITS, 80.0, 2048, 1.0, ShadowQuality::SOFT_VERY_LOW)),
        "medium" => Some((ShadowMode::PARALLEL_4_SPLITS, 120.0, 4096, 1.0, ShadowQuality::SOFT_LOW)),
        "high" => Some((ShadowMode::PARALLEL_4_SPLITS, 200.0, 8192, 1.5, ShadowQuality::SOFT_HIGH)),
        _ => None,
    }
}

/// The sun (by night the moon): the zone's one light. It stands still and casts the shadows,
/// only its colour and strength change with the time of day; the game makes it, and zone scenes add a preview one in the editor.
/// Shadows: `set_quality`.
#[derive(GodotClass)]
#[class(tool, base=DirectionalLight3D)]
pub struct RoseSun {
    base: Base<DirectionalLight3D>,
    /// "off", "low", "medium" or "high".
    quality: String,
    /// Whether the sun gives any shadow now (none when the sky's shadow darkness is 0).
    has_shadow: bool,
}

#[godot_api]
impl IDirectionalLight3D for RoseSun {
    fn init(base: Base<DirectionalLight3D>) -> Self {
        Self { base, quality: String::new(), has_shadow: true }
    }

    fn ready(&mut self) {
        let mut light = self.base_mut();
        // The ROSE shaders take the sun's colour from the global values, not from the light.
        light.set_param(Param::SHADOW_BIAS, 0.05);
        light.set_param(Param::SHADOW_NORMAL_BIAS, 1.5);
        light.set_param(Param::SHADOW_FADE_START, 0.85);
        light.set_shadow_reverse_cull_face(false);
        drop(light);
        if self.quality.is_empty() {
            self.set_quality("medium".into());
        }
    }
}

#[godot_api]
impl RoseSun {
    /// Points the sun and sets the zone light from `get_lighting_at`'s dictionary.
    #[func]
    pub fn apply(&mut self, lighting: VarDictionary) {
        let mut server = RenderingServer::singleton();
        for (key, global) in [
            ("map_ambient", "rose_map_ambient"),
            ("character_ambient", "rose_character_ambient"),
            ("character_diffuse", "rose_character_diffuse"),
            ("fog_color", "rose_fog_color"),
            ("fog_density", "rose_fog_density"),
            ("day_weight", "rose_sky_day_weight"),
            ("sun_color", "rose_sun_color"),
            ("sun_strength", "rose_sun_strength"),
            ("shadow_darkness", "rose_shadow_darkness"),
        ] {
            if let Some(value) = lighting.get(key) {
                server.global_shader_parameter_set(global, &value);
            }
        }
        if let Some(direction) = lighting.get("sun_direction").and_then(|d| d.try_to::<Vector3>().ok()) {
            let to_ground = -direction.normalized();
            // Turns too small to see are skipped (the shadow map is drawn again either way).
            let current = -self.base().get_global_transform().basis.col_c();
            if current.dot(to_ground) < 0.999_999 {
                // Godot lights shine along their -Z axis.
                let up = if to_ground.y.abs() > 0.99 { Vector3::FORWARD } else { Vector3::UP };
                let back = -to_ground;
                let right = up.cross(back).normalized();
                self.base_mut().set_basis(Basis::from_cols(right, back.cross(right), back));
            }
        }
        let darkness = lighting.get("shadow_darkness").and_then(|d| d.try_to::<f32>().ok()).unwrap_or(0.0);
        let has_shadow = darkness > 0.001;
        if has_shadow != self.has_shadow {
            self.has_shadow = has_shadow;
            self.update_shadow();
        }
    }

    /// The shadow setting: "off" (the sun still shades), "low", "medium" or "high".
    #[func]
    pub fn set_quality(&mut self, name: GString) {
        let name = name.to_string();
        if name == self.quality {
            return;
        }
        self.quality = name;
        if let Some((mode, distance, size, blur, filter)) = quality(&self.quality) {
            let mut server = RenderingServer::singleton();
            server.directional_shadow_atlas_set_size(size, true);
            server.directional_soft_shadow_filter_set_quality(filter);
            let mut light = self.base_mut();
            light.set_shadow_mode(mode);
            light.set_param(Param::SHADOW_MAX_DISTANCE, distance);
            light.set_param(Param::SHADOW_BLUR, blur);
        }
        self.update_shadow();
    }

    /// The shadow setting in use.
    #[func]
    pub fn get_quality(&self) -> GString {
        self.quality.as_str().into()
    }
}

impl RoseSun {
    fn update_shadow(&mut self) {
        let on = self.has_shadow && quality(&self.quality).is_some();
        self.base_mut().set_shadow(on);
    }
}
