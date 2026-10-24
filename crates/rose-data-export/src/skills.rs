//! Skills, status effects and job classes: `skills.toml` (one `[[skill]]` per skill level),
//! `status-effects.toml` (`[[status_effect]]`) and `job-classes.toml` (`[[job_class]]`). The
//! keys are the property names of the Godot resources (`RoseSkill`, `RoseStatusEffect`,
//! `RoseJobClass`) they are exported from; a key left out has its default (0, empty text,
//! `false`, no stat, none).
//!
//! Each level of a skill is its own skill with its own id, as in iROSE's LIST_SKILL.STB:
//! characters store the id of the level they learnt. A link to another skill, a status
//! effect, a job class or a monster is written as its id. Two fields stay plain numbers
//! because they often name the thing itself, which a resource can't link: a skill's
//! `base_skill_id` (its first level) and a status effect's `apply_status_effect_1/2`.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    num::NonZeroUsize,
    sync::Arc,
    time::Duration,
};

use anyhow::{bail, Context};
use arrayvec::ArrayVec;
use rose_data::{
    AbilityType, EffectFileId, EffectId, ItemClass, JobClassData, JobClassDatabase, JobClassId,
    JobId, MotionId, NpcId, SkillActionMode, SkillAddAbility, SkillBasicCommand, SkillCastingEffect,
    SkillCooldown, SkillDamageType, SkillData, SkillDatabase, SkillId, SkillTargetFilter, SkillType,
    SoundId, StatusEffectClearedByType, StatusEffectData, StatusEffectDatabase, StatusEffectId,
    StatusEffectType, StringDatabase, ZoneId,
};
use serde::{Deserialize, Serialize};

use crate::Format;

pub const SKILLS_FILE: &str = "skills";
pub const STATUS_EFFECTS_FILE: &str = "status-effects";
pub const JOB_CLASSES_FILE: &str = "job-classes";

/// The status effect the game uses for a summon's lifetime running out (iROSE's row 43).
pub const DECREASE_SUMMON_LIFE_STATUS_EFFECT: u16 = 43;

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

/// What a model point field holds for "none" (iROSE writes 999).
pub const NO_POINT: i32 = -1;

fn no_point() -> i32 {
    NO_POINT
}

fn is_no_point(value: &i32) -> bool {
    *value == NO_POINT
}

fn point(value: Option<usize>) -> i32 {
    value.map_or(NO_POINT, |v| v as i32)
}

fn point_of(value: i32) -> Option<usize> {
    (value >= 0).then_some(value as usize)
}

/// 100 = the motion's own speed.
fn speed_percent(speed: f32) -> u32 {
    (speed * 100.0).round() as u32
}

fn number<T: Copy>(id: Option<T>, get: fn(&T) -> u16) -> u16 {
    id.as_ref().map_or(0, get)
}

/// Gives `text` a `'static` lifetime by keeping it in `keep`, which the database owns.
fn keep(keep: &mut Vec<String>, text: &str) -> &'static str {
    if text.is_empty() {
        return "";
    }
    let owned = text.to_string();
    // SAFETY: the String's buffer never moves or changes while it sits in `keep`, and
    // `keep` lives in the database (`with_text`) as long as the data pointing into it.
    let text: &'static str = unsafe { std::mem::transmute::<&str, &'static str>(owned.as_str()) };
    keep.push(owned);
    text
}

