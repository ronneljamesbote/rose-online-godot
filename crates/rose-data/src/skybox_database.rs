use glam::Vec4;
use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct SkyboxId(u16);

id_wrapper_impl!(SkyboxId, u16);

#[derive(Enum, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SkyboxState {
    Morning,
    Day,
    Evening,
    Night,
}

/// A sky: the dome around a zone and the light it gives at each time of day. The mesh and
/// textures are paths: a client file (`3DDATA\SKY\DAY_SKY.ZMS`) or a Godot resource
/// (`res://assets/rose/3ddata/sky/day_sky.mesh`).
#[derive(Clone, Debug, PartialEq)]
pub struct SkyboxData {
    pub id: SkyboxId,
    pub mesh: String,
    pub texture_day: String,
    pub texture_night: String,
    pub map_ambient_color: EnumMap<SkyboxState, Vec4>,
    pub character_ambient_color: EnumMap<SkyboxState, Vec4>,
    pub character_diffuse_color: EnumMap<SkyboxState, Vec4>,
    /// The sun's colour on the ground and buildings (iROSE has no sun: it starts as the
    /// character diffuse colour, see `default_sun`).
    pub sun_color: EnumMap<SkyboxState, Vec4>,
    /// How strongly the sun lights surfaces facing it (1 = as bright as the ground light).
    pub sun_strength: EnumMap<SkyboxState, f32>,
    /// How much of the ground light the sun gives, so how dark shadows are: 0 = no shadows,
    /// 0.4 = shade is 60 % as bright as sunlight.
    pub shadow_darkness: EnumMap<SkyboxState, f32>,
}

impl SkyboxData {
    /// The sun of a sky that has none yet (iROSE's): the character diffuse colour, strength
    /// 1.5 and shadows a little lighter in the morning and evening and lightest at night.
    pub fn default_sun(character_diffuse_color: &EnumMap<SkyboxState, Vec4>) -> (EnumMap<SkyboxState, Vec4>, EnumMap<SkyboxState, f32>, EnumMap<SkyboxState, f32>) {
        (
            *character_diffuse_color,
            enum_map::enum_map! { _ => 1.5 },
            enum_map::enum_map! {
                SkyboxState::Morning => 0.35,
                SkyboxState::Day => 0.4,
                SkyboxState::Evening => 0.35,
                SkyboxState::Night => 0.3,
            },
        )
    }
}

pub struct SkyboxDatabase {
    skyboxs: Vec<Option<SkyboxData>>,
}

impl SkyboxDatabase {
    pub fn new(skyboxs: Vec<Option<SkyboxData>>) -> Self {
        Self { skyboxs }
    }

    pub fn get_skybox_data(&self, id: SkyboxId) -> Option<&SkyboxData> {
        self.skyboxs.get(id.get() as usize).and_then(|x| x.as_ref())
    }

    pub fn iter(&self) -> impl Iterator<Item = &SkyboxData> {
        self.skyboxs.iter().filter_map(|x| x.as_ref())
    }
}
