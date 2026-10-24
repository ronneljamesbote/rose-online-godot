//! The icons and models of the committed export (`export/data/`: Godot paths of the imported
//! icons and scenes) are the ones the iROSE tables name: every item's icon, ground model and
//! worn models, every monster's and NPC's model, every skill's and status effect's icon. Needs
//! the client data: ROSE_DATA_IDX=<path to data.idx> (skipped without it).

use rose_data::NpcDatabaseOptions;
use rose_data_export::{compare_item_databases, compare_npc_databases, compare_skill_databases, Format, ItemsExport, NpcModels, NpcsExport, SkillsExport};
use rose_file_readers::{VfsIndex, VirtualFilesystem};

/// The differences that are about art.
fn art(differences: Vec<String>) -> Vec<String> {
    differences.into_iter().filter(|d| d.contains("icon:") || d.contains("model")).collect()
}

#[test]
fn the_exported_art_is_the_iroses() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    let vfs = VirtualFilesystem::new(vec![Box::new(VfsIndex::load(std::path::Path::new(&data_idx)).unwrap())]);
    let strings = rose_data_irose::get_string_database(&vfs, 1).unwrap();
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(rose_data_export::DATA_DIR);
    let read = |name: &str| Ok(std::fs::read_to_string(dir.join(name)).ok());

    let items = ItemsExport::from_files(|name| read(&rose_data_export::items_file(name))).unwrap().unwrap();
    let mut differences = art(compare_item_databases(
        &rose_data_irose::get_item_database(&vfs, strings.clone()).unwrap(),
        &items.to_database(strings.clone()).unwrap(),
    ));

    let options = NpcDatabaseOptions { load_frame_data: false };
    let models = NpcModels {
        motions: rose_data_irose::get_npc_motions(&vfs, &options).unwrap(),
        conversations: rose_data_irose::get_npc_conversations(&vfs).unwrap(),
        action_map: rose_data_irose::get_npc_action_map(),
    };
    let npcs = NpcsExport::from_files_as(Format::Toml, |name| read(&rose_data_export::data_file_as(name, Format::Toml))).unwrap().unwrap();
    let irose_npcs = rose_data_irose::get_npc_database(&vfs, strings.clone(), &options).unwrap();
    // The motions follow the model, so a difference there shows as one too.
    differences.extend(art(compare_npc_databases(&irose_npcs, &npcs.to_database(strings.clone(), models).unwrap())));

    let skills = SkillsExport::from_files_as(Format::Toml, |name| read(&rose_data_export::data_file_as(name, Format::Toml))).unwrap().unwrap();
    let (s, e, j) = skills.to_databases(strings.clone()).unwrap();
    let irose_skills = rose_data_irose::get_skill_database(&vfs, strings.clone()).unwrap();
    let irose_effects = rose_data_irose::get_status_effect_database(&vfs, strings.clone()).unwrap();
    let irose_classes = rose_data_irose::get_job_class_database(&vfs, strings).unwrap();
    differences.extend(art(compare_skill_databases((&irose_skills, &irose_effects, &irose_classes), (&s, &e, &j))));
    assert!(differences.is_empty(), "{}", differences.join("\n"));

    // Every path names an imported file (when the import was run here).
    let godot = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../godot");
    if godot.join("assets/rose/3ddata/control/res/item1").is_dir() {
        let text: String = std::fs::read_dir(dir.join("items")).unwrap().map(|f| std::fs::read_to_string(f.unwrap().path()).unwrap()).collect::<String>()
            + &std::fs::read_to_string(dir.join("monsters.toml")).unwrap()
            + &std::fs::read_to_string(dir.join("npcs.toml")).unwrap()
            + &std::fs::read_to_string(dir.join("skills.toml")).unwrap()
            + &std::fs::read_to_string(dir.join("status-effects.toml")).unwrap();
        let mut paths = 0;
        for path in text.split('"').filter(|t| t.starts_with("res://assets/")) {
            assert!(godot.join(path.trim_start_matches("res://")).is_file(), "{path} is missing");
            paths += 1;
        }
        eprintln!("{paths} icon and model paths, all there");
    }
}
