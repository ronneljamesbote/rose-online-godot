//! Monster pages and town NPC pages (with the NPC's whole dialog).

use std::collections::{BTreeMap, BTreeSet};

use rose_data::NpcId;
use serde_yaml::Value;

use crate::{
    page::{metres, one_line, round1, row, seq, strs, Page, Pages},
    script::{tree, Effect, Line},
    Ctx, Dialog,
};

pub fn write_monsters(cx: &Ctx, pages: &mut Pages) {
    let game = cx.game;
    let mut summoned_by: BTreeMap<u16, BTreeSet<u16>> = BTreeMap::new();
    for s in game.skills.iter() {
        if let Some(n) = s.summon_npc_id {
            summoned_by.entry(n.get()).or_default().insert(cx.skill_base.get(&s.id.get()).copied().unwrap_or(s.id.get()));
        }
    }

    let mut list = Page::new("monsters", "list");
    list.set("title", "Monsters");
    list.set("columns", strs(["level", "hp", "attack", "defence", "xp", "zones"].map(String::from)));
    list.line("# Monsters");
    pages.add(list);

    for (&id, (path, name)) in &cx.monster_pages {
        let Some(npc) = NpcId::new(id).and_then(|n| game.npcs.get_npc(n)) else { continue };
        let mut p = Page::new(path.clone(), "monster");
        p.set("id", id as i64);
        p.set("name", name.as_str());
        p.set("status", "in-game");
        p.set("level", npc.level as i64);
        // The data holds HP per level; a monster's real max HP is level × that.
        p.set("hp", (npc.level as i64) * (npc.health_points as i64));
        p.set("hp_per_level", npc.health_points as i64);
        p.set("attack", npc.attack as i64);
        p.set("hit", npc.hit as i64);
        p.set("defence", npc.defence as i64);
        p.set("resistance", npc.resistance as i64);
        p.set("avoid", npc.avoid as i64);
        p.set("attack_speed", npc.attack_speed as i64);
        p.set("attack_range", round1(npc.attack_range as f32 / 100.0));
        p.set("damage", if npc.is_attack_magic_damage { "magic" } else { "physical" });
        p.set("walk_speed", npc.walk_speed as i64);
        p.set("run_speed", npc.run_speed as i64);
        p.set("xp", npc.reward_xp as i64);
        p.set("drop_item_rate", npc.drop_item_rate as i64);
        p.set("drop_money_rate", npc.drop_money_rate as i64);
        let spawns = cx.spawns.get(&id).map(Vec::as_slice).unwrap_or_default();
        let zones: BTreeSet<u16> = spawns.iter().map(|s| s.zone).collect();
        p.set_some("zones", zones.len() as i64);
        if let Some(by) = summoned_by.get(&id) {
            p.set("summoned_by", strs(by.iter().map(|s| cx.skill_link(*s))));
        }
        if npc.drop_table_index != 0 {
            let drops = cx.drop_row(npc.drop_table_index as usize);
            if !drops.is_empty() {
                p.set(
                    "drops",
                    seq(drops.iter().map(|(item, share)| row([("item", cx.item_link(*item).into()), ("slots_of_30", round1(*share))]))),
                );
            }
        }
        let quests: BTreeSet<usize> = cx
            .trigger_quests
            .get(&npc.death_quest_trigger_name)
            .cloned()
            .unwrap_or_default();
        if !quests.is_empty() {
            p.set("quests", strs(quests.iter().map(|q| cx.quest_link(*q))));
        }
        if !spawns.is_empty() {
            p.set(
                "spawns",
                seq(spawns.iter().map(|s| {
                    row([
                        ("zone", cx.zone_link(s.zone).into()),
                        ("x", metres(s.x)),
                        ("y", metres(s.y)),
                        ("count", (s.count as i64).into()),
                        ("group", if s.tactic { "reinforcements" } else { "basic" }.into()),
                    ])
                })),
            );
        }
        p.set(
            "source",
            row([
                ("data", format!("LIST_NPC.STB row {id}, ITEM_DROP.STB row {}", npc.drop_table_index).into()),
                ("code", strs(["module/src/monster_brain.rs".into(), "crates/rose-game-irose/src/data/drop_table.rs".into()])),
            ]),
        );
        p.line(format!("# {name}"));
        p.line("");
        p.line("Speeds are in centimetres per second, attack range in metres. Spawn positions are in metres.");
        p.line("HP is the monster's real maximum: its level × `hp_per_level`, the value in the game data.");
        p.line("How drops are picked (the zone's table, slots, money) is on [[rules/drops|Drops]];");
        p.line("how the XP value turns into experience is on [[rules/experience|Experience]].");
        pages.add(p);
    }
}

