//! GM commands for dev servers: one reducer, `gm`, that takes a chat line such as
//! `/item 10 1 5` and runs it. This file is only compiled with the `dev` feature, so a
//! production server doesn't have the reducer or the gm_state table at all.
//!
//! Every player on a dev server may use them. Commands act on the caller's own character;
//! `@Name` as the last word acts on that player instead. Results come back as one notice
//! with a line per result, each starting with "GM: "; errors as the reducer's error. The list is in wiki/rules/gm-commands.md.

use rose_data::{SkillData, SkillId};
use rose_data_irose::IroseSkillPageType;
use rose_game_common::components::{BasicStatType, BasicStats, HotbarSlot, Money, SkillList};
use rose_game_data::GameData;
use spacetimedb::{Identity, ReducerContext, Table};
use std::cell::RefCell;

use crate::{
    character, combat, entity, game_data::game, items, my_player, now_us, npc, player, position, quests, stamina,
    world, xp_event, zone_info, EntityKind, Player, XpEvent,
};

/// Highest level /level sets (iROSE's level cap).
const MAX_LEVEL: u32 = 250;
const MAX_SPAWN: u32 = 50;
const FIRST_JOBS: [u16; 4] = [111, 211, 311, 411];
const SECOND_JOBS: [u16; 8] = [121, 122, 221, 222, 321, 322, 421, 422];

/// Per character GM state: god mode, walking speed, and the skills GM commands gave for
/// free (/learnall, /skill), which give no skill points back when they are removed.
#[spacetimedb::table(accessor = gm_state)]
pub struct GmState {
    #[primary_key]
    pub identity: Identity,
    pub god: bool,
    pub speed_percent: u32,
    /// First-level ids of the free skills.
    pub free_skills: Vec<u16>,
}

/// name, arguments, what it does. /help prints this; the client's suggestions
/// (godot/gm/gm_commands.gd) keep the same list.
pub const COMMANDS: &[(&str, &str, &str)] = &[
    ("help", "[command]", "lists the commands, or explains one"),
    ("info", "", "zone, position, and the id of what you target"),
    ("item", "type number [amount]", "puts an item in the bag"),
    ("zuly", "amount", "gives money (a negative amount takes it)"),
    ("xp", "amount", "gives experience"),
    ("level", "n", "sets the level with its stat and skill points"),
    ("job", "id", "changes the job: 0 Visitor, 111 Soldier, 121 Knight, 122 Champion, 211 Muse, 221 Mage, 222 Cleric, 311 Hawker, 321 Raider, 322 Scout, 411 Dealer, 421 Bourgeois, 422 Artisan"),
    ("skill", "id", "learns a skill, skipping its requirements"),
    ("learnall", "", "learns every skill of the job line at its highest level"),
    ("resetstats", "", "stats back to their starting values, spent points back"),
    ("resetskills", "", "removes learned skills, spent skill points back"),
    ("resetlevel", "", "back to a level 1 Visitor"),
    ("stat", "name value", "sets STR, DEX, INT, CON, CHA or SEN"),
    ("tp", "zone [x y]", "warps to a zone, at its start or at a spot (metres, as /info shows)"),
    ("zones", "", "lists zone ids and names"),
    ("goto", "npc-id", "stands you next to an NPC"),
    ("heal", "", "full HP and MP"),
    ("god", "", "turns taking no damage on or off"),
    ("speed", "percent", "changes walking speed (100 is normal)"),
    ("spawn", "monster-id [count]", "spawns monsters around you (they don't respawn)"),
    ("kill", "", "kills your target"),
    ("respawn", "", "clears all monsters so spawn points refill"),
    ("quest", "give id [item amount] | clear | trigger name", "quest helpers"),
    ("jobvar", "index value", "sets a job variable (job change quests)"),
    ("stamina", "value", "sets stamina"),
    ("durability", "slot life", "sets an equipped item's life (slot 0 face ... 6 weapon ... 10 earring)"),
    ("grade", "slot grade [gem]", "sets the grade of worn items: slot is a number as /durability or head, body, back, hands, feet, weapon, offhand, face, armor (head to feet, back, off-hand) or all; grade 0 to 9, or +N / -N to raise or lower it; a gem number sets that gem in a single item (0 takes it out)"),
];

