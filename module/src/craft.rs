//! Item crafting with the craft skills (Sword Craft, Armor Craft, Gem Cutting, ...). rose-offline
//! has no crafting, so the steps and formulas follow the iROSE game server
//! (Recv_cli_CREATE_ITEM_REQ): each material is a step with a success roll, a failed step
//! keeps the materials used so far, and a made item's durability and bonus option depend
//! on how well the steps went. Crafting gives experience either way.
//!
//! Also the other iROSE item services (Proc_CRAFT_GEMMING_REQ, _BREAKUP_REQ, _UPGRADE_REQ):
//! setting a gem into a socket, disassembling items into materials (or taking a gem back
//! out) and refining equipment to a higher grade, either with a Dealer skill (paid in MP) or
//! at an NPC (paid in Zuly).

use rand::Rng;
use rose_data::{EquipmentItem, Item, ItemClass, ItemReference, ItemType, SkillType, StackableItem};
use rose_game_common::components::{DroppedItem, Inventory, ItemSlot, Money, SkillSlot};
use rose_game_data::GameData;
use spacetimedb::{ReducerContext, SpacetimeType, Table};

use crate::{character, combat, distance, game_data::game, items, my_player, now_us, npc, player, position, skills, xp_event, Player, XpEvent};

/// The world production rate (iROSE economy default).
const WORLD_PRODUCTION: f32 = 100.0;
/// Items of rare type 2 always come with a gem socket, rare type 1 sometimes (rose-offline's
/// reading of the iROSE item data; rose-next's own data numbers these differently).
pub const RARE_TYPE_SOCKETED: u32 = 2;
pub const RARE_TYPE_MAYBE_SOCKETED: u32 = 1;
/// Gem numbers above this are set gems; lower ones are an item's bonus option.
const FIRST_GEM: u16 = 300;
/// The skill item_make_number of Item Disassembly and Item Refining.
const MAKE_DISASSEMBLE: u32 = 41;
const MAKE_REFINE: u32 = 42;
const MAX_GRADE: u8 = 9;
const MAX_DURABILITY: i32 = 120;
/// How close an NPC must be for its services.
const NPC_RANGE_CM: f32 = 1500.0;

#[derive(SpacetimeType, Clone, Copy, Debug)]
pub struct CraftSlot {
    pub page: u8,
    pub index: u16,
}

/// Who does the work: our own craft skill in this skill slot (costs MP) or an NPC (costs Zuly).
#[derive(SpacetimeType, Clone, Copy, Debug)]
pub enum CraftTool {
    Skill(CraftSlot),
    Npc(u64),
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
            if data.rare_type == RARE_TYPE_SOCKETED
                || (data.rare_type == RARE_TYPE_MAYBE_SOCKETED && data.quality as i32 + 60 > rng.gen_range(0..400))
            {
                e.has_socket = true;
                e.is_appraised = true;
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

/// Set a gem from the bag into the socket of an equipped item (0 face .. 10 earring, as
/// unequip_item). The gem's stats count while the item is worn.
#[spacetimedb::reducer]
pub fn insert_gem(ctx: &ReducerContext, equipment_slot: u8, page: u8, index: u16) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, _) = my_player(ctx)?;
    let slot = items::inventory_slot(page, index)?;
    let mut inventory = p.inventory();
    let gem = inventory.get_item(slot).ok_or("nothing there")?;
    let gem_number = gem.get_item_number() as u16;
    let is_jewel = gem.get_item_type() == ItemType::Gem
        && game.items.get_base_item(gem.get_item_reference()).is_some_and(|d| d.class == ItemClass::Jewel);
    if !is_jewel || gem_number <= FIRST_GEM {
        return Err("that isn't a gem".into());
    }
    let mut equipment = p.equipment();
    let item = equipment
        .get_equipment_slot_mut(items::equipment_index(equipment_slot)?)
        .as_mut()
        .ok_or("nothing equipped there")?;
    if !item.has_socket {
        return Err("that item has no gem socket".into());
    }
    if item.gem > FIRST_GEM {
        return Err("that socket already holds a gem".into());
    }
    inventory.try_take_quantity(slot, 1).ok_or("nothing there")?;
    item.gem = gem_number;
    item.is_appraised = true;
    let name = game.items.get_base_item(ItemReference::gem(gem_number as usize)).map_or("the gem".into(), |d| d.name.to_string());
    p.set_inventory(&inventory);
    p.set_equipment(&equipment);
    ctx.db.player().identity().update(p.clone());
    character::refresh_player(ctx, &game, &p, false);
    items::notify(ctx, p.identity, format!("Set {name}"));
    Ok(())
}

