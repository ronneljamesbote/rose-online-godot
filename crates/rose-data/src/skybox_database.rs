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
