//! Headless test client: connects, picks the nearest monster, fights it, and checks
//! animation cancelling: moving mid-swing cancels it (no damage, cooldown refunded),
//! moving right after a hit is free but keeps the cooldown. Prints a timeline.
//!
//! Usage: rose-stdb-bot [ws://127.0.0.1:3000] [melee|ranged]

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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(2).map(|a| a.as_str()) == Some("load") {
        let n = args.get(3).and_then(|a| a.parse().ok()).unwrap_or(50);
        let secs = args.get(4).and_then(|a| a.parse().ok()).unwrap_or(60);
        return load(args[1].clone(), n, secs);
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