/// One skill level.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SkillExport {
    /// The number (iROSE's LIST_SKILL.STB row). Characters store it.
    pub id: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    #[serde(skip_serializing_if = "is_default")]
    pub description: String,
    /// The skill's first level (0 = this one): the levels of a skill share it.
    #[serde(skip_serializing_if = "is_default")]
    pub base_skill_id: u16,
    /// Always written: the resource's own default is 1.
    pub level: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub skill_type: SkillType,
    #[serde(skip_serializing_if = "is_default")]
    pub page: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub icon_number: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub learn_point_cost: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub learn_money_cost: u32,

    #[serde(skip_serializing_if = "is_default")]
    pub required_job_class: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub required_planet: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub required_union_1: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub required_union_2: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub required_union_3: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub required_skill_1: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub required_skill_1_level: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub required_skill_2: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub required_skill_2_level: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub required_skill_3: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub required_skill_3_level: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub required_stat_1: Option<AbilityType>,
    #[serde(skip_serializing_if = "is_default")]
    pub required_stat_1_value: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub required_stat_2: Option<AbilityType>,
    #[serde(skip_serializing_if = "is_default")]
    pub required_stat_2_value: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub required_item_class_1: Option<ItemClass>,
    #[serde(skip_serializing_if = "is_default")]
    pub required_item_class_2: Option<ItemClass>,
    #[serde(skip_serializing_if = "is_default")]
    pub required_item_class_3: Option<ItemClass>,
    #[serde(skip_serializing_if = "is_default")]
    pub required_item_class_4: Option<ItemClass>,
    #[serde(skip_serializing_if = "is_default")]
    pub required_item_class_5: Option<ItemClass>,

    #[serde(skip_serializing_if = "is_default")]
    pub use_stat_1: Option<AbilityType>,
    #[serde(skip_serializing_if = "is_default")]
    pub use_stat_1_value: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub use_stat_2: Option<AbilityType>,
    #[serde(skip_serializing_if = "is_default")]
    pub use_stat_2_value: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub cooldown_ms: u32,
    /// Skills of one group (0 = none) share a cooldown.
    #[serde(skip_serializing_if = "is_default")]
    pub cooldown_group: u32,

    #[serde(skip_serializing_if = "is_default")]
    pub target_filter: SkillTargetFilter,
    #[serde(skip_serializing_if = "is_default")]
    pub cast_range: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub scope: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub action_mode: SkillActionMode,
    #[serde(skip_serializing_if = "is_default")]
    pub basic_command: Option<SkillBasicCommand>,
    #[serde(skip_serializing_if = "is_default")]
    pub damage_type: SkillDamageType,
    #[serde(skip_serializing_if = "is_default")]
    pub power: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub harm: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub item_make_number: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub success_ratio: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub status_effect_1: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub status_effect_2: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub status_effect_seconds: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub add_stat_1: Option<AbilityType>,
    #[serde(skip_serializing_if = "is_default")]
    pub add_stat_1_value: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub add_stat_1_rate: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub add_stat_2: Option<AbilityType>,
    #[serde(skip_serializing_if = "is_default")]
    pub add_stat_2_value: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub add_stat_2_rate: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub summon_monster: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub warp_zone: u16,
    /// Centimetres.
    #[serde(skip_serializing_if = "is_default")]
    pub warp_x: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub warp_y: i32,

    #[serde(skip_serializing_if = "is_default")]
    pub casting_motion_id: u16,
    /// Percent (100 = the motion's own speed); 0 reads as 100.
    #[serde(skip_serializing_if = "is_default")]
    pub casting_motion_speed: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub casting_repeat_motion_id: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub casting_repeat_motion_count: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub casting_effect_1: u16,
    #[serde(default = "no_point", skip_serializing_if = "is_no_point")]
    pub casting_effect_1_point: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub casting_effect_2: u16,
    #[serde(default = "no_point", skip_serializing_if = "is_no_point")]
    pub casting_effect_2_point: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub casting_effect_3: u16,
    #[serde(default = "no_point", skip_serializing_if = "is_no_point")]
    pub casting_effect_3_point: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub casting_effect_4: u16,
    #[serde(default = "no_point", skip_serializing_if = "is_no_point")]
    pub casting_effect_4_point: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub action_motion_id: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub action_motion_speed: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub bullet_effect: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub bullet_point: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub bullet_fire_sound: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub hit_effect: u16,
    #[serde(default = "no_point", skip_serializing_if = "is_no_point")]
    pub hit_effect_point: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub hit_sound: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub hit_dummy_effect_1: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub hit_dummy_sound_1: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub hit_dummy_effect_2: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub hit_dummy_sound_2: u16,
}

/// A stat and its value, from the export's `<name>` and `<name>_value` pair.
fn stat<V: Copy>(stat: Option<AbilityType>, value: V) -> Option<(AbilityType, V)> {
    stat.map(|stat| (stat, value))
}

