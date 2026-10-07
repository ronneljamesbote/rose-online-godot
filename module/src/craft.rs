//! Item crafting with the craft skills (Sword Craft, Armor Craft, Gem Cutting, ...). rose-offline
//! has no crafting, so the steps and formulas follow the iROSE game server
//! (Recv_cli_CREATE_ITEM_REQ): each material is a step with a success roll, a failed step
//! keeps the materials used so far, and a made item's durability and bonus option depend
//! on how well the steps went. Crafting gives experience either way.

use rand::Rng;
use rose_data::{Item, ItemReference, ItemType, SkillType};
use rose_game_common::components::{DroppedItem, Inventory, ItemSlot, SkillSlot};
use rose_game_data::GameData;
use spacetimedb::{ReducerContext, SpacetimeType, Table};

use crate::{character, game_data::game, items, my_player, now_us, player, position, skills, xp_event, XpEvent};

/// The world production rate (iROSE economy default).
const WORLD_PRODUCTION: f32 = 100.0;
/// Items whose rare type is this always come with a gem socket.
const RARE_TYPE_SOCKETED: u32 = 4;

#[derive(SpacetimeType, Clone, Copy, Debug)]
pub struct CraftSlot {
    pub page: u8,
    pub index: u16,
}

fn item_type(number: u8) -> Result<ItemType, String> {
    rose_data_irose::decode_item_type(number as usize).ok_or_else(|| "no such item type".into())
}

/// Whether this bag item can be the material of this recipe step.
fn fits_material(game: &GameData, recipe: &rose_game_data::CraftRecipe, step: usize, item: &Item) -> bool {
    let reference = item.get_item_reference();
    game.items.get_base_item(reference).is_some_and(|d| recipe.accepts(step, reference, d.class))
}

