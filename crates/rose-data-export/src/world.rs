//! The world: zone settings (`zones.toml`, one `[[zone]]` per zone), warp gates
//! (`warp-gates.toml`), crafting and refining recipes (`craft-recipes.toml`) and what a new
//! character starts with (`new-characters.toml`). The keys are the property names of the
//! Godot resources (`RoseZoneSettings`, `RoseWarpGate`, `RoseCraftRecipe`,
//! `RoseNewCharacter`) they are exported from; a key left out has its default (0, empty
//! text, none).
//!
//! A zone's spawns, NPCs and named spots stay in its map files, or in `export/zones/` for a
//! zone edited as Godot scenes ([`crate::zone_layouts`]). A warp gate's target zone is
//! written as the zone's id, an item as `weapon/3`.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use anyhow::{bail, Context};
use rose_data::{
    ItemClass, ItemReference, SkyboxId, WarpGateData, WarpGateDatabase, WarpGateId, ZoneId,
    ZoneSettings,
};
use serde::{Deserialize, Serialize};

use crate::npcs::{item_name, parse_item_name};
use crate::Format;

pub const ZONES_FILE: &str = "zones";
pub const WARP_GATES_FILE: &str = "warp-gates";
pub const CRAFT_RECIPES_FILE: &str = "craft-recipes";
pub const NEW_CHARACTERS_FILE: &str = "new-characters";

/// What a number field that can be empty holds for "none".
pub const NONE: i32 = -1;

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

fn none() -> i32 {
    NONE
}

fn is_none(value: &i32) -> bool {
    *value == NONE
}

/// A zone's settings.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ZoneExport {
    /// The number (iROSE's LIST_ZONE.STB row). Saved characters store it.
    pub id: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    #[serde(skip_serializing_if = "is_default")]
    pub description: String,
    /// The ZON map file (the iROSE map files, for a zone not edited as Godot scenes).
    #[serde(skip_serializing_if = "is_default")]
    pub map_file: String,
    #[serde(skip_serializing_if = "is_default")]
    pub start_spot: String,
    #[serde(skip_serializing_if = "is_default")]
    pub revive_spot: String,
    /// The music, a Godot audio file (`res://assets/rose/sound/bgm/plain01_junon.res`) or a
    /// sound file in the client data.
    #[serde(skip_serializing_if = "is_default")]
    pub music_day: String,
    #[serde(skip_serializing_if = "is_default")]
    pub music_night: String,
    /// The sky (LIST_SKY.STB row), -1 = none.
    #[serde(default = "none", skip_serializing_if = "is_none")]
    pub sky: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub minimap: String,
    #[serde(skip_serializing_if = "is_default")]
    pub minimap_start_x: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub minimap_start_y: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub decoration_models: String,
    #[serde(skip_serializing_if = "is_default")]
    pub building_models: String,
    #[serde(skip_serializing_if = "is_default")]
    pub day_cycle: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub morning_time: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub day_time: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub evening_time: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub night_time: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub pvp_state: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub planet: u32,
    /// The footstep sounds (a `footsteps.toml` entry), 0 = none.
    #[serde(skip_serializing_if = "is_default")]
    pub footsteps: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub sector_size: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub vehicle_use_flags: u32,
    /// Metres from the camera at which the game fades out small decorations (0 = never);
    /// left out when it is the usual 250.
    #[serde(default = "default_fade", skip_serializing_if = "is_default_fade")]
    pub decoration_fade_distance: u32,
}

fn default_fade() -> u32 {
    rose_data::DEFAULT_DECORATION_FADE_DISTANCE
}

fn is_default_fade(value: &u32) -> bool {
    *value == rose_data::DEFAULT_DECORATION_FADE_DISTANCE
}