/// Run a GM command. `target` is the entity the caller has targeted (0 for none), for
/// /info and /kill.
#[spacetimedb::reducer]
pub fn gm(ctx: &ReducerContext, line: String, target: u64) -> Result<(), String> {
    let (caller, _) = my_player(ctx)?;
    let game = game(ctx)?;
    let mut words: Vec<&str> = line.trim().trim_start_matches('/').split_whitespace().collect();
    let Some(command) = words.first().map(|w| w.to_ascii_lowercase()) else {
        return Err("type /help for the GM commands".into());
    };
    words.remove(0);
    // "@Name" as the last word acts on another player.
    let mut who = caller.clone();
    if let Some(name) = words.last().and_then(|w| w.strip_prefix('@')) {
        who = ctx
            .db
            .player()
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(name))
            .ok_or_else(|| format!("there is no player called {name}"))?;
        words.pop();
    }
    let run = Gm { ctx, game: &game, command: &command, args: &words, target, lines: RefCell::new(Vec::new()) };
    let result = run.command(&command, who);
    log::info!("gm {}: {line} -> {result:?}", caller.name);
    // One notice for all the lines, so they arrive in order.
    let lines = run.lines.take();
    if !lines.is_empty() {
        items::notify(ctx, caller.identity, lines.join("\n"));
    }
    result
}

struct Gm<'a> {
    ctx: &'a ReducerContext,
    game: &'a GameData,
    command: &'a str,
    args: &'a [&'a str],
    target: u64,
    /// The answer, sent as one notice when the command is done.
    lines: RefCell<Vec<String>>,
}