fn pair<V: Copy + Default>(list: &[(AbilityType, V)], i: usize) -> (Option<AbilityType>, V) {
    list.get(i).map_or((None, V::default()), |(s, v)| (Some(*s), *v))
}

impl SkillExport {
    pub fn from_data(skill: &SkillData) -> Self {
        let mut out = Self {
            id: skill.id.get(),
            name: skill.name.to_string(),
            description: skill.description.to_string(),
            base_skill_id: number(skill.base_skill_id.filter(|b| *b != skill.id), SkillId::get),
            level: skill.level,
            skill_type: skill.skill_type,
            page: skill.page as u32,
            icon_number: skill.icon_number,
            learn_point_cost: skill.learn_point_cost,
            learn_money_cost: skill.learn_money_cost,
            required_job_class: number(skill.required_job_class, JobClassId::get),
            required_planet: skill.required_planet.map_or(0, |p| p.get() as u32),
            target_filter: skill.target_filter,
            cast_range: skill.cast_range,
            scope: skill.scope,
            action_mode: skill.action_mode,
            basic_command: skill.basic_command,
            damage_type: skill.damage_type,
            power: skill.power,
            harm: skill.harm,
            item_make_number: skill.item_make_number,
            success_ratio: skill.success_ratio,
            status_effect_1: number(skill.status_effects[0], StatusEffectId::get),
            status_effect_2: number(skill.status_effects[1], StatusEffectId::get),
            status_effect_seconds: skill.status_effect_duration.as_secs() as u32,
            summon_monster: number(skill.summon_npc_id, NpcId::get),
            warp_zone: number(skill.warp_zone_id, ZoneId::get),
            warp_x: skill.warp_zone_x as i32,
            warp_y: skill.warp_zone_y as i32,
            casting_motion_id: number(skill.casting_motion_id, MotionId::get),
            casting_motion_speed: speed_percent(skill.casting_motion_speed),
            casting_repeat_motion_id: number(skill.casting_repeat_motion_id, MotionId::get),
            casting_repeat_motion_count: skill.casting_repeat_motion_count,
            action_motion_id: number(skill.action_motion_id, MotionId::get),
            action_motion_speed: speed_percent(skill.action_motion_speed),
            bullet_effect: number(skill.bullet_effect_id, EffectId::get),
            bullet_point: skill.bullet_link_dummy_bone_id,
            bullet_fire_sound: number(skill.bullet_fire_sound_id, SoundId::get),
            hit_effect: number(skill.hit_effect_file_id, EffectFileId::get),
            hit_effect_point: point(skill.hit_link_dummy_bone_id),
            hit_sound: number(skill.hit_sound_id, SoundId::get),
            hit_dummy_effect_1: number(skill.hit_dummy_effect_file_id[0], EffectFileId::get),
            hit_dummy_effect_2: number(skill.hit_dummy_effect_file_id[1], EffectFileId::get),
            hit_dummy_sound_1: number(skill.hit_dummy_sound_id[0], SoundId::get),
            hit_dummy_sound_2: number(skill.hit_dummy_sound_id[1], SoundId::get),
            ..Default::default()
        };
        let union = |i: usize| skill.required_union.get(i).map_or(0, |u| u.get() as u32);
        (out.required_union_1, out.required_union_2, out.required_union_3) = (union(0), union(1), union(2));
        let needs = |i: usize| skill.required_skills.get(i).map_or((0, 0), |(s, l)| (s.get(), *l));
        (out.required_skill_1, out.required_skill_1_level) = needs(0);
        (out.required_skill_2, out.required_skill_2_level) = needs(1);
        (out.required_skill_3, out.required_skill_3_level) = needs(2);
        (out.required_stat_1, out.required_stat_1_value) = pair(&skill.required_ability, 0);
        (out.required_stat_2, out.required_stat_2_value) = pair(&skill.required_ability, 1);
        let class = |i: usize| skill.required_equipment_class.get(i).copied();
        out.required_item_class_1 = class(0);
        out.required_item_class_2 = class(1);
        out.required_item_class_3 = class(2);
        out.required_item_class_4 = class(3);
        out.required_item_class_5 = class(4);
        (out.use_stat_1, out.use_stat_1_value) = pair(&skill.use_ability, 0);
        (out.use_stat_2, out.use_stat_2_value) = pair(&skill.use_ability, 1);
        match skill.cooldown {
            SkillCooldown::Skill { duration } => out.cooldown_ms = duration.as_millis() as u32,
            SkillCooldown::Group { group, duration } => {
                out.cooldown_ms = duration.as_millis() as u32;
                out.cooldown_group = group.get() as u32;
            }
        }
        let add = |i: usize| skill.add_ability[i].as_ref().map_or((None, 0, 0), |a| (Some(a.ability_type), a.value, a.rate));
        (out.add_stat_1, out.add_stat_1_value, out.add_stat_1_rate) = add(0);
        (out.add_stat_2, out.add_stat_2_value, out.add_stat_2_rate) = add(1);
        let cast = |i: usize| {
            skill.casting_effects[i]
                .as_ref()
                .map_or((0, NO_POINT), |e| (e.effect_file_id.get(), point(e.effect_dummy_bone_id)))
        };
        (out.casting_effect_1, out.casting_effect_1_point) = cast(0);
        (out.casting_effect_2, out.casting_effect_2_point) = cast(1);
        (out.casting_effect_3, out.casting_effect_3_point) = cast(2);
        (out.casting_effect_4, out.casting_effect_4_point) = cast(3);
        out
    }

