//! Skill pages: one per skill, with a row per level.

use std::collections::BTreeMap;

use rose_data::{AbilityType, ItemData, ItemType, SkillCooldown, SkillData, SkillDamageType};
use serde_yaml::Value;

use crate::{
    page::{one_line, round1, row, seq, strs, Page, Pages},
    Ctx,
};

pub fn write(cx: &Ctx, pages: &mut Pages) {
    let game = cx.game;
    let mut levels: BTreeMap<u16, Vec<&SkillData>> = BTreeMap::new();
    for skill in game.skills.iter() {
        let base = cx.skill_base.get(&skill.id.get()).copied().unwrap_or(skill.id.get());
        if cx.skill_pages.contains_key(&base) {
            levels.entry(base).or_default().push(skill);
        }
    }
    let mut books: BTreeMap<u16, Vec<String>> = BTreeMap::new();
    for item in game.items.iter_items(ItemType::Consumable) {
        if let Some(ItemData::Consumable(c)) = game.items.get_item(item) {
            if let Some(s) = c.learn_skill_id {
                let base = cx.skill_base.get(&s.get()).copied().unwrap_or(s.get());
                if cx.item_pages.contains_key(&crate::ikey(item)) {
                    books.entry(base).or_default().push(cx.item_link(item));
                }
            }
        }
    }

    let mut list = Page::new("skills", "list");
    list.set("title", "Skills");
    list.set("columns", strs(["type", "job", "max_level", "target"].map(String::from)));
    list.line("# Skills");
    pages.add(list);

    for (base, mut lv) in levels {
        lv.sort_by_key(|s| s.level);
        let first = lv[0];
        let (path, name) = &cx.skill_pages[&base];
        let mut p = Page::new(path.clone(), "skill");
        p.set("id", base as i64);
        p.set("name", name.as_str());
        p.set("status", "in-game");
        p.set("icon", format!("skill/{}", first.icon_number));
        let t = game.string_database.get_skill_type(first.skill_type);
        p.set("type", if t.is_empty() { format!("{:?}", first.skill_type) } else { t.to_string() });
        if let Some(j) = first.required_job_class {
            p.set("job", cx.job_class_name(j));
        } else {
            p.set("job", "any");
        }
        p.set("max_level", lv.iter().map(|s| s.level).max().unwrap_or(1) as i64);
        let target = game.string_database.get_skill_target_filter(first.target_filter);
        p.set_some("target", if target.is_empty() { format!("{:?}", first.target_filter) } else { target.to_string() });
        if first.power > 0 {
            p.set("damage_type", match first.damage_type {
                SkillDamageType::ContinuousAttack => "continuous attack",
                SkillDamageType::WeaponAttack => "weapon attack",
                SkillDamageType::MagicAttack => "magic attack",
                SkillDamageType::NaturalMagic => "natural magic",
            });
        }
        let weapons: Vec<String> = first
            .required_equipment_class
            .iter()
            .map(|c| {
                let n = game.string_database.get_item_class(*c);
                if n.is_empty() { format!("{c:?}") } else { n.to_string() }
            })
            .collect();
        p.set_some("needs_weapon", weapons.join(", "));
        if let Some(n) = first.summon_npc_id {
            p.set("summons", cx.npc_link(n.get()));
        }
        if let Some(z) = first.warp_zone_id {
            p.set("warps_to", cx.zone_link(z.get()));
        }
        if let Some(b) = books.get(&base) {
            p.set("skill_books", strs(b.clone()));
        }

        let rows: Vec<Value> = lv
            .iter()
            .map(|s| {
                let level_req = s.required_ability.iter().find(|(a, _)| *a == AbilityType::Level).map(|(_, v)| *v);
                let other_req: Vec<String> = s
                    .required_ability
                    .iter()
                    .filter(|(a, _)| *a != AbilityType::Level)
                    .map(|(a, v)| format!("{} {v}", cx.ability_name(*a)))
                    .chain(s.required_skills.iter().map(|(id, l)| {
                        let base = cx.skill_base.get(&id.get()).copied().unwrap_or(id.get());
                        match cx.skill_pages.get(&base) {
                            Some((p, n)) => format!("{} level {l}", crate::page::link(p, n)),
                            None => format!("skill {} level {l}", id.get()),
                        }
                    }))
                    .collect();
                let cost: Vec<String> =
                    s.use_ability.iter().map(|(a, v)| format!("{} {v}", cx.ability_name(*a))).collect();
                let effects: Vec<String> = s.status_effects.iter().flatten().map(|e| cx.status_effect_name(*e)).collect();
                let adds: Vec<String> = s
                    .add_ability
                    .iter()
                    .flatten()
                    .map(|a| {
                        let mut t = format!("{}", cx.ability_name(a.ability_type));
                        if a.value != 0 {
                            t += &format!(" {}{}", if a.value > 0 { "+" } else { "" }, a.value);
                        }
                        if a.rate != 0 {
                            t += &format!(" {}{}%", if a.rate > 0 { "+" } else { "" }, a.rate);
                        }
                        t
                    })
                    .collect();
                let cooldown = match s.cooldown {
                    SkillCooldown::Skill { duration } | SkillCooldown::Group { duration, .. } => duration.as_secs_f32(),
                };
                row([
                    ("level", (s.level as i64).into()),
                    ("id", (s.id.get() as i64).into()),
                    ("needs_level", level_req.map_or(Value::Null, |v| (v as i64).into())),
                    ("needs", if other_req.is_empty() { Value::Null } else { other_req.join(", ").into() }),
                    ("learn_points", (s.learn_point_cost as i64).into()),
                    ("cost", if cost.is_empty() { Value::Null } else { cost.join(", ").into() }),
                    ("power", if s.power == 0 { Value::Null } else { (s.power as i64).into() }),
                    ("range", if s.cast_range == 0 { Value::Null } else { round1(s.cast_range as f32 / 100.0) }),
                    ("area", if s.scope == 0 { Value::Null } else { round1(s.scope as f32 / 100.0) }),
                    ("cooldown", if cooldown == 0.0 { Value::Null } else { round1(cooldown) }),
                    (
                        "duration",
                        if s.status_effect_duration.is_zero() {
                            Value::Null
                        } else {
                            round1(s.status_effect_duration.as_secs_f32())
                        },
                    ),
                    ("success", if s.success_ratio == 0 { Value::Null } else { (s.success_ratio as i64).into() }),
                    ("effects", if effects.is_empty() { Value::Null } else { effects.join(", ").into() }),
                    ("changes", if adds.is_empty() { Value::Null } else { adds.join(", ").into() }),
                ])
            })
            .collect();
        p.set("levels", seq(rows));
        p.set(
            "source",
            row([
                ("data", format!("LIST_SKILL.STB rows {}", lv.iter().map(|s| s.id.get().to_string()).collect::<Vec<_>>().join(", ")).into()),
                ("code", "module/src/skills.rs".into()),
            ]),
        );
        p.line(format!("# {name}"));
        let desc = one_line(first.description);
        if !desc.is_empty() {
            p.line("");
            p.line(desc);
        }
        p.line("");
        p.line("Range and area are in metres, cooldown and duration in seconds, success in percent.");
        p.line("How power turns into damage is on [[rules/skills|Skills (rules)]].");
        pages.add(p);
    }
}
