//! Item pages, one folder per item type.

use std::collections::{BTreeMap, BTreeSet};

use rose_data::{ItemClass, ItemData, VehiclePartIndex, VehicleType};
use rose_data_irose::decode_item_base1000;
use rose_file_readers::{QsdCondition, QsdReward};
use serde_yaml::Value;

use crate::{
    ikey,
    page::{link, one_line, round1, row, seq, strs, Page, Pages},
    Ctx, ItemKey, ITEM_TYPES,
};

fn class_name(cx: &Ctx, class: ItemClass) -> String {
    let n = cx.game.string_database.get_item_class(class);
    if n.is_empty() {
        format!("{class:?}")
    } else {
        n.to_string()
    }
}

pub fn write(cx: &Ctx, pages: &mut Pages) {
    let game = cx.game;

    // Who refers to each item.
    let mut dropped_by: BTreeMap<ItemKey, Vec<(u16, f32)>> = BTreeMap::new();
    for &id in cx.monster_pages.keys() {
        let Some(npc) = rose_data::NpcId::new(id).and_then(|n| game.npcs.get_npc(n)) else { continue };
        if npc.drop_table_index == 0 {
            continue;
        }
        for (item, share) in cx.drop_row(npc.drop_table_index as usize) {
            dropped_by.entry(ikey(item)).or_default().push((id, share));
        }
    }
    let mut zone_drops: BTreeMap<ItemKey, Vec<u16>> = BTreeMap::new();
    for &zone in cx.zone_pages.keys() {
        for (item, _) in cx.drop_row(zone as usize) {
            zone_drops.entry(ikey(item)).or_default().push(zone);
        }
    }
    let mut sold_by: BTreeMap<ItemKey, BTreeSet<u16>> = BTreeMap::new();
    for &id in cx.npc_pages.keys() {
        let Some(npc) = rose_data::NpcId::new(id).and_then(|n| game.npcs.get_npc(n)) else { continue };
        for tab in npc.store_tabs.iter().flatten() {
            if let Some(tab) = game.npcs.get_store_tab(*tab) {
                for item in tab.items.values() {
                    sold_by.entry(ikey(*item)).or_default().insert(id);
                }
            }
        }
    }
    let mut quest_rewards: BTreeMap<ItemKey, BTreeSet<usize>> = BTreeMap::new();
    let mut quest_needs: BTreeMap<ItemKey, BTreeSet<usize>> = BTreeMap::new();
    for (name, trigger) in &game.quests.triggers {
        let Some(quests) = cx.trigger_quests.get(name) else { continue };
        for r in &trigger.rewards {
            if let QsdReward::AddItem { item, .. } | QsdReward::CalculatedItem { item, .. } = r {
                if let Some(i) = decode_item_base1000(item.to_sn()) {
                    quest_rewards.entry(ikey(i)).or_default().extend(quests.iter().copied());
                }
            }
        }
        for c in &trigger.conditions {
            if let QsdCondition::QuestItem { item: Some(item), .. } = c {
                if let Some(i) = decode_item_base1000(item.to_sn()) {
                    quest_needs.entry(ikey(i)).or_default().extend(quests.iter().copied());
                }
            }
        }
    }
    let mut used_in: BTreeMap<ItemKey, BTreeSet<ItemKey>> = BTreeMap::new();
    for (item_type, _, _) in ITEM_TYPES {
        for item in game.items.iter_items(item_type) {
            let Some(base) = game.items.get_base_item(item) else { continue };
            if base.craft_material == 0 || !cx.item_pages.contains_key(&ikey(item)) {
                continue;
            }
            if let Some(Some(recipe)) = game.craft_recipes.get(base.craft_material as usize) {
                for m in recipe.materials.iter().flatten() {
                    if let Some(mi) = m.item {
                        used_in.entry(ikey(mi)).or_default().insert(ikey(item));
                    }
                }
            }
        }
    }

    let mut items = Page::new("items", "list");
    items.set("title", "Items");
    items.line("# Items");
    pages.add(items);

    for (item_type, folder, title) in ITEM_TYPES {
        let mut list = Page::new(format!("items/{folder}"), "list");
        list.set("title", title);
        let columns: &[&str] = match folder {
            "weapon" => &["class", "level", "attack", "attack_speed", "range", "price"],
            "subweapon" | "head" | "body" | "hands" | "face" => &["class", "level", "defence", "resistance", "price"],
            "feet" | "back" => &["class", "level", "defence", "move_speed", "price"],
            "jewellery" => &["class", "level", "bonus", "price"],
            "consumable" => &["class", "effect", "price"],
            "gem" => &["bonus", "price"],
            "vehicle" => &["class", "part", "price"],
            _ => &["class", "price"],
        };
        list.set("columns", strs(columns.iter().map(|s| s.to_string())));
        list.line(format!("# {title}"));
        pages.add(list);

        for item in game.items.iter_items(item_type) {
            let Some((path, name)) = cx.item_pages.get(&ikey(item)) else { continue };
            let Some(base) = game.items.get_base_item(item) else { continue };
            let mut p = Page::new(path.clone(), "item");
            p.set("id", format!("{folder}/{}", item.item_number));
            p.set("name", name.as_str());
            p.set("status", "in-game");
            p.set("icon", format!("item/{}", base.icon_index));
            p.set("class", class_name(cx, base.class));
            let level = base.equip_ability_requirement.iter().find(|(a, _)| *a == rose_data::AbilityType::Level);
            if let Some((_, l)) = level {
                p.set("level", *l as i64);
            }

            match game.items.get_item(item) {
                Some(ItemData::Weapon(w)) => {
                    p.set("attack", w.attack_power as i64);
                    p.set("attack_speed", w.attack_speed as i64);
                    p.set("range", round1(w.attack_range as f32 / 100.0));
                    p.set("damage", if w.is_magic_damage { "magic" } else { "physical" });
                    p.set("two_handed", base.class.is_two_handed_weapon());
                }
                Some(ItemData::Back(b)) => {
                    p.set_some("move_speed", b.move_speed as i64);
                }
                Some(ItemData::Feet(f)) => {
                    p.set_some("move_speed", f.move_speed as i64);
                }
                Some(ItemData::Gem(g)) => {
                    p.set_some("bonus", bonus_text(cx, &g.gem_add_ability));
                }
                Some(ItemData::Consumable(c)) => {
                    let mut effect = Vec::new();
                    if let Some((a, v)) = c.add_ability {
                        effect.push(format!("{} +{v}", cx.ability_name(a)));
                    }
                    if let Some((s, v)) = c.apply_status_effect {
                        effect.push(format!("{} ({v})", cx.status_effect_name(s)));
                    }
                    if let Some(s) = c.learn_skill_id {
                        effect.push(format!("teaches {}", cx.skill_link(s.get())));
                    }
                    if let Some(s) = c.use_skill_id {
                        effect.push(format!("uses {}", cx.skill_link(s.get())));
                    }
                    if c.add_fuel != 0 {
                        effect.push(format!("fuel +{}", c.add_fuel));
                    }
                    p.set_some("effect", effect.join(", "));
                    if let Some((a, v)) = c.ability_requirement {
                        p.set("needs", format!("{} {v}", cx.ability_name(a)));
                    }
                    p.set_some("cooldown_seconds", round1(c.cooldown_duration.as_secs_f32()));
                }
                Some(ItemData::Vehicle(v)) => {
                    p.set("vehicle", match v.vehicle_type {
                        VehicleType::Cart => "cart",
                        VehicleType::CastleGear => "castle gear",
                    });
                    p.set("part", match v.vehicle_part {
                        VehiclePartIndex::Body => "body",
                        VehiclePartIndex::Engine => "engine",
                        VehiclePartIndex::Leg => "legs or wheels",
                        VehiclePartIndex::Arms => "arms",
                    });
                    p.set_some("move_speed", v.move_speed as i64);
                    p.set_some("max_fuel", v.max_fuel as i64);
                    p.set_some("fuel_use", v.fuel_use_rate as i64);
                    p.set_some("attack", v.attack_power as i64);
                    p.set_some("attack_speed", v.attack_speed as i64);
                    if v.attack_range != 0 {
                        p.set("range", round1(v.attack_range as f32 / 100.0));
                    }
                    p.set_some("seat", v.has_seat);
                    if let Some((s, l)) = v.equip_skill_requirement {
                        p.set("needs_skill", format!("{} level {l}", cx.skill_link(s.get())));
                    }
                }
                _ => {}
            }
            if item_type.is_equipment_item() {
                p.set_some("defence", base.defence as i64);
                p.set_some("resistance", base.resistance as i64);
                p.set_some("durability", base.durability as i64);
                p.set_some("quality", base.quality as i64);
                if let Some(j) = base.equip_job_class_requirement {
                    p.set("job", cx.job_class_name(j));
                }
                let needs: Vec<String> = base
                    .equip_ability_requirement
                    .iter()
                    .filter(|(a, _)| *a != rose_data::AbilityType::Level)
                    .map(|(a, v)| format!("{} {v}", cx.ability_name(*a)))
                    .collect();
                p.set_some("needs", needs.join(", "));
                p.set_some("bonus", bonus_text(cx, &base.add_ability));
                p.set_some("sockets", base.rare_type == 2 || base.rare_type == 3);
            }
            p.set("price", base.base_price as i64);
            p.set_some("weight", base.weight as i64);
            p.set_some("trade", match base.trade_restriction {
                1 => "can't be traded or dropped",
                2 => "can't be traded",
                _ => "",
            });

            // Crafting: made with a skill, from a recipe.
            if base.craft_skill_type != 0 {
                let skill = game
                    .skills
                    .iter()
                    .filter(|s| s.item_make_number == base.craft_skill_type)
                    .min_by_key(|s| s.level);
                if let Some(s) = skill {
                    p.set("crafted_with", format!("{} level {}", cx.skill_link(s.base_skill_id.unwrap_or(s.id).get()), base.craft_skill_level));
                }
                if let Some(Some(recipe)) = game.craft_recipes.get(base.craft_material as usize) {
                    let mats: Vec<Value> = recipe
                        .materials
                        .iter()
                        .flatten()
                        .map(|m| {
                            let what = match m.item {
                                Some(i) => cx.item_link(i),
                                None => format!(
                                    "any {}",
                                    cx.game.data_decoder.decode_item_class(recipe.raw_material_class as usize)
                                        .map_or_else(|| format!("class {}", recipe.raw_material_class), |c| class_name(cx, c))
                                ),
                            };
                            row([("material", what.into()), ("quantity", (m.quantity as i64).into())])
                        })
                        .collect();
                    p.set("recipe", seq(mats));
                    p.set_some("craft_difficulty", base.craft_difficulty as i64);
                }
            }

            let key = ikey(item);
            if let Some(list) = dropped_by.get(&key) {
                let mut rows: Vec<_> = list.iter().collect();
                rows.sort_by_key(|(m, _)| *m);
                p.set(
                    "dropped_by",
                    seq(rows.iter().map(|(m, share)| {
                        let level = rose_data::NpcId::new(*m).and_then(|n| game.npcs.get_npc(n)).map_or(0, |n| n.level);
                        row([
                            ("monster", cx.npc_link(*m).into()),
                            ("level", (level as i64).into()),
                            ("slots_of_30", round1(*share)),
                        ])
                    })),
                );
            }
            if let Some(zones) = zone_drops.get(&key) {
                let mut z = zones.clone();
                z.sort();
                p.set("dropped_in_zones", strs(z.iter().map(|z| cx.zone_link(*z))));
            }
            if let Some(npcs) = sold_by.get(&key) {
                p.set("sold_by", strs(npcs.iter().map(|n| cx.npc_link(*n))));
            }
            if let Some(q) = quest_rewards.get(&key) {
                p.set("quest_reward", strs(q.iter().map(|q| cx.quest_link(*q))));
            }
            if let Some(q) = quest_needs.get(&key) {
                p.set("needed_for_quest", strs(q.iter().map(|q| cx.quest_link(*q))));
            }
            if let Some(list) = used_in.get(&key) {
                p.set(
                    "used_to_craft",
                    strs(list.iter().filter_map(|k| cx.item_pages.get(k)).map(|(p, n)| link(p, n))),
                );
            }
            p.set(
                "source",
                row([
                    ("data", format!("{} row {}", stb_name(folder), item.item_number).into()),
                    ("code", "module/src/items.rs".into()),
                ]),
            );
            p.line(format!("# {name}"));
            let desc = one_line(base.description);
            if !desc.is_empty() {
                p.line("");
                p.line(desc);
            }
            pages.add(p);
        }
    }
}

fn stb_name(folder: &str) -> &'static str {
    match folder {
        "weapon" => "LIST_WEAPON.STB",
        "subweapon" => "LIST_SUBWPN.STB",
        "head" => "LIST_CAP.STB",
        "body" => "LIST_BODY.STB",
        "hands" => "LIST_ARMS.STB",
        "feet" => "LIST_FOOT.STB",
        "back" => "LIST_BACK.STB",
        "face" => "LIST_FACEITEM.STB",
        "jewellery" => "LIST_JEWEL.STB",
        "consumable" => "LIST_USEITEM.STB",
        "gem" => "LIST_JEMITEM.STB",
        "material" => "LIST_NATURAL.STB",
        "quest" => "LIST_QUESTITEM.STB",
        _ => "LIST_PAT.STB",
    }
}

fn bonus_text(cx: &Ctx, list: &[(rose_data::AbilityType, i32)]) -> String {
    list.iter()
        .filter(|(_, v)| *v != 0)
        .map(|(a, v)| format!("{} {}{v}", cx.ability_name(*a), if *v > 0 { "+" } else { "" }))
        .collect::<Vec<_>>()
        .join(", ")
}
