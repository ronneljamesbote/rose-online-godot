use enum_map::{Enum, EnumMap};

use crate::{MotionFileData, MotionId};

#[derive(Copy, Clone, Debug, Enum)]
pub enum CharacterMotionAction {
    Stop1,
    Stop2,
    Walk,
    Run,
    Sitting,
    Sit,
    Standup,
    Stop3,
    Attack,
    Attack2,
    Attack3,
    Hit,
    Fall,
    Die,
    Raise,
    Jump1,
    Jump2,
    Pickitem,
}

#[derive(Copy, Clone, Debug, Enum)]
pub enum VehicleMotionAction {
    Stop,
    Move,
    Attack1,
    Attack2,
    Attack3,
    Die,
    Special1,
    Special2,
}

pub struct CharacterMotionDatabase {
    weapon_type_count: usize,
    motion_indices: Vec<u16>,
    motion_data: Vec<Vec<Option<MotionFileData>>>, // [gender][motion id]
    action_map: EnumMap<CharacterMotionAction, MotionId>,
    vehicle_action_map: EnumMap<VehicleMotionAction, u16>,
}

pub struct CharacterMotionDatabaseOptions {
    pub load_frame_data: bool,
}

impl CharacterMotionDatabase {
    pub fn new(
        weapon_type_count: usize,
        motion_indices: Vec<u16>,
        motion_paths: Vec<Vec<Option<MotionFileData>>>,
        action_map: EnumMap<CharacterMotionAction, MotionId>,
        vehicle_action_map: EnumMap<VehicleMotionAction, u16>,
    ) -> Self {
        Self {
            weapon_type_count,
            motion_indices,
            motion_data: motion_paths,
            action_map,
            vehicle_action_map,
        }
    }

    /// How many weapon motion types (TYPE_MOTION.STB columns) there are.
    pub fn weapon_type_count(&self) -> usize {
        self.weapon_type_count
    }

    /// Every motion file, per gender (0 is shared), by motion index.
    pub fn motion_files(&self, gender: usize) -> impl Iterator<Item = &MotionFileData> {
        self.motion_data.get(gender).into_iter().flatten().flatten()
    }

    /// Replaces the timing of every entry for the motion file `path` (normalised, as
    /// `VfsPathBuf` stores it). Returns how many entries matched.
    pub fn set_motion_timing(
        &mut self,
        path: &std::path::Path,
        duration: std::time::Duration,
        total_attack_frames: usize,
        first_attack_time: Option<std::time::Duration>,
    ) -> usize {
        let mut matched = 0;
        for motion in self.motion_data.iter_mut().flatten().flatten() {
            if motion.path.path() == path {
                motion.duration = duration;
                motion.total_attack_frames = total_attack_frames;
                motion.first_attack_time = first_attack_time;
                matched += 1;
            }
        }
        matched
    }

    pub fn get_character_motion(
        &self,
        motion_id: MotionId,
        weapon_motion_type: usize,
        gender: usize,
    ) -> Option<&MotionFileData> {
        // Only ever use this weapon_motion_type
        let index = *self
            .motion_indices
            .get(motion_id.get() as usize * self.weapon_type_count + weapon_motion_type)?
            as usize;

        // First try specific gender
        if gender != 0 {
            if let Some(motion_file_data) = self
                .motion_data
                .get(gender)
                .and_then(|x| x.get(index).and_then(|x| x.as_ref()))
            {
                return Some(motion_file_data);
            }
        }

        // Fallback to gender 0
        self.motion_data
            .get(0)
            .and_then(|x| x.get(index).and_then(|x| x.as_ref()))
    }

    pub fn find_first_character_motion(
        &self,
        motion_id: MotionId,
        weapon_motion_type: usize,
        gender: usize,
    ) -> Option<&MotionFileData> {
        // Check if weapon has a motion index
        let mut index = *self
            .motion_indices
            .get(motion_id.get() as usize * self.weapon_type_count + weapon_motion_type)?
            as usize;

        // Fallback to no weapon
        if index == 0 {
            index = *self
                .motion_indices
                .get(motion_id.get() as usize * self.weapon_type_count)?
                as usize;
        }

        // Check if gender != 0 has motion, else fall back to gender 0
        if gender != 0 {
            if let Some(motion_file_data) = self
                .motion_data
                .get(gender)
                .and_then(|x| x.get(index).and_then(|x| x.as_ref()))
            {
                return Some(motion_file_data);
            }
        }

        self.motion_data
            .get(0)
            .and_then(|x| x.get(index).and_then(|x| x.as_ref()))
    }

    pub fn get_character_action_motion(
        &self,
        action: CharacterMotionAction,
        weapon_motion_type: usize,
        gender: usize,
    ) -> Option<&MotionFileData> {
        self.get_character_motion(self.action_map[action], weapon_motion_type, gender)
    }

    pub fn find_first_character_action_motion(
        &self,
        action: CharacterMotionAction,
        weapon_motion_type: usize,
        gender: usize,
    ) -> Option<&MotionFileData> {
        self.find_first_character_motion(self.action_map[action], weapon_motion_type, gender)
    }

    pub fn get_vehicle_action_motion(
        &self,
        action: VehicleMotionAction,
        base_motion_index: usize,
        weapon_motion_type: usize,
    ) -> Option<&MotionFileData> {
        let index = *self.motion_indices.get(
            (base_motion_index + self.vehicle_action_map[action] as usize) * self.weapon_type_count
                + weapon_motion_type,
        )? as usize;

        self.motion_data
            .get(0)
            .and_then(|x| x.get(index).and_then(|x| x.as_ref()))
    }
}