impl ZoneExport {
    pub fn from_settings(zone: &ZoneSettings) -> Self {
        Self {
            id: zone.id.get(),
            name: zone.name.clone(),
            description: zone.description.clone(),
            map_file: zone.map_file.clone(),
            start_spot: zone.start_spot.clone(),
            revive_spot: zone.revive_spot.clone(),
            music_day: zone.music_day.clone(),
            music_night: zone.music_night.clone(),
            sky: zone.skybox_id.map_or(NONE, |s| s.get() as i32),
            minimap: zone.minimap.clone(),
            minimap_start_x: zone.minimap_start_x,
            minimap_start_y: zone.minimap_start_y,
            decoration_models: zone.decoration_models.clone(),
            building_models: zone.building_models.clone(),
            day_cycle: zone.day_cycle,
            morning_time: zone.morning_time,
            day_time: zone.day_time,
            evening_time: zone.evening_time,
            night_time: zone.night_time,
            pvp_state: zone.pvp_state,
            planet: zone.planet,
            footsteps: zone.footstep_type.unwrap_or(0),
            sector_size: zone.sector_size,
            vehicle_use_flags: zone.vehicle_use_flags,
            decoration_fade_distance: zone.decoration_fade_distance,
        }
    }

    pub fn to_settings(&self) -> Result<ZoneSettings, anyhow::Error> {
        if self.map_file.is_empty() {
            bail!("zone {} has no map file", self.id);
        }
        let optional = |value: i32, what: &str| -> Result<Option<u32>, anyhow::Error> {
            match value {
                NONE => Ok(None),
                v if v >= 0 => Ok(Some(v as u32)),
                v => bail!("zone {}: {what} {v} (-1 is none)", self.id),
            }
        };
        Ok(ZoneSettings {
            id: ZoneId::new(self.id).context("zone ids start at 1")?,
            name: self.name.clone(),
            description: self.description.clone(),
            map_file: self.map_file.clone(),
            start_spot: self.start_spot.clone(),
            revive_spot: self.revive_spot.clone(),
            music_day: self.music_day.clone(),
            music_night: self.music_night.clone(),
            skybox_id: optional(self.sky, "sky")?.map(|s| SkyboxId::new(s as u16)),
            minimap: self.minimap.clone(),
            minimap_start_x: self.minimap_start_x,
            minimap_start_y: self.minimap_start_y,
            decoration_models: self.decoration_models.clone(),
            building_models: self.building_models.clone(),
            day_cycle: self.day_cycle,
            morning_time: self.morning_time,
            day_time: self.day_time,
            evening_time: self.evening_time,
            night_time: self.night_time,
            pvp_state: self.pvp_state,
            planet: self.planet,
            footstep_type: (self.footsteps != 0).then_some(self.footsteps),
            sector_size: self.sector_size,
            vehicle_use_flags: self.vehicle_use_flags,
            decoration_fade_distance: self.decoration_fade_distance,
        })
    }
}

/// A warp gate: where walking into it takes you.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WarpGateExport {
    /// The number (iROSE's WARP.STB row). The gates placed in a zone name it.
    pub id: u16,
    /// The zone it takes you to (its id).
    pub target_zone: u16,
    /// The named spot of that zone you arrive at.
    #[serde(skip_serializing_if = "is_default")]
    pub target_spot: String,
}

impl WarpGateExport {
    pub fn from_data(id: WarpGateId, gate: &WarpGateData) -> Self {
        Self { id: id.get(), target_zone: gate.target_zone.get(), target_spot: gate.target_event_object.clone() }
    }

    pub fn to_data(&self) -> Result<WarpGateData, anyhow::Error> {
        Ok(WarpGateData {
            target_zone: ZoneId::new(self.target_zone).with_context(|| format!("warp gate {}: no target zone", self.id))?,
            target_event_object: self.target_spot.clone(),
        })
    }
}

/// A crafting or refining recipe: up to four materials, used one step after another. The
/// first step may take any item of a class instead of one item.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CraftRecipeExport {
    /// The number (iROSE's LIST_PRODUCT.STB row). Items name their recipe by it; rows 1 to 10
    /// refine weapons, 11 to 20 everything else (one row per grade).
    pub id: u16,
    /// For finding it only (what it makes).
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    /// The class any item of which can be the first step's material, when `material_1` is
    /// empty.
    #[serde(skip_serializing_if = "is_default")]
    pub raw_material_class: Option<ItemClass>,
    #[serde(skip_serializing_if = "is_default")]
    pub material_1: String,
    #[serde(skip_serializing_if = "is_default")]
    pub material_1_quantity: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub material_2: String,
    #[serde(skip_serializing_if = "is_default")]
    pub material_2_quantity: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub material_3: String,
    #[serde(skip_serializing_if = "is_default")]
    pub material_3_quantity: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub material_4: String,
    #[serde(skip_serializing_if = "is_default")]
    pub material_4_quantity: u32,
}

