//! Every game database the server needs, loaded from the data exported from Godot
//! (`export/`) where it has them, else from the iROSE client files. Ported from
//! rose-offline-server's `irose/data` (game data and character creator) without Bevy, so the
//! same code runs natively and inside the SpacetimeDB module.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use anyhow::Context;
use rose_data::{
    AiDatabase, CharacterMotionDatabase, DataDecoder, ItemClass, ItemDatabase, ItemReference, ItemType,
    JobClassDatabase, MotionFileData, NpcDatabase, NpcDatabaseOptions, QuestDatabase, SkillDatabase,
    NpcConversationId, NpcId, SkillId, SkillIds, StatusEffectDatabase, StringDatabase, WarpGateData, WarpGateDatabase, ZoneData,
    ZoneDatabase, ZoneId, ZoneMonsterSpawnPoint, ZoneNpcSpawn, ZoneSettings,
};
use rose_data_irose::{
    decode_item_base1000, decode_item_class, encode_item_class, encode_item_type, get_ai_database,
    get_character_motion_database_with, get_data_decoder, get_item_database, get_job_class_database,
    get_npc_action_map, get_npc_conversations, get_npc_database, get_npc_database_with, get_npc_motions_with,
    get_quest_database, get_skill_database, zmo_motion_file_data,
    get_status_effect_database, get_string_database, get_warp_gate_database, get_zone_database_from_settings,
    get_zone_settings,
};
use rose_data_export::world::{
    CraftRecipeExport, Gender, NewCharacterExport, RecipeStep, WarpGateExport, ZoneExport,
};
use rose_data_export::{
    npcs::{item_name, DROP_ROW_COLUMNS},
    Format, IroseCodes, ItemsExport, MotionTimings, NpcModels, NpcsExport, QuestsExport, SkillsExport, WorldExport,
    ZoneLayout,
};
use rose_file_readers::{stb_column, StbFile, VfsFile, VfsPathBuf, VirtualFilesystem};
use glam::Vec3;
use rose_game_common::{
    components::BasicStats,
    data::{AbilityValueCalculator, DropTable},
};
use rose_game_irose::data::{get_ability_value_calculator, get_drop_table, get_drop_table_from_rows};

pub struct GameData {
    pub character_creator: CharacterCreator,
    pub ability_value_calculator: Box<dyn AbilityValueCalculator + Send + Sync>,
    pub data_decoder: Box<dyn DataDecoder + Send + Sync>,
    pub drop_table: Box<dyn DropTable + Send + Sync>,
    pub ai: Arc<AiDatabase>,
    pub items: Arc<ItemDatabase>,
    pub job_class: Arc<JobClassDatabase>,
    pub motions: Arc<CharacterMotionDatabase>,
    pub npcs: Arc<NpcDatabase>,
    pub quests: Arc<QuestDatabase>,
    pub skills: Arc<SkillDatabase>,
    pub status_effects: Arc<StatusEffectDatabase>,
    pub string_database: Arc<StringDatabase>,
    pub warp_gates: Arc<WarpGateDatabase>,
    pub zones: Arc<ZoneDatabase>,
    /// Crafting recipes, by an item's `craft_material`.
    pub craft_recipes: Vec<Option<CraftRecipe>>,
}

pub fn load_game_data(vfs: &VirtualFilesystem) -> Result<GameData, anyhow::Error> {
    let string_database = get_string_database(vfs, 1).context("string database")?;
    let items = Arc::new(load_items(vfs, string_database.clone()).context("item database")?);
    let motion_source = MotionSource::from_game_files(vfs).context("motion timings")?;
    let (npcs, drop_rows) = load_npcs(vfs, string_database.clone(), &motion_source).context("npc database")?;
    let motions = load_motions(vfs, &motion_source).context("motion database")?;
    motion_source.check_all_used(&motions, &npcs)?;
    let npcs = Arc::new(npcs);
    let (skills, status_effects, job_class) = load_skills(vfs, string_database.clone()).context("skill database")?;
    check_summons(&skills, &npcs)?;
    let skills = Arc::new(skills);
    let world = load_world(vfs, &string_database).context("world")?;
    world.check_items(&items)?;
    let mut zones = get_zone_database_from_settings(vfs, string_database.clone(), &world.zone_settings);
    apply_zone_layouts(vfs, &mut zones, &world.zone_settings).context("zone layouts")?;
    let zones = Arc::new(zones);
    let drop_table: Box<dyn DropTable + Send + Sync> = match drop_rows {
        Some(rows) => get_drop_table_from_rows(&rows, DROP_ROW_COLUMNS, items.clone(), npcs.clone()),
        None => get_drop_table(vfs, items.clone(), npcs.clone()).context("drop table")?,
    };

    Ok(GameData {
        character_creator: world.character_creator,
        ability_value_calculator: get_ability_value_calculator(items.clone(), skills.clone(), npcs.clone()),
        data_decoder: get_data_decoder(),
        drop_table,
        ai: Arc::new(get_ai_database(vfs).context("ai database")?),
        job_class: Arc::new(job_class),
        motions: Arc::new(motions),
        quests: Arc::new(load_quests(vfs, string_database.clone()).context("quest database")?),
        status_effects: Arc::new(status_effects),
        warp_gates: Arc::new(world.warp_gates),
        craft_recipes: world.craft_recipes,
        items,
        npcs,
        skills,
        zones,
        string_database,
    })
}

