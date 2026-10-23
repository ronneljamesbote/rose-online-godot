//! Monsters, NPCs, shop tabs and drop tables from the iROSE tables, written as the export and
//! read back, are the same. Needs the client data: ROSE_DATA_IDX=<path to data.idx>
//! (skipped without it).

use rose_data::NpcDatabaseOptions;
use rose_data_export::{compare_drop_rows, compare_npc_databases, npcs::DROP_ROW_COLUMNS, Format, NpcModels, NpcsExport};
use rose_file_readers::{VfsIndex, VirtualFilesystem};

#[test]
fn npcs_survive_the_export() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    let vfs = VirtualFilesystem::new(vec![Box::new(VfsIndex::load(std::path::Path::new(&data_idx)).unwrap())]);
    let strings = rose_data_irose::get_string_database(&vfs, 1).unwrap();
    let items = rose_data_irose::get_item_database(&vfs, strings.clone()).unwrap();
    let zones = rose_data_irose::get_zone_database(&vfs, strings.clone()).unwrap();
    let (export, skipped) = rose_game_data::npcs_export_from_irose(&vfs, strings.clone(), |i| items.get_base_item(i).is_some(), &zones, []).unwrap();
    for line in &skipped {
        eprintln!("skipped: {line}");
    }
    let files = export.to_files_as(Format::Toml).unwrap();
    let read = NpcsExport::from_files_as(Format::Toml, |name| {
        Ok(files.iter().find(|(n, _)| *n == name).map(|(_, t)| t.clone()))
    })
    .unwrap()
    .unwrap();
    let options = NpcDatabaseOptions { load_frame_data: false };
    let models = NpcModels {
        motions: rose_data_irose::get_npc_motions(&vfs, &options).unwrap(),
        conversations: rose_data_irose::get_npc_conversations(&vfs).unwrap(),
        action_map: rose_data_irose::get_npc_action_map(),
    };
    let npcs = read.to_database(strings.clone(), models).unwrap();
    let irose = rose_data_irose::get_npc_database(&vfs, strings, &options).unwrap();
    let mut differences = compare_npc_databases(&irose, &npcs);
    let rows = read.drop_rows(rose_game_data::encode_item_base1000).unwrap();
    let irose_rows = rose_game_irose::data::read_drop_rows(&vfs, DROP_ROW_COLUMNS).unwrap();
    differences.extend(compare_drop_rows(&irose_rows, &rows));
    // The one place the converter can't keep: a shop place naming an item that doesn't exist.
    differences.retain(|d| !skipped.iter().any(|s| d.starts_with(&format!("shop tab {}:", s.split(' ').nth(2).unwrap()))));
    assert!(differences.is_empty(), "{}", differences.join("\n"));
    let bytes: usize = files.iter().map(|(_, t)| t.len()).sum();
    eprintln!(
        "{} monsters, {} NPCs, {} shop tabs, {} drop tables, {bytes} bytes",
        read.monsters.len(),
        read.npcs.len(),
        read.shop_tabs.len(),
        read.drop_tables.len()
    );
}
