//! Headless test client: connects, picks the nearest monster, fights it, and checks
//! animation cancelling: moving mid-swing cancels it (no damage, cooldown refunded),
//! moving right after a hit is free but keeps the cooldown. Prints a timeline.
//!
//! Usage: rose-stdb-bot [ws://127.0.0.1:3000] [melee|ranged]
//!
//! `rose-stdb-bot URL cast` checks skill cast cancelling instead: the grace window, a
//! locked-in cast holding the next command, and going back to the attack after a skill.
//! The character needs Blessing in active slot 0 and Berserk in slot 1 and plenty of MP
//! first: it waits for MP 500 while the admin sets it up (give_skill, set_hp_mp).
//!
//! `rose-stdb-bot URL flee` checks that a swing finishes once it has started: it hits the
//! nearest monster until it fights back, then keeps stepping away from it. Every swing the
//! monster starts should land; none should be restarted while it chases.

mod module_bindings;

use module_bindings::*;
use spacetimedb_sdk::{DbContext, Table};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Default)]
struct Log {
    hits: Vec<(Instant, u64, u64, i32, bool, bool)>,
    my_motion_updates: Vec<Instant>,
}

fn pos(m: &Motion, now_us: i64) -> (f32, f32) {
    let (dx, dy) = (m.to_x - m.from_x, m.to_y - m.from_y);
    let d = (dx * dx + dy * dy).sqrt();
    if d < 0.01 || m.speed <= 0.0 {
        return (m.to_x, m.to_y);
    }
    let f = (((now_us - m.started_at_us).max(0) as f32 / 1e6) * m.speed / d).min(1.0);
    (m.from_x + dx * f, m.from_y + dy * f)
}

fn now_us() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros() as i64
}