/// The items exported from the Godot resources (`export/data/items/`, in the game files at
/// `EXPORT/DATA/ITEMS/`, as JSON from the upload or as the TOML files themselves), or
/// without an export the iROSE item tables.
pub fn load_items(vfs: &VirtualFilesystem, strings: Arc<StringDatabase>) -> Result<ItemDatabase, anyhow::Error> {
    // The upload sends JSON; TOML is the export as the repository has it (`compare`).
    for format in [Format::Json, Format::Toml] {
        let export = ItemsExport::from_files_as(format, |name| {
            read_export_file(vfs, &rose_data_export::items_file_as(name, format))
        })?;
        if let Some(export) = export {
            return export.to_database(strings);
        }
    }
    get_item_database(vfs, strings)
}

/// A file of the data export among the game files (`EXPORT/DATA/...`, as JSON from the
/// upload or as the TOML files themselves), or None when it isn't there.
fn read_export_file(vfs: &VirtualFilesystem, path: &str) -> Result<Option<String>, anyhow::Error> {
    let path = format!("{}/{}", rose_data_export::DATA_GAME_DIR, path).to_uppercase();
    if !vfs.exists(path.as_str()) {
        return Ok(None);
    }
    let file = vfs.open_file(path.as_str())?;
    let bytes: &[u8] = match &file {
        VfsFile::Buffer(data) => data,
        VfsFile::View(data) => data,
    };
    Ok(Some(std::str::from_utf8(bytes).with_context(|| path.clone())?.to_string()))
}

/// The skills, status effects and job classes exported from the Godot resources
/// (`export/data/skills.toml` ..., as JSON from the upload or as the TOML files
/// themselves), or without an export the iROSE tables (LIST_SKILL.STB, LIST_STATUS.STB,
/// LIST_CLASS.STB).
pub fn load_skills(
    vfs: &VirtualFilesystem,
    strings: Arc<StringDatabase>,
) -> Result<(SkillDatabase, StatusEffectDatabase, JobClassDatabase), anyhow::Error> {
    if let Some(export) = read_skills_export(vfs)? {
        return export.to_databases(strings);
    }
    Ok((
        get_skill_database(vfs, strings.clone()).context("LIST_SKILL.STB")?,
        get_status_effect_database(vfs, strings.clone()).context("LIST_STATUS.STB")?,
        get_job_class_database(vfs, strings).context("LIST_CLASS.STB")?,
    ))
}

/// The skill export as the game files carry it (JSON from the upload, else TOML).
pub fn read_skills_export(vfs: &VirtualFilesystem) -> Result<Option<SkillsExport>, anyhow::Error> {
    for format in [Format::Json, Format::Toml] {
        let export = SkillsExport::from_files_as(format, |name| {
            read_export_file(vfs, &rose_data_export::data_file_as(name, format))
        })?;
        if export.is_some() {
            return Ok(export);
        }
    }
    Ok(None)
}

/// iROSE's numbers for ability and item types, which the quest blocks hold for the quest
/// engine.
pub fn irose_codes() -> IroseCodes {
    IroseCodes {
        decode_ability: rose_data_irose::decode_ability_type,
        encode_ability: rose_data_irose::encode_ability_type,
        decode_item_type: rose_data_irose::decode_item_type,
        encode_item_type: rose_data_irose::encode_item_type,
    }
}

/// The quest export as the game files carry it (JSON from the upload, else TOML).
pub fn read_quests_export(vfs: &VirtualFilesystem) -> Result<Option<QuestsExport>, anyhow::Error> {
    for format in [Format::Json, Format::Toml] {
        let export = QuestsExport::from_files_as(format, |name| {
            read_export_file(vfs, &rose_data_export::data_file_as(name, format))
        })?;
        if export.is_some() {
            return Ok(export);
        }
    }
    Ok(None)
}

/// The quests and quest triggers exported from the Godot resources
/// (`export/data/quests.toml`, `quest-triggers.toml`), or without an export the iROSE files
/// (LIST_QUEST.STB, LIST_QUEST_S.STL, QUEST_S.STB and the QSD files LIST_QUESTDATA.STB lists).
pub fn load_quests(vfs: &VirtualFilesystem, strings: Arc<StringDatabase>) -> Result<QuestDatabase, anyhow::Error> {
    match read_quests_export(vfs)? {
        Some(export) => export.to_database(strings, &irose_codes()),
        None => get_quest_database(vfs, strings),
    }
}

/// Every monster a skill summons must be there.
pub fn check_summons(skills: &SkillDatabase, npcs: &NpcDatabase) -> Result<(), anyhow::Error> {
    for skill in skills.iter() {
        if let Some(npc) = skill.summon_npc_id.filter(|n| npcs.get_npc(*n).is_none()) {
            anyhow::bail!("skill {} summons monster {}, which isn't there", skill.id.get(), npc.get());
        }
    }
    Ok(())
}