/// Check the tool can do this work (`make` is the skill's item_make_number) and charge its
/// cost: `mp` with a skill, `zuly` at an NPC.
fn pay_for_work(ctx: &ReducerContext, game: &GameData, p: &mut Player, id: u64, tool: CraftTool, make: u32, mp: i32, zuly: i64) -> Result<(), String> {
    match tool {
        CraftTool::Skill(slot) => {
            let skill_id = p
                .skill_list()
                .get_skill(SkillSlot(slot.page as usize, slot.index as usize))
                .ok_or("no skill there")?;
            let skill = game.skills.get_skill(skill_id).ok_or("unknown skill")?;
            if !matches!(skill.skill_type, SkillType::CreateWindow) || skill.item_make_number != make {
                return Err(format!("{} can't do that", skill.name));
            }
            let mut c = ctx.db.combat().entity_id().find(id).ok_or("no combat")?;
            if c.mp < mp {
                return Err(format!("needs {mp} MP"));
            }
            c.mp -= mp;
            ctx.db.combat().entity_id().update(c);
        }
        CraftTool::Npc(npc_entity_id) => {
            let n = ctx.db.npc().entity_id().find(npc_entity_id).ok_or("no such NPC")?;
            let t = now_us(ctx);
            let (me, there) = (position(ctx, id, t).ok_or("no position")?, position(ctx, npc_entity_id, t).ok_or("no position")?);
            if n.zone_id != p.zone_id || distance(me, there) > NPC_RANGE_CM {
                return Err("you are too far away".into());
            }
            let mut inventory = p.inventory();
            inventory.try_take_money(Money(zuly)).map_err(|_| format!("needs {zuly} Zuly"))?;
            p.set_inventory(&inventory);
        }
    }
    Ok(())
}

fn quality(game: &GameData, item: ItemReference) -> i32 {
    game.items.get_base_item(item).map_or(0, |d| d.quality as i32)
}

/// Put an item in the bag, or at our feet when the bag is full.
fn give(ctx: &ReducerContext, p: &mut Player, id: u64, item: Item) {
    let mut inventory = p.inventory();
    match inventory.try_add_item(item) {
        Ok(_) => p.set_inventory(&inventory),
        Err(item) => {
            let at = position(ctx, id, now_us(ctx)).unwrap_or((p.last_x, p.last_y));
            items::drop_on_ground(ctx, p.zone_id, at, &DroppedItem::Item(item), Some(p.identity));
        }
    }
}