impl Gm<'_> {
    fn say(&self, text: impl AsRef<str>) {
        self.lines.borrow_mut().push(format!("GM: {}", text.as_ref()));
    }

    fn arg<T: std::str::FromStr>(&self, i: usize, what: &str) -> Result<T, String> {
        let word = self.args.get(i).ok_or_else(|| format!("missing {what} ({})", self.usage()))?;
        word.parse().map_err(|_| format!("{what} must be a number, not {word}"))
    }

    fn opt_arg<T: std::str::FromStr>(&self, i: usize, what: &str, default: T) -> Result<T, String> {
        if self.args.len() > i { self.arg(i, what) } else { Ok(default) }
    }

    fn usage(&self) -> String {
        COMMANDS.iter().find(|c| c.0 == self.command).map_or("see /help".into(), |c| format!("/{} {}", c.0, c.1))
    }

    fn save(&self, p: &Player) {
        self.ctx.db.player().identity().update(p.clone());
    }

    /// Save a player and recalculate their HP, MP and ability values.
    fn save_refresh(&self, p: &Player, full_heal: bool) {
        self.save(p);
        character::refresh_player(self.ctx, self.game, p, full_heal);
    }

    fn command(&self, command: &str, mut p: Player) -> Result<(), String> {
        let (ctx, game) = (self.ctx, self.game);
        let name = p.name.clone();
        match command {
            "help" => self.help(),
            "info" => self.info(&p),
            "item" => {
                let (kind, number) = (self.arg::<u8>(0, "item type")?, self.arg::<u16>(1, "item number")?);
                let amount = self.opt_arg(2, "amount", 1u32)?;
                let item = items::give_item_to(game, &mut p, kind, number, amount)?;
                self.save(&p);
                self.say(format!("gave {name} {item} (item {kind}:{number})"));
                Ok(())
            }
            "zuly" => {
                let amount: i64 = self.arg(0, "amount")?;
                let mut inventory = p.inventory();
                inventory.money = Money((inventory.money.0.saturating_add(amount)).max(0));
                let total = inventory.money.0;
                p.set_inventory(&inventory);
                self.save(&p);
                self.say(format!("{name} has {total} Zuly"));
                Ok(())
            }
            "xp" => {
                let xp: u64 = self.arg(0, "amount")?;
                character::reward_xp(ctx, game, p.identity, xp);
                ctx.db.xp_event().insert(XpEvent { identity: p.identity, xp, level: 0 });
                let level = ctx.db.player().identity().find(p.identity).map_or(p.level, |q| q.level);
                self.say(format!("gave {name} {xp} XP (level {level})"));
                Ok(())
            }
            "level" => self.set_level(p, self.arg(0, "level")?),
            "job" => self.set_job(p, self.arg(0, "job id")?),
            "skill" => {
                let id: u16 = self.arg(0, "skill id")?;
                let data = SkillId::new(id).and_then(|id| game.skills.get_skill(id)).ok_or(format!("no skill {id}"))?;
                let mut list = p.skill_list();
                list.add_skill(data).ok_or("no room for more skills")?;
                p.set_skill_list(&list);
                self.with_free_skills(&p, |free| {
                    if !free.contains(&base_of(data).get()) {
                        free.push(base_of(data).get())
                    }
                });
                self.save_refresh(&p, false);
                self.say(format!("{name} learned {} level {}", data.name, data.level));
                Ok(())
            }
            "learnall" => self.learn_all(p),
            "resetstats" => {
                let refund = reset_stats(game, &mut p);
                self.after_reset(&mut p);
                self.say(format!("{name}'s stats are back to their starting values, {refund} stat points back ({} unspent)", p.stat_points));
                Ok(())
            }
            "resetskills" => {
                let (removed, refund) = self.with_free_skills(&p.clone(), |free| reset_skills(game, &mut p, free));
                self.after_reset(&mut p);
                self.say(format!("removed {removed} of {name}'s skills, {refund} skill points back ({} unspent)", p.skill_points));
                Ok(())
            }
            "resetlevel" => {
                self.with_free_skills(&p.clone(), |free| reset_level(ctx, game, &mut p, free));
                self.after_reset(&mut p);
                self.say(format!("{name} is a level 1 Visitor again"));
                Ok(())
            }
            "stat" => {
                let stat = match self.args.first().map(|s| s.to_ascii_uppercase()).as_deref() {
                    Some("STR") => BasicStatType::Strength,
                    Some("DEX") => BasicStatType::Dexterity,
                    Some("INT") => BasicStatType::Intelligence,
                    Some("CON") => BasicStatType::Concentration,
                    Some("CHA") => BasicStatType::Charm,
                    Some("SEN") => BasicStatType::Sense,
                    _ => return Err("the stat is STR, DEX, INT, CON, CHA or SEN".into()),
                };
                let value: i32 = self.arg(1, "value")?;
                let mut basic = p.basic_stats();
                basic.set(stat, value.max(1));
                p.set_basic_stats(&basic);
                self.save_refresh(&p, false);
                self.say(format!("{name}'s {} is {}", self.args[0].to_ascii_uppercase(), value.max(1)));
                Ok(())
            }
            "tp" => {
                let zone_id: u16 = self.arg(0, "zone id")?;
                let zone = ctx.db.zone_info().zone_id().find(zone_id).ok_or(format!("no zone {zone_id} (try /zones)"))?;
                let at = if self.args.len() >= 3 {
                    let (x, y): (f32, f32) = (self.arg(1, "x")?, self.arg(2, "y")?);
                    let at = (x * 100.0, y * 100.0);
                    if at.0 < zone.min_x || at.0 > zone.max_x || at.1 < zone.min_y || at.1 > zone.max_y {
                        return Err(format!(
                            "that spot is outside {} (x {:.0} to {:.0}, y {:.0} to {:.0})",
                            zone.name,
                            zone.min_x / 100.0,
                            zone.max_x / 100.0,
                            zone.min_y / 100.0,
                            zone.max_y / 100.0
                        ));
                    }
                    at
                } else {
                    (zone.start_x, zone.start_y)
                };
                world::teleport(ctx, &mut p, zone_id, at);
                self.say(format!("{name} went to {} ({zone_id}) at {:.0} {:.0}", zone.name, at.0 / 100.0, at.1 / 100.0));
                Ok(())
            }
            "zones" => {
                let mut zones: Vec<(u16, String)> = ctx.db.zone_info().iter().map(|z| (z.zone_id, z.name)).collect();
                zones.sort();
                for chunk in zones.chunks(6) {
                    let line: Vec<String> = chunk.iter().map(|(id, name)| format!("{id} {name}")).collect();
                    self.say(line.join(" · "));
                }
                Ok(())
            }
            "goto" => {
                let npc_id: u16 = self.arg(0, "NPC id")?;
                let npc = ctx.db.npc().iter().find(|n| n.npc_id == npc_id).ok_or(format!("no NPC {npc_id} in the world"))?;
                let at = position(ctx, npc.entity_id, now_us(ctx)).ok_or("the NPC isn't placed")?;
                let npc_name = ctx.db.entity().entity_id().find(npc.entity_id).map_or(String::new(), |e| e.name);
                world::teleport(ctx, &mut p, npc.zone_id, (at.0 + 200.0, at.1));
                self.say(format!("{name} is next to {npc_name} ({npc_id})"));
                Ok(())
            }
            "heal" => {
                let id = p.entity_id.ok_or(format!("{name} isn't online"))?;
                let mut c = ctx.db.combat().entity_id().find(id).ok_or("no combat row")?;
                if c.dead_until_us.is_some() || c.hp <= 0 {
                    return Err(format!("{name} has fallen; get up first"));
                }
                c.hp = c.max_hp;
                c.mp = c.max_mp;
                ctx.db.combat().entity_id().update(c);
                self.say(format!("{name} has full HP and MP"));
                Ok(())
            }
            "god" => {
                let mut state = self.state(p.identity);
                state.god = !state.god;
                let on = state.god;
                self.set_state(state);
                self.say(format!("god mode {} for {name}", if on { "on" } else { "off" }));
                Ok(())
            }
            "speed" => {
                let percent: u32 = self.arg(0, "percent")?;
                if !(10..=1000).contains(&percent) {
                    return Err("speed is 10 to 1000 percent".into());
                }
                let mut state = self.state(p.identity);
                state.speed_percent = percent;
                self.set_state(state);
                character::refresh_player(ctx, game, &p, false);
                self.say(format!("{name} walks at {percent}% speed"));
                Ok(())
            }
            "spawn" => self.spawn(&p),
            "kill" => {
                let id = p.entity_id.ok_or(format!("{name} isn't online"))?;
                if self.target == 0 {
                    return Err("target something first".into());
                }
                let target = ctx.db.entity().entity_id().find(self.target).ok_or("your target is gone")?;
                if target.kind == EntityKind::Npc {
                    return Err("town NPCs can't be killed".into());
                }
                let c = ctx.db.combat().entity_id().find(self.target).ok_or("that can't be killed")?;
                if c.hp <= 0 || c.dead_until_us.is_some() {
                    return Err("your target is already dead".into());
                }
                crate::deal_damage(ctx, game, id, self.target, c.hp, false, now_us(ctx));
                self.say(format!("killed {}", target.name));
                Ok(())
            }
            "respawn" => {
                crate::clear_monsters(ctx);
                self.say("cleared every monster; the spawn points refill");
                Ok(())
            }
            "quest" => match self.args.first().copied() {
                Some("give") => {
                    let id: u32 = self.arg(1, "quest id")?;
                    let (item, amount) = (self.opt_arg(2, "item", 0u32)?, self.opt_arg(3, "amount", 0u32)?);
                    let quest_name = game.quests.get_quest_data(id as usize).map(|q| q.name.to_string()).ok_or(format!("no quest {id}"))?;
                    quests::give_quest_to(ctx, game, p, id, item, amount)?;
                    self.say(format!("gave {name} {quest_name} ({id})"));
                    Ok(())
                }
                Some("clear") => {
                    let mut state = p.quest_state();
                    for quest in state.active_quests.iter_mut() {
                        *quest = None;
                    }
                    p.set_quest_state(&state);
                    self.save(&p);
                    self.say(format!("cleared {name}'s quests"));
                    Ok(())
                }
                Some("trigger") => {
                    let trigger = self.args.get(1).ok_or("missing the trigger name")?;
                    let ok = quests::run_trigger(ctx, game, p.identity, trigger);
                    self.say(format!("trigger {trigger} for {name}: {}", if ok { "ran" } else { "its conditions failed" }));
                    Ok(())
                }
                _ => Err("/quest give id [item amount], /quest clear or /quest trigger name".into()),
            },
            "jobvar" => {
                let (index, value): (usize, u16) = (self.arg(0, "index")?, self.arg(1, "value")?);
                let mut state = p.quest_state();
                *state.job_variables.get_mut(index).ok_or("job variables are 0 to 2")? = value;
                p.set_quest_state(&state);
                self.save(&p);
                self.say(format!("{name}'s job variable {index} is {value}"));
                Ok(())
            }
            "stamina" => {
                let value: u32 = self.arg(0, "value")?;
                stamina::add(ctx, p.identity, value as i32 - stamina::get(ctx, p.identity) as i32);
                self.say(format!("{name}'s stamina is {}", stamina::get(ctx, p.identity)));
                Ok(())
            }
            "durability" => {
                let (slot, life): (u8, u16) = (self.arg(0, "slot")?, self.arg(1, "life")?);
                crate::durability::set_item_life(&mut p, 0, slot, 0, life)?;
                self.save_refresh(&p, false);
                self.say(format!("set the life of {name}'s item in slot {slot} to {life}"));
                Ok(())
            }
            "grade" => {
                let slots = grade_slots(self.args.first().copied().unwrap_or_default())
                    .ok_or_else(|| format!("missing or unknown slot ({})", self.usage()))?;
                let change = self.args.get(1).copied().ok_or_else(|| format!("missing grade ({})", self.usage()))?;
                let relative = change.starts_with('+') || change.starts_with('-');
                let amount: i32 = change.parse().map_err(|_| format!("the grade must be a number like 5, +1 or -2, not {change}"))?;
                if !relative && !(0..=9).contains(&amount) {
                    return Err("the grade is 0 to 9 (or +N / -N to change it)".into());
                }
                let gem: Option<u16> = self.args.get(2).map(|_| self.arg(2, "gem")).transpose()?;
                if let Some(gem) = gem {
                    if slots.len() > 1 {
                        return Err("a gem goes in one item: name one slot".into());
                    }
                    if gem != 0 && game.items.get_gem_item(gem as usize).is_none() {
                        return Err(format!("{gem} is not a gem (gems are 301 and up)"));
                    }
                }
                let mut done = Vec::new();
                for &(slot, slot_name) in &slots {
                    let result = crate::durability::with_equipped_item(&mut p, slot, |item| {
                        let grade = if relative { (item.grade as i32 + amount).clamp(0, 9) } else { amount };
                        item.grade = grade as u8;
                        if let Some(gem) = gem {
                            item.has_socket |= gem != 0;
                            item.gem = gem;
                        }
                        Ok(grade)
                    });
                    match result {
                        Ok(grade) => done.push(format!("{slot_name} +{grade}")),
                        Err(e) if slots.len() == 1 => return Err(format!("{slot_name}: {e}")),
                        Err(_) => {} // an empty slot in a group
                    }
                }
                if done.is_empty() {
                    return Err("nothing worn in those slots".into());
                }
                self.save_refresh(&p, false);
                let gem_text = gem.map_or(String::new(), |gem| if gem == 0 { ", gem taken out".into() } else { format!(", gem {gem}") });
                self.say(format!("{name}: {}{gem_text}", done.join(", ")));
                Ok(())
            }
            other => Err(format!("no command /{other} (type /help)")),
        }
    }

    fn help(&self) -> Result<(), String> {
        if let Some(name) = self.args.first() {
            let name = name.trim_start_matches('/').to_ascii_lowercase();
            let (command, args, text) =
                COMMANDS.iter().find(|c| c.0 == name).ok_or(format!("no command /{name} (type /help)"))?;
            self.say(format!("/{command} {args}: {text}"));
            return Ok(());
        }
        self.say("commands (/help name explains one; add @Name to act on another player):");
        let names: Vec<String> = COMMANDS.iter().map(|c| format!("/{}", c.0)).collect();
        for chunk in names.chunks(9) {
            self.say(chunk.join(" "));
        }
        Ok(())
    }

    fn info(&self, p: &Player) -> Result<(), String> {
        let ctx = self.ctx;
        let id = p.entity_id.ok_or(format!("{} isn't online", p.name))?;
        let t = now_us(ctx);
        let (x, y) = position(ctx, id, t).unwrap_or((p.last_x, p.last_y));
        let zone = ctx.db.zone_info().zone_id().find(p.zone_id).map_or(String::new(), |z| z.name);
        self.say(format!(
            "{}: zone {} ({}) at {:.1} {:.1}, level {} job {}, entity {id}",
            p.name,
            zone,
            p.zone_id,
            x / 100.0,
            y / 100.0,
            p.level,
            p.job
        ));
        if self.target != 0 {
            match ctx.db.entity().entity_id().find(self.target) {
                Some(e) => {
                    let what = match e.kind {
                        EntityKind::Monster => format!("monster {}", e.npc_id),
                        EntityKind::Npc => format!("NPC {}", e.npc_id),
                        EntityKind::Player => "player".into(),
                    };
                    let at = position(ctx, e.entity_id, t).map_or(String::new(), |(x, y)| format!(" at {:.1} {:.1}", x / 100.0, y / 100.0));
                    self.say(format!("target: {} ({what}, entity {}){at}", e.name, e.entity_id));
                }
                None => self.say("target: gone"),
            }
        }
        Ok(())
    }

    fn state(&self, identity: Identity) -> GmState {
        self.ctx.db.gm_state().identity().find(identity).unwrap_or(GmState { identity, god: false, speed_percent: 100, free_skills: Vec::new() })
    }

    /// Run `f` with the character's list of free skills, and save the list after.
    fn with_free_skills<R>(&self, p: &Player, f: impl FnOnce(&mut Vec<u16>) -> R) -> R {
        let mut state = self.state(p.identity);
        let result = f(&mut state.free_skills);
        self.set_state(state);
        result
    }

    fn set_state(&self, state: GmState) {
        if self.ctx.db.gm_state().identity().find(state.identity).is_some() {
            self.ctx.db.gm_state().identity().update(state);
        } else {
            self.ctx.db.gm_state().insert(state);
        }
    }

    /// Clear hotbar slots of removed skills, take off gear that can't be worn now, save
    /// and refresh with full HP and MP.
    fn after_reset(&self, p: &mut Player) {
        clear_removed_skills(p);
        self.save_refresh(p, true);
        for problem in unequip_unwearable(self.ctx, self.game, p) {
            self.say(problem);
        }
    }

    fn set_level(&self, mut p: Player, level: u32) -> Result<(), String> {
        let (ctx, game) = (self.ctx, self.game);
        if !(1..=MAX_LEVEL).contains(&level) {
            return Err(format!("the level is 1 to {MAX_LEVEL}"));
        }
        let before = p.level;
        let calc = &game.ability_value_calculator;
        if level > before {
            for l in (before + 1)..=level {
                p.stat_points += calc.calculate_levelup_reward_stat_points(l);
                p.skill_points += calc.calculate_levelup_reward_skill_points(l);
            }
        } else if level < before {
            // Lowering takes back the points those levels gave, so spent ones come back first.
            reset_stats(game, &mut p);
            self.with_free_skills(&p.clone(), |free| reset_skills(game, &mut p, free));
            for l in (level + 1)..=before {
                p.stat_points = p.stat_points.saturating_sub(calc.calculate_levelup_reward_stat_points(l));
                p.skill_points = p.skill_points.saturating_sub(calc.calculate_levelup_reward_skill_points(l));
            }
        }
        p.level = level;
        p.xp = 0;
        crate::death::clear_debt(ctx, p.identity);
        self.after_reset(&mut p);
        for l in (before + 1)..=level {
            quests::run_trigger(ctx, game, p.identity, &format!("levelup_{l}"));
        }
        let p = ctx.db.player().identity().find(p.identity).unwrap_or(p);
        self.say(format!("{} is level {level} ({} stat points, {} skill points unspent)", p.name, p.stat_points, p.skill_points));
        Ok(())
    }

    /// /job id with the job rules: a second job sets its first job's state with it, a first
    /// job removes the second job's, 0 is a Visitor. Skills the new job can't use go, with
    /// their points back.
    fn set_job(&self, mut p: Player, job: u16) -> Result<(), String> {
        if job != 0 && !FIRST_JOBS.contains(&job) && !SECOND_JOBS.contains(&job) {
            return Err(format!(
                "no job {job}; jobs are 0 Visitor, {} (first) and {} (second)",
                FIRST_JOBS.map(|j| j.to_string()).join(" "),
                SECOND_JOBS.map(|j| j.to_string()).join(" ")
            ));
        }
        p.job = job;
        let mut state = p.quest_state();
        if job == 0 {
            state.job_variables = [0; 3];
        } else {
            state.job_variables[0] = 1;
            state.job_variables[1] = if SECOND_JOBS.contains(&job) { 2 } else { 0 };
        }
        p.set_quest_state(&state);
        let (removed, refund) =
            self.with_free_skills(&p.clone(), |free| drop_skills(self.game, &mut p, free, |skill| usable_by(self.game, skill, job)));
        self.after_reset(&mut p);
        let mut text = format!("{} is now {}", p.name, job_name(job));
        if removed > 0 {
            text += &format!(", removed {removed} skills the job can't use ({refund} skill points back)");
        }
        self.say(text);
        Ok(())
    }

    fn learn_all(&self, mut p: Player) -> Result<(), String> {
        let game = self.game;
        if p.job == 0 {
            return Err("Visitors have no job skills; pick a job with /job first".into());
        }
        let clan = IroseSkillPageType::Clan as usize;
        // The highest level of every skill of the job line.
        let mut best: Vec<&SkillData> = Vec::new();
        for skill in game.skills.iter() {
            let line = job_class_of(game, skill).is_some_and(|class| !class.jobs.is_empty());
            if !line || skill.page == clan || !usable_by(game, skill, p.job) {
                continue;
            }
            let base = base_of(skill);
            match best.iter_mut().find(|b| base_of(b) == base) {
                Some(b) if b.level < skill.level => *b = skill,
                Some(_) => {}
                None => best.push(skill),
            }
        }
        // Prerequisite order: skills that need others come after them.
        let depth = |skill: &SkillData| prerequisite_depth(game, skill, 0);
        best.sort_by_key(|s| (depth(s), s.id.get()));
        let mut list = p.skill_list();
        let (mut learned, mut raised, mut full) = (0, 0, 0);
        let mut given = Vec::new();
        for skill in best {
            match list.find_skill_level(&game.skills, base_of(skill)) {
                Some((_, _, level)) if level >= skill.level => {}
                Some((slot, _, _)) => {
                    *list.get_slot_mut(slot).unwrap() = Some(skill.id);
                    raised += 1;
                    given.push(base_of(skill).get());
                }
                None => {
                    if list.add_skill(skill).is_some() {
                        learned += 1;
                        given.push(base_of(skill).get());
                    } else {
                        full += 1;
                    }
                }
            }
        }
        self.with_free_skills(&p, |free| free.extend(given.iter().filter(|b| !free.contains(b)).copied().collect::<Vec<_>>()));
        p.set_skill_list(&list);
        self.save_refresh(&p, false);
        let mut text = format!("{} learned {learned} skills and raised {raised} to their highest level", p.name);
        if full > 0 {
            text += &format!(" ({full} didn't fit in the skill window)");
        }
        self.say(text);
        Ok(())
    }

    fn spawn(&self, p: &Player) -> Result<(), String> {
        let ctx = self.ctx;
        let npc_id: u16 = self.arg(0, "monster id")?;
        let count = self.opt_arg(1, "count", 1u32)?.clamp(1, MAX_SPAWN);
        let npc = rose_data::NpcId::new(npc_id).and_then(|id| self.game.npcs.get_npc(id)).ok_or(format!("no monster {npc_id}"))?;
        let id = p.entity_id.ok_or(format!("{} isn't online", p.name))?;
        let (x, y) = position(ctx, id, now_us(ctx)).unwrap_or((p.last_x, p.last_y));
        // A spawn point that isn't in the table: the monsters never come back.
        let point = world::MonsterSpawn {
            spawn_id: 0,
            zone_id: p.zone_id,
            x,
            y,
            range: 300.0 + count as f32 * 20.0,
            interval_secs: 0,
            limit_count: 0,
            tactic_points: 0,
            basic: Vec::new(),
            tactic: Vec::new(),
            current_tactics_value: 0,
            next_check_at_us: 0,
        };
        for _ in 0..count {
            world::spawn_monster(ctx, self.game, &point, npc_id);
        }
        self.say(format!("spawned {count} × {} ({npc_id}) around {}", npc.name, p.name));
        Ok(())
    }
}