/// One step of a recipe: the item (None: any item of the raw material class) and how many.
pub type RecipeStep = (Option<ItemReference>, u32);

impl CraftRecipeExport {
    /// A recipe from its four steps.
    pub fn from_steps(id: u16, name: String, raw_material_class: Option<ItemClass>, steps: &[Option<RecipeStep>; 4]) -> Self {
        let mut recipe = Self { id, name, raw_material_class, ..Default::default() };
        let item = |step: &Option<RecipeStep>| step.and_then(|(item, _)| item).map(item_name).unwrap_or_default();
        let quantity = |step: &Option<RecipeStep>| step.map_or(0, |(_, q)| q);
        (recipe.material_1, recipe.material_1_quantity) = (item(&steps[0]), quantity(&steps[0]));
        (recipe.material_2, recipe.material_2_quantity) = (item(&steps[1]), quantity(&steps[1]));
        (recipe.material_3, recipe.material_3_quantity) = (item(&steps[2]), quantity(&steps[2]));
        (recipe.material_4, recipe.material_4_quantity) = (item(&steps[3]), quantity(&steps[3]));
        recipe
    }

    /// The four steps. A step with no item and no quantity is empty; the first step without an
    /// item takes any item of the raw material class.
    pub fn steps(&self) -> Result<[Option<RecipeStep>; 4], anyhow::Error> {
        let pairs = [
            (&self.material_1, self.material_1_quantity),
            (&self.material_2, self.material_2_quantity),
            (&self.material_3, self.material_3_quantity),
            (&self.material_4, self.material_4_quantity),
        ];
        let mut steps: [Option<RecipeStep>; 4] = Default::default();
        for (i, (item, quantity)) in pairs.into_iter().enumerate() {
            let item = if item.is_empty() { None } else { Some(parse_item_name(item)?) };
            steps[i] = match item {
                Some(item) => Some((Some(item), quantity.max(1))),
                None if i == 0 && quantity > 0 => {
                    if self.raw_material_class.is_none() {
                        bail!("recipe {}: the first material is empty and there is no raw material class", self.id);
                    }
                    Some((None, quantity))
                }
                None if quantity > 0 => bail!("recipe {}: material {} has a quantity but no item", self.id, i + 1),
                None => None,
            };
        }
        if steps[0].is_none() {
            bail!("recipe {} has no first material", self.id);
        }
        Ok(steps)
    }

    pub fn items(&self) -> impl Iterator<Item = &str> {
        [&self.material_1, &self.material_2, &self.material_3, &self.material_4]
            .into_iter()
            .map(|s| s.as_str())
            .filter(|s| !s.is_empty())
    }
}

/// Which body a new character has.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

/// What a new character of one gender starts with.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct NewCharacterExport {
    pub gender: Gender,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub concentration: i32,
    pub charm: i32,
    pub sense: i32,
    #[serde(skip_serializing_if = "is_default")]
    pub face: String,
    #[serde(skip_serializing_if = "is_default")]
    pub head: String,
    #[serde(skip_serializing_if = "is_default")]
    pub body: String,
    #[serde(skip_serializing_if = "is_default")]
    pub back: String,
    #[serde(skip_serializing_if = "is_default")]
    pub hands: String,
    #[serde(skip_serializing_if = "is_default")]
    pub feet: String,
    #[serde(skip_serializing_if = "is_default")]
    pub weapon: String,
    #[serde(skip_serializing_if = "is_default")]
    pub sub_weapon: String,
    /// Equipment in the bag (up to 10).
    #[serde(skip_serializing_if = "is_default")]
    pub bag_equipment: Vec<String>,
    /// Consumables in the bag (up to 5), with how many of each in `bag_consumable_counts`.
    #[serde(skip_serializing_if = "is_default")]
    pub bag_consumables: Vec<String>,
    #[serde(skip_serializing_if = "is_default")]
    pub bag_consumable_counts: Vec<u32>,
    /// Materials in the bag (up to 5), with how many of each in `bag_material_counts`.
    #[serde(skip_serializing_if = "is_default")]
    pub bag_materials: Vec<String>,
    #[serde(skip_serializing_if = "is_default")]
    pub bag_material_counts: Vec<u32>,
}