/// Disassemble a bag item into some of the materials it is made of. An item with a set gem
/// gives the gem back instead, which can lose a grade or break.
#[spacetimedb::reducer]
pub fn disassemble_item(ctx: &ReducerContext, tool: CraftTool, page: u8, index: u16) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, id) = my_player(ctx)?;
    if !crate::is_alive(ctx, id) {
        return Err("dead".into());
    }
    let slot = items::inventory_slot(page, index)?;
    let item = p.inventory().get_item(slot).cloned().ok_or("nothing there")?;
    let reference = item.get_item_reference();
    let data = game.items.get_base_item(reference).ok_or("unknown item")?;
    let item_quality = data.quality as i32;
    let mut rng = ctx.rng();

    if let Item::Equipment(equipment) = &item {
        if equipment.has_socket && equipment.gem > FIRST_GEM {
            let gem_quality = quality(&game, ItemReference::gem(equipment.gem as usize));
            pay_for_work(ctx, &game, &mut p, id, tool, MAKE_DISASSEMBLE, item_quality / 2 + gem_quality, (item_quality * 5 + 50) as i64)?;
            let mut inventory = p.inventory();
            let Some(Item::Equipment(target)) = inventory.get_item_slot_mut(slot).and_then(|s| s.as_mut()) else {
                return Err("nothing there".into());
            };
            let mut gem = target.gem;
            target.gem = 0;
            p.set_inventory(&inventory);
            let text;
            if rng.gen_range(0..100) + 1 + gem_quality / 6 <= 30 {
                if gem_quality <= 35 {
                    ctx.db.player().identity().update(p.clone());
                    items::notify(ctx, p.identity, "The gem broke");
                    return Ok(());
                }
                gem -= 1;
                text = "The gem came out a grade lower";
            } else {
                text = "The gem came out";
            }
            if let Some(gem_item) = StackableItem::new(ItemReference::gem(gem as usize), 1) {
                give(ctx, &mut p, id, Item::Stackable(gem_item));
            }
            ctx.db.player().identity().update(p.clone());
            items::notify(ctx, p.identity, text);
            return Ok(());
        }
    }

    let recipe = (data.craft_material != 0)
        .then(|| game.craft_recipes.get(data.craft_material as usize).cloned().flatten())
        .flatten()
        .ok_or_else(|| format!("{} can't be taken apart", data.name))?;
    pay_for_work(ctx, &game, &mut p, id, tool, MAKE_DISASSEMBLE, item_quality + 30, (item_quality * 10 + 20) as i64)?;
    let (durability, life) = match &item {
        Item::Equipment(e) => (e.durability as i32, e.life as i32),
        Item::Stackable(_) => (0, 100),
    };
    let mut inventory = p.inventory();
    inventory.try_take_quantity(slot, 1).ok_or("nothing there")?;
    p.set_inventory(&inventory);
    let mut got = Vec::new();
    for (step, material) in recipe.materials.iter().enumerate() {
        let Some(material) = material else { break };
        let output = match material.item {
            Some(item) => item,
            None if step == 0 => {
                // Raw material classes 421.. give that class's material of about the item's quality.
                let grade = ((item_quality - 20) / 12).clamp(1, 10) as usize;
                ItemReference::new(ItemType::Material, (recipe.raw_material_class as usize).saturating_sub(421) * 10 + grade)
            }
            None => break,
        };
        let quantity = material.quantity as i64 * (111 + rng.gen_range(0..40)) * (durability / 2 + life / 10 + 100) as i64 / 60000;
        if quantity <= 0 {
            continue;
        }
        let Some(made) = Item::from_item_data(game.items.get_base_item(output).ok_or("unknown material")?, quantity as u32) else { continue };
        got.push(items::item_name(&game, &made));
        give(ctx, &mut p, id, made);
    }
    ctx.db.player().identity().update(p.clone());
    items::notify(
        ctx,
        p.identity,
        if got.is_empty() { format!("{} fell apart into nothing useful", data.name) } else { format!("Got {}", got.join(", ")) },
    );
    Ok(())
}

/// The refining recipe for raising this item one grade: LIST_PRODUCT rows 1-10 for weapons,
/// 11-20 for everything else.
pub fn refine_recipe_index(item: &EquipmentItem) -> usize {
    let base = if item.item.item_type == ItemType::Weapon { 1 } else { 11 };
    base + item.grade as usize
}

