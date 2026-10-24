//! The skies from the iROSE tables, written as the export and read back, are the same; so
//! are the committed export's (`export/data/skies.toml`), whose meshes and textures are the
//! imported copies of the client's, and its interface pictures and cursors are the client's
//! own. Needs the client data: ROSE_DATA_IDX=<path to data.idx> (skipped without it).

use rose_data_export::{compare_interface, compare_sky_databases, Format, InterfaceExport};
use rose_file_readers::{VfsIndex, VirtualFilesystem};

#[test]
fn skies_survive_the_export() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    let vfs = VirtualFilesystem::new(vec![Box::new(VfsIndex::load(std::path::Path::new(&data_idx)).unwrap())]);
    let skies = rose_data_irose::get_skybox_database(&vfs).unwrap();
    let files = InterfaceExport::from_database(&skies).to_files_as(Format::Toml).unwrap();
    let read = InterfaceExport::from_files_as(Format::Toml, |name| Ok(files.iter().find(|(n, _)| *n == name).map(|(_, t)| t.clone())))
        .unwrap()
        .unwrap();
    let differences = compare_sky_databases(&skies, &read.to_database().unwrap());
    assert!(differences.is_empty(), "{}", differences.join("\n"));
    eprintln!("{} skies", read.skies.len());

    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(rose_data_export::DATA_DIR);
    let committed = InterfaceExport::from_files_as(Format::Toml, |name| {
        Ok(std::fs::read_to_string(dir.join(rose_data_export::data_file_as(name, Format::Toml))).ok())
    })
    .unwrap()
    .expect("export/data/skies.toml");
    let mut differences = compare_sky_databases(&skies, &committed.to_database().unwrap());
    differences.extend(compare_interface(&Default::default(), &committed.interface));
    assert!(differences.is_empty(), "{}", differences.join("\n"));
}
