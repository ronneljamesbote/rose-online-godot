//! Sounds, hit sounds and footsteps from the iROSE tables, written as the export and read
//! back, are the same; so are the committed export's (`export/data/sounds.toml` ...), whose
//! files are the imported copies of the client's. Needs the client data:
//! ROSE_DATA_IDX=<path to data.idx> (skipped without it).

use std::collections::BTreeSet;

use rose_data_export::{compare_sound_databases, Format, SoundsExport};
use rose_file_readers::{VfsIndex, VirtualFilesystem};

#[test]
fn sounds_survive_the_export() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    let vfs = VirtualFilesystem::new(vec![Box::new(VfsIndex::load(std::path::Path::new(&data_idx)).unwrap())]);
    let sounds = rose_data_irose::get_sound_database(&vfs).unwrap();
    let all: BTreeSet<u16> = sounds.iter().map(|s| s.id.get()).collect();
    let files = SoundsExport::from_database(&sounds, &all, &BTreeSet::new()).to_files_as(Format::Toml).unwrap();
    let read = SoundsExport::from_files_as(Format::Toml, |name| Ok(files.iter().find(|(n, _)| *n == name).map(|(_, t)| t.clone())))
        .unwrap()
        .unwrap();
    let differences = compare_sound_databases(&sounds, &read.to_database().unwrap(), |_| true);
    assert!(differences.is_empty(), "{}", differences.join("\n"));
    eprintln!("{} sounds, {} hit sound sets, {} footstep sets", read.sounds.len(), read.hit_sounds.len(), read.footsteps.len());

    // The committed export leaves out the sounds whose file the client doesn't have.
    let root = std::path::Path::new(&data_idx).parent().unwrap().to_path_buf();
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(rose_data_export::DATA_DIR);
    let committed = SoundsExport::from_files_as(Format::Toml, |name| {
        Ok(std::fs::read_to_string(dir.join(rose_data_export::data_file_as(name, Format::Toml))).ok())
    })
    .unwrap()
    .expect("export/data/sounds.toml");
    let has_file = |path: &str| {
        path.starts_with("res://") || {
            let mut at = root.clone();
            path.replace('\\', "/").split('/').filter(|p| !p.is_empty()).all(|part| {
                let found = std::fs::read_dir(&at)
                    .ok()
                    .and_then(|d| d.flatten().find(|e| e.file_name().to_string_lossy().eq_ignore_ascii_case(part)));
                found.map(|e| at = e.path()).is_some()
            }) && at.is_file()
        }
    };
    let differences = compare_sound_databases(&sounds, &committed.to_database().unwrap(), has_file);
    assert!(differences.is_empty(), "{}", differences.join("\n"));
}
