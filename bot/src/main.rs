//! Headless test client: connects, picks the nearest monster, fights it, then
//! kites it while moving to prove attack-while-moving. Prints a timeline.
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
    hits: Vec<(Instant, u64, u64, i32, bool, bool, bool)>,
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
            log.lock().unwrap().hits.push((Instant::now(), ev.attacker, ev.defender, ev.amount, ev.is_critical, ev.killed, ev.attacker_moving));
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
    conn.reducers.set_loadout(ranged).ok();
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

    let mut kiting_started: Option<Instant> = None;
    let deadline = Instant::now() + Duration::from_secs(120);
    let mut kite_dir = 1.0f32;
    while Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(100));
        let alive = conn.db.entity().entity_id().find(&target_id).is_some();
        let my_combat = conn.db.combat().entity_id().find(&me).unwrap();
        if !alive {
            println!("[{:5.1}s] target died", start.elapsed().as_secs_f32());
            break;
        }
        if my_combat.dead_until_us.is_some() {
            println!("[{:5.1}s] we died", start.elapsed().as_secs_f32());
            break;
        }
        // After the first hits land, start kiting: keep walking back and forth while the target is set.
        let my_hits = log.lock().unwrap().hits.iter().filter(|h| h.1 == me).count();
        if my_hits >= 2 && kiting_started.is_none() {
            kiting_started = Some(Instant::now());
            println!("[{:5.1}s] start kiting: move_to while keeping the attack target", start.elapsed().as_secs_f32());
        }
        if let Some(k) = kiting_started {
            let m = conn.db.motion().entity_id().find(&me).unwrap();
            let moving = { let p = pos(&m, now_us()); (p.0 - m.to_x).abs() > 1.0 || (p.1 - m.to_y).abs() > 1.0 };
            if !moving {
                // Walk 3 m sideways, alternating, staying inside bow range.
                let p = pos(&m, now_us());
                kite_dir = -kite_dir;
                conn.reducers.move_to(p.0 + 300.0 * kite_dir, p.1 + 150.0).unwrap();
            }
            let _ = k;
        }
    }

    // Summarise: hits we landed while our own motion said we were moving.
    let l = log.lock().unwrap();
    let mine: Vec<_> = l.hits.iter().filter(|h| h.1 == me).collect();
    let taken: Vec<_> = l.hits.iter().filter(|h| h.2 == me).collect();
    let hits_while_kiting = kiting_started.map_or(0, |k| mine.iter().filter(|h| h.0 >= k).count());
    println!("hits landed: {} (total {} dmg, {} crits, {} misses)", mine.len(), mine.iter().map(|h| h.3).sum::<i32>(),
        mine.iter().filter(|h| h.4).count(), mine.iter().filter(|h| h.3 == 0).count());
    println!("hits landed after kiting started: {}, of which the server saw us moving: {}", hits_while_kiting,
        mine.iter().filter(|h| h.6).count());
    println!("hits taken: {} (total {} dmg)", taken.len(), taken.iter().map(|h| h.3).sum::<i32>());
    let gaps: Vec<f64> = mine.windows(2).map(|w| (w[1].0 - w[0].0).as_secs_f64() * 1000.0).collect();
    if !gaps.is_empty() {
        println!("swing gaps (ms): min {:.0} max {:.0} avg {:.0}", gaps.iter().cloned().fold(f64::MAX, f64::min),
            gaps.iter().cloned().fold(0.0, f64::max), gaps.iter().sum::<f64>() / gaps.len() as f64);
    }
    if let Some(ts) = conn.db.tick_stats().id().find(&0) {
        println!("combat ticks: {}, worst gap between ticks {:.0} ms", ts.ticks, ts.max_gap_us as f64 / 1000.0);
    }
    conn.disconnect().ok();
}
