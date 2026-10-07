//! Every game database the server needs, loaded from the iROSE client files. Ported from
//! rose-offline-server's `irose/data` (game data and character creator) without Bevy, so the
//! same code runs natively and inside the SpacetimeDB module.

use std::sync::Arc;

use anyhow::Context;
use rose_data::{
    AiDatabase, CharacterMotionDatabase, CharacterMotionDatabaseOptions, DataDecoder, ItemDatabase,
    ItemReference, JobClassDatabase, NpcDatabase, NpcDatabaseOptions, QuestDatabase, SkillDatabase,
    SkillId, SkillIds, StatusEffectDatabase, StringDatabase, WarpGateDatabase, ZoneDatabase,
};
use rose_data_irose::{
    decode_item_base1000, get_ai_database, get_character_motion_database, get_data_decoder,
    get_item_database, get_job_class_database, get_npc_database, get_quest_database,
    get_skill_database, get_status_effect_database, get_string_database, get_warp_gate_database,
    get_zone_database,
};
use rose_file_readers::{stb_column, StbFile, VirtualFilesystem};
use rose_game_common::{
    components::BasicStats,
    data::{AbilityValueCalculator, DropTable},
};
use rose_game_irose::data::{get_ability_value_calculator, get_drop_table};

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
}

pub fn load_game_data(vfs: &VirtualFilesystem) -> Result<GameData, anyhow::Error> {
    let string_database = get_string_database(vfs, 1).context("string database")?;
    let items = Arc::new(get_item_database(vfs, string_database.clone()).context("item database")?);
    let npcs = Arc::new(
        get_npc_database(vfs, string_database.clone(), &NpcDatabaseOptions { load_frame_data: true })
            .context("npc database")?,
    );
    let skills = Arc::new(get_skill_database(vfs, string_database.clone()).context("skill database")?);
    let zones = Arc::new(get_zone_database(vfs, string_database.clone()).context("zone database")?);
    let drop_table = get_drop_table(vfs, items.clone(), npcs.clone()).context("drop table")?;

    Ok(GameData {
        character_creator: CharacterCreator::load(vfs).context("character creator")?,
        ability_value_calculator: get_ability_value_calculator(items.clone(), skills.clone(), npcs.clone()),
        data_decoder: get_data_decoder(),
        drop_table,
        ai: Arc::new(get_ai_database(vfs).context("ai database")?),
        job_class: Arc::new(get_job_class_database(vfs, string_database.clone()).context("job class database")?),
        motions: Arc::new(
            get_character_motion_database(vfs, &CharacterMotionDatabaseOptions { load_frame_data: true })
                .context("motion database")?,
        ),
        quests: Arc::new(get_quest_database(vfs, string_database.clone()).context("quest database")?),
        status_effects: Arc::new(
            get_status_effect_database(vfs, string_database.clone()).context("status effect database")?,
        ),
        warp_gates: Arc::new(get_warp_gate_database(vfs).context("warp gate database")?),
        items,
        npcs,
        skills,
        zones,
        string_database,
    })
}

/// Starting stats, gear and skills for a new character, from INIT_AVATAR.STB.
pub struct CharacterCreator {
    /// Index 0 is male, 1 is female.
    pub genders: [StartingCharacter; 2],
    pub skills: Vec<SkillId>,
}

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

impl CharacterCreator {
    fn load(vfs: &VirtualFilesystem) -> Result<Self, anyhow::Error> {
        let data = StbInitAvatar(vfs.read_file::<StbFile, _>("3DDATA/STB/INIT_AVATAR.STB")?);
        let male = data.character(0).context("INIT_AVATAR row 0")?;
        let female = data.character(1).context("INIT_AVATAR row 1")?;
        let skills = [
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
        .collect();
        Ok(Self { genders: [male, female], skills })
    }
}
