use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};
use std::{
    num::{NonZeroU16, NonZeroUsize},
    str::FromStr,
    time::Duration,
};

use crate::SoundId;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct EffectId(NonZeroU16);

id_wrapper_impl!(EffectId, NonZeroU16, u16);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct EffectFileId(NonZeroU16);

id_wrapper_impl!(EffectFileId, NonZeroU16, u16);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EffectBulletMoveType {
    Linear,
    Parabola,
    Immediate,
}

#[derive(Debug)]
pub struct EffectData {
    pub id: EffectId,
    pub point_effects: ArrayVec<EffectFileId, 4>,
    pub trail_colour_index: Option<NonZeroUsize>,
    pub trail_duration: Duration,
    pub hit_effect_normal: Option<EffectFileId>,
    pub hit_effect_critical: Option<EffectFileId>,
    pub bullet_effect: Option<EffectFileId>,
    pub bullet_move_type: Option<EffectBulletMoveType>,
    pub bullet_speed: f32,
    pub fire_sound_id: Option<SoundId>,
    pub hit_sound_id: Option<SoundId>,
}

#[derive(Debug)]
pub struct EffectDatabase {
    effects: Vec<Option<EffectData>>,
    effect_files: Vec<Option<String>>,
}

impl EffectDatabase {
    /// `effect_files` are the effect files by number: a Godot resource path
    /// (`res://assets/rose/3ddata/effect/levelup_01.scn`, an effect scene) or a file in the
    /// client data (`3DData\Effect\levelup_01.eft`), as written; None or empty when there is none.
    pub fn new(effects: Vec<Option<EffectData>>, effect_files: Vec<Option<String>>) -> Self {
        Self {
            effects,
            effect_files,
        }
    }

    pub fn get_effect(&self, id: EffectId) -> Option<&EffectData> {
        self.effects.get(id.get() as usize).and_then(|x| x.as_ref())
    }

    pub fn get_effect_file(&self, id: EffectFileId) -> Option<&str> {
        self.effect_files
            .get(id.get() as usize)
            .and_then(|x| x.as_deref())
            .filter(|path| !path.is_empty())
    }

    pub fn iter(&self) -> impl Iterator<Item = &EffectData> {
        self.effects.iter().filter_map(|x| x.as_ref())
    }

    pub fn iter_files(&self) -> impl Iterator<Item = (EffectFileId, &str)> {
        self.effect_files
            .iter()
            .enumerate()
            .filter_map(|(id, path)| {
                path.as_deref()
                    .filter(|path| !path.is_empty())
                    .and_then(|path| Some((EffectFileId::new(id as u16)?, path)))
            })
    }
}
