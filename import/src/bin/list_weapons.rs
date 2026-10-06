//! Lists weapons with no ability requirement, to pick test-character gear.
//! Usage: list_weapons <data.idx> [max id]
use std::path::Path;
use rose_data_irose::{get_item_database, get_string_database};
use rose_file_readers::{HostFilesystemDevice, VfsIndex, VirtualFilesystem};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let idx = Path::new(&args[1]);
    let max: usize = args.get(2).and_then(|a| a.parse().ok()).unwrap_or(400);
    let vfs = VirtualFilesystem::new(vec![
        Box::new(VfsIndex::load(idx).unwrap()),
        Box::new(HostFilesystemDevice::new(idx.parent().unwrap().to_path_buf())),
    ]);
    let strings = get_string_database(&vfs, 1).unwrap();
    let items = get_item_database(&vfs, strings).unwrap();
    for id in 1..max {
        if let Some(w) = items.get_weapon_item(id) {
            if w.item_data.name.is_empty() { continue; }
            println!("{:4} {:?} {:30} range {:5} atk {:3} spd {:3} motion {} req {:?}", id, w.item_data.class, w.item_data.name,
                w.attack_range, w.attack_power, w.attack_speed, w.motion_type, w.item_data.equip_ability_requirement);
        }
    }
}
