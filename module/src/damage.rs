//! Physical damage, ported from rose-offline's
//! `rose-game-irose/src/data/ability_values.rs` (calculate_damage,
//! calculate_damage_success_rate, calculate_attack_damage_physical).
//! Only the player-vs-monster branches are ported; PvP is out of scope.

use crate::Stats;
use rand::Rng;

pub struct Damage {
    pub amount: i32,
    pub is_critical: bool,
}

fn success_rate(rng: &mut impl Rng, a: &Stats, d: &Stats) -> i32 {
    let value = (a.level + 10) - (d.level as f32 * 1.1) as i32 + rng.gen_range(1..=50);
    if value <= 0 {
        0
    } else {
        (value as f32
            * ((a.hit as f32 * 1.1 - d.avoid as f32 * 0.93
                + rng.gen_range(1..=60) as f32
                + 5.0
                + a.level as f32 * 0.2)
                / 80.0)) as i32
    }
}

pub fn roll(rng: &mut impl Rng, a: &Stats, d: &Stats) -> Damage {
    let rate = success_rate(rng, a, d);
    if rate < 20 && (rng.gen_range(1..=100) + (0.6 * (a.level - d.level) as f32) as i32) < 94 {
        return Damage { amount: 0, is_critical: false };
    }
    let crit_roll = 16 * (3 * rng.gen_range(1..=100) + a.level + 30) / (a.critical + 70);
    let ap = a.attack_power as f32;
    let def = d.defence as f32;
    if crit_roll < 20 {
        let damage = ap
            * (rate as f32 * 0.05 + 29.0)
            * ((a.attack_power - d.defence + 230) as f32 / (100.0 * (def + d.avoid as f32 * 0.3 + 5.0)));
        Damage { amount: damage.max(10.0).min(2047.0) as i32, is_critical: true }
    } else {
        let damage = ap
            * (rate as f32 * 0.03 + 26.0)
            * ((a.attack_power - d.defence + 250) as f32 / (145.0 * (def + d.avoid as f32 * 0.4 + 5.0)));
        Damage { amount: damage.max(5.0).min(2047.0) as i32, is_critical: false }
    }
}