/// iROSE's number for an item (type × 1000 + number), as drop tables hold them.
pub fn encode_item_base1000(item: ItemReference) -> Option<i32> {
    let item_type = encode_item_type(item.item_type)?;
    (item.item_number < 1000).then(|| (item_type * 1000 + item.item_number) as i32)
}

/// The monster and NPC export as the game files carry it (JSON from the upload, else TOML).
pub fn read_npcs_export(vfs: &VirtualFilesystem) -> Result<Option<NpcsExport>, anyhow::Error> {
    for format in [Format::Json, Format::Toml] {
        let export = NpcsExport::from_files_as(format, |name| {
            read_export_file(vfs, &rose_data_export::data_file_as(name, format))
        })?;
        if export.is_some() {
            return Ok(export);
        }
    }
    Ok(None)
}

/// The monsters and NPCs (with their shop tabs) and the drop table rows exported from the
/// Godot resources (`export/data/monsters.toml` ...), or without an export the iROSE
/// tables (LIST_NPC.STB, LIST_SELL.STB; no rows: the drop table is ITEM_DROP.STB). Which
/// motion files they play still comes from LIST_NPC.CHR (their timings from `motions`), the
/// conversations from LIST_EVENT.STB.
pub fn load_npcs(
    vfs: &VirtualFilesystem,
    strings: Arc<StringDatabase>,
    motions: &MotionSource,
) -> Result<(NpcDatabase, Option<BTreeMap<u32, Vec<i32>>>), anyhow::Error> {
    let npc_motions = get_npc_motions_with(vfs, &mut |path| motions.motion_file(vfs, path))?;
    if let Some(export) = read_npcs_export(vfs)? {
        let models = NpcModels {
            motions: npc_motions,
            conversations: get_npc_conversations(vfs)?,
            action_map: get_npc_action_map(),
        };
        let rows = export.drop_rows(encode_item_base1000)?;
        return Ok((export.to_database(strings, models)?, Some(rows)));
    }
    Ok((get_npc_database_with(vfs, strings, npc_motions)?, None))
}

/// The monsters, NPCs, drop tables and shop tabs of the iROSE tables as the export holds
/// them: the converter's starting point and the same-data check. A town NPC is an id any of
/// `zones` places as an NPC (or one of `more_town_npcs`, the zone scenes' NPCs); every other
/// row is a monster. A shop place naming an item `item_exists` doesn't know is left out;
/// returns those.
pub fn npcs_export_from_irose(
    vfs: &VirtualFilesystem,
    strings: Arc<StringDatabase>,
    item_exists: impl Fn(ItemReference) -> bool,
    zones: &ZoneDatabase,
    more_town_npcs: impl IntoIterator<Item = u16>,
) -> Result<(NpcsExport, Vec<String>), anyhow::Error> {
    let npcs = get_npc_database(vfs, strings, &NpcDatabaseOptions { load_frame_data: false })?;
    let mut town_npcs: BTreeSet<u16> = zones.iter().flat_map(|z| z.npcs.iter().map(|n| n.npc_id.get())).collect();
    town_npcs.extend(more_town_npcs);
    let mut rows = rose_game_irose::data::read_drop_rows(vfs, DROP_ROW_COLUMNS).context("ITEM_DROP.STB")?;
    // A table is named after the monsters using it, else the zone with its number.
    let mut users: BTreeMap<u32, Vec<&str>> = BTreeMap::new();
    for npc in npcs.iter() {
        if npc.drop_table_index != 0 && !town_npcs.contains(&npc.id.get()) {
            let name = npc.name.trim();
            let list = users.entry(npc.drop_table_index).or_default();
            if !name.is_empty() && !list.contains(&name) {
                list.push(name);
            }
            // A monster's table is kept even when the row is empty, so the link stays.
            rows.entry(npc.drop_table_index).or_insert_with(|| vec![0; DROP_ROW_COLUMNS]);
        }
    }
    let rows: BTreeMap<u32, (String, Vec<i32>)> = rows
        .into_iter()
        .map(|(row, cells)| {
            let name = match users.get(&row) {
                Some(names) if names.len() > 3 => format!("{} and others", names[..3].join(", ")),
                Some(names) if !names.is_empty() => names.join(", "),
                _ => ZoneId::new(row as u16)
                    .and_then(|z| zones.get_zone(z))
                    .map(|z| format!("zone {}", z.name.trim()))
                    .unwrap_or_default(),
            };
            (row, (name, cells))
        })
        .collect();
    NpcsExport::from_database(
        &npcs,
        &town_npcs,
        &rows,
        |v| decode_item_base1000(v.max(0) as usize),
        item_exists,
    )
}

/// Where the length and hit frames of each motion file come from.
pub enum MotionSource {
    /// Only the paths, no timings (what the client and the same-data checks need).
    Paths,
    /// The ZMO files of the client data.
    Zmo,
    /// `export/motions.toml`, exported from the hit markers of the Godot animations, by
    /// motion file (normalised, as `VfsPathBuf` stores it). No ZMO file is read; a motion the
    /// file doesn't list has no animation, so it is left out as a missing ZMO file is.
    Export(BTreeMap<std::path::PathBuf, MotionFileData>),
}