/// God mode: the character takes no damage.
pub fn is_god(ctx: &ReducerContext, entity_id: u64) -> bool {
    ctx.db
        .gm_state()
        .iter()
        .any(|g| g.god && ctx.db.player().identity().find(g.identity).is_some_and(|p| p.entity_id == Some(entity_id)))
}

/// Walking speed set with /speed, as a factor.
pub fn speed_factor(ctx: &ReducerContext, identity: Identity) -> f32 {
    ctx.db.gm_state().identity().find(identity).map_or(1.0, |g| g.speed_percent as f32 / 100.0)
}

fn base_of(skill: &SkillData) -> SkillId {
    skill.base_skill_id.unwrap_or(skill.id)
}

fn prerequisite_depth(game: &GameData, skill: &SkillData, guard: u32) -> u32 {
    if guard > 8 {
        return guard;
    }
    skill
        .required_skills
        .iter()
        .filter_map(|(id, _)| game.skills.get_skill(*id))
        .map(|required| 1 + prerequisite_depth(game, required, guard + 1))
        .max()
        .unwrap_or(0)
}

/// The first job of a second job (121 Knight: 111 Soldier).
fn first_job_of(job: u16) -> u16 {
    job / 100 * 100 + 11
}

/// A skill's job class. Only a skill's first level names it in the data, so the higher
/// levels take their first level's.
fn job_class_of<'a>(game: &'a GameData, skill: &SkillData) -> Option<&'a rose_data::JobClassData> {
    let first = game.skills.get_skill(base_of(skill)).unwrap_or(skill);
    skill.required_job_class.or(first.required_job_class).and_then(|id| game.job_class.get(id))
}