fn load(uri: String, n: usize, secs: u64) {
    let mut conns = vec![];
    for i in 0..n {
        let c = DbConnection::builder().with_uri(uri.clone()).with_database_name("rose").build().expect("connect");
        c.subscription_builder().subscribe_to_all_tables();
        c.run_threaded();
        conns.push(c);
        if i % 25 == 24 { println!("{} bots connected", i + 1); }
    }
    std::thread::sleep(Duration::from_secs(3));
    let start = Instant::now();
    let mut calls = 0u64;
    let mut rng_state = 12345u64;
    let mut rnd = move || { rng_state ^= rng_state << 13; rng_state ^= rng_state >> 7; rng_state ^= rng_state << 17; (rng_state % 10_000) as f32 / 10_000.0 };
    while start.elapsed() < Duration::from_secs(secs) {
        for c in &conns {
            let Some(me) = c.try_identity().and_then(|i| c.db.player().identity().find(&i)).and_then(|p| p.entity_id) else { continue };
            if rnd() > 0.5 { continue; }
            let Some(m) = c.db.motion().entity_id().find(&me) else { continue };
            let p = pos(&m, now_us());
            if rnd() < 0.3 {
                let target = c.db.entity().iter().filter(|e| e.kind == EntityKind::Monster)
                    .filter_map(|e| c.db.motion().entity_id().find(&e.entity_id).map(|mm| (e.entity_id, pos(&mm, now_us()))))
                    .min_by(|a, b| ((a.1.0-p.0).powi(2)+(a.1.1-p.1).powi(2)).total_cmp(&((b.1.0-p.0).powi(2)+(b.1.1-p.1).powi(2))));
                if let Some((t, _)) = target { c.reducers.attack(t).ok(); calls += 1; }
            } else {
                c.reducers.move_to(p.0 + (rnd() - 0.5) * 2000.0, p.1 + (rnd() - 0.5) * 2000.0).ok();
                calls += 1;
            }
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    let c = &conns[0];
    if let Some(ts) = c.db.tick_stats().id().find(&0) {
        println!("{} bots, {} reducer calls in {}s; worst gap between combat ticks {:.0} ms", n, calls, secs, ts.max_gap_us as f64 / 1000.0);
    }
    println!("entities visible to one bot: {}", c.db.entity().count());
    for c in conns { c.disconnect().ok(); }
}

/// The cast checks (see the top of the file).
fn cast(uri: String) {
    let start = Instant::now();
    let conn = DbConnection::builder().with_uri(uri).with_database_name("rose").build().expect("connect");
    conn.subscription_builder().subscribe_to_all_tables();
    conn.run_threaded();
    let me = loop {
        if let Some(id) = conn.try_identity().and_then(|i| conn.db.player().identity().find(&i)).and_then(|p| p.entity_id) {
            if conn.db.motion().entity_id().find(&id).is_some() {
                break id;
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let name = format!("Bot{}", me);
    conn.reducers.set_name(name.clone()).ok();
    println!("my entity {} name {}", me, name);
    let t = || start.elapsed().as_secs_f32();
    let combat = || conn.db.combat().entity_id().find(&me).unwrap();
    let cast_row = || conn.db.skill_cast().entity_id().find(&me);
    let wait = |f: &dyn Fn() -> bool, secs: u64| -> bool {
        let until = Instant::now() + Duration::from_secs(secs);
        while Instant::now() < until {
            if f() {
                return true;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        false
    };
    if !wait(&|| combat().mp >= 500, 60) {
        println!("FAIL setup: MP never reached 500");
        return;
    }
    let cooldown_until = |key: u32| conn.db.skill_cooldown().iter().filter(|r| r.entity_id == me && r.key == key).map(|r| r.until_us).max().unwrap_or(0);
    let my_pos = || pos(&conn.db.motion().entity_id().find(&me).unwrap(), now_us());
    let started = || cast_row().and_then(|c| c.started_at_us);
    let mut checks: Vec<(&str, bool)> = Vec::new();
    const BLESSING: u32 = 971;

    // 1. Grace window: move straight after the casting starts. Nothing is spent.
    let mp0 = combat().mp;
    conn.reducers.cast_skill(1, 0, None, 0.0, 0.0).unwrap();
    wait(&|| started().is_some(), 3);
    let p = my_pos();
    conn.reducers.move_to(p.0 + 200.0, p.1).unwrap();
    std::thread::sleep(Duration::from_millis(600));
    println!("[{:5.1}s] cast Blessing and moved at once: MP {} -> {}", t(), mp0, combat().mp);
    checks.push(("moving in the grace window cancels the cast", cast_row().is_none()));
    checks.push(("a cast cancelled in the grace window costs no MP", combat().mp == mp0));
    checks.push(("a cast cancelled in the grace window starts no cooldown", cooldown_until(BLESSING) <= now_us()));

    // 2. Locked in: move after the grace window. The cast lands, then we walk.
    std::thread::sleep(Duration::from_millis(500));
    let mp0 = combat().mp;
    conn.reducers.cast_skill(1, 0, None, 0.0, 0.0).unwrap();
    checks.push(("the same skill can be cast again at once", wait(&|| started().is_some(), 3)));
    let c = cast_row().unwrap();
    let begun = c.started_at_us.unwrap();
    println!("[{:5.1}s] cast Blessing again: takes effect {:.0} ms after the casting starts", t(), (c.effect_at_us - begun) as f64 / 1000.0);
    // Halfway between the end of the grace window and the effect.
    let at = begun + (300_000 + c.effect_at_us - begun) / 2;
    while now_us() < at {
        std::thread::sleep(Duration::from_millis(1));
    }
    let p = my_pos();
    let moved_at = now_us();
    conn.reducers.move_to(p.0 + 200.0, p.1).unwrap();
    std::thread::sleep(Duration::from_millis(60));
    checks.push(("moving while locked in doesn't cancel the cast", cast_row().is_some_and(|c| !c.applied)));
    checks.push(("moving while locked in waits (no walking yet)", conn.db.motion().entity_id().find(&me).unwrap().started_at_us < moved_at));
    wait(&|| conn.db.motion().entity_id().find(&me).unwrap().started_at_us >= moved_at, 3);
    let walk = conn.db.motion().entity_id().find(&me).unwrap().started_at_us;
    println!("[{:5.1}s] moved {:.0} ms into the cast; the walk started {:.0} ms after the effect; MP {} -> {}",
        t(), (moved_at - begun) as f64 / 1000.0, (walk - c.effect_at_us) as f64 / 1000.0, mp0, combat().mp);
    checks.push(("the held move starts when the skill takes effect", walk >= c.effect_at_us - 1000 && walk < c.effect_at_us + 250_000));
    checks.push(("a locked-in cast costs its MP", combat().mp < mp0));
    checks.push(("a locked-in cast starts its cooldown", cooldown_until(BLESSING) > now_us()));

    // 3. Go back to the attack after a skill: fight the nearest monster, use Blessing (a
    //    "stop" skill) and then Berserk (a "restore" skill) mid-fight.
    let target = conn
        .db
        .entity()
        .iter()
        .filter(|e| e.kind == EntityKind::Monster && e.npc_id < 316)
        .filter_map(|e| conn.db.motion().entity_id().find(&e.entity_id).map(|m| (e.entity_id, pos(&m, now_us()))))
        .min_by(|a, b| {
            let p = my_pos();
            let da = (a.1 .0 - p.0).powi(2) + (a.1 .1 - p.1).powi(2);
            let db = (b.1 .0 - p.0).powi(2) + (b.1 .1 - p.1).powi(2);
            da.total_cmp(&db)
        })
        .map(|(id, _)| id)
        .expect("no monsters");
    for (slot, skill) in [(0u16, "Blessing"), (1, "Berserk")] {
        conn.reducers.attack(target).unwrap();
        if !wait(&|| combat().swing_hit_at_us.is_some(), 60) {
            println!("FAIL setup: never got to swing at #{}", target);
            break;
        }
        wait(&|| cooldown_until(BLESSING) <= now_us(), 10);
        conn.reducers.cast_skill(1, slot, None, 0.0, 0.0).unwrap();
        if !wait(&|| cast_row().is_some(), 2) {
            println!("[{:5.1}s] {} was refused", t(), skill);
            continue;
        }
        let gone = wait(&|| cast_row().is_none(), 5);
        let back = gone && wait(&|| combat().attack_target == Some(target), 1);
        let alive = conn.db.combat().entity_id().find(&target).is_some_and(|c| c.hp > 0 && c.dead_until_us.is_none());
        println!("[{:5.1}s] {} mid-fight: attack target afterwards {:?} (target alive: {})", t(), skill, combat().attack_target, alive);
        if !alive {
            println!("SKIP {}: the monster died during the skill", skill);
            break;
        }
        checks.push((if slot == 0 { "a stop skill goes back to the attack" } else { "a restore skill goes back to the attack" }, back));
        if conn.db.entity().entity_id().find(&target).is_none() || combat().attack_target.is_none() {
            break;
        }
    }
    for (name, ok) in &checks {
        println!("{} {}", if *ok { "PASS" } else { "FAIL" }, name);
    }
    conn.disconnect().ok();
}

fn flee(uri: String) {
    let conn = DbConnection::builder().with_uri(uri).with_database_name("rose").build().expect("connect");
    conn.subscription_builder().subscribe_to_all_tables();
    conn.run_threaded();
    let me = loop {
        if let Some(id) = conn.try_identity().and_then(|i| conn.db.player().identity().find(&i)).and_then(|p| p.entity_id) {
            if conn.db.motion().entity_id().find(&id).is_some() {
                break id;
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    std::thread::sleep(Duration::from_secs(3));
    let my_pos = || pos(&conn.db.motion().entity_id().find(&me).unwrap(), now_us());
    let p = my_pos();
    let (target, _) = conn.db.entity().iter()
        .filter(|e| e.kind == EntityKind::Monster && e.npc_id < 316)
        // Tough enough to live through a few hits.
        .filter(|e| conn.db.combat().entity_id().find(&e.entity_id).is_some_and(|c| c.hp >= 100))
        .filter_map(|e| conn.db.motion().entity_id().find(&e.entity_id).map(|m| (e.entity_id, pos(&m, now_us()))))
        .min_by(|a, b| ((a.1.0 - p.0).powi(2) + (a.1.1 - p.1).powi(2)).total_cmp(&((b.1.0 - p.0).powi(2) + (b.1.1 - p.1).powi(2))))
        .expect("no monsters spawned");
    // Count the monster's swing starts and the hits it lands on us.
    let counts = Arc::new(Mutex::new((0u32, 0u32, false)));
    {
        let c1 = counts.clone();
        spacetimedb_sdk::__codegen::WithUpdate::on_update(&conn.db.combat(), move |_, old, new| {
            let mut c = c1.lock().unwrap();
            if new.entity_id == target && c.2 && old.swing_hit_at_us.is_none() && new.swing_hit_at_us.is_some() {
                c.0 += 1;
            }
        });
        let c2 = counts.clone();
        spacetimedb_sdk::__codegen::WithInsert::on_insert(&conn.db.damage_event(), move |_, ev| {
            let mut c = c2.lock().unwrap();
            if ev.attacker == target && ev.defender == me && c.2 {
                c.1 += 1;
            }
        });
    }
    // Hit it until it fights back, then stop hitting it.
    conn.reducers.attack(target).unwrap();
    let t0 = Instant::now();
    while conn.db.combat().entity_id().find(&target).and_then(|c| c.attack_target) != Some(me) {
        assert!(t0.elapsed() < Duration::from_secs(90), "monster never fought back");
        assert!(conn.db.combat().entity_id().find(&target).is_some_and(|c| c.hp > 0), "monster died first");
        std::thread::sleep(Duration::from_millis(20));
    }
    let p = my_pos();
    conn.reducers.move_to(p.0 + 1.0, p.1).ok();
    println!("monster #{} fights back; stepping away from it for 20 s", target);
    counts.lock().unwrap().2 = true;
    let t0 = Instant::now();
    while t0.elapsed() < Duration::from_secs(20) {
        let p = my_pos();
        let Some(m) = conn.db.motion().entity_id().find(&target) else { break };
        let q = pos(&m, now_us());
        let (dx, dy) = (p.0 - q.0, p.1 - q.1);
        let d = (dx * dx + dy * dy).sqrt().max(1.0);
        // Walk 2 m straight away from the monster, then wait for it to catch up.
        conn.reducers.move_to(p.0 + dx / d * 200.0, p.1 + dy / d * 200.0).ok();
        std::thread::sleep(Duration::from_millis(1200));
    }
    counts.lock().unwrap().2 = false;
    let in_progress = conn.db.combat().entity_id().find(&target).is_some_and(|c| c.swing_hit_at_us.is_some()) as u32;
    let (starts, hits, _) = *counts.lock().unwrap();
    println!("monster swings started: {}, landed: {}, still winding up: {}", starts, hits, in_progress);
    println!("{} every started swing lands", if starts > 0 && starts <= hits + in_progress { "PASS" } else { "FAIL" });
    conn.disconnect().ok();
}

/// The global cooldown: Blessing (skills slot 1,0), then Berserk (1,1) pressed while
/// Blessing is locked in, and again right after it took effect. Berserk's casting must
/// start no sooner than 0.25 s after Blessing took effect.
fn gcd(uri: String) {
    let conn = DbConnection::builder().with_uri(uri).with_database_name("rose").build().expect("connect");
    conn.subscription_builder().subscribe_to_all_tables();
    conn.run_threaded();
    let me = loop {
        if let Some(id) = conn.try_identity().and_then(|i| conn.db.player().identity().find(&i)).and_then(|p| p.entity_id) {
            if conn.db.motion().entity_id().find(&id).is_some() {
                break id;
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let name = format!("Bot{}", me);
    conn.reducers.set_name(name.clone()).ok();
    println!("my entity {} name {}", me, name);
    let combat = || conn.db.combat().entity_id().find(&me).unwrap();
    let cast_row = || conn.db.skill_cast().entity_id().find(&me);
    let wait = |f: &dyn Fn() -> bool, secs: u64| -> bool {
        let until = Instant::now() + Duration::from_secs(secs);
        while Instant::now() < until {
            if f() {
                return true;
            }
            std::thread::sleep(Duration::from_millis(2));
        }
        false
    };
    if !wait(&|| combat().mp >= 100, 60) {
        println!("FAIL setup: MP never reached 100");
        return;
    }
    let global_until = || conn.db.skill_cooldown().iter().filter(|r| r.entity_id == me && r.key == 0).map(|r| r.until_us).max().unwrap_or(0);
    let mut checks: Vec<(String, bool)> = Vec::new();
    for round in ["pressed while Blessing is locked in", "pressed just after Blessing took effect"] {
        wait(&|| cast_row().is_none(), 5);
        // Every cooldown over (Berserk's own is 6 s).
        wait(&|| conn.db.skill_cooldown().iter().all(|r| r.entity_id != me || r.until_us < now_us()), 15);
        std::thread::sleep(Duration::from_millis(500));
        conn.reducers.cast_skill(1, 0, None, 0.0, 0.0).unwrap();
        if !wait(&|| cast_row().is_some_and(|c| c.skill_id == 971 && c.started_at_us.is_some()), 3) {
            println!("FAIL setup: Blessing didn't start");
            return;
        }
        let blessing = cast_row().unwrap();
        let (begun, effect) = (blessing.started_at_us.unwrap(), blessing.effect_at_us);
        let press_at = if round.contains("locked") { begun + 350_000 } else { effect + 20_000 };
        while now_us() < press_at {
            std::thread::sleep(Duration::from_millis(1));
        }
        conn.reducers.cast_skill(1, 1, None, 0.0, 0.0).unwrap();
        let berserk_started = || cast_row().filter(|c| c.skill_id != 971).and_then(|c| c.started_at_us);
        if !wait(&|| berserk_started().is_some(), 3) {
            checks.push((format!("Berserk {round} starts"), false));
            continue;
        }
        let gap = (berserk_started().unwrap() - effect) as f64 / 1000.0;
        println!("Berserk {round}: Blessing took effect, Berserk's casting started {gap:.0} ms later (global cooldown until +{:.0} ms)",
            (global_until() - effect) as f64 / 1000.0);
        checks.push((format!("Berserk {round} waits for the 0.25 s global cooldown"), gap >= 245.0 && gap < 400.0));
    }
    for (name, ok) in &checks {
        println!("{} {}", if *ok { "PASS" } else { "FAIL" }, name);
    }
    conn.disconnect().ok();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(2).map(|a| a.as_str()) == Some("load") {
        let n = args.get(3).and_then(|a| a.parse().ok()).unwrap_or(50);
        let secs = args.get(4).and_then(|a| a.parse().ok()).unwrap_or(60);
        return load(args[1].clone(), n, secs);
    }
    if args.get(2).map(|a| a.as_str()) == Some("cast") {
        return cast(args[1].clone());
    }
    if args.get(2).map(|a| a.as_str()) == Some("gcd") {
        return gcd(args[1].clone());
    }
    if args.get(2).map(|a| a.as_str()) == Some("flee") {
        return flee(args[1].clone());
    }
    let uri = args.get(1).cloned().unwrap_or_else(|| "ws://127.0.0.1:3000".into());
    let ranged = args.get(2).map_or(true, |a| a == "ranged");
    let log = Arc::new(Mutex::new(Log::default()));
    let start = Instant::now();

    let conn = DbConnection::builder()
        .with_uri(uri)
        .with_database_name("rose")
        .on_connect(|_, identity, _| println!("connected as {}", identity.to_hex()))
        .build()
        .expect("connect");
    {
        let log = log.clone();
        spacetimedb_sdk::__codegen::WithInsert::on_insert(&conn.db.damage_event(), move |_, ev| {
            log.lock().unwrap().hits.push((Instant::now(), ev.attacker, ev.defender, ev.amount, ev.is_critical, ev.killed));
        });
    }
    conn.subscription_builder().subscribe_to_all_tables();
    conn.run_threaded();

    // Wait for our character.
    let me = loop {
        if let Some(id) = conn.try_identity().and_then(|i| conn.db.player().identity().find(&i)).and_then(|p| p.entity_id) {
            if conn.db.motion().entity_id().find(&id).is_some() {
                break id;
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    {
        let log = log.clone();
        spacetimedb_sdk::__codegen::WithUpdate::on_update(&conn.db.motion(), move |_, _, new| {
            if new.entity_id == me {
                log.lock().unwrap().my_motion_updates.push(Instant::now());
            }
        });
    }
    conn.reducers.set_name(format!("Bot{}", me)).ok();
    if ranged {
        // New characters carry a Short Bow (equipment page, slot 1) and arrows (materials,
        // slot 0). The bow needs level 10 and DEX 29: give_xp and set_basic_stat as admin.
        conn.reducers.equip_item(0, 1).ok();
        conn.reducers.equip_item(2, 0).ok();
    }
    println!("my entity {} ({} loadout)", me, if ranged { "ranged" } else { "melee" });

    // Wait for monsters to spawn.
    std::thread::sleep(Duration::from_secs(3));
    let my_pos = || pos(&conn.db.motion().entity_id().find(&me).unwrap(), now_us());

    // Walk next to the nearest monster first (spawns are a long way from the start point).
    let nearest = |from: (f32, f32)| {
        conn.db
            .entity()
            .iter()
            .filter(|e| e.kind == EntityKind::Monster && e.npc_id < 316) // skip butterflies
            .filter_map(|e| conn.db.motion().entity_id().find(&e.entity_id).map(|m| (e, pos(&m, now_us()))))
            .min_by(|a, b| {
                let da = (a.1 .0 - from.0).powi(2) + (a.1 .1 - from.1).powi(2);
                let db = (b.1 .0 - from.0).powi(2) + (b.1 .1 - from.1).powi(2);
                da.total_cmp(&db)
            })
    };
    let monsters = conn.db.entity().iter().filter(|e| e.kind == EntityKind::Monster).count();
    let (target, tpos) = nearest(my_pos()).expect("no monsters spawned");
    let p = my_pos();
    let d = ((tpos.0 - p.0).powi(2) + (tpos.1 - p.1).powi(2)).sqrt();
    println!("{} monsters alive; nearest is {} #{} at {:.0} m", monsters, target.name, target.entity_id, d / 100.0);

    // Move-response latency: time from move_to to our motion row update arriving.
    let mut latencies = vec![];
    for i in 0..5 {
        let before = log.lock().unwrap().my_motion_updates.len();
        let p = my_pos();
        let t0 = Instant::now();
        conn.reducers.move_to(p.0 + 100.0 * (i as f32 + 1.0), p.1).unwrap();
        while log.lock().unwrap().my_motion_updates.len() == before && t0.elapsed() < Duration::from_secs(2) {
            std::thread::sleep(Duration::from_millis(1));
        }
        latencies.push(t0.elapsed().as_secs_f64() * 1000.0);
        std::thread::sleep(Duration::from_millis(300));
    }
    println!("move_to -> motion update latency (ms): {:?}", latencies.iter().map(|l| format!("{:.1}", l)).collect::<Vec<_>>());

    // Teleporting isn't allowed, so walk there. Attack makes us chase when out of range.
    let target_id = target.entity_id;
    conn.reducers.attack(target_id).unwrap();
    println!("[{:5.1}s] attack #{} (chasing)", start.elapsed().as_secs_f32(), target_id);

    let t0 = Instant::now();
    let deadline = t0 + Duration::from_secs(120);
    let my_hits = |log: &Arc<Mutex<Log>>| log.lock().unwrap().hits.iter().filter(|h| h.1 == me).count();
    let combat = |conn: &DbConnection| conn.db.combat().entity_id().find(&me).unwrap();
    let wait = |what: &str, f: &dyn Fn() -> bool| -> bool {
        while Instant::now() < deadline {
            if f() {
                return true;
            }
            if conn.db.entity().entity_id().find(&target_id).is_none() || combat(&conn).dead_until_us.is_some() {
                println!("[{:5.1}s] fight ended while waiting for {}", start.elapsed().as_secs_f32(), what);
                return false;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        false
    };
    let step_away = |conn: &DbConnection| {
        let m = conn.db.motion().entity_id().find(&me).unwrap();
        let p = pos(&m, now_us());
        conn.reducers.move_to(p.0 + 200.0, p.1 + 100.0).unwrap();
    };
    let mut checks: Vec<(&str, bool)> = Vec::new();

    // 1. Fight normally until two hits land.
    if wait("two hits", &|| my_hits(&log) >= 2) {
        println!("[{:5.1}s] two hits landed", start.elapsed().as_secs_f32());

        // 2. Cancel during the wind-up: move as soon as the next swing starts.
        if wait("a swing to start", &|| combat(&conn).swing_hit_at_us.is_some()) {
            let hits_before = my_hits(&log);
            let hit_at = combat(&conn).swing_hit_at_us.unwrap();
            step_away(&conn);
            std::thread::sleep(Duration::from_millis(300));
            let c = combat(&conn);
            println!("[{:5.1}s] moved {:.0} ms before the hit frame", start.elapsed().as_secs_f32(), (hit_at - now_us() + 300_000) as f64 / 1000.0);
            checks.push(("moving clears the attack target", c.attack_target.is_none()));
            checks.push(("moving mid-swing cancels it", c.swing_hit_at_us.is_none()));
            checks.push(("cancelled swing refunds the cooldown", c.next_attack_at_us <= now_us()));
            std::thread::sleep(Duration::from_millis(1500));
            checks.push(("cancelled swing deals no damage", my_hits(&log) == hits_before));

            // 3. Attack again: the swing starts right away instead of waiting out a cooldown.
            let hits_before = my_hits(&log);
            let again = Instant::now();
            conn.reducers.attack(target_id).unwrap();
            if wait("the next hit", &|| my_hits(&log) > hits_before) {
                let ms = again.elapsed().as_secs_f64() * 1000.0;
                println!("[{:5.1}s] re-attack to hit: {:.0} ms (includes walking back into range)", start.elapsed().as_secs_f32(), ms);

                // 4. Cancel the recovery: move right after the hit. Movement starts at once,
                //    but the attack-speed cooldown still applies to the next swing.
                let m_before = conn.db.motion().entity_id().find(&me).unwrap().started_at_us;
                let moved = Instant::now();
                step_away(&conn);
                let ok = wait("our move", &|| conn.db.motion().entity_id().find(&me).unwrap().started_at_us != m_before);
                println!("[{:5.1}s] move after hit applied in {:.1} ms", start.elapsed().as_secs_f32(), moved.elapsed().as_secs_f64() * 1000.0);
                checks.push(("moving after the hit starts immediately", ok));
                checks.push(("recovery cancel keeps the cooldown", combat(&conn).next_attack_at_us > now_us()));
            }
        }
    }

    let l = log.lock().unwrap();
    let mine: Vec<_> = l.hits.iter().filter(|h| h.1 == me).collect();
    let taken: Vec<_> = l.hits.iter().filter(|h| h.2 == me).collect();
    println!("hits landed: {} (total {} dmg, {} crits, {} misses)", mine.len(), mine.iter().map(|h| h.3).sum::<i32>(),
        mine.iter().filter(|h| h.4).count(), mine.iter().filter(|h| h.3 == 0).count());
    println!("hits taken: {} (total {} dmg)", taken.len(), taken.iter().map(|h| h.3).sum::<i32>());
    for (name, ok) in &checks {
        println!("{} {}", if *ok { "PASS" } else { "FAIL" }, name);
    }
    if let Some(ts) = conn.db.tick_stats().id().find(&0) {
        println!("combat ticks: {}, worst gap between ticks {:.0} ms", ts.ticks, ts.max_gap_us as f64 / 1000.0);
    }
    conn.disconnect().ok();
}
