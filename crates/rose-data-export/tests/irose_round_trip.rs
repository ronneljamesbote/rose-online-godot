//! Items from the iROSE tables, written as the export and read back, are the same items.
//! Needs the client data: ROSE_DATA_IDX=<path to data.idx> (skipped without it).

use rose_data_export::{compare_item_databases, ItemsExport};
use rose_file_readers::{VfsIndex, VirtualFilesystem};

#[test]
fn items_survive_the_export() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    let vfs = VirtualFilesystem::new(vec![Box::new(
        VfsIndex::load(std::path::Path::new(&data_idx)).unwrap(),
    )]);
    let strings = rose_data_irose::get_string_database(&vfs, 1).unwrap();
    let irose = rose_data_irose::get_item_database(&vfs, strings.clone()).unwrap();
    let files = ItemsExport::from_database(&irose).to_files().unwrap();
    let read = ItemsExport::from_files(|name| {
        Ok(files
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, t)| t.clone()))
    })
    .unwrap()
    .unwrap();
    let export = read.to_database(strings).unwrap();
    let differences = compare_item_databases(&irose, &export);
    assert!(differences.is_empty(), "{}", differences.join("\n"));
    let bytes: usize = files.iter().map(|(_, t)| t.len()).sum();
    eprintln!(
        "{} items, {} files, {bytes} bytes",
        read.count(),
        files.len()
    );
}