impl MotionSource {
    /// `export/motions.toml` when the game files carry it, else the ZMO files.
    pub fn from_game_files(vfs: &VirtualFilesystem) -> Result<Self, anyhow::Error> {
        if !vfs.exists(rose_data_export::MOTIONS_GAME_FILE) {
            return Ok(Self::Zmo);
        }
        let file = vfs.open_file(rose_data_export::MOTIONS_GAME_FILE)?;
        let bytes: &[u8] = match &file {
            VfsFile::Buffer(data) => data,
            VfsFile::View(data) => data,
        };
        let timings = MotionTimings::parse(std::str::from_utf8(bytes)?).context("export/motions.toml")?;
        Ok(Self::from_timings(&timings))
    }

    pub fn from_timings(timings: &MotionTimings) -> Self {
        let motions = timings
            .motion
            .iter()
            .map(|timing| {
                let path = VfsPathBuf::new(&timing.file);
                let data = MotionFileData {
                    path: path.clone(),
                    duration: timing.duration(),
                    total_attack_frames: timing.hit_frames.len(),
                    first_attack_time: timing.first_hit_time(),
                };
                (path.path().to_path_buf(), data)
            })
            .collect();
        Self::Export(motions)
    }

    /// The data of the motion file `path` (as FILE_MOTION.STB or LIST_NPC.CHR names it);
    /// None for an empty path or a motion that has no file.
    pub fn motion_file(&self, vfs: &VirtualFilesystem, path: &str) -> Option<MotionFileData> {
        match self {
            Self::Paths => zmo_motion_file_data(vfs, path, false),
            Self::Zmo => zmo_motion_file_data(vfs, path, true),
            Self::Export(motions) if !path.is_empty() => motions.get(VfsPathBuf::new(path).path()).cloned(),
            Self::Export(_) => None,
        }
    }

    /// Every motion of the export is one a player, monster or NPC plays.
    pub fn check_all_used(&self, motions: &CharacterMotionDatabase, npcs: &NpcDatabase) -> Result<(), anyhow::Error> {
        let Self::Export(timings) = self else { return Ok(()) };
        let used: BTreeSet<&std::path::Path> = (0..3)
            .flat_map(|gender| motions.motion_files(gender))
            .chain(npcs.iter().flat_map(|npc| npc.motion_data.iter().map(|(_, m)| m)))
            .map(|m| m.path.path())
            .collect();
        if let Some(path) = timings.keys().find(|path| !used.contains(path.as_path())) {
            anyhow::bail!("export/motions.toml: {} is not a player, monster or NPC motion", path.display());
        }
        Ok(())
    }
}

/// The player motions (FILE_MOTION.STB, TYPE_MOTION.STB), their timings from `source`.
pub fn load_motions(vfs: &VirtualFilesystem, source: &MotionSource) -> Result<CharacterMotionDatabase, anyhow::Error> {
    get_character_motion_database_with(vfs, &mut |path| source.motion_file(vfs, path))
}

/// Every motion whose length or hits differ between two loads (the ZMO files and the
/// export), and every motion only one of them has: the player motions by gender and file,
/// each monster's and NPC's by its id and motion number.
pub fn compare_motions(
    a: (&CharacterMotionDatabase, &NpcDatabase),
    b: (&CharacterMotionDatabase, &NpcDatabase),
) -> Vec<String> {
    fn timings(motions: &CharacterMotionDatabase, npcs: &NpcDatabase) -> BTreeMap<String, String> {
        let ms = |t: std::time::Duration| format!("{:.3} ms", t.as_secs_f64() * 1000.0);
        let timing = |m: &MotionFileData| {
            let first = m.first_attack_time.map_or("-".to_string(), ms);
            format!("{}, {} hits, first at {first}", ms(m.duration), m.total_attack_frames)
        };
        let mut out = BTreeMap::new();
        for gender in 0..3 {
            for m in motions.motion_files(gender) {
                out.insert(format!("player motion {gender} {}", m.path.path().display()), timing(m));
            }
        }
        for npc in npcs.iter() {
            for (id, m) in &npc.motion_data {
                out.insert(format!("npc {} motion {} {}", npc.id.get(), id.get(), m.path.path().display()), timing(m));
            }
        }
        out
    }
    let (a, b) = (timings(a.0, a.1), timings(b.0, b.1));
    let mut differences = Vec::new();
    for (key, x) in &a {
        match b.get(key) {
            None => differences.push(format!("{key}: {x} -> missing")),
            Some(y) if x != y => differences.push(format!("{key}: {x} -> {y}")),
            Some(_) => {}
        }
    }
    for (key, y) in &b {
        if !a.contains_key(key) {
            differences.push(format!("{key}: missing -> {y}"));
        }
    }
    differences
}

