//! Effect files and attack effects from the iROSE tables, written as the export and read back,
//! are the same; so are the committed export's (`export/data/effect-files.toml` ...), whose
//! files are the imported effect scenes of the client's EFT files. Needs the client data:
//! ROSE_DATA_IDX=<path to data.idx> (skipped without it).

use std::collections::BTreeSet;

use rose_data_export::{compare_effect_databases, EffectsExport, Format};
use rose_file_readers::{EftFile, VfsIndex, VirtualFilesystem};

#[test]
fn effects_survive_the_export() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    let vfs = VirtualFilesystem::new(vec![Box::new(VfsIndex::load(std::path::Path::new(&data_idx)).unwrap())]);
    let effects = rose_data_irose::get_effect_database(&vfs).unwrap();
    let all_files: BTreeSet<u16> = effects.iter_files().map(|(id, _)| id.get()).collect();
    let all_effects: BTreeSet<u16> = effects.iter().map(|e| e.id.get()).collect();
    let files = EffectsExport::from_database(&effects, &all_files, &all_effects).to_files_as(Format::Toml).unwrap();
    let read = EffectsExport::from_files_as(Format::Toml, |name| Ok(files.iter().find(|(n, _)| *n == name).map(|(_, t)| t.clone())))
        .unwrap()
        .unwrap();
    let differences = compare_effect_databases(&effects, &read.to_database().unwrap(), |_| true);
    assert!(differences.is_empty(), "{}", differences.join("\n"));
    eprintln!("{} effect files, {} attack effects", read.effect_files.len(), read.attack_effects.len());

    // The committed export leaves out the effects whose file the client can't show.
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(rose_data_export::DATA_DIR);
    let committed = EffectsExport::from_files_as(Format::Toml, |name| {
        Ok(std::fs::read_to_string(dir.join(rose_data_export::data_file_as(name, Format::Toml))).ok())
    })
    .unwrap()
    .expect("export/data/effect-files.toml");
    let has_file = |path: &str| path.starts_with("res://") || vfs.read_file::<EftFile, _>(path).is_ok();
    let differences = compare_effect_databases(&effects, &committed.to_database().unwrap(), has_file);
    assert!(differences.is_empty(), "{}", differences.join("\n"));
}
