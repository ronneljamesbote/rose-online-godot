//! Accounts. Players sign up and sign in on the website (../web), which gives the game a
//! short-lived token signed with its own key. SpacetimeDB checks the signature against the
//! website's published keys and gives each account the same identity every time, so this
//! module only has to check who issued the token. Passwords never reach the game server.
//!
//! With no issuer set (local testing, the load-test bots) anyone may connect and gets a
//! test character straight away, as before accounts existed.

use spacetimedb::{Identity, ReducerContext, Table};

use crate::bank::bank;
use crate::{character, game_data, party, player, require_admin, spawn_player_entity, START_POSITION, START_ZONE};

/// Faces and hair styles the character creation screen offers (rose-offline-client's
/// CREATE_CHARACTER_FACE_LIST and CREATE_CHARACTER_HAIR_LIST).
pub const FACES: [u8; 7] = [1, 8, 15, 22, 29, 36, 43];
pub const HAIRS: [u8; 5] = [0, 5, 10, 15, 20];
const NAME_MIN: usize = 3;
const NAME_MAX: usize = 16;
const AUDIENCE: &str = "rose";

#[spacetimedb::table(accessor = auth_config)]
pub struct AuthConfig {
    #[primary_key]
    pub id: u8,
    /// The website's address as it signs tokens (its ROSE_AUTH_ISSUER). Empty: no accounts.
    pub issuer: String,
}

pub fn issuer(ctx: &ReducerContext) -> String {
    ctx.db.auth_config().id().find(0).map(|c| c.issuer).unwrap_or_default()
}

pub fn accounts_required(ctx: &ReducerContext) -> bool {
    !issuer(ctx).is_empty()
}

/// Refuse connections that didn't sign in through the website.
pub fn check_connection(ctx: &ReducerContext) -> Result<(), String> {
    let issuer = issuer(ctx);
    if issuer.is_empty() {
        return Ok(());
    }
    let auth = ctx.sender_auth();
    let Some(jwt) = auth.jwt() else {
        return Err("sign in with your account".into());
    };
    if jwt.issuer() != issuer || !jwt.audience().iter().any(|a| a == AUDIENCE) {
        return Err("sign in with your account".into());
    }
    Ok(())
}

/// Admin: trust tokens from this website (its ROSE_AUTH_ISSUER), or "" to let anyone in.
#[spacetimedb::reducer]
pub fn set_auth_issuer(ctx: &ReducerContext, issuer: String) -> Result<(), String> {
    require_admin(ctx)?;
    let issuer = issuer.trim().trim_end_matches('/').to_string();
    let row = AuthConfig { id: 0, issuer };
    if ctx.db.auth_config().id().find(0).is_some() {
        ctx.db.auth_config().id().update(row);
    } else {
        ctx.db.auth_config().insert(row);
    }
    Ok(())
}

/// Why a character name can't be used, if it can't: 3 to 16 letters and digits, starting
/// with a letter, and not taken (ignoring case).
pub fn name_problem(ctx: &ReducerContext, name: &str, except: Option<Identity>) -> Option<String> {
    let length = name.chars().count();
    if !(NAME_MIN..=NAME_MAX).contains(&length) {
        return Some(format!("names are {NAME_MIN} to {NAME_MAX} characters"));
    }
    if !name.chars().all(|c| c.is_ascii_alphanumeric()) || !name.starts_with(|c: char| c.is_ascii_alphabetic()) {
        return Some("names use letters and digits and start with a letter".into());
    }
    if ctx.db.player().iter().any(|p| Some(p.identity) != except && p.name.eq_ignore_ascii_case(name)) {
        return Some(format!("{name} is taken"));
    }
    None
}

/// Make this account's character. gender 0 male, 1 female; face and hair from FACES and HAIRS.
#[spacetimedb::reducer]
pub fn create_character(ctx: &ReducerContext, name: String, gender: u8, face: u8, hair: u8) -> Result<(), String> {
    check_connection(ctx)?;
    if ctx.db.player().identity().find(ctx.sender()).is_some() {
        return Err("this account already has a character".into());
    }
    let name = name.trim().to_string();
    if let Some(problem) = name_problem(ctx, &name, None) {
        return Err(problem);
    }
    if gender > 1 || !FACES.contains(&face) || !HAIRS.contains(&hair) {
        return Err("pick a face and hair style from the list".into());
    }
    let game = game_data::game(ctx)?;
    let (x, y) = START_POSITION;
    let mut player = character::new_player(&game, ctx.sender(), name, gender, (START_ZONE, x, y));
    player.face = face;
    player.hair = hair;
    player.online = true;
    player.connection = ctx.connection_id();
    spawn_player_entity(ctx, &game, &mut player);
    ctx.db.player().insert(player);
    Ok(())
}

/// Admin: give an existing character (one made before accounts, say) to an account, by the
/// account's identity. Both must be offline; the account must have no character yet.
#[spacetimedb::reducer]
pub fn assign_character(ctx: &ReducerContext, name: String, account: Identity) -> Result<(), String> {
    require_admin(ctx)?;
    if ctx.db.player().identity().find(account).is_some() {
        return Err("that account already has a character".into());
    }
    let mut p = ctx.db.player().iter().find(|p| p.name == name).ok_or("no such character")?;
    if p.online || p.entity_id.is_some() {
        return Err("the character is online".into());
    }
    let old = p.identity;
    party::forget_member(ctx, old);
    ctx.db.player().identity().delete(old);
    p.identity = account;
    p.connection = None;
    ctx.db.player().insert(p);
    if let Some(mut b) = ctx.db.bank().identity().find(old) {
        ctx.db.bank().identity().delete(old);
        b.identity = account;
        ctx.db.bank().insert(b);
    }
    Ok(())
}
