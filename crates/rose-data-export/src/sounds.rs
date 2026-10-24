//! Sounds: `sounds.toml` (one `[[sound]]` per sound, iROSE's FILE_SOUND.STB),
//! `hit-sounds.toml` (`[[hit_sounds]]`, the rows of LIST_HITSOUND.STB: what a weapon sounds
//! like hitting armour, cloth or stone) and `footsteps.toml` (`[[footsteps]]`, the columns of
//! LIST_STEPSOUND.STB: the footstep sound of each ground texture of a zone). The keys are the
//! property names of the Godot resources (`RoseSound`, `RoseHitSounds`, `RoseFootsteps`)
//! they are exported from.
//!
//! A sound's `stream` is the path of its Godot audio file (`res://assets/rose/sound/...`,
//! made by the import from the client's WAV and OGG files, or one of our own); a link to a
//! sound is its id. Only the game plays sounds: the server doesn't read these files.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::bail;
use rose_data::{SoundData, SoundDatabase, SoundId};
use serde::{Deserialize, Serialize};

use crate::Format;

pub const SOUNDS_FILE: &str = "sounds";
pub const HIT_SOUNDS_FILE: &str = "hit-sounds";
pub const FOOTSTEPS_FILE: &str = "footsteps";

/// The columns of a hit sound set, by what is hit (a monster's `hit_sound_material_type`;
/// players count as cloth).
pub const HIT_MATERIALS: [&str; 3] = ["armour", "cloth", "stone"];

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

fn number(id: Option<SoundId>) -> u16 {
    id.map_or(0, |id| id.get())
}

fn trim_end(mut list: Vec<u16>) -> Vec<u16> {
    while list.last() == Some(&0) {
        list.pop();
    }
    list
}

/// What two paths of one sound have in common: the client file (`Sound\attack\Sword_001.wav`)
/// and its imported copy (`res://assets/rose/sound/attack/sword_001.res`) both give
/// `sound/attack/sword_001`. Another Godot file keeps its whole path.
pub fn sound_stem(path: &str) -> String {
    let path = path.trim();
    let inner = ["res://assets/rose/", "res://assets/ours/"].iter().find_map(|prefix| path.strip_prefix(prefix));
    if path.starts_with("res://") && inner.is_none() {
        return path.to_string();
    }
    let path = inner.unwrap_or(path).replace('\\', "/").to_ascii_lowercase();
    let path = path.trim_start_matches("./").trim_start_matches('/');
    match path.rfind('.') {
        Some(dot) if !path[dot..].contains('/') => path[..dot].to_string(),
        _ => path.to_string(),
    }
}

/// One sound.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SoundExport {
    /// The number (iROSE's FILE_SOUND.STB row). Items, monsters, skills and the hit and
    /// footstep sounds name it.
    pub id: u16,
    /// For finding it only (the file's name).
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    /// The audio file; empty = silent (iROSE names files the client doesn't have).
    #[serde(skip_serializing_if = "is_default")]
    pub stream: String,
}

/// What one kind of weapon sounds like hitting each material (a weapon's `hit_sounds`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct HitSoundsExport {
    /// The number (iROSE's LIST_HITSOUND.STB row).
    pub id: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    #[serde(skip_serializing_if = "is_default")]
    pub on_armour: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub on_cloth: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub on_stone: u16,
}

