//! The motion lengths and hit frames of `export/motions.toml` (exported from the Godot
//! animations) are the ones in the ZMO files, for every player, monster and NPC motion, and
//! loading them reads no ZMO file. Needs the client data: ROSE_DATA_IDX=<path to data.idx>
//! (skipped without it).

use std::sync::{Arc, Mutex};

use rose_file_readers::{VfsFile, VfsIndex, VfsPath, VirtualFilesystem, VirtualFilesystemDevice};
use rose_game_data::MotionSource;

/// Passes reads through and counts the ZMO files opened.
struct CountZmo<D>(D, Arc<Mutex<usize>>);

impl<D: VirtualFilesystemDevice> VirtualFilesystemDevice for CountZmo<D> {
    fn open_file(&self, path: &VfsPath) -> Result<VfsFile<'_>, anyhow::Error> {
        if path.path().extension().is_some_and(|e| e.eq_ignore_ascii_case("zmo")) {
            *self.1.lock().unwrap() += 1;
        }
        self.0.open_file(path)
    }

    fn exists(&self, path: &VfsPath) -> bool {
        self.0.exists(path)
    }
}

#[test]
fn the_exported_motions_are_the_zmo_ones() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    let opened = Arc::new(Mutex::new(0));
    let index = VfsIndex::load(std::path::Path::new(&data_idx)).unwrap();
    let vfs = VirtualFilesystem::new(vec![Box::new(CountZmo(index, opened.clone()))]);
    let strings = rose_data_irose::get_string_database(&vfs, 1).unwrap();
    let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(rose_data_export::MOTIONS_FILE);
    let timings = rose_data_export::MotionTimings::load(&file).unwrap();
    assert!(!timings.motion.is_empty(), "{} is empty", file.display());

    let export = MotionSource::from_timings(&timings);
    let motions = rose_game_data::load_motions(&vfs, &export).unwrap();
    let (npcs, _) = rose_game_data::load_npcs(&vfs, strings.clone(), &export).unwrap();
    export.check_all_used(&motions, &npcs).unwrap();
    assert_eq!(*opened.lock().unwrap(), 0, "ZMO files read with the export");

    let zmo_motions = rose_game_data::load_motions(&vfs, &MotionSource::Zmo).unwrap();
    let (zmo_npcs, _) = rose_game_data::load_npcs(&vfs, strings, &MotionSource::Zmo).unwrap();
    assert!(*opened.lock().unwrap() > 0);
    let differences = rose_game_data::compare_motions((&zmo_motions, &zmo_npcs), (&motions, &npcs));
    assert!(differences.is_empty(), "{}", differences.join("\n"));
}
