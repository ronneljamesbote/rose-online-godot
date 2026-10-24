//! Where the art of an item, monster, skill or status effect comes from: its model and its
//! icon, as a path.
//!
//! The export (and so the game) names a Godot resource by its `res://` path: an imported
//! one (`res://assets/rose/3ddata/avatar/list_mbody/12.tscn`, an icon
//! `res://assets/rose/3ddata/control/res/item1/919.tres`) or our own
//! (`res://assets/ours/...`). The iROSE tables name an object of a model list or a sprite of
//! an icon sheet by its number, which the iROSE loaders write as the list's path and the
//! number: `3DDATA/AVATAR/LIST_MBODY.ZSC#12`, `3DDATA/CONTROL/RES/ITEM1.TSI#919`
//! ([`client_part`]). The import saves that object or sprite at the same place
//! ([`art_stem`]), so both name the same art. Empty = none.

/// The model list or icon sheet `list` (a client path) with object or sprite `number`.
pub fn client_part(list: &str, number: u32) -> String {
    format!("{list}#{number}")
}

/// The client list and number of a path written by [`client_part`], or of an imported
/// resource's path (`res://assets/rose/3ddata/avatar/list_mbody/12.tscn` gives
/// `3DDATA/AVATAR/LIST_MBODY` and 12; the caller adds the list's extension).
pub fn client_list(path: &str) -> Option<(String, u32)> {
    if let Some((list, number)) = path.split_once('#') {
        return Some((list.to_string(), number.parse().ok()?));
    }
    let stem = art_stem(path);
    if stem.starts_with("res://") {
        return None;
    }
    let (list, number) = stem.rsplit_once('/')?;
    Some((list.to_ascii_uppercase(), number.parse().ok()?))
}

/// What the two paths of one piece of art have in common: `3DDATA/AVATAR/LIST_MBODY.ZSC#12`
/// and `res://assets/rose/3ddata/avatar/list_mbody/12.tscn` (or the same under
/// `res://assets/ours/`) both give `3ddata/avatar/list_mbody/12`. Any other Godot path keeps
/// its whole path.
pub fn art_stem(path: &str) -> String {
    let path = path.trim();
    if let Some((list, number)) = path.split_once('#') {
        return format!("{}/{}", art_stem(list), number.trim());
    }
    let inner = ["res://assets/rose/", "res://assets/ours/"].iter().find_map(|prefix| path.strip_prefix(prefix));
    if path.starts_with("res://") && inner.is_none() {
        return path.to_string();
    }
    let path = inner.unwrap_or(path).replace('\\', "/").to_ascii_lowercase();
    let path = path.trim_start_matches("./").trim_start_matches('/');
    match path.rfind('.') {
        // `.particles.res` style double extensions are not used for models or icons.
        Some(dot) if !path[dot..].contains('/') => path[..dot].to_string(),
        _ => path.to_string(),
    }
}

/// Whether two art paths name the same art (see [`art_stem`]).
pub fn same_art(a: &str, b: &str) -> bool {
    a.trim().is_empty() && b.trim().is_empty() || art_stem(a) == art_stem(b)
}

/// The icon sheets of iROSE.
pub const ITEM_ICONS: &str = "3DDATA/CONTROL/RES/ITEM1.TSI";
pub const SKILL_ICONS: &str = "3DDATA/CONTROL/RES/SKILLICON.TSI";
pub const STATUS_EFFECT_ICONS: &str = "3DDATA/CONTROL/RES/STATEICON.TSI";
/// The models of items lying on the ground.
pub const FIELD_ITEM_MODELS: &str = "3DDATA/ITEM/LIST_FIELDITEM.ZSC";
/// Monster and NPC models (skeleton, parts and motions per entry).
pub const NPC_MODELS: &str = "3DDATA/NPC/LIST_NPC.CHR";
/// Cart and castle gear parts.
pub const VEHICLE_MODELS: &str = "3DDATA/PAT/LIST_PAT.ZSC";

/// The model lists of worn items, male and female (the same list for both where the item
/// looks the same on everyone). An item's model is the object with its own number.
pub fn worn_model_lists(item_type: crate::ItemType) -> Option<(&'static str, &'static str)> {
    use crate::ItemType;
    Some(match item_type {
        ItemType::Face => ("3DDATA/AVATAR/LIST_FACEIEM.ZSC", "3DDATA/AVATAR/LIST_FACEIEM.ZSC"),
        ItemType::Head => ("3DDATA/AVATAR/LIST_MCAP.ZSC", "3DDATA/AVATAR/LIST_WCAP.ZSC"),
        ItemType::Body => ("3DDATA/AVATAR/LIST_MBODY.ZSC", "3DDATA/AVATAR/LIST_WBODY.ZSC"),
        ItemType::Hands => ("3DDATA/AVATAR/LIST_MARMS.ZSC", "3DDATA/AVATAR/LIST_WARMS.ZSC"),
        ItemType::Feet => ("3DDATA/AVATAR/LIST_MFOOT.ZSC", "3DDATA/AVATAR/LIST_WFOOT.ZSC"),
        ItemType::Back => ("3DDATA/AVATAR/LIST_BACK.ZSC", "3DDATA/AVATAR/LIST_BACK.ZSC"),
        ItemType::Weapon => ("3DDATA/WEAPON/LIST_WEAPON.ZSC", "3DDATA/WEAPON/LIST_WEAPON.ZSC"),
        ItemType::SubWeapon => ("3DDATA/WEAPON/LIST_SUBWPN.ZSC", "3DDATA/WEAPON/LIST_SUBWPN.ZSC"),
        ItemType::Vehicle => (VEHICLE_MODELS, VEHICLE_MODELS),
        _ => return None,
    })
}

/// The LIST_NPC.CHR entry a monster or NPC model names (its motions and, without an imported
/// scene, its skeleton and parts), or None for a model of our own.
pub fn npc_model_entry(model: &str) -> Option<u16> {
    let (list, number) = client_list(model)?;
    (art_stem(&list) == art_stem(NPC_MODELS)).then(|| u16::try_from(number).ok()).flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stems_match() {
        assert_eq!(art_stem("3DDATA/AVATAR/LIST_MBODY.ZSC#12"), "3ddata/avatar/list_mbody/12");
        assert_eq!(art_stem("res://assets/rose/3ddata/avatar/list_mbody/12.tscn"), "3ddata/avatar/list_mbody/12");
        assert_eq!(art_stem("res://assets/ours/3ddata/avatar/list_mbody/12.scn"), "3ddata/avatar/list_mbody/12");
        assert_eq!(art_stem("res://art/sword.tscn"), "res://art/sword.tscn");
        assert!(same_art(&client_part(ITEM_ICONS, 919), "res://assets/rose/3ddata/control/res/item1/919.tres"));
        assert_eq!(client_list("res://assets/rose/3ddata/npc/list_npc/12.tscn"), Some(("3DDATA/NPC/LIST_NPC".into(), 12)));
        assert_eq!(npc_model_entry("res://assets/rose/3ddata/npc/list_npc/12.tscn"), Some(12));
        assert_eq!(npc_model_entry("3DDATA/NPC/LIST_NPC.CHR#7"), Some(7));
        assert_eq!(npc_model_entry("res://art/dragon.tscn"), None);
    }
}
