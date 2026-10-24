//! Quests and quest triggers from the iROSE files, written as the export and read back, are
//! the same. Needs the client data: ROSE_DATA_IDX=<path to data.idx> (skipped without it).

use rose_data_export::{compare_quest_databases, Format, IroseCodes, QuestsExport};
use rose_file_readers::{VfsIndex, VirtualFilesystem};

fn codes() -> IroseCodes {
    IroseCodes {
        decode_ability: rose_data_irose::decode_ability_type,
        encode_ability: rose_data_irose::encode_ability_type,
        decode_item_type: rose_data_irose::decode_item_type,
        encode_item_type: rose_data_irose::encode_item_type,
    }
}

#[test]
fn quests_survive_the_export() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    let vfs = VirtualFilesystem::new(vec![Box::new(VfsIndex::load(std::path::Path::new(&data_idx)).unwrap())]);
    let strings = rose_data_irose::get_string_database(&vfs, 1).unwrap();
    let quests = rose_data_irose::get_quest_database(&vfs, strings.clone()).unwrap();
    let mut export = QuestsExport::from_database(&quests, &codes()).unwrap();
    // As the converter does: a quest a trigger names gets a resource even without text.
    let named: std::collections::BTreeSet<u16> = export
        .triggers
        .iter()
        .flat_map(|t| t.condition.iter().filter_map(|c| c.quest()).chain(t.reward.iter().filter_map(|r| r.quest())))
        .collect();
    for id in named {
        if !export.quests.iter().any(|q| q.id == id) {
            export.quests.push(rose_data_export::quests::QuestDataExport { id, ..Default::default() });
        }
    }
    let files = export.to_files_as(Format::Toml).unwrap();
    let read = QuestsExport::from_files_as(Format::Toml, |name| Ok(files.iter().find(|(n, _)| *n == name).map(|(_, t)| t.clone())))
        .unwrap()
        .unwrap();
    let back = read.to_database(strings, &codes()).unwrap();
    let monsters = (1..u16::MAX).collect();
    let differences = compare_quest_databases(&quests, &back, &codes(), &monsters).unwrap();
    assert!(differences.is_empty(), "{}", differences.join("\n"));
    let bytes: usize = files.iter().map(|(_, t)| t.len()).sum();
    eprintln!("{} quests, {} triggers, {bytes} bytes", read.quests.len(), read.triggers.len());
}
