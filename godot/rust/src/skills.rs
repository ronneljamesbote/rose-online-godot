//! Skills and status effects for the UI: icons from SKILLICON.TSI and STATEICON.TSI, and
//! descriptions for skill slots and tooltips.

use godot::prelude::*;
use rose_data::{AbilityType, SkillCooldown, SkillData, SkillId, SkillType};

use crate::{
    data,
    items::{icon, IconSheet},
};

fn split_camel_case(raw: &str) -> String {
    let mut out = String::new();
    for (i, ch) in raw.chars().enumerate() {
        if i > 0 && ch.is_uppercase() {
            out.push(' ');
        }
        out.push(ch);
    }
    out
}

/// The next level of a skill, if it has one.
pub fn next_level(skill: &SkillData) -> Option<&'static SkillData> {
    let game = data::get()?;
    SkillId::new(skill.id.get() + 1)
        .and_then(|id| game.skills.get_skill(id))
        .filter(|next| next.base_skill_id == skill.base_skill_id && next.level == skill.level + 1)
}

/// Everything a skill slot or tooltip shows: id, name, level, icon, type, passive,
/// target (whether it needs a target entity), area (whether it aims at the ground),
/// next_cost (skill points for the next level, -1 at the highest) and tooltip.
pub fn skill_dict(skill: &SkillData) -> VarDictionary {
    let mut d = VarDictionary::new();
    d.set("id", skill.id.get() as i64);
    d.set("name", skill.name);
    d.set("level", skill.level as i64);
    if let Some(texture) = icon(IconSheet::Skill, skill.icon_number) {
        d.set("icon", &texture);
    }
    let kind = split_camel_case(&format!("{:?}", skill.skill_type));
    d.set("type", kind.as_str());
    d.set("passive", matches!(skill.skill_type, SkillType::Passive));
    d.set("target", skill.skill_type.is_target_skill());
    d.set("area", matches!(skill.skill_type, SkillType::AreaTarget));
    let next = next_level(skill);
    d.set("next_cost", next.map_or(-1, |n| n.learn_point_cost as i64));

    let mut lines = vec![format!("{}   Level {}", kind, skill.level)];
    for &(ability, value) in skill.use_ability.iter() {
        lines.push(match ability {
            AbilityType::Mana => format!("Uses {value} MP"),
            AbilityType::Health => format!("Uses {value} HP"),
            other => format!("Uses {value} {other:?}"),
        });
    }
    if skill.cast_range > 0 {
        lines.push(format!("Range {:.1} m", skill.cast_range as f32 / 100.0));
    }
    if skill.scope > 0 {
        lines.push(format!("Area {:.1} m", skill.scope as f32 / 100.0));
    }
    if skill.power > 0 && !matches!(skill.skill_type, SkillType::Passive) {
        lines.push(format!("Power {}", skill.power));
    }
    let seconds = skill.status_effect_duration.as_secs();
    if seconds > 0 {
        lines.push(format!("Lasts {seconds} s"));
    }
    let cooldown = match skill.cooldown {
        SkillCooldown::Skill { duration } | SkillCooldown::Group { duration, .. } => duration.as_secs_f32(),
    };
    if cooldown > 0.0 {
        lines.push(format!("Cooldown {cooldown:.0} s"));
    }
    if let Some(game) = data::get() {
        if let Some(job_class) = skill.required_job_class.and_then(|id| game.job_classes.get(id)) {
            lines.push(format!("For {}", job_class.name));
        }
        for &(id, level) in skill.required_skills.iter() {
            if let Some(required) = game.skills.get_skill(id) {
                lines.push(format!("Needs {} level {}", required.name, level));
            }
        }
    }
    for &(ability, value) in skill.required_ability.iter() {
        lines.push(format!("Needs {ability:?} {value}"));
    }
    if let Some(next) = next {
        lines.push(format!("Next level: {} skill points", next.learn_point_cost));
    }
    if !skill.description.is_empty() {
        lines.push(skill.description.to_string());
    }
    d.set("tooltip", lines.join("\n").as_str());
    d
}