/// Whether a job may have a skill: it has no job class, or the class takes the job or,
/// for a second job, its first job.
fn usable_by(game: &GameData, skill: &SkillData, job: u16) -> bool {
    let Some(class) = job_class_of(game, skill) else { return true };
    if class.jobs.is_empty() {
        return true;
    }
    let has = |j: u16| class.jobs.iter().any(|c| c.get() == j);
    has(job) || (SECOND_JOBS.contains(&job) && has(first_job_of(job)))
}

fn job_name(job: u16) -> String {
    let name = match job {
        0 => "Visitor",
        111 => "Soldier",
        121 => "Knight",
        122 => "Champion",
        211 => "Muse",
        221 => "Mage",
        222 => "Cleric",
        311 => "Hawker",
        321 => "Raider",
        322 => "Scout",
        411 => "Dealer",
        421 => "Bourgeois",
        422 => "Artisan",
        _ => return format!("job {job}"),
    };
    format!("{name} ({job})")
}

/// Skill points spent on a skill at its current level: every level's cost up to it.
fn skill_points_spent(game: &GameData, skill: &SkillData) -> u32 {
    (1..=skill.level)
        .filter_map(|level| SkillId::new(skill.id.get() + level as u16 - skill.level as u16))
        .filter_map(|id| game.skills.get_skill(id))
        .filter(|s| base_of(s) == base_of(skill))
        .map(|s| s.learn_point_cost)
        .sum()
}