    /// The skills it needs.
    pub fn skill_links(&self) -> impl Iterator<Item = u16> {
        [self.required_skill_1, self.required_skill_2, self.required_skill_3]
            .into_iter()
            .filter(|s| *s != 0)
    }

    pub fn to_data(&self, text: &mut Vec<String>) -> Result<SkillData, anyhow::Error> {
        let mut required_union = ArrayVec::new();
        for union in [self.required_union_1, self.required_union_2, self.required_union_3] {
            if let Some(union) = NonZeroUsize::new(union as usize) {
                required_union.push(union);
            }
        }
        let mut required_skills = ArrayVec::new();
        for (skill, level) in [
            (self.required_skill_1, self.required_skill_1_level),
            (self.required_skill_2, self.required_skill_2_level),
            (self.required_skill_3, self.required_skill_3_level),
        ] {
            if let Some(skill) = SkillId::new(skill) {
                required_skills.push((skill, level));
            }
        }
        let required_equipment_class = [
            self.required_item_class_1,
            self.required_item_class_2,
            self.required_item_class_3,
            self.required_item_class_4,
            self.required_item_class_5,
        ]
        .into_iter()
        .flatten()
        .collect();
        let duration = Duration::from_millis(self.cooldown_ms as u64);
        let cooldown = match NonZeroUsize::new(self.cooldown_group as usize) {
            Some(group) => SkillCooldown::Group { group, duration },
            None => SkillCooldown::Skill { duration },
        };
        let add = |ability: Option<AbilityType>, value: i32, rate: i32| {
            ability.map(|ability_type| SkillAddAbility { ability_type, value, rate })
        };
        let cast = |effect: u16, point: i32| {
            EffectFileId::new(effect).map(|effect_file_id| SkillCastingEffect {
                effect_file_id,
                effect_dummy_bone_id: point_of(point),
            })
        };
        let motion = |id: u16| (id != 0).then(|| MotionId::new(id));
        let speed = |percent: u32| if percent == 0 { 1.0 } else { percent as f32 / 100.0 };
        Ok(SkillData {
            id: SkillId::new(self.id).context("skill ids start at 1")?,
            name: keep(text, &self.name),
            description: keep(text, &self.description),
            base_skill_id: SkillId::new(if self.base_skill_id == 0 { self.id } else { self.base_skill_id }),
            level: self.level,
            learn_point_cost: self.learn_point_cost,
            learn_money_cost: self.learn_money_cost,
            skill_type: self.skill_type,
            page: self.page as usize,
            icon_number: self.icon_number,
            use_ability: [stat(self.use_stat_1, self.use_stat_1_value), stat(self.use_stat_2, self.use_stat_2_value)]
                .into_iter()
                .flatten()
                .collect(),
            required_ability: [
                stat(self.required_stat_1, self.required_stat_1_value),
                stat(self.required_stat_2, self.required_stat_2_value),
            ]
            .into_iter()
            .flatten()
            .collect(),
            required_job_class: JobClassId::new(self.required_job_class),
            required_planet: NonZeroUsize::new(self.required_planet as usize),
            required_skills,
            required_union,
            required_equipment_class,
            action_mode: self.action_mode,
            action_motion_id: motion(self.action_motion_id),
            action_motion_speed: speed(self.action_motion_speed),
            add_ability: [
                add(self.add_stat_1, self.add_stat_1_value, self.add_stat_1_rate),
                add(self.add_stat_2, self.add_stat_2_value, self.add_stat_2_rate),
            ],
            basic_command: self.basic_command,
            bullet_effect_id: EffectId::new(self.bullet_effect),
            bullet_link_dummy_bone_id: self.bullet_point,
            bullet_fire_sound_id: SoundId::new(self.bullet_fire_sound),
            cast_range: self.cast_range,
            casting_motion_id: motion(self.casting_motion_id),
            casting_motion_speed: speed(self.casting_motion_speed),
            casting_repeat_motion_id: motion(self.casting_repeat_motion_id),
            casting_repeat_motion_count: self.casting_repeat_motion_count.max(1),
            casting_effects: [
                cast(self.casting_effect_1, self.casting_effect_1_point),
                cast(self.casting_effect_2, self.casting_effect_2_point),
                cast(self.casting_effect_3, self.casting_effect_3_point),
                cast(self.casting_effect_4, self.casting_effect_4_point),
            ],
            cooldown,
            damage_type: self.damage_type,
            harm: self.harm,
            hit_effect_file_id: EffectFileId::new(self.hit_effect),
            hit_link_dummy_bone_id: point_of(self.hit_effect_point),
            hit_sound_id: SoundId::new(self.hit_sound),
            hit_dummy_effect_file_id: [
                EffectFileId::new(self.hit_dummy_effect_1),
                EffectFileId::new(self.hit_dummy_effect_2),
            ],
            hit_dummy_sound_id: [SoundId::new(self.hit_dummy_sound_1), SoundId::new(self.hit_dummy_sound_2)],
            item_make_number: self.item_make_number,
            power: self.power,
            scope: self.scope,
            status_effects: [StatusEffectId::new(self.status_effect_1), StatusEffectId::new(self.status_effect_2)],
            status_effect_duration: Duration::from_secs(self.status_effect_seconds as u64),
            success_ratio: self.success_ratio,
            summon_npc_id: NpcId::new(self.summon_monster),
            target_filter: self.target_filter,
            warp_zone_id: ZoneId::new(self.warp_zone),
            warp_zone_x: self.warp_x as f32,
            warp_zone_y: self.warp_y as f32,
        })
    }
}