/// Make `item_number` of `item_type` (iROSE type number) with the craft skill in this skill
/// slot, using the bag items in `materials` for the recipe's steps in order.
#[spacetimedb::reducer]
pub fn craft_item(
    ctx: &ReducerContext,
    skill_page: u8,
    skill_index: u16,
    item_type_number: u8,
    item_number: u16,
    materials: Vec<CraftSlot>,
) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, id) = my_player(ctx)?;
    if !crate::is_alive(ctx, id) {
        return Err("dead".into());
    }
    let skill_id = p.skill_list().get_skill(SkillSlot(skill_page as usize, skill_index as usize)).ok_or("no skill there")?;
    let skill = game.skills.get_skill(skill_id).ok_or("unknown skill")?;
    if !matches!(skill.skill_type, SkillType::CreateWindow) {
        return Err(format!("{} doesn't craft", skill.name));
    }
    let target = ItemReference::new(item_type(item_type_number)?, item_number as usize);
    let data = game.items.get_base_item(target).ok_or("unknown item")?;
    if data.craft_skill_type == 0 || data.craft_skill_type != skill.item_make_number {
        return Err(format!("{} can't make {}", skill.name, data.name));
    }
    if skill.level < data.craft_skill_level {
        return Err(format!("{} needs {} level {}", data.name, skill.name, data.craft_skill_level));
    }
    let recipe = game.craft_recipes.get(data.craft_material as usize).and_then(|r| r.clone()).ok_or("there is no recipe for that")?;

    // Check every material before using any.
    let mut inventory: Inventory = p.inventory();
    let mut steps = Vec::new();
    for (step, material) in recipe.materials.iter().enumerate() {
        let Some(material) = material else { continue };
        let slot = materials.get(step).ok_or("put in every material")?;
        let slot: ItemSlot = items::inventory_slot(slot.page, slot.index)?;
        let item = inventory.get_item(slot).ok_or("put in every material")?;
        if !fits_material(&game, &recipe, step, item) {
            return Err("that isn't the right material".into());
        }
        let quantity = if matches!(item, Item::Stackable(_)) { material.quantity } else { 1 };
        if item.get_quantity() < quantity {
            return Err(format!("needs {} of {}", quantity, items::item_name(&game, item)));
        }
        let quality = game.items.get_base_item(item.get_item_reference()).map_or(0, |d| d.quality);
        steps.push((slot, quantity, quality as f32));
    }
    if steps.iter().enumerate().any(|(i, a)| steps[..i].iter().any(|b| b.0 == a.0)) {
        return Err("use a different bag slot for each material".into());
    }

    let t = now_us(ctx);
    skills::check_can_use(ctx, &game, &p, id, skill, t)?;
    skills::pay_costs(ctx, &game, id, skill, t);
    p = ctx.db.player().identity().find(p.identity).ok_or("no player")?;

    let av = items::player_ability_values(ctx, &game, &p);
    let concentration = av.get_concentration() as f32;
    let sense = av.get_sense() as f32;
    let level = p.level as f32;
    let skill_level = skill.level as f32;
    let difficulty = data.craft_difficulty as f32;
    let quality = data.quality as f32;
    let count = steps.len() as f32;
    let mut rng = ctx.rng();

    let mut progress = [0f32; 4];
    let mut material_quality = 0f32;
    let mut plus = 0f32;
    let mut failed_at = None;
    for (step, &(slot, quantity, mat_quality)) in steps.iter().enumerate() {
        let roll = rng.gen_range(0..100) as f32;
        let needed;
        match step {
            0 => {
                material_quality = mat_quality;
                needed = (difficulty + 35.0) * (quality + 15.0) / 16.0;
                progress[0] = material_quality * (roll + 71.0) * (0.5 * concentration + difficulty / 2.0 + 530.0) * WORLD_PRODUCTION / 800000.0;
                plus = ((progress[0] - needed) * 30.0 / (progress[0] + quality)).trunc();
            }
            1 => {
                needed = (difficulty + 15.0) * (quality + 140.0) / (count + 3.0) / 4.0;
                progress[1] = (material_quality + difficulty / 2.0)
                    * (roll + 96.0)
                    * (0.5 * concentration + skill_level * 6.0 + material_quality * 2.0 + 770.0)
                    / (count + 7.0)
                    / 1600.0;
                plus += ((progress[1] - needed) * 20.0 / progress[1].max(1.0)).trunc();
            }
            2 => {
                needed = (difficulty + 90.0) * (quality + 30.0) / (count + 3.0) / 4.0;
                progress[2] = (progress[0] / 6.0 + quality)
                    * (roll + 81.0)
                    * (0.3 * concentration + skill_level * 5.0 + material_quality * 2.0 + 600.0)
                    / (count + 7.0)
                    / 2000.0;
                plus += ((progress[2] - needed) * 10.0 / progress[2].max(1.0)).trunc();
            }
            _ => {
                needed = (difficulty + 40.0) * (quality + 60.0) / (count + 2.0) / 6.0;
                progress[3] = (progress[0] + progress[1] + 40.0) * (roll + 51.0) / 200.0;
                plus += ((progress[3] - needed) * 10.0 / progress[3].max(1.0)).trunc();
            }
        }
        // Each step uses its material, pass or fail.
        inventory.try_take_quantity(slot, quantity);
        if progress[step] < needed {
            failed_at = Some(step);
            break;
        }
    }
    p.set_inventory(&inventory);
    ctx.db.player().identity().update(p.clone());

    let xp = match failed_at {
        Some(_) => 1.0 + ((level + 50.0) * progress[0] * (count + 4.0) / 1300.0).trunc(),
        None => 1.0 + ((level + 35.0) * (progress[0] + level) * (count + 4.0) * (difficulty + 20.0) / 23000.0).trunc(),
    };

    if let Some(step) = failed_at {
        items::notify(ctx, p.identity, format!("Crafting {} failed at step {} of {}", data.name, step + 1, steps.len()));
    } else {
        let mut made = Item::from_item_data(data, 1).ok_or("unknown item")?;
        if let Item::Equipment(ref mut e) = made {
            e.is_crafted = true;
            let durability = (data.durability as f32 + 15.0) * (plus * 1.3 + skill_level * 2.0 + 120.0)
                / (rng.gen_range(0..100) as f32 + 81.0)
                * 0.6;
            e.durability = durability.clamp(0.0, 100.0) as u8;
            if data.rare_type == RARE_TYPE_SOCKETED {
                e.has_socket = true;
            } else {
                let r = 1.0 + rng.gen_range(0..100) as f32;
                let option = (((sense + 220.0 - level / 2.0) * (plus + 20.0) * 0.4 + difficulty * 35.0 - 1600.0 - r) / (r + 17.0)
                    - 85.0) as i32;
                if option > 0 {
                    e.is_appraised = true;
                    let modulo = (((quality + 12.0) * 3.2) as i32).clamp(1, 300);
                    e.gem = (option % modulo) as u16;
                }
            }
        }
        let name = items::item_name(&game, &made);
        let mut inventory = p.inventory();
        match inventory.try_add_item(made) {
            Ok(_) => {
                p.set_inventory(&inventory);
                ctx.db.player().identity().update(p.clone());
            }
            Err(made) => {
                // A full bag: it lands at your feet.
                let at = position(ctx, id, t).unwrap_or((p.last_x, p.last_y));
                items::drop_on_ground(ctx, p.zone_id, at, &DroppedItem::Item(made), Some(p.identity));
            }
        }
        items::notify(ctx, p.identity, format!("Crafted {name}"));
    }
    character::reward_xp(ctx, &game, p.identity, xp as u64);
    let level = ctx.db.player().identity().find(p.identity).map_or(0, |p| p.level);
    ctx.db.xp_event().insert(XpEvent { identity: p.identity, xp: xp as u64, level });
    Ok(())
}