/// Remove the skills `keep` says no to and give their skill points back.
/// Returns how many went and the points given back.
fn drop_skills(game: &GameData, p: &mut Player, free: &mut Vec<u16>, keep: impl Fn(&SkillData) -> bool) -> (u32, u32) {
    let mut list: SkillList = p.skill_list();
    let (mut removed, mut refund) = (0, 0);
    for page in list.pages.iter_mut() {
        for slot in page.skills.iter_mut() {
            let Some(skill) = slot.and_then(|id| game.skills.get_skill(id)) else { continue };
            if !keep(skill) {
                let base = base_of(skill).get();
                if free.contains(&base) {
                    free.retain(|b| *b != base);
                } else {
                    refund += skill_points_spent(game, skill);
                }
                removed += 1;
                *slot = None;
            }
        }
    }
    p.skill_points += refund;
    p.set_skill_list(&list);
    (removed, refund)
}

/// Every skill but the starting ones goes, with its skill points back.
fn reset_skills(game: &GameData, p: &mut Player, free: &mut Vec<u16>) -> (u32, u32) {
    let starting: Vec<SkillId> =
        game.character_creator.skills.iter().filter_map(|id| game.skills.get_skill(*id)).map(base_of).collect();
    drop_skills(game, p, free, |skill| starting.contains(&base_of(skill)))
}