/// The spawn points, town NPCs and named spots of `export/zones/` (exported from the
/// Godot zone scenes), when the game files carry them, in place of the zone's IFO and ZON
/// ones. The start and revive positions follow the spots the zone's settings name.
pub fn apply_zone_layouts(
    vfs: &VirtualFilesystem,
    zones: &mut ZoneDatabase,
    settings: &[ZoneSettings],
) -> Result<(), anyhow::Error> {
    for zone_settings in settings {
        let id = zone_settings.id;
        let path = rose_data_export::zone_game_file(id.get());
        if !vfs.exists(path.as_str()) {
            continue;
        }
        let file = vfs.open_file(path.as_str())?;
        let bytes: &[u8] = match &file {
            VfsFile::Buffer(data) => data,
            VfsFile::View(data) => data,
        };
        let layout = ZoneLayout::parse(std::str::from_utf8(bytes)?).with_context(|| format!("zone {}", id.get()))?;
        if layout.zone != id.get() {
            anyhow::bail!("{path} says zone = {}", layout.zone);
        }
        let zone = zones.get_zone_mut(id).with_context(|| format!("zone {} has no map files", id.get()))?;
        apply_zone_layout(zone, &layout, &zone_settings.start_spot, &zone_settings.revive_spot)
            .with_context(|| format!("zone {}", id.get()))?;
    }
    Ok(())
}

/// Puts one exported layout into a zone (positions from metres to the server's centimetres).
pub fn apply_zone_layout(
    zone: &mut ZoneData,
    layout: &ZoneLayout,
    start_name: &str,
    revive_name: &str,
) -> Result<(), anyhow::Error> {
    let at = |x: f32, y: f32, z: f32| Vec3::new(x, y, z) * 100.0;
    let npc_id = |n: u32| NpcId::new(n as u16).filter(|_| n <= u16::MAX as u32).with_context(|| format!("NPC {n}"));
    let list = |pairs: &[[u32; 2]]| -> Result<Vec<(NpcId, usize)>, anyhow::Error> {
        pairs.iter().map(|[m, count]| Ok((npc_id(*m)?, *count as usize))).collect()
    };
    zone.event_positions = layout.event.iter().map(|e| (e.name.clone(), at(e.x, e.y, e.z))).collect();
    if let Some(start) = zone.event_positions.get(start_name) {
        zone.start_position = *start;
    }
    zone.revive_positions =
        layout.event.iter().filter(|e| e.name == revive_name).map(|e| at(e.x, e.y, e.z)).collect();
    zone.monster_spawns = layout
        .spawn
        .iter()
        .map(|s| {
            Ok(ZoneMonsterSpawnPoint {
                position: at(s.x, s.y, s.z),
                basic_spawns: list(&s.basic)?,
                tactic_spawns: list(&s.tactic)?,
                interval: s.interval,
                limit_count: s.limit,
                range: s.range,
                tactic_points: s.tactic_points,
            })
        })
        .collect::<Result<_, anyhow::Error>>()?;
    zone.npcs = layout
        .npc
        .iter()
        .map(|n| {
            Ok(ZoneNpcSpawn {
                npc_id: npc_id(n.npc)?,
                position: at(n.x, n.y, n.z),
                direction: n.direction,
                conversation: NpcConversationId::new(n.conversation.clone()),
            })
        })
        .collect::<Result<_, anyhow::Error>>()?;
    Ok(())
}

/// The world: the zones' settings, the warp gates, the crafting recipes and the new
/// characters.
pub struct World {
    pub zone_settings: Vec<ZoneSettings>,
    pub warp_gates: WarpGateDatabase,
    pub craft_recipes: Vec<Option<CraftRecipe>>,
    pub character_creator: CharacterCreator,
}

impl World {
    /// Every item a recipe or a new character names must be there.
    pub fn check_items(&self, items: &ItemDatabase) -> Result<(), anyhow::Error> {
        let exists = |item: ItemReference| items.get_base_item(item).is_some();
        for (id, recipe) in self.craft_recipes.iter().enumerate() {
            for material in recipe.iter().flat_map(|r| r.materials.iter().flatten()) {
                if let Some(item) = material.item.filter(|i| !exists(*i)) {
                    anyhow::bail!("recipe {id} names {}, which isn't there", item_name(item));
                }
            }
        }
        for (gender, start) in ["male", "female"].iter().zip(&self.character_creator.genders) {
            let stacks = start.inventory_consumables.iter().chain(&start.inventory_materials).map(|(i, _)| i);
            for item in start.equipped_items.iter().chain(&start.inventory_equipment).chain(stacks) {
                if !exists(*item) {
                    anyhow::bail!("a new {gender} character starts with {}, which isn't there", item_name(*item));
                }
            }
        }
        Ok(())
    }
}

/// The world export as the game files carry it (JSON from the upload, else TOML).
pub fn read_world_export(vfs: &VirtualFilesystem) -> Result<Option<WorldExport>, anyhow::Error> {
    for format in [Format::Json, Format::Toml] {
        let export = WorldExport::from_files_as(format, |name| {
            read_export_file(vfs, &rose_data_export::data_file_as(name, format))
        })?;
        if export.is_some() {
            return Ok(export);
        }
    }
    Ok(None)
}

/// The world from the export (`export/data/zones.toml` ...), or without one from the iROSE
/// tables (LIST_ZONE.STB, WARP.STB, LIST_PRODUCT.STB, INIT_AVATAR.STB).
pub fn load_world(vfs: &VirtualFilesystem, strings: &StringDatabase) -> Result<World, anyhow::Error> {
    match read_world_export(vfs)? {
        Some(export) => world_from_export(&export),
        None => load_world_irose(vfs, strings),
    }
}