fn items(list: &[String]) -> Result<Vec<ItemReference>, anyhow::Error> {
    list.iter().filter(|s| !s.is_empty()).map(|s| parse_item_name(s)).collect()
}

fn stacks(list: &[String], counts: &[u32], what: &str) -> Result<Vec<(ItemReference, u32)>, anyhow::Error> {
    let mut out = Vec::new();
    for (i, item) in list.iter().enumerate() {
        if item.is_empty() {
            continue;
        }
        let count = counts.get(i).copied().with_context(|| format!("{what} {}: no count", i + 1))?;
        out.push((parse_item_name(item)?, count));
    }
    Ok(out)
}

impl NewCharacterExport {
    /// The equipped items, in slot order (face, head, body, back, hands, feet, weapon, sub
    /// weapon), leaving out the empty slots.
    pub fn equipped_items(&self) -> Result<Vec<ItemReference>, anyhow::Error> {
        items(&[
            self.face.clone(),
            self.head.clone(),
            self.body.clone(),
            self.back.clone(),
            self.hands.clone(),
            self.feet.clone(),
            self.weapon.clone(),
            self.sub_weapon.clone(),
        ])
    }

    pub fn bag_equipment(&self) -> Result<Vec<ItemReference>, anyhow::Error> {
        if self.bag_equipment.len() > 10 {
            bail!("{:?}: more than 10 bag equipment items", self.gender);
        }
        items(&self.bag_equipment)
    }

    pub fn bag_consumables(&self) -> Result<Vec<(ItemReference, u32)>, anyhow::Error> {
        if self.bag_consumables.len() > 5 {
            bail!("{:?}: more than 5 consumables", self.gender);
        }
        stacks(&self.bag_consumables, &self.bag_consumable_counts, "consumable")
    }

    pub fn bag_materials(&self) -> Result<Vec<(ItemReference, u32)>, anyhow::Error> {
        if self.bag_materials.len() > 5 {
            bail!("{:?}: more than 5 materials", self.gender);
        }
        stacks(&self.bag_materials, &self.bag_material_counts, "material")
    }

