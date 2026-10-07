//! Connection to the SpacetimeDB server (module `rose`).
//!
//! GDScript polls this node: `get_entities()` returns every entity with its position on
//! the server's motion path at the current time, and `poll_damage_events()` drains the
//! hits since the last call. Messages are processed in `_process` with `frame_tick`, so
//! table callbacks run on the main thread.

use std::sync::{Arc, Mutex};

use godot::{classes::INode, prelude::*};
use spacetimedb_sdk::{
    __codegen::{WithInsert, WithUpdate},
    DbContext, Table,
};

use crate::module_bindings::*;

fn local_now_us() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_micros() as i64)
}

/// Position on a straight-line move at server time `t_us`, in ROSE cm. Same formula as the module.
fn motion_position(m: &Motion, t_us: i64) -> (f32, f32) {
    let (dx, dy) = (m.to_x - m.from_x, m.to_y - m.from_y);
    let d = (dx * dx + dy * dy).sqrt();
    if d < 0.01 || m.speed <= 0.0 {
        return (m.to_x, m.to_y);
    }
    let f = (((t_us - m.started_at_us).max(0) as f32 / 1e6) * m.speed / d).min(1.0);
    (m.from_x + dx * f, m.from_y + dy * f)
}

/// ROSE (x, y) in cm to Godot (x, z) in metres.
fn to_godot(x: f32, y: f32) -> (f32, f32) {
    (x / 100.0, -y / 100.0)
}

#[derive(Default)]
struct Shared {
    damage: Vec<DamageEvent>,
    /// Smallest (local receive time - server start time) seen on a fresh motion change.
    /// Covers clock skew between this PC and the server plus the fastest one-way delay.
    clock_offset_us: Option<i64>,
}

#[derive(GodotClass)]
#[class(base=Node, init)]
pub struct RoseNet {
    base: Base<Node>,
    conn: Option<DbConnection>,
    shared: Arc<Mutex<Shared>>,
    error: String,
}

impl RoseNet {
    fn server_now_us(&self) -> i64 {
        local_now_us() - self.shared.lock().unwrap().clock_offset_us.unwrap_or(0)
    }

    fn my_id(&self) -> Option<u64> {
        let c = self.conn.as_ref()?;
        c.try_identity().and_then(|i| c.db.player().identity().find(&i)).and_then(|p| p.entity_id)
    }

    fn fail(&mut self, error: String) {
        godot_warn!("rose: server connection: {error}");
        self.error = error;
        self.conn = None;
    }
}

#[godot_api]
impl INode for RoseNet {
    fn process(&mut self, _delta: f64) {
        let Some(c) = self.conn.as_ref() else { return };
        if let Err(error) = c.frame_tick() {
            self.fail(format!("{error}"));
        }
    }
}

#[godot_api]
impl RoseNet {
    /// Connects and subscribes to every public table. The identity token is kept in
    /// `token_path`, so the same file gives the same character on the next run.
    #[func]
    fn connect_to(&mut self, uri: GString, token_path: GString) -> bool {
        let token_path = token_path.to_string();
        let shared = self.shared.clone();
        let saved = token_path.clone();
        let result = DbConnection::builder()
            .with_uri(uri.to_string())
            .with_database_name("rose")
            .with_token(std::fs::read_to_string(&token_path).ok())
            .on_connect(move |_, _, token| {
                std::fs::write(&saved, token).ok();
            })
            .build();
        let conn = match result {
            Ok(conn) => conn,
            Err(error) => {
                self.fail(format!("{error}"));
                return false;
            }
        };

        let s = shared.clone();
        conn.db.damage_event().on_insert(move |_, ev| s.lock().unwrap().damage.push(ev.clone()));
        let s = shared.clone();
        conn.db.motion().on_update(move |_, _, new| {
            let sample = local_now_us() - new.started_at_us;
            let mut s = s.lock().unwrap();
            s.clock_offset_us = Some(s.clock_offset_us.map_or(sample, |o| o.min(sample)));
        });
        conn.subscription_builder().subscribe_to_all_tables();
        self.error.clear();
        self.conn = Some(conn);
        true
    }

    #[func]
    fn disconnect_from(&mut self) {
        if let Some(c) = self.conn.take() {
            c.disconnect().ok();
        }
    }

    #[func]
    fn is_online(&self) -> bool {
        self.conn.as_ref().is_some_and(|c| c.is_active())
    }

    /// True once our own entity and its motion and combat rows have arrived.
    #[func]
    fn is_ready(&self) -> bool {
        let (Some(c), Some(id)) = (self.conn.as_ref(), self.my_id()) else { return false };
        c.db.motion().entity_id().find(&id).is_some() && c.db.combat().entity_id().find(&id).is_some()
    }

    #[func]
    fn get_error(&self) -> GString {
        GString::from(self.error.as_str())
    }

