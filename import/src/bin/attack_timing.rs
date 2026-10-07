//! Prints attack animation timing (length and hit frames) for the test character's weapons,
//! so the server's swing timing matches what the client animates.
//! Usage: attack_timing <data.idx> [weapon item ids...]
use std::path::Path;
use rose_data::{CharacterMotionAction, CharacterMotionDatabaseOptions};
use rose_data_irose::{get_character_motion_database, get_item_database, get_string_database};
use rose_file_readers::{HostFilesystemDevice, VfsIndex, VirtualFilesystem, ZmoFile};

/// Frame events that mark a hit in a ZMO attack animation (same set rose-file-readers counts).
pub fn is_attack_event(e: u16) -> bool {
    matches!(e, 10 | 20..=28 | 56..=57 | 66..=67)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let idx = Path::new(&args[1]);
    let ids: Vec<usize> = if args.len() > 2 { args[2..].iter().map(|a| a.parse().unwrap()).collect() } else { vec![2, 202] };
    let vfs = VirtualFilesystem::new(vec![
        Box::new(VfsIndex::load(idx).unwrap()),
        Box::new(HostFilesystemDevice::new(idx.parent().unwrap().to_path_buf())),
    ]);
    let strings = get_string_database(&vfs, 1).unwrap();
    let items = get_item_database(&vfs, strings).unwrap();
    let motions = get_character_motion_database(&vfs, &CharacterMotionDatabaseOptions { load_frame_data: true }).unwrap();
    for id in ids {
        let w = items.get_weapon_item(id).unwrap();
        println!("{} {} motion_type {} attack_speed {} range {}", id, w.item_data.name, w.motion_type, w.attack_speed, w.attack_range);
        for action in [CharacterMotionAction::Attack, CharacterMotionAction::Attack2, CharacterMotionAction::Attack3] {
            let Some(m) = motions.get_character_action_motion(action, w.motion_type as usize, 0) else { continue };
            let zmo = vfs.read_file::<ZmoFile, _>(&m.path).unwrap();
            let hits: Vec<u32> = zmo.frame_events.iter().enumerate()
                .filter(|(_, e)| is_attack_event(**e))
                .map(|(f, _)| (f as u32 * 1000) / zmo.fps as u32).collect();
            println!("  {:?} {} ms, hit at {:?} ms ({})", action, m.duration.as_millis(), hits, m.path.path().display());
        }
    }
}