pub fn write_npcs(cx: &Ctx, pages: &mut Pages) {
    let game = cx.game;
    let mut list = Page::new("npcs", "list");
    list.set("title", "NPCs");
    list.set("columns", strs(["zone", "services"].map(String::from)));
    list.line("# NPCs");
    pages.add(list);

    for (&id, (path, name)) in &cx.npc_pages {
        let Some(npc) = NpcId::new(id).and_then(|n| game.npcs.get_npc(n)) else { continue };
        let placements = cx.placements.get(&id).map(Vec::as_slice).unwrap_or_default();
        let mut p = Page::new(path.clone(), "npc");
        p.set("id", id as i64);
        p.set("name", name.as_str());
        p.set("status", "in-game");
        if let Some(first) = placements.first() {
            p.set("zone", cx.zone_link(first.zone));
        }

        let conversations: BTreeSet<&str> = placements.iter().map(|p| p.conversation.as_str()).collect();
        let dialogs: Vec<(&str, &Dialog)> =
            conversations.iter().filter_map(|c| cx.dialogs.get(*c).map(|d| (*c, d))).collect();
        let mut effects = Vec::new();
        for (_, d) in &dialogs {
            for e in d.all_effects() {
                if !effects.contains(&e) {
                    effects.push(e);
                }
            }
        }
        let services: Vec<String> = effects
            .iter()
            .filter_map(|e| match e {
                Effect::Service(s) => Some(service_name(s).to_string()),
                _ => None,
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        p.set_some("services", services.join(", "));
        let mut quests: BTreeSet<usize> = effects
            .iter()
            .filter_map(|e| match e {
                Effect::CheckTrigger(t) | Effect::RunTrigger(t) => cx.trigger_quests.get(t),
                _ => None,
            })
            .flatten()
            .copied()
            .collect();
        quests.extend(effects.iter().filter_map(|e| match e {
            Effect::HasQuest(q, _) | Effect::Carries { quest: q, .. } => Some(*q),
            _ => None,
        }));
        quests.retain(|q| cx.quest_pages.contains_key(q));
        if !quests.is_empty() {
            p.set("quests", strs(quests.iter().map(|q| cx.quest_link(*q))));
        }
        p.set(
            "locations",
            seq(placements.iter().map(|pl| {
                row([("zone", cx.zone_link(pl.zone).into()), ("x", metres(pl.x)), ("y", metres(pl.y))])
            })),
        );
        let mut shop = Vec::new();
        for tab in npc.store_tabs.iter().flatten() {
            let Some(tab) = game.npcs.get_store_tab(*tab) else { continue };
            let mut items: Vec<_> = tab.items.iter().collect();
            items.sort_by_key(|(slot, _)| **slot);
            for (_, item) in items {
                let price = game.items.get_base_item(*item).map_or(0, |b| b.base_price);
                shop.push(row([
                    ("tab", tab.name.into()),
                    ("item", cx.item_link(*item).into()),
                    ("base_price", (price as i64).into()),
                ]));
            }
        }
        if !shop.is_empty() {
            p.set("shop", Value::Sequence(shop));
        }
        p.set(
            "source",
            row([
                (
                    "data",
                    format!(
                        "LIST_NPC.STB row {id}; dialog {}",
                        conversations
                            .iter()
                            .filter_map(|c| game.npcs.get_conversation(&rose_data::NpcConversationId::new(c.to_string())))
                            .map(|c| c.filename.clone())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                    .into(),
                ),
                ("code", strs(["module/src/npcs.rs".into(), "godot/rust/src/conversation.rs".into()])),
            ]),
        );

        p.line(format!("# {name}"));
        for (i, (_, d)) in dialogs.iter().enumerate() {
            p.line("");
            p.line(if dialogs.len() > 1 { format!("## Dialog {}", i + 1) } else { "## Dialog".into() });
            p.line("");
            p.line("The NPC says the **last** message in a list whose condition passes; the lines under");
            p.line("it are your choices. See [[rules/npc-dialogs|NPC dialogs]].");
            p.line("");
            if let Some(open) = d.con.initial_messages.first() {
                let cond = describe_effects(cx, &d.effects(&open.condition_function));
                if !cond.is_empty() {
                    p.line(format!("Talks to you only if {cond}."));
                    p.line("");
                }
            }
            let mut out = String::new();
            render(cx, d, &tree(&d.con, d.script.as_ref()), 0, &mut out);
            p.body.push_str(&out);
        }
        pages.add(p);
    }
}

fn service_name(s: &str) -> &str {
    match s {
        "opens the shop" => "shop",
        "opens storage" => "storage",
        "opens refining" => "refining",
        "opens disassembly" => "disassembly",
        "opens repair" => "repair",
        "saves this town as where you get up after dying" => "save point",
        "appraises items (not in game yet)" => "appraisal",
        "opens item delivery (not in game yet)" => "item delivery",
        "founds a clan (not in game yet)" | "disbands your clan (not in game yet)" => "clans",
        other => other,
    }
}

/// Effects in words, with quest steps linked to their quest.
pub fn describe_effects(cx: &Ctx, effects: &[Effect]) -> String {
    let mut parts = Vec::new();
    for e in effects {
        parts.push(match e {
            Effect::CheckTrigger(t) => format!("quest step `{t}` passes{}", quest_note(cx, t)),
            Effect::RunTrigger(t) => format!("runs quest step `{t}`{}", quest_note(cx, t)),
            Effect::Service(s) => s.to_string(),
            Effect::HasQuest(q, true) => format!("you have {}", cx.quest_link(*q)),
            Effect::HasQuest(q, false) => format!("you don't have {}", cx.quest_link(*q)),
            Effect::Carries { quest, item, op, count } => {
                format!("you carry {op} {count} × {} for {}", cx.item_link_sn(*item), cx.quest_link(*quest))
            }
            Effect::Check(c) => c.clone(),
        });
    }
    parts.join("; ")
}

fn quest_note(cx: &Ctx, trigger: &str) -> String {
    match cx.trigger_quests.get(trigger) {
        Some(q) if !q.is_empty() => {
            format!(" ({})", q.iter().map(|q| cx.quest_link(*q)).collect::<Vec<_>>().join(", "))
        }
        _ => String::new(),
    }
}

fn render(cx: &Ctx, d: &Dialog, lines: &[Line], depth: usize, out: &mut String) {
    let indent = "  ".repeat(depth);
    for line in lines {
        match line {
            Line::Npc { text, condition, children } => {
                let t = one_line(&cx.dialog_text(*text));
                let cond = describe_effects(cx, &d.effects(condition));
                let mut s = format!("{indent}- **NPC:** {}", if t.is_empty() { "…" } else { &t });
                if !cond.is_empty() {
                    s += &format!(" *(if {cond})*");
                }
                out.push_str(&s);
                out.push('\n');
                render(cx, d, children, depth + 1, out);
            }
            Line::Choice { text, condition, action, next } => {
                let t = one_line(&cx.dialog_text(*text));
                let cond = describe_effects(cx, &d.effects(condition));
                let act = describe_effects(cx, &d.effects(action));
                let mut s = format!("{indent}- **You:** {}", if t.is_empty() { "…" } else { &t });
                if !cond.is_empty() {
                    s += &format!(" *(if {cond})*");
                }
                if !act.is_empty() {
                    s += &format!(" → {act}");
                }
                out.push_str(&s);
                out.push('\n');
                render(cx, d, next, depth + 1, out);
            }
            Line::Again => {
                out.push_str(&format!("{indent}- *(back to an earlier part of this dialog)*\n"));
            }
        }
    }
}