    #[func]
    fn my_entity_id(&self) -> i64 {
        self.my_id().map_or(-1, |id| id as i64)
    }

    /// Every entity in our zone: id, kind ("player" or "monster"), name, npc_id, position
    /// x/z and destination to_x/to_z (Godot metres), moving, speed (m/s), chasing, hp,
    /// max_hp, target (-1 for none), swinging, hit_in (seconds until the swing's hit frame),
    /// dead, and ranged for players.
    #[func]
    fn get_entities(&self) -> VarArray {
        let mut out = VarArray::new();
        let Some(c) = self.conn.as_ref() else { return out };
        let t = self.server_now_us();
        let zone = self.my_id().and_then(|id| c.db.entity().entity_id().find(&id)).map(|e| e.zone_id);
        let ranged: std::collections::HashMap<u64, bool> =
            c.db.player().iter().filter_map(|p| p.entity_id.map(|id| (id, p.ranged))).collect();

        for e in c.db.entity().iter() {
            if zone.is_some_and(|z| z != e.zone_id) {
                continue;
            }
            let Some(m) = c.db.motion().entity_id().find(&e.entity_id) else { continue };
            let (x, y) = motion_position(&m, t);
            let (gx, gz) = to_godot(x, y);
            let (tx, tz) = to_godot(m.to_x, m.to_y);
            let moving = (x - m.to_x).abs() > 0.5 || (y - m.to_y).abs() > 0.5;

            let mut d = VarDictionary::new();
            d.set("id", e.entity_id as i64);
            d.set("kind", if e.kind == EntityKind::Player { "player" } else { "monster" });
            d.set("name", e.name.as_str());
            d.set("npc_id", e.npc_id as i64);
            d.set("x", gx);
            d.set("z", gz);
            d.set("to_x", tx);
            d.set("to_z", tz);
            d.set("moving", moving);
            d.set("speed", m.speed / 100.0);
            d.set("chasing", m.chase_target.is_some());
            d.set("ranged", ranged.get(&e.entity_id).copied().unwrap_or(false));
            if let Some(cb) = c.db.combat().entity_id().find(&e.entity_id) {
                d.set("hp", cb.hp);
                d.set("max_hp", cb.max_hp);
                d.set("target", cb.attack_target.map_or(-1, |t| t as i64));
                d.set("swinging", cb.swing_hit_at_us.is_some());
                d.set("hit_in", cb.swing_hit_at_us.map_or(0.0, |at| (at - t) as f64 / 1e6));
                d.set("dead", cb.dead_until_us.is_some_and(|until| until > t) || cb.hp <= 0);
            }
            out.push(&d.to_variant());
        }
        out
    }

    /// Hits since the last call: attacker, defender, amount, critical, killed.
    #[func]
    fn poll_damage_events(&self) -> VarArray {
        let mut out = VarArray::new();
        for ev in std::mem::take(&mut self.shared.lock().unwrap().damage) {
            let mut d = VarDictionary::new();
            d.set("attacker", ev.attacker as i64);
            d.set("defender", ev.defender as i64);
            d.set("amount", ev.amount);
            d.set("critical", ev.is_critical);
            d.set("killed", ev.killed);
            out.push(&d.to_variant());
        }
        out
    }

    /// Estimated clock difference to the server in milliseconds (0 until the first move).
    #[func]
    fn clock_offset_ms(&self) -> f64 {
        self.shared.lock().unwrap().clock_offset_us.unwrap_or(0) as f64 / 1000.0
    }

    /// Click-to-move to a Godot position (metres).
    #[func]
    fn move_to(&self, x: f32, z: f32) {
        if let Some(c) = self.conn.as_ref() {
            c.reducers.move_to(x * 100.0, -z * 100.0).ok();
        }
    }

    /// Our character ran into a wall at a Godot position (metres): the server stops the move there.
    #[func]
    fn move_collision(&self, x: f32, z: f32) {
        if let Some(c) = self.conn.as_ref() {
            c.reducers.move_collision(x * 100.0, -z * 100.0).ok();
        }
    }

    #[func]
    fn attack(&self, target: i64) {
        if let Some(c) = self.conn.as_ref() {
            c.reducers.attack(target as u64).ok();
        }
    }

    #[func]
    fn stop(&self) {
        if let Some(c) = self.conn.as_ref() {
            c.reducers.stop().ok();
        }
    }

    #[func]
    fn set_player_name(&self, name: GString) {
        if let Some(c) = self.conn.as_ref() {
            c.reducers.set_name(name.to_string()).ok();
        }
    }

    #[func]
    fn set_loadout(&self, ranged: bool) {
        if let Some(c) = self.conn.as_ref() {
            c.reducers.set_loadout(ranged).ok();
        }
    }
}