/// The footstep sound of each ground texture number in the zones that use it (a zone's
/// `footsteps`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FootstepsExport {
    /// The number (iROSE's LIST_STEPSOUND.STB column, the zone's footstep type).
    pub id: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    /// The sound of ground texture 0, 1, 2 ... (0 = the default step); trailing zeros left out.
    #[serde(skip_serializing_if = "is_default")]
    pub sounds: Vec<u16>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SoundFile {
    #[serde(default)]
    sound: Vec<SoundExport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HitSoundsFile {
    #[serde(default)]
    hit_sounds: Vec<HitSoundsExport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FootstepsFile {
    #[serde(default)]
    footsteps: Vec<FootstepsExport>,
}

/// Every sound, hit sound set and footstep set, as exported.
#[derive(Debug, Default)]
pub struct SoundsExport {
    pub sounds: Vec<SoundExport>,
    pub hit_sounds: Vec<HitSoundsExport>,
    pub footsteps: Vec<FootstepsExport>,
}

/// A sound's name from its file: `Sound\attack\Sword_001.wav` -> `Sword_001`.
fn file_name(path: &str) -> String {
    let file = path.rsplit(['/', '\\']).next().unwrap_or(path);
    file.rsplit_once('.').map_or(file, |(stem, _)| stem).to_string()
}

impl SoundsExport {
    /// The converter's starting point: every sound with a file, every sound something names
    /// (`used`, even without a file, so the link stays), the hit sound rows that have a sound
    /// or that a weapon names (`used_hit_sounds`) and the footstep columns that have a sound.
    /// Sound paths stay as the database has them (the converter links the imported files).
    pub fn from_database(db: &SoundDatabase, used: &BTreeSet<u16>, used_hit_sounds: &BTreeSet<u32>) -> Self {
        let mut sounds: BTreeMap<u16, SoundExport> = db
            .iter()
            .filter(|s| !s.path.is_empty())
            .map(|s| (s.id.get(), SoundExport { id: s.id.get(), name: file_name(&s.path), stream: s.path.clone() }))
            .collect();
        let hits = db.hit_sound_ids();
        let materials = db.hit_sound_material_types().max(1);
        let mut hit_sounds = Vec::new();
        for (row, ids) in hits.chunks(materials).enumerate() {
            let at = |i: usize| number(ids.get(i).copied().flatten());
            let set = HitSoundsExport { id: row as u32, name: String::new(), on_armour: at(0), on_cloth: at(1), on_stone: at(2) };
            if row == 0 || ((set.on_armour, set.on_cloth, set.on_stone) == (0, 0, 0) && !used_hit_sounds.contains(&set.id)) {
                continue;
            }
            hit_sounds.push(set);
        }
        let steps = db.step_sound_ids();
        let types = db.step_sound_zone_types().max(1);
        let mut footsteps = Vec::new();
        for column in 1..types {
            let ids = (0..steps.len() / types).map(|tile| number(steps[column + tile * types])).collect();
            let set = FootstepsExport { id: column as u32, name: String::new(), sounds: trim_end(ids) };
            if !set.sounds.is_empty() {
                footsteps.push(set);
            }
        }
        let named = hit_sounds
            .iter()
            .flat_map(|h| [h.on_armour, h.on_cloth, h.on_stone])
            .chain(footsteps.iter().flat_map(|f| f.sounds.iter().copied()))
            .chain(used.iter().copied());
        for id in named.filter(|id| *id != 0) {
            sounds.entry(id).or_insert_with(|| SoundExport { id, name: format!("sound {id}"), stream: String::new() });
        }
        // A hit sound set is named after its weapon (the family of its first sound's file).
        for set in &mut hit_sounds {
            let first = [set.on_armour, set.on_cloth, set.on_stone].into_iter().find(|id| *id != 0);
            let family = first.and_then(|id| sounds.get(&id)).map(|s| s.name.split('_').next().unwrap_or("").to_string());
            set.name = format!("{} hits", family.filter(|f| !f.is_empty()).unwrap_or_else(|| "weapon".into()));
        }
        Self { sounds: sounds.into_values().collect(), hit_sounds, footsteps }
    }

    /// The sound database the game reads (the columns and rows as iROSE's tables lay them out).
    pub fn to_database(&self) -> Result<SoundDatabase, anyhow::Error> {
        self.check()?;
        let mut sounds: Vec<Option<SoundData>> = Vec::new();
        for sound in &self.sounds {
            let at = sound.id as usize;
            if sounds.len() <= at {
                sounds.resize_with(at + 1, || None);
            }
            sounds[at] = Some(SoundData { id: SoundId::new(sound.id).unwrap(), path: sound.stream.clone() });
        }
        let materials = HIT_MATERIALS.len();
        let rows = self.hit_sounds.iter().map(|h| h.id as usize + 1).max().unwrap_or(0);
        let mut hit_sounds = vec![None; rows * materials];
        for set in &self.hit_sounds {
            let at = set.id as usize * materials;
            for (i, id) in [set.on_armour, set.on_cloth, set.on_stone].into_iter().enumerate() {
                hit_sounds[at + i] = SoundId::new(id);
            }
        }
        let types = self.footsteps.iter().map(|f| f.id as usize + 1).max().unwrap_or(0);
        let tiles = self.footsteps.iter().map(|f| f.sounds.len()).max().unwrap_or(0);
        let mut step_sounds = vec![None; tiles * types];
        for set in &self.footsteps {
            for (tile, id) in set.sounds.iter().enumerate() {
                step_sounds[set.id as usize + tile * types] = SoundId::new(*id);
            }
        }
        Ok(SoundDatabase::new(sounds, step_sounds, types, hit_sounds, materials))
    }

    /// Ids used twice, and links to sounds that aren't there.
    pub fn check(&self) -> Result<(), anyhow::Error> {
        let mut sounds = BTreeSet::new();
        for sound in &self.sounds {
            if sound.id == 0 {
                bail!("sound 0: ids start at 1");
            }
            if !sounds.insert(sound.id) {
                bail!("{SOUNDS_FILE}: sound {} is there twice", sound.id);
            }
        }
        let missing = |what: String, id: u16| -> Result<(), anyhow::Error> {
            if id != 0 && !sounds.contains(&id) {
                bail!("{what} names sound {id}, which isn't there");
            }
            Ok(())
        };
        let mut sets = BTreeSet::new();
        for set in &self.hit_sounds {
            if set.id == 0 {
                bail!("hit sounds 0: ids start at 1 (0 is no hit sound)");
            }
            if !sets.insert(set.id) {
                bail!("{HIT_SOUNDS_FILE}: hit sounds {} are there twice", set.id);
            }
            for id in [set.on_armour, set.on_cloth, set.on_stone] {
                missing(format!("hit sounds {}", set.id), id)?;
            }
        }
        let mut sets = BTreeSet::new();
        for set in &self.footsteps {
            if set.id == 0 {
                bail!("footsteps 0: ids start at 1 (0 is no footsteps)");
            }
            if !sets.insert(set.id) {
                bail!("{FOOTSTEPS_FILE}: footsteps {} are there twice", set.id);
            }
            for id in &set.sounds {
                missing(format!("footsteps {}", set.id), *id)?;
            }
        }
        Ok(())
    }

    /// Whether a link names something that is there: a sound, hit sound set or footstep set.
    pub fn has_sound(&self, id: u16) -> bool {
        id == 0 || self.sounds.iter().any(|s| s.id == id)
    }

    pub fn has_hit_sounds(&self, id: u32) -> bool {
        id == 0 || self.hit_sounds.iter().any(|s| s.id == id)
    }

    pub fn has_footsteps(&self, id: u32) -> bool {
        id == 0 || self.footsteps.iter().any(|s| s.id == id)
    }

    /// The export's files: (file name without extension, contents).
    pub fn to_files_as(&self, format: Format) -> Result<Vec<(&'static str, String)>, anyhow::Error> {
        let mut sound = self.sounds.clone();
        sound.sort_by_key(|s| s.id);
        let mut hit_sounds = self.hit_sounds.clone();
        hit_sounds.sort_by_key(|s| s.id);
        let mut footsteps = self.footsteps.clone();
        footsteps.sort_by_key(|s| s.id);
        Ok(vec![
            (SOUNDS_FILE, format.write(SOUNDS_FILE, &SoundFile { sound })?),
            (HIT_SOUNDS_FILE, format.write(HIT_SOUNDS_FILE, &HitSoundsFile { hit_sounds })?),
            (FOOTSTEPS_FILE, format.write(FOOTSTEPS_FILE, &FootstepsFile { footsteps })?),
        ])
    }

    /// Reads the export's files through `read` (file name without extension -> its text, or
    /// None when it isn't there). None when there is none of them; some missing is an error.
    pub fn from_files_as(
        format: Format,
        mut read: impl FnMut(&str) -> Result<Option<String>, anyhow::Error>,
    ) -> Result<Option<Self>, anyhow::Error> {
        let files = [SOUNDS_FILE, HIT_SOUNDS_FILE, FOOTSTEPS_FILE];
        let mut texts = Vec::new();
        for name in files {
            texts.push(read(name)?);
        }
        match texts.iter().filter(|t| t.is_some()).count() {
            0 => return Ok(None),
            n if n < files.len() => bail!(
                "the sound export has no {}",
                files
                    .iter()
                    .zip(&texts)
                    .filter(|(_, t)| t.is_none())
                    .map(|(n, _)| format!("{n}.{}", format.extension()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            _ => {}
        }
        let text = |i: usize| texts[i].as_deref().unwrap();
        Ok(Some(Self {
            sounds: format.parse::<SoundFile>(SOUNDS_FILE, text(0))?.sound,
            hit_sounds: format.parse::<HitSoundsFile>(HIT_SOUNDS_FILE, text(1))?.hit_sounds,
            footsteps: format.parse::<FootstepsFile>(FOOTSTEPS_FILE, text(2))?.footsteps,
        }))
    }
}

/// Every difference between two sound databases as the game uses them: each sound's file
/// (by `sound_stem`, so a client file and its imported copy are the same; a file `has_file`
/// says isn't there counts as none, as the game plays nothing for it), and the sound of every
/// weapon hit type and material and of every ground texture and footstep type.
pub fn compare_sound_databases(a: &SoundDatabase, b: &SoundDatabase, has_file: impl Fn(&str) -> bool) -> Vec<String> {
    let mut out = Vec::new();
    let files = |db: &SoundDatabase| {
        db.iter()
            .filter(|s| !s.path.is_empty() && has_file(&s.path))
            .map(|s| (s.id.get(), sound_stem(&s.path)))
            .collect::<BTreeMap<_, _>>()
    };
    let (fa, fb) = (files(a), files(b));
    for id in fa.keys().chain(fb.keys()).collect::<BTreeSet<_>>() {
        match (fa.get(id), fb.get(id)) {
            (Some(x), Some(y)) if x == y => {}
            (x, y) => out.push(format!("sound {id}: {} -> {}", x.map_or("none", |s| s.as_str()), y.map_or("none", |s| s.as_str()))),
        }
    }
    let stem = |s: Option<&SoundData>| s.filter(|s| !s.path.is_empty() && has_file(&s.path)).map(|s| sound_stem(&s.path));
    let rows = |db: &SoundDatabase| db.hit_sound_ids().len() / db.hit_sound_material_types().max(1);
    for row in 0..rows(a).max(rows(b)) {
        for (material, name) in HIT_MATERIALS.iter().enumerate() {
            let (x, y) = (stem(a.get_hit_sound(row, material)), stem(b.get_hit_sound(row, material)));
            if x != y {
                out.push(format!("hit sounds {row} on {name}: {x:?} -> {y:?}"));
            }
        }
    }
    let tiles = |db: &SoundDatabase| db.step_sound_ids().len() / db.step_sound_zone_types().max(1);
    let types = a.step_sound_zone_types().max(b.step_sound_zone_types());
    for tile in 0..tiles(a).max(tiles(b)) {
        for zone_type in 0..types {
            let get = |db: &SoundDatabase| if zone_type < db.step_sound_zone_types() { stem(db.get_step_sound(tile, zone_type)) } else { None };
            let (x, y) = (get(a), get(b));
            if x != y {
                out.push(format!("footsteps {zone_type} on ground texture {tile}: {x:?} -> {y:?}"));
            }
        }
    }
    out
}