/// Stats back to the starting values (INIT_AVATAR), with every point spent on them back.
fn reset_stats(game: &GameData, p: &mut Player) -> u32 {
    let start = &game.character_creator.genders[if p.gender == 1 { 1 } else { 0 }].basic_stats;
    let mut basic = p.basic_stats();
    let mut refund = 0;
    for stat in [
        BasicStatType::Strength,
        BasicStatType::Dexterity,
        BasicStatType::Intelligence,
        BasicStatType::Concentration,
        BasicStatType::Charm,
        BasicStatType::Sense,
    ] {
        let mut step = BasicStats { ..start.clone() };
        for value in start.get(stat)..basic.get(stat) {
            step.set(stat, value);
            refund += game.ability_value_calculator.calculate_basic_stat_increase_cost(&step, stat).unwrap_or(0);
        }
        basic.set(stat, start.get(stat));
    }
    p.set_basic_stats(&basic);
    p.stat_points += refund;
    refund
}

/// Level 1 Visitor with no experience, starting stats and skills and no points to spend:
/// a new character apart from items, Zuly and quests.
fn reset_level(ctx: &ReducerContext, game: &GameData, p: &mut Player, free: &mut Vec<u16>) {
    reset_stats(game, p);
    reset_skills(game, p, free);
    p.level = 1;
    p.xp = 0;
    p.job = 0;
    p.stat_points = 0;
    p.skill_points = 0;
    let mut state = p.quest_state();
    state.job_variables = [0; 3];
    p.set_quest_state(&state);
    crate::death::clear_debt(ctx, p.identity);
}