/// One status effect.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct StatusEffectExport {
    /// The number (iROSE's LIST_STATUS.STB row).
    pub id: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    #[serde(skip_serializing_if = "is_default")]
    pub description: String,
    #[serde(skip_serializing_if = "is_default")]
    pub start_message: String,
    #[serde(skip_serializing_if = "is_default")]
    pub end_message: String,
    #[serde(skip_serializing_if = "is_default")]
    pub status_effect_type: StatusEffectType,
    #[serde(skip_serializing_if = "is_default")]
    pub can_be_reapplied: bool,
    #[serde(skip_serializing_if = "is_default")]
    pub cleared_by: StatusEffectClearedByType,
    /// The effects it runs (often itself) and their amount per second; the first amount
    /// is also what a poison takes per second.
    #[serde(skip_serializing_if = "is_default")]
    pub apply_status_effect_1: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub apply_status_effect_1_value: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub apply_status_effect_2: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub apply_status_effect_2_value: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub effect: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub icon_id: u32,
}

impl StatusEffectExport {
    pub fn from_data(effect: &StatusEffectData) -> Self {
        let apply = |i: usize| effect.apply_status_effects.get(i).map_or((0, 0), |(s, v)| (s.get(), *v));
        let (apply_status_effect_1, apply_status_effect_1_value) = apply(0);
        let (apply_status_effect_2, apply_status_effect_2_value) = apply(1);
        Self {
            id: effect.id.get(),
            name: effect.name.to_string(),
            description: effect.description.to_string(),
            start_message: effect.start_message.to_string(),
            end_message: effect.end_message.to_string(),
            status_effect_type: effect.status_effect_type,
            can_be_reapplied: effect.can_be_reapplied,
            cleared_by: effect.cleared_by_type,
            apply_status_effect_1,
            apply_status_effect_1_value,
            apply_status_effect_2,
            apply_status_effect_2_value,
            effect: number(effect.effect_file_id, EffectFileId::get),
            icon_id: effect.icon_id,
        }
    }