/// Refine a bag equipment item to the next grade with the recipe's materials (in `materials`,
/// one bag slot per step). Failing lowers the grade; either way durability changes.
#[spacetimedb::reducer]
pub fn refine_item(ctx: &ReducerContext, tool: CraftTool, page: u8, index: u16, materials: Vec<CraftSlot>) -> Result<(), String> {
    let game = game(ctx)?;
    let (mut p, id) = my_player(ctx)?;
    if !crate::is_alive(ctx, id) {
        return Err("dead".into());
    }
    let slot = items::inventory_slot(page, index)?;
    let mut inventory = p.inventory();
    let Some(Item::Equipment(target)) = inventory.get_item(slot).cloned() else {
        return Err("only equipment can be refined".into());
    };
    if target.grade >= MAX_GRADE {
        return Err("that is already the highest grade".into());
    }
    let recipe = game.craft_recipes.get(refine_recipe_index(&target)).cloned().flatten().ok_or("there is no recipe for that")?;
    let item_quality = quality(&game, target.item);

    let mut steps = Vec::new();
    for (step, material) in recipe.materials.iter().enumerate() {
        let Some(material) = material else { continue };
        let slot = materials.get(step).ok_or("put in every material")?;
        let slot: ItemSlot = items::inventory_slot(slot.page, slot.index)?;
        let item = inventory.get_item(slot).ok_or("put in every material")?;
        if !fits_material(&game, &recipe, step, item) {
            return Err("that isn't the right material".into());
        }
        if item.get_quantity() < material.quantity {
            return Err(format!("needs {} of {}", material.quantity, items::item_name(&game, item)));
        }
        steps.push((slot, material.quantity, quality(&game, item.get_item_reference())));
    }
    if steps.iter().enumerate().any(|(i, a)| steps[..i].iter().any(|b| b.0 == a.0)) {
        return Err("use a different bag slot for each material".into());
    }
    let grade = target.grade as i64;
    let mp = ((grade + 4) as f32 * (item_quality + 20) as f32 * 0.9) as i32;
    let zuly = (grade * (grade + 1) * item_quality as i64 * (item_quality as i64 + 20)) as f32 * 0.2;
    pay_for_work(ctx, &game, &mut p, id, tool, MAKE_REFINE, mp, zuly as i64)?;
    inventory.money = p.inventory().money;
    for &(slot, quantity, _) in &steps {
        inventory.try_take_quantity(slot, quantity);
    }

    let mut rng = ctx.rng();
    let material_quality = steps.first().map_or(1, |s| s.2).max(1) as i64;
    let durability = target.durability as i64;
    let success = (grade + 2) * (grade + 3) * (grade * 5 + item_quality as i64 * 3 + 250) * (61 + rng.gen_range(0..100)) * 320
        / (material_quality * (durability + 180) * (WORLD_PRODUCTION as i64 + 10))
        + 200;
    let change = (200 + (material_quality + 5) * 10 + (1 + rng.gen_range(0..100)) * 3 - (grade + 6) * 80) / 40;
    let Some(Item::Equipment(item)) = inventory.get_item_slot_mut(slot).and_then(|s| s.as_mut()) else {
        return Err("nothing there".into());
    };
    let name = game.items.get_base_item(item.item).map_or(String::new(), |d| d.name.to_string());
    let text;
    if success < 1000 {
        if change > 0 {
            item.durability = (item.durability as i64 + change).min(MAX_DURABILITY as i64) as u8;
        }
        item.grade += 1;
        text = format!("Refined {name} to grade {}", item.grade);
    } else {
        item.durability = (item.durability as i64 + change).clamp(0, MAX_DURABILITY as i64) as u8;
        let lost = ((grade + 1) * (grade + 10) / (41 + rng.gen_range(0..100))) as u8;
        item.grade = item.grade.saturating_sub(lost);
        text = if lost > 0 { format!("Refining failed: {name} fell to grade {}", item.grade) } else { format!("Refining {name} failed") };
    }
    p.set_inventory(&inventory);
    ctx.db.player().identity().update(p.clone());
    items::notify(ctx, p.identity, text);
    Ok(())
}
