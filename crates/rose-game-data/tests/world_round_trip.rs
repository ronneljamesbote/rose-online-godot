//! Zone settings, warp gates, crafting recipes and new characters from the iROSE tables,
//! written as the export and read back, are the same; so are the zones built from the
//! settings. Needs the client data: ROSE_DATA_IDX=<path to data.idx> (skipped without it).

use rose_data_export::{Format, WorldExport};
use rose_file_readers::{VfsIndex, VirtualFilesystem};

#[test]
fn the_world_survives_the_export() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    let vfs = VirtualFilesystem::new(vec![Box::new(VfsIndex::load(std::path::Path::new(&data_idx)).unwrap())]);
    let strings = rose_data_irose::get_string_database(&vfs, 1).unwrap();
    let items = rose_data_irose::get_item_database(&vfs, strings.clone()).unwrap();
    let (export, lost) = rose_game_data::world_export_from_irose(&vfs, &strings, &items).unwrap();
    for line in &lost {
        eprintln!("not kept: {line}");
    }
    let files = export.to_files_as(Format::Toml).unwrap();
    let read = WorldExport::from_files_as(Format::Toml, |name| {
        Ok(files.iter().find(|(n, _)| *n == name).map(|(_, t)| t.clone()))
    })
    .unwrap()
    .unwrap();
    let world = rose_game_data::world_from_export(&read).unwrap();
    world.check_items(&items).unwrap();
    let irose = rose_game_data::load_world_irose(&vfs, &strings).unwrap();
    let differences = rose_game_data::compare_worlds(&irose, &world);
    assert!(differences.is_empty(), "{}", differences.join("\n"));

    // The zones and the zone list built from the exported settings are the ones iROSE gives.
    let a = rose_data_irose::get_zone_database(&vfs, strings.clone()).unwrap();
    let b = rose_data_irose::get_zone_database_from_settings(&vfs, strings.clone(), &world.zone_settings);
    let zones = |db: &rose_data::ZoneDatabase| {
        db.iter()
            .map(|z| {
                format!(
                    "{} {} {} {} {:?} {:?} {} {} {} {} {} {} {:?} {} {} {}",
                    z.id.get(),
                    z.name,
                    z.description,
                    z.sector_size,
                    z.start_position,
                    z.revive_positions,
                    z.monster_spawns.len(),
                    z.npcs.len(),
                    z.day_cycle,
                    z.morning_time,
                    z.night_time,
                    z.event_positions.len(),
                    z.skybox_id,
                    z.pvp_state,
                    z.planet,
                    z.vehicle_use_flags,
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(zones(&a), zones(&b));
    let a = rose_data_irose::get_zone_list(&vfs, strings.clone()).unwrap();
    let b = rose_data_irose::get_zone_list_from_settings(strings, &world.zone_settings);
    let list = |l: &rose_data::ZoneList| {
        l.iter()
            .map(|z| {
                format!(
                    "{} {} {:?} {} {} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                    z.id.get(),
                    z.name,
                    z.minimap_path,
                    z.minimap_start_x,
                    z.minimap_start_y,
                    z.zon_file_path,
                    z.zsc_cnst_path,
                    z.zsc_deco_path,
                    z.skybox_id,
                    z.background_music_day,
                    z.background_music_night,
                    z.footstep_type,
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(list(&a), list(&b));
    let bytes: usize = files.iter().map(|(_, t)| t.len()).sum();
    eprintln!(
        "{} zones, {} warp gates, {} recipes, {} new characters, {bytes} bytes",
        read.zones.len(),
        read.warp_gates.len(),
        read.craft_recipes.len(),
        read.new_characters.len()
    );
}