/// The world from the iROSE tables. A warp gate to a zone without a map file (which leads
/// nowhere) is left out.
pub fn load_world_irose(vfs: &VirtualFilesystem, strings: &StringDatabase) -> Result<World, anyhow::Error> {
    let zone_settings = get_zone_settings(vfs, strings).context("LIST_ZONE.STB")?;
    let gates = get_warp_gate_database(vfs).context("WARP.STB")?;
    let warp_gates = gates
        .iter()
        .filter(|(_, gate)| zone_settings.iter().any(|z| z.id == gate.target_zone))
        .map(|(id, gate)| (*id, WarpGateData { target_zone: gate.target_zone, target_event_object: gate.target_event_object.clone() }))
        .collect();
    Ok(World {
        zone_settings,
        warp_gates: WarpGateDatabase::new(warp_gates),
        craft_recipes: load_craft_recipes(vfs).context("LIST_PRODUCT.STB")?,
        character_creator: CharacterCreator::load(vfs).context("INIT_AVATAR.STB")?,
    })
}

/// The world from the export.
pub fn world_from_export(export: &WorldExport) -> Result<World, anyhow::Error> {
    export.check()?;
    let mut craft_recipes = Vec::new();
    for recipe in export.recipe_steps()? {
        craft_recipes.push(recipe.map(|(recipe, steps)| craft_recipe(recipe, &steps)));
    }
    Ok(World {
        zone_settings: export.zone_settings()?,
        warp_gates: export.warp_gate_database()?,
        craft_recipes,
        character_creator: CharacterCreator::from_export(export)?,
    })
}

fn craft_recipe(recipe: &CraftRecipeExport, steps: &[Option<RecipeStep>; 4]) -> CraftRecipe {
    CraftRecipe {
        raw_material_class: recipe.raw_material_class.map_or(0, item_class_number),
        materials: steps.map(|step| step.map(|(item, quantity)| CraftMaterial { item, quantity })),
    }
}

/// The iROSE world tables as the export holds them: the converter's starting point and the
/// same-data check. Recipes are named after the items `items` makes with them. Also returns
/// what couldn't be kept (warp gates to zones without a map file).
pub fn world_export_from_irose(
    vfs: &VirtualFilesystem,
    strings: &StringDatabase,
    items: &ItemDatabase,
) -> Result<(WorldExport, Vec<String>), anyhow::Error> {
    let world = load_world_irose(vfs, strings)?;
    let mut lost: Vec<(u16, String)> = get_warp_gate_database(vfs)?
        .iter()
        .filter(|(id, _)| world.warp_gates.get_warp_gate(**id).is_none())
        .map(|(id, gate)| {
            (id.get(), format!("warp gate {} goes to zone {}, which has no map file", id.get(), gate.target_zone.get()))
        })
        .collect();
    lost.sort();
    let lost = lost.into_iter().map(|(_, line)| line).collect();
    let zones = world.zone_settings.iter().map(ZoneExport::from_settings).collect();
    let mut warp_gates: Vec<WarpGateExport> =
        world.warp_gates.iter().map(|(id, gate)| WarpGateExport::from_data(*id, gate)).collect();
    warp_gates.sort_by_key(|g| g.id);

    // A recipe is named after the items made with it (rows 1 to 20 refine).
    let mut makes: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for item_type in [
        ItemType::Face,
        ItemType::Head,
        ItemType::Body,
        ItemType::Hands,
        ItemType::Feet,
        ItemType::Back,
        ItemType::Jewellery,
        ItemType::Weapon,
        ItemType::SubWeapon,
        ItemType::Consumable,
        ItemType::Gem,
        ItemType::Material,
        ItemType::Quest,
        ItemType::Vehicle,
    ] {
        for item in items.iter_items(item_type) {
            let Some(data) = items.get_base_item(item) else { continue };
            let name = data.name.trim();
            if data.craft_material != 0 && !name.is_empty() {
                let list = makes.entry(data.craft_material).or_default();
                if !list.iter().any(|n| n == name) {
                    list.push(name.to_string());
                }
            }
        }
    }
    let mut craft_recipes = Vec::new();
    for (id, recipe) in world.craft_recipes.iter().enumerate() {
        let Some(recipe) = recipe else { continue };
        let name = match id {
            1..=10 => format!("Refining weapons from grade {} to {}", id - 1, id),
            11..=20 => format!("Refining armour from grade {} to {}", id - 11, id - 10),
            _ => match makes.get(&(id as u32)) {
                Some(names) if names.len() > 3 => format!("{} and others", names[..3].join(", ")),
                Some(names) => names.join(", "),
                None => String::new(),
            },
        };
        let raw = (recipe.raw_material_class != 0)
            .then(|| decode_item_class(recipe.raw_material_class as usize))
            .flatten();
        if recipe.raw_material_class != 0 && raw.is_none() {
            anyhow::bail!("recipe {id}: raw material class {} is not an item class", recipe.raw_material_class);
        }
        let steps = recipe.materials.clone().map(|m| m.map(|m| (m.item, m.quantity)));
        craft_recipes.push(CraftRecipeExport::from_steps(id as u16, name, raw, &steps));
    }

    let avatar = StbInitAvatar(vfs.read_file::<StbFile, _>("3DDATA/STB/INIT_AVATAR.STB")?);
    let mut new_characters = Vec::new();
    for (row, gender) in [(0, Gender::Male), (1, Gender::Female)] {
        let start = avatar.character(row).with_context(|| format!("INIT_AVATAR row {row}"))?;
        let slot = |column: usize| avatar.slot(row, column).map(item_name).unwrap_or_default();
        let names = |list: &[(ItemReference, u32)]| list.iter().map(|(i, _)| item_name(*i)).collect::<Vec<_>>();
        let counts = |list: &[(ItemReference, u32)]| list.iter().map(|(_, c)| *c).collect::<Vec<_>>();
        let stats = &start.basic_stats;
        new_characters.push(NewCharacterExport {
            gender,
            strength: stats.strength,
            dexterity: stats.dexterity,
            intelligence: stats.intelligence,
            concentration: stats.concentration,
            charm: stats.charm,
            sense: stats.sense,
            face: slot(6),
            head: slot(7),
            body: slot(8),
            back: slot(9),
            hands: slot(10),
            feet: slot(11),
            weapon: slot(12),
            sub_weapon: slot(13),
            bag_equipment: start.inventory_equipment.iter().map(|i| item_name(*i)).collect(),
            bag_consumables: names(&start.inventory_consumables),
            bag_consumable_counts: counts(&start.inventory_consumables),
            bag_materials: names(&start.inventory_materials),
            bag_material_counts: counts(&start.inventory_materials),
        });
    }
    let export = WorldExport { zones, warp_gates, craft_recipes, new_characters };
    export.check()?;
    Ok((export, lost))
}