/// Hotbar slots that pointed at a skill that is gone are emptied.
fn clear_removed_skills(p: &mut Player) {
    let list = p.skill_list();
    let mut hotbar = p.hotbar();
    for slot in hotbar.pages.iter_mut().flatten() {
        if let Some(HotbarSlot::Skill(skill_slot)) = slot {
            if list.get_skill(*skill_slot).is_none() {
                *slot = None;
            }
        }
    }
    p.set_hotbar(&hotbar);
}

/// Worn items the character can't wear any more go to the bag. Returns a line for each
/// one that couldn't (bag full). Expects the player saved and refreshed, and saves again.
fn unequip_unwearable(ctx: &ReducerContext, game: &GameData, p: &mut Player) -> Vec<String> {
    use rose_data::EquipmentIndex as E;
    let mut problems = Vec::new();
    let mut moved = false;
    for index in [E::Weapon, E::SubWeapon, E::Head, E::Body, E::Hands, E::Feet, E::Back, E::Face, E::Necklace, E::Ring, E::Earring] {
        let mut equipment = p.equipment();
        let Some(item) = equipment.get_equipment_item(index).cloned() else { continue };
        let Some(data) = game.items.get_base_item(item.item) else { continue };
        if items::check_requirements(ctx, game, p, data).is_ok() {
            continue;
        }
        let mut inventory = p.inventory();
        let taken = equipment.get_equipment_slot_mut(index).take().unwrap();
        match inventory.try_add_equipment_item(taken) {
            Ok(_) => {
                p.set_equipment(&equipment);
                p.set_inventory(&inventory);
                moved = true;
                // Taking off one item can drop the stats another one needs.
                ctx.db.player().identity().update(p.clone());
                character::refresh_player(ctx, game, p, false);
            }
            Err(_) => problems.push(format!("{} can't wear {} any more, but the bag is full", p.name, data.name)),
        }
    }
    if moved {
        problems.insert(0, "gear that can't be worn any more went to the bag".into());
    }
    problems
}

/// The worn slots /grade acts on (numbered as /durability), with their names: one slot by
/// number or name, or the groups "armor" and "all".
fn grade_slots(word: &str) -> Option<Vec<(u8, &'static str)>> {
    const NAMES: [&str; 11] = ["face", "head", "body", "back", "hands", "feet", "weapon", "offhand", "necklace", "ring", "earring"];
    let word = word.to_ascii_lowercase();
    let pick = |slots: &[u8]| slots.iter().map(|&s| (s, NAMES[s as usize])).collect();
    match word.as_str() {
        "armor" | "armour" => Some(pick(&[1, 2, 3, 4, 5, 7])),
        "all" => Some(pick(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10])),
        "shield" | "off-hand" => Some(pick(&[7])),
        _ => {
            let slot = word.parse::<u8>().ok().or_else(|| NAMES.iter().position(|n| *n == word).map(|i| i as u8))?;
            ((slot as usize) < NAMES.len()).then(|| pick(&[slot]))
        }
    }
}
