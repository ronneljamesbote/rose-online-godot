//! Skills, status effects and job classes from the iROSE tables, written as the export and
//! read back, are the same. Needs the client data: ROSE_DATA_IDX=<path to data.idx>
//! (skipped without it).

use rose_data_export::{compare_skill_databases, Format, SkillsExport};
use rose_file_readers::{VfsIndex, VirtualFilesystem};

#[test]
fn skills_survive_the_export() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    let vfs = VirtualFilesystem::new(vec![Box::new(VfsIndex::load(std::path::Path::new(&data_idx)).unwrap())]);
    let strings = rose_data_irose::get_string_database(&vfs, 1).unwrap();
    let skills = rose_data_irose::get_skill_database(&vfs, strings.clone()).unwrap();
    let effects = rose_data_irose::get_status_effect_database(&vfs, strings.clone()).unwrap();
    let classes = rose_data_irose::get_job_class_database(&vfs, strings.clone()).unwrap();
    let files = SkillsExport::from_databases(&skills, &effects, &classes).to_files_as(Format::Toml).unwrap();
    let read = SkillsExport::from_files_as(Format::Toml, |name| {
        Ok(files.iter().find(|(n, _)| *n == name).map(|(_, t)| t.clone()))
    })
    .unwrap()
    .unwrap();
    let (s, e, c) = read.to_databases(strings).unwrap();
    let differences = compare_skill_databases((&skills, &effects, &classes), (&s, &e, &c));
    assert!(differences.is_empty(), "{}", differences.join("\n"));
    let bytes: usize = files.iter().map(|(_, t)| t.len()).sum();
    eprintln!(
        "{} skills, {} status effects, {} job classes, {bytes} bytes",
        read.skills.len(),
        read.status_effects.len(),
        read.job_classes.len()
    );
}