    pub fn to_data(&self, text: &mut Vec<String>) -> Result<StatusEffectData, anyhow::Error> {
        let mut apply_status_effects = ArrayVec::new();
        for (effect, value) in [
            (self.apply_status_effect_1, self.apply_status_effect_1_value),
            (self.apply_status_effect_2, self.apply_status_effect_2_value),
        ] {
            if let Some(effect) = StatusEffectId::new(effect) {
                apply_status_effects.push((effect, value));
            }
        }
        Ok(StatusEffectData {
            id: StatusEffectId::new(self.id).context("status effect ids start at 1")?,
            name: keep(text, &self.name),
            description: keep(text, &self.description),
            start_message: keep(text, &self.start_message),
            end_message: keep(text, &self.end_message),
            status_effect_type: self.status_effect_type,
            can_be_reapplied: self.can_be_reapplied,
            cleared_by_type: self.cleared_by,
            apply_status_effects,
            apply_per_second_value: self.apply_status_effect_1_value,
            effect_file_id: EffectFileId::new(self.effect),
            icon_id: self.icon_id,
        })
    }
}

/// One job class: a set of jobs that skills and items can require.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct JobClassExport {
    /// The number (iROSE's LIST_CLASS.STB row).
    pub id: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    /// Job numbers (0 visitor, 111 soldier ...), up to 8.
    #[serde(skip_serializing_if = "is_default")]
    pub jobs: Vec<u16>,
}

impl JobClassExport {
    pub fn from_data(class: &JobClassData) -> Self {
        Self { id: class.id.get(), name: class.name.to_string(), jobs: class.jobs.iter().map(|j| j.get()).collect() }
    }

