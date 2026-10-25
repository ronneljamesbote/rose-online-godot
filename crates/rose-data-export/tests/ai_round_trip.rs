//! The monster AI from the iROSE files (FILE_AI.STB, the AIP files and AI_S.STB), written as
//! the export and read back, is the same. Needs the client data: ROSE_DATA_IDX=<path to
//! data.idx> (skipped without it).

use rose_data_export::{compare_ai_databases, AiExport, Format, ItemCodes};
use rose_file_readers::{VfsIndex, VirtualFilesystem};

fn codes() -> ItemCodes {
    ItemCodes {
        decode: rose_data_irose::decode_item_base1000,
        encode: |item| {
            let item_type = rose_data_irose::encode_item_type(item.item_type)?;
            (item.item_number < 1000).then(|| (item_type * 1000 + item.item_number) as i32)
        },
    }
}

#[test]
fn ai_survives_the_export() {
    let Some(data_idx) = std::env::var_os("ROSE_DATA_IDX") else {
        eprintln!("ROSE_DATA_IDX is not set, skipped");
        return;
    };
    let vfs = VirtualFilesystem::new(vec![Box::new(VfsIndex::load(std::path::Path::new(&data_idx)).unwrap())]);
    let ai = rose_data_irose::get_ai_database(&vfs).unwrap();
    let export = AiExport::from_database(&ai, &Default::default(), &codes()).unwrap();
    let files = export.to_files_as(Format::Toml).unwrap();
    let read = AiExport::from_files_as(Format::Toml, |name| Ok(files.iter().find(|(n, _)| *n == name).map(|(_, t)| t.clone())))
        .unwrap()
        .unwrap();
    assert_eq!(read, export);
    let back = read.to_database(&codes()).unwrap();
    // Nothing is folded away here: every monster, skill and quest step counts as there.
    let mut everything = rose_data_export::AiKnown { npcs: (1..u16::MAX).collect(), skills: (0..u16::MAX).collect(), ..Default::default() };
    everything.steps = read.named().steps.iter().map(|s| s.to_string()).collect();
    let differences = compare_ai_databases(&ai, &back, &codes(), &everything).unwrap();
    assert!(differences.is_empty(), "{}", differences.join("\n"));
    // And block by block as the server reads them (a text by what it says).
    let shown = |db: &rose_data::AiDatabase, a: &rose_file_readers::AipAction| match a {
        rose_file_readers::AipAction::Say(id) => format!("Say({:?})", db.get_ai_string(*id)),
        rose_file_readers::AipAction::Message(t, id) => format!("Message({t:?}, {:?})", db.get_ai_string(*id)),
        other => format!("{other:?}"),
    };
    let mut blocks = 0;
    for (id, aip) in &ai.aips {
        let other = &back.aips[id];
        assert_eq!(aip.idle_trigger_interval, other.idle_trigger_interval);
        assert_eq!(aip.damage_trigger_new_target_chance, other.damage_trigger_new_target_chance);
        fn triggers(f: &rose_file_readers::AipFile) -> [Option<Vec<&rose_file_readers::AipEvent>>; 6] {
            [&f.trigger_on_created, &f.trigger_on_idle, &f.trigger_on_attack_move, &f.trigger_on_damaged, &f.trigger_on_kill, &f.trigger_on_dead]
                .map(|t| t.as_ref().map(|t| t.events.iter().collect::<Vec<_>>()))
        }
        for (x, y) in triggers(aip).iter().zip(triggers(other).iter()) {
            let (x, y) = (x.as_ref().expect("iROSE has every trigger"), y.as_ref().expect("so does the export"));
            assert_eq!(x.len(), y.len(), "AI {id}");
            for (e, f) in x.iter().zip(y.iter()) {
                assert_eq!(e.name, f.name);
                assert_eq!(format!("{:?}", e.conditions), format!("{:?}", f.conditions), "AI {id}");
                let actions = |db, e: &rose_file_readers::AipEvent| e.actions.iter().map(|a| shown(db, a)).collect::<Vec<_>>();
                assert_eq!(actions(&ai, e), actions(&back, f), "AI {id}");
                blocks += e.conditions.len() + e.actions.len();
            }
        }
    }
    eprintln!("{blocks} blocks the same");
    let events: usize = read.ai.iter().map(|a| a.events().count()).sum();
    let bytes: usize = files.iter().map(|(_, t)| t.len()).sum();
    eprintln!("{} AIs, {events} events, {} texts, {bytes} bytes", read.ai.len(), back.strings.len());
}