    pub fn items(&self) -> impl Iterator<Item = &str> {
        [&self.face, &self.head, &self.body, &self.back, &self.hands, &self.feet, &self.weapon, &self.sub_weapon]
            .into_iter()
            .chain(&self.bag_equipment)
            .chain(&self.bag_consumables)
            .chain(&self.bag_materials)
            .map(|s| s.as_str())
            .filter(|s| !s.is_empty())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ZoneFile {
    #[serde(default)]
    zone: Vec<ZoneExport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WarpGateFile {
    #[serde(default)]
    warp_gate: Vec<WarpGateExport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CraftRecipeFile {
    #[serde(default)]
    craft_recipe: Vec<CraftRecipeExport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct NewCharacterFile {
    #[serde(default)]
    new_character: Vec<NewCharacterExport>,
}

/// Every zone, warp gate, recipe and new character, as exported.
#[derive(Debug, Default)]
pub struct WorldExport {
    pub zones: Vec<ZoneExport>,
    pub warp_gates: Vec<WarpGateExport>,
    pub craft_recipes: Vec<CraftRecipeExport>,
    pub new_characters: Vec<NewCharacterExport>,
}

impl WorldExport {
    /// The zones' settings, by id.
    pub fn zone_settings(&self) -> Result<Vec<ZoneSettings>, anyhow::Error> {
        let mut zones: Vec<ZoneSettings> = self.zones.iter().map(|z| z.to_settings()).collect::<Result<_, _>>()?;
        zones.sort_by_key(|z| z.id.get());
        Ok(zones)
    }

    pub fn warp_gate_database(&self) -> Result<WarpGateDatabase, anyhow::Error> {
        let mut gates = HashMap::new();
        for gate in &self.warp_gates {
            gates.insert(WarpGateId::new(gate.id), gate.to_data()?);
        }
        Ok(WarpGateDatabase::new(gates))
    }

    /// The recipes' steps by id (index = recipe number).
    pub fn recipe_steps(&self) -> Result<Vec<Option<(&CraftRecipeExport, [Option<RecipeStep>; 4])>>, anyhow::Error> {
        let mut out: Vec<Option<(&CraftRecipeExport, [Option<RecipeStep>; 4])>> = Vec::new();
        for recipe in &self.craft_recipes {
            let at = recipe.id as usize;
            if out.len() <= at {
                out.resize_with(at + 1, || None);
            }
            out[at] = Some((recipe, recipe.steps()?));
        }
        Ok(out)
    }

    pub fn new_character(&self, gender: Gender) -> Result<&NewCharacterExport, anyhow::Error> {
        self.new_characters
            .iter()
            .find(|c| c.gender == gender)
            .with_context(|| format!("{NEW_CHARACTERS_FILE}: no {gender:?} new character"))
    }

    /// Ids used twice, warp gates to zones that aren't there, recipes and new characters
    /// that don't read, a gender missing or there twice. (Whether the items are there is
    /// checked by [`WorldExport::check_items`].)
    pub fn check(&self) -> Result<(), anyhow::Error> {
        let mut zones = BTreeSet::new();
        for zone in &self.zones {
            zone.to_settings()?;
            if !zones.insert(zone.id) {
                bail!("{ZONES_FILE}: zone {} is there twice", zone.id);
            }
        }
        let mut gates = BTreeSet::new();
        for gate in &self.warp_gates {
            if gate.id == 0 {
                bail!("warp gate 0: ids start at 1");
            }
            if !gates.insert(gate.id) {
                bail!("{WARP_GATES_FILE}: warp gate {} is there twice", gate.id);
            }
            if !zones.contains(&gate.target_zone) {
                bail!("warp gate {} goes to zone {}, which isn't there", gate.id, gate.target_zone);
            }
        }
        let mut recipes = BTreeSet::new();
        for recipe in &self.craft_recipes {
            if recipe.id == 0 {
                bail!("recipe 0: ids start at 1");
            }
            if !recipes.insert(recipe.id) {
                bail!("{CRAFT_RECIPES_FILE}: recipe {} is there twice", recipe.id);
            }
            recipe.steps()?;
        }
        let mut genders = BTreeMap::new();
        for character in &self.new_characters {
            if genders.insert(character.gender, ()).is_some() {
                bail!("{NEW_CHARACTERS_FILE}: {:?} is there twice", character.gender);
            }
            character.equipped_items()?;
            character.bag_equipment()?;
            character.bag_consumables()?;
            character.bag_materials()?;
        }
        for gender in [Gender::Male, Gender::Female] {
            self.new_character(gender)?;
        }
        Ok(())
    }

    /// Every item a recipe or a new character names must be there.
    pub fn check_items(&self, item_exists: impl Fn(ItemReference) -> bool) -> Result<(), anyhow::Error> {
        for recipe in &self.craft_recipes {
            for item in recipe.items() {
                if !item_exists(parse_item_name(item)?) {
                    bail!("recipe {} names {item}, which isn't there", recipe.id);
                }
            }
        }
        for character in &self.new_characters {
            for item in character.items() {
                if !item_exists(parse_item_name(item)?) {
                    bail!("the {:?} new character starts with {item}, which isn't there", character.gender);
                }
            }
        }
        Ok(())
    }

    /// The export's files: (file name without extension, contents).
    pub fn to_files_as(&self, format: Format) -> Result<Vec<(&'static str, String)>, anyhow::Error> {
        let mut zone = self.zones.clone();
        zone.sort_by_key(|z| z.id);
        let mut warp_gate = self.warp_gates.clone();
        warp_gate.sort_by_key(|g| g.id);
        let mut craft_recipe = self.craft_recipes.clone();
        craft_recipe.sort_by_key(|r| r.id);
        let mut new_character = self.new_characters.clone();
        new_character.sort_by_key(|c| c.gender);
        Ok(vec![
            (ZONES_FILE, format.write(ZONES_FILE, &ZoneFile { zone })?),
            (WARP_GATES_FILE, format.write(WARP_GATES_FILE, &WarpGateFile { warp_gate })?),
            (CRAFT_RECIPES_FILE, format.write(CRAFT_RECIPES_FILE, &CraftRecipeFile { craft_recipe })?),
            (NEW_CHARACTERS_FILE, format.write(NEW_CHARACTERS_FILE, &NewCharacterFile { new_character })?),
        ])
    }

    /// Reads the export's files through `read` (file name without extension -> its text, or
    /// None when it isn't there). None when there is none of them; some missing is an error.
    pub fn from_files_as(
        format: Format,
        mut read: impl FnMut(&str) -> Result<Option<String>, anyhow::Error>,
    ) -> Result<Option<Self>, anyhow::Error> {
        let files = [ZONES_FILE, WARP_GATES_FILE, CRAFT_RECIPES_FILE, NEW_CHARACTERS_FILE];
        let mut texts = Vec::new();
        for name in files {
            texts.push(read(name)?);
        }
        match texts.iter().filter(|t| t.is_some()).count() {
            0 => return Ok(None),
            n if n < files.len() => bail!(
                "the world export has no {}",
                files
                    .iter()
                    .zip(&texts)
                    .filter(|(_, t)| t.is_none())
                    .map(|(n, _)| format!("{n}.{}", format.extension()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            _ => {}
        }
        let text = |i: usize| texts[i].as_deref().unwrap();
        Ok(Some(Self {
            zones: format.parse::<ZoneFile>(ZONES_FILE, text(0))?.zone,
            warp_gates: format.parse::<WarpGateFile>(WARP_GATES_FILE, text(1))?.warp_gate,
            craft_recipes: format.parse::<CraftRecipeFile>(CRAFT_RECIPES_FILE, text(2))?.craft_recipe,
            new_characters: format.parse::<NewCharacterFile>(NEW_CHARACTERS_FILE, text(3))?.new_character,
        }))
    }
}

/// Every difference between two lists of zone settings (by their debug output, so every
/// field counts). Music counts by `sound_stem`: a client file and its imported copy are the
/// same music.
pub fn compare_zone_settings(a: &[ZoneSettings], b: &[ZoneSettings]) -> Vec<String> {
    let by_id = |list: &[ZoneSettings]| {
        list.iter()
            .map(|z| {
                let mut z = z.clone();
                z.music_day = crate::sounds::sound_stem(&z.music_day);
                z.music_night = crate::sounds::sound_stem(&z.music_night);
                (z.id.get(), format!("{z:#?}"))
            })
            .collect::<BTreeMap<_, _>>()
    };
    compare_text("zone", by_id(a), by_id(b))
}

/// Every difference between two warp gate databases.
pub fn compare_warp_gates(a: &WarpGateDatabase, b: &WarpGateDatabase) -> Vec<String> {
    let by_id = |db: &WarpGateDatabase| {
        db.iter()
            .map(|(id, g)| (id.get(), format!("zone {} spot {:?}", g.target_zone.get(), g.target_event_object)))
            .collect::<BTreeMap<_, _>>()
    };
    compare_text("warp gate", by_id(a), by_id(b))
}

/// One line per thing that differs, given each thing's text by number.
pub fn compare_text(what: &str, a: BTreeMap<u16, String>, b: BTreeMap<u16, String>) -> Vec<String> {
    let mut out = Vec::new();
    let ids: BTreeSet<u16> = a.keys().chain(b.keys()).copied().collect();
    for id in ids {
        match (a.get(&id), b.get(&id)) {
            (Some(_), None) => out.push(format!("{what} {id}: missing")),
            (None, Some(_)) => out.push(format!("{what} {id}: extra")),
            (Some(x), Some(y)) if x != y => {
                for (l, r) in x.lines().zip(y.lines()).filter(|(l, r)| l != r) {
                    out.push(format!("{what} {id}: {} -> {}", l.trim(), r.trim()));
                }
                if x.lines().count() != y.lines().count() {
                    out.push(format!("{what} {id}: the fields differ"));
                }
            }
            _ => {}
        }
    }
    out
}