/// Every difference between two worlds (the same-data check).
pub fn compare_worlds(a: &World, b: &World) -> Vec<String> {
    let mut out = rose_data_export::compare_zone_settings(&a.zone_settings, &b.zone_settings);
    out.extend(rose_data_export::compare_warp_gates(&a.warp_gates, &b.warp_gates));
    let recipes = |list: &[Option<CraftRecipe>]| {
        list.iter()
            .enumerate()
            .filter_map(|(id, r)| r.as_ref().map(|r| (id as u16, format!("{r:#?}"))))
            .collect::<BTreeMap<_, _>>()
    };
    out.extend(rose_data_export::world::compare_text("recipe", recipes(&a.craft_recipes), recipes(&b.craft_recipes)));
    let characters = |c: &CharacterCreator| {
        c.genders.iter().enumerate().map(|(i, g)| (i as u16, format!("{g:#?}"))).collect::<BTreeMap<_, _>>()
    };
    out.extend(rose_data_export::world::compare_text(
        "new character (0 male, 1 female)",
        characters(&a.character_creator),
        characters(&b.character_creator),
    ));
    out
}

/// One crafting step's material: a given item, or for the first step (`item` None) any
/// item of the recipe's raw material class.
#[derive(Clone, Debug, PartialEq)]
pub struct CraftMaterial {
    pub item: Option<ItemReference>,
    pub quantity: u32,
}

/// A crafting or refining recipe (a `RoseCraftRecipe`, iROSE's LIST_PRODUCT.STB row): up to
/// four materials, used one step after another.
#[derive(Clone, Debug, PartialEq)]
pub struct CraftRecipe {
    /// The item class number (ITEM STB column 4) the first material may be any item of.
    pub raw_material_class: u32,
    pub materials: [Option<CraftMaterial>; 4],
}

impl CraftRecipe {
    pub fn material_count(&self) -> usize {
        self.materials.iter().filter(|m| m.is_some()).count()
    }

    /// Whether an item (with its class) can be this step's material.
    pub fn accepts(&self, step: usize, item: ItemReference, class: ItemClass) -> bool {
        match self.materials.get(step).and_then(|m| m.as_ref()) {
            Some(CraftMaterial { item: Some(wanted), .. }) => item == *wanted,
            Some(CraftMaterial { item: None, .. }) => item_class_number(class) == self.raw_material_class,
            None => false,
        }
    }
}

/// The class number of an item class, as LIST_PRODUCT.STB names raw materials.
pub fn item_class_number(class: ItemClass) -> u32 {
    encode_item_class(class).unwrap_or(0) as u32
}

/// The crafting recipes of LIST_PRODUCT.STB, by row.
pub fn load_craft_recipes(vfs: &VirtualFilesystem) -> Result<Vec<Option<CraftRecipe>>, anyhow::Error> {
    let stb = vfs.read_file::<StbFile, _>("3DDATA/STB/LIST_PRODUCT.STB")?;
    let mut recipes = Vec::with_capacity(stb.rows());
    for row in 0..stb.rows() {
        let raw_material_class = stb.try_get_int(row, 1).unwrap_or(0).max(0) as u32;
        let mut materials: [Option<CraftMaterial>; 4] = Default::default();
        for (step, slot) in materials.iter_mut().enumerate() {
            let number = stb.try_get_int(row, 2 + step * 2).unwrap_or(0).max(0) as usize;
            let quantity = stb.try_get_int(row, 3 + step * 2).unwrap_or(0).max(0) as u32;
            let item = decode_item_base1000(number);
            if item.is_some() || (step == 0 && raw_material_class != 0 && quantity > 0) {
                *slot = Some(CraftMaterial { item, quantity: quantity.max(1) });
            }
        }
        recipes.push(materials[0].is_some().then_some(CraftRecipe { raw_material_class, materials }));
    }
    Ok(recipes)
}