    /// None for a class with no jobs (it takes everyone, as a blank LIST_CLASS row does).
    pub fn to_data(&self, text: &mut Vec<String>) -> Result<Option<JobClassData>, anyhow::Error> {
        if self.jobs.is_empty() {
            return Ok(None);
        }
        let mut jobs = ArrayVec::new();
        for job in &self.jobs {
            jobs.try_push(JobId::new(*job)).ok().with_context(|| format!("job class {}: more than 8 jobs", self.id))?;
        }
        Ok(Some(JobClassData {
            id: JobClassId::new(self.id).context("job class ids start at 1")?,
            name: keep(text, &self.name),
            jobs,
        }))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SkillFile {
    #[serde(default)]
    skill: Vec<SkillExport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StatusEffectFile {
    #[serde(default)]
    status_effect: Vec<StatusEffectExport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct JobClassFile {
    #[serde(default)]
    job_class: Vec<JobClassExport>,
}

/// Every skill, status effect and job class, as exported.
#[derive(Debug, Default)]
pub struct SkillsExport {
    pub skills: Vec<SkillExport>,
    pub status_effects: Vec<StatusEffectExport>,
    pub job_classes: Vec<JobClassExport>,
}

impl SkillsExport {
    /// The converter's starting point and the same-data check.
    pub fn from_databases(
        skills: &SkillDatabase,
        status_effects: &StatusEffectDatabase,
        job_classes: &JobClassDatabase,
    ) -> Self {
        let mut effects: Vec<_> = status_effects.iter().map(StatusEffectExport::from_data).collect();
        effects.sort_by_key(|e| e.id);
        Self {
            skills: skills.iter().map(SkillExport::from_data).collect(),
            status_effects: effects,
            job_classes: job_classes.iter().map(JobClassExport::from_data).collect(),
        }
    }

    /// The three databases. `strings` is only kept for the other databases that share it.
    pub fn to_databases(
        &self,
        strings: Arc<StringDatabase>,
    ) -> Result<(SkillDatabase, StatusEffectDatabase, JobClassDatabase), anyhow::Error> {
        self.check()?;
        let mut text = Vec::new();
        let mut skills: Vec<Option<SkillData>> = Vec::new();
        for skill in &self.skills {
            let at = skill.id as usize;
            if skills.len() <= at {
                skills.resize_with(at + 1, || None);
            }
            skills[at] = Some(skill.to_data(&mut text).with_context(|| format!("skill {}", skill.id))?);
        }
        let mut effects = HashMap::new();
        for effect in &self.status_effects {
            effects.insert(effect.id, effect.to_data(&mut text).with_context(|| format!("status effect {}", effect.id))?);
        }
        let mut classes: Vec<Option<JobClassData>> = Vec::new();
        for class in &self.job_classes {
            let at = class.id as usize;
            if classes.len() <= at {
                classes.resize_with(at + 1, || None);
            }
            classes[at] = class.to_data(&mut text)?;
        }
        let skills = SkillDatabase::new(strings.clone(), skills).with_text(std::mem::take(&mut text));
        let effects = StatusEffectDatabase::new(
            strings.clone(),
            effects,
            StatusEffectId::new(DECREASE_SUMMON_LIFE_STATUS_EFFECT).unwrap(),
        )
        .with_text(std::mem::take(&mut text));
        // A number with no class below the last one takes everyone (a blank LIST_CLASS row),
        // one past it no one.
        let classes = JobClassDatabase::new(strings, classes).with_text(text);
        Ok((skills, effects, classes))
    }

    /// Ids used twice, and links to skills, status effects or job classes that aren't there.
    /// (Monsters a skill summons are checked by the game data loader, which has them.)
    pub fn check(&self) -> Result<(), anyhow::Error> {
        let mut skills = BTreeSet::new();
        for skill in &self.skills {
            if skill.id == 0 {
                bail!("skill 0: ids start at 1");
            }
            if !skills.insert(skill.id) {
                bail!("{SKILLS_FILE}: skill {} is there twice", skill.id);
            }
        }
        let mut effects = BTreeSet::new();
        for effect in &self.status_effects {
            if effect.id == 0 {
                bail!("status effect 0: ids start at 1");
            }
            if !effects.insert(effect.id) {
                bail!("{STATUS_EFFECTS_FILE}: status effect {} is there twice", effect.id);
            }
        }
        let mut classes = BTreeSet::new();
        for class in &self.job_classes {
            if class.id == 0 {
                bail!("job class 0: ids start at 1");
            }
            if !classes.insert(class.id) {
                bail!("{JOB_CLASSES_FILE}: job class {} is there twice", class.id);
            }
        }
        for skill in &self.skills {
            for other in skill.skill_links() {
                if !skills.contains(&other) {
                    bail!("skill {} names skill {other}, which isn't there", skill.id);
                }
            }
            for effect in [skill.status_effect_1, skill.status_effect_2] {
                if effect != 0 && !effects.contains(&effect) {
                    bail!("skill {} names status effect {effect}, which isn't there", skill.id);
                }
            }
            if skill.required_job_class != 0 && !classes.contains(&skill.required_job_class) {
                bail!("skill {} names job class {}, which isn't there", skill.id, skill.required_job_class);
            }
        }
        for effect in &self.status_effects {
            for other in [effect.apply_status_effect_1, effect.apply_status_effect_2] {
                if other != 0 && !effects.contains(&other) {
                    bail!("status effect {} names status effect {other}, which isn't there", effect.id);
                }
            }
        }
        Ok(())
    }

    /// The export's files: (file name without extension, contents).
    pub fn to_files_as(&self, format: Format) -> Result<Vec<(&'static str, String)>, anyhow::Error> {
        let mut skill = self.skills.clone();
        skill.sort_by_key(|s| s.id);
        let mut status_effect = self.status_effects.clone();
        status_effect.sort_by_key(|e| e.id);
        let mut job_class = self.job_classes.clone();
        job_class.sort_by_key(|c| c.id);
        Ok(vec![
            (SKILLS_FILE, format.write(SKILLS_FILE, &SkillFile { skill })?),
            (STATUS_EFFECTS_FILE, format.write(STATUS_EFFECTS_FILE, &StatusEffectFile { status_effect })?),
            (JOB_CLASSES_FILE, format.write(JOB_CLASSES_FILE, &JobClassFile { job_class })?),
        ])
    }

    /// Reads the export's files through `read` (file name without extension -> its text, or
    /// None when it isn't there). None when there is none of them; some missing is an error.
    pub fn from_files_as(
        format: Format,
        mut read: impl FnMut(&str) -> Result<Option<String>, anyhow::Error>,
    ) -> Result<Option<Self>, anyhow::Error> {
        let files = [SKILLS_FILE, STATUS_EFFECTS_FILE, JOB_CLASSES_FILE];
        let mut texts = Vec::new();
        for name in files {
            texts.push(read(name)?);
        }
        match texts.iter().filter(|t| t.is_some()).count() {
            0 => return Ok(None),
            n if n < files.len() => bail!(
                "the skill export has no {}",
                files
                    .iter()
                    .zip(&texts)
                    .filter(|(_, t)| t.is_none())
                    .map(|(n, _)| format!("{n}.{}", format.extension()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            _ => {}
        }
        let text = |i: usize| texts[i].as_deref().unwrap();
        Ok(Some(Self {
            skills: format.parse::<SkillFile>(SKILLS_FILE, text(0))?.skill,
            status_effects: format.parse::<StatusEffectFile>(STATUS_EFFECTS_FILE, text(1))?.status_effect,
            job_classes: format.parse::<JobClassFile>(JOB_CLASSES_FILE, text(2))?.job_class,
        }))
    }
}

/// One line per field that differs between `a` and `b` (by their debug output, so every
/// field counts), for things keyed by number.
fn compare_debug<T: std::fmt::Debug>(what: &str, a: BTreeMap<u16, &T>, b: BTreeMap<u16, &T>, out: &mut Vec<String>) {
    let ids: BTreeSet<u16> = a.keys().chain(b.keys()).copied().collect();
    for id in ids {
        match (a.get(&id), b.get(&id)) {
            (Some(_), None) => out.push(format!("{what} {id}: missing")),
            (None, Some(_)) => out.push(format!("{what} {id}: extra")),
            (x, y) => {
                let (x, y) = (format!("{x:#?}"), format!("{y:#?}"));
                if x != y {
                    for (l, r) in x.lines().zip(y.lines()).filter(|(l, r)| l != r) {
                        out.push(format!("{what} {id}: {} -> {}", l.trim(), r.trim()));
                    }
                    if x.lines().count() != y.lines().count() {
                        out.push(format!("{what} {id}: the fields differ"));
                    }
                }
            }
        }
    }
}

/// Every difference between two sets of skill, status effect and job class databases.
pub fn compare_skill_databases(
    a: (&SkillDatabase, &StatusEffectDatabase, &JobClassDatabase),
    b: (&SkillDatabase, &StatusEffectDatabase, &JobClassDatabase),
) -> Vec<String> {
    let mut out = Vec::new();
    fn skills(db: &SkillDatabase) -> BTreeMap<u16, &SkillData> {
        db.iter().map(|s| (s.id.get(), s)).collect()
    }
    fn effects(db: &StatusEffectDatabase) -> BTreeMap<u16, &StatusEffectData> {
        db.iter().map(|e| (e.id.get(), e)).collect()
    }
    compare_debug("skill", skills(a.0), skills(b.0), &mut out);
    compare_debug("status effect", effects(a.1), effects(b.1), &mut out);
    if a.1.get_decrease_summon_life_status_effect().map(|e| e.id) != b.1.get_decrease_summon_life_status_effect().map(|e| e.id) {
        out.push("status effects: the summon lifetime effect differs".into());
    }
    let classes = |db: &JobClassDatabase| {
        db.iter().map(|c| (c.id.get(), (c.name, c.jobs.iter().map(|j| j.get()).collect::<Vec<_>>()))).collect::<BTreeMap<_, _>>()
    };
    let (ca, cb) = (classes(a.2), classes(b.2));
    compare_debug("job class", ca.iter().map(|(k, v)| (*k, v)).collect(), cb.iter().map(|(k, v)| (*k, v)).collect(), &mut out);
    out
}