/// Starting stats, gear and skills for a new character (the `RoseNewCharacter` resources,
/// iROSE's INIT_AVATAR.STB).
pub struct CharacterCreator {
    /// Index 0 is male, 1 is female.
    pub genders: [StartingCharacter; 2],
    pub skills: Vec<SkillId>,
}

#[derive(Debug)]
pub struct StartingCharacter {
    pub basic_stats: BasicStats,
    pub equipped_items: Vec<ItemReference>,
    pub inventory_equipment: Vec<ItemReference>,
    pub inventory_consumables: Vec<(ItemReference, u32)>,
    pub inventory_materials: Vec<(ItemReference, u32)>,
}

struct StbInitAvatar(StbFile);

impl StbInitAvatar {
    stb_column! { 0, get_strength, i32 }
    stb_column! { 1, get_dexterity, i32 }
    stb_column! { 2, get_intelligence, i32 }
    stb_column! { 3, get_concentration, i32 }
    stb_column! { 4, get_charm, i32 }
    stb_column! { 5, get_sense, i32 }

    fn items(&self, row: usize, columns: impl Iterator<Item = usize>) -> Vec<ItemReference> {
        columns
            .filter_map(|column| decode_item_base1000(self.0.try_get_int(row, column).unwrap_or(0) as usize))
            .collect()
    }

    fn stacks(&self, row: usize, first_column: usize) -> Vec<(ItemReference, u32)> {
        (0..5)
            .filter_map(|i| {
                let item = self.0.try_get_int(row, first_column + i * 2).unwrap_or(0) as usize;
                let quantity = self.0.try_get_int(row, first_column + 1 + i * 2).unwrap_or(0) as u32;
                decode_item_base1000(item).map(|item| (item, quantity))
            })
            .collect()
    }

    /// The item in one cell (an equipment slot's column).
    fn slot(&self, row: usize, column: usize) -> Option<ItemReference> {
        decode_item_base1000(self.0.try_get_int(row, column).unwrap_or(0).max(0) as usize)
    }

    fn character(&self, row: usize) -> Option<StartingCharacter> {
        Some(StartingCharacter {
            basic_stats: BasicStats {
                strength: self.get_strength(row)?,
                dexterity: self.get_dexterity(row)?,
                intelligence: self.get_intelligence(row)?,
                concentration: self.get_concentration(row)?,
                charm: self.get_charm(row)?,
                sense: self.get_sense(row)?,
            },
            equipped_items: self.items(row, 6..=13),
            inventory_equipment: self.items(row, 14..24),
            inventory_consumables: self.stacks(row, 24),
            inventory_materials: self.stacks(row, 34),
        })
    }
}

/// The skills every new character has (in code, not in the game data).
fn starting_skills() -> Vec<SkillId> {
    [
        SkillIds::Sit,
        SkillIds::PickUp,
        SkillIds::Attack,
        SkillIds::Trade,
        SkillIds::RideRequest,
        SkillIds::Jump,
        SkillIds::Hi,
        SkillIds::Bow,
        SkillIds::Salutation,
        SkillIds::RecoveryKiss,
        SkillIds::CharmingKiss,
        SkillIds::Laugh,
        SkillIds::FightCheer,
        SkillIds::BreakDown,
        SkillIds::Tantrum,
        SkillIds::Applause,
    ]
    .into_iter()
    .filter_map(|id| SkillId::new(id as u16))
    .collect()
}

impl CharacterCreator {
    /// From INIT_AVATAR.STB.
    pub fn load(vfs: &VirtualFilesystem) -> Result<Self, anyhow::Error> {
        let data = StbInitAvatar(vfs.read_file::<StbFile, _>("3DDATA/STB/INIT_AVATAR.STB")?);
        let male = data.character(0).context("INIT_AVATAR row 0")?;
        let female = data.character(1).context("INIT_AVATAR row 1")?;
        Ok(Self { genders: [male, female], skills: starting_skills() })
    }

    /// From the data export.
    pub fn from_export(world: &WorldExport) -> Result<Self, anyhow::Error> {
        let character = |gender: Gender| -> Result<StartingCharacter, anyhow::Error> {
            let c = world.new_character(gender)?;
            Ok(StartingCharacter {
                basic_stats: BasicStats {
                    strength: c.strength,
                    dexterity: c.dexterity,
                    intelligence: c.intelligence,
                    concentration: c.concentration,
                    charm: c.charm,
                    sense: c.sense,
                },
                equipped_items: c.equipped_items()?,
                inventory_equipment: c.bag_equipment()?,
                inventory_consumables: c.bag_consumables()?,
                inventory_materials: c.bag_materials()?,
            })
        };
        Ok(Self { genders: [character(Gender::Male)?, character(Gender::Female)?], skills: starting_skills() })
    }
}
