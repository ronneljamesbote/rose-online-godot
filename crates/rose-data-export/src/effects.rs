//! Effects: `effect-files.toml` (one `[[effect_file]]` per effect, iROSE's FILE_EFFECT.STB:
//! an effect scene by number) and `attack-effects.toml` (`[[attack_effect]]`, the rows of
//! LIST_EFFECT.STB: how an attack looks, with the effects on the weapon, its trail, the
//! projectile and the hit). The keys are the property names of the Godot resources
//! (`RoseEffectFile`, `RoseAttackEffect`) they are exported from.
//!
//! An effect file's `scene` is the path of its Godot effect scene
//! (`res://assets/rose/3ddata/effect/levelup_01.scn`, made by the import from the client's
//! EFT and PTL files, or one of our own); a link to an effect file, an attack effect or a
//! sound is its id. Only the game shows effects: the server doesn't read these files.

use std::{
    collections::{BTreeMap, BTreeSet},
    num::NonZeroUsize,
    time::Duration,
};

use anyhow::bail;
use arrayvec::ArrayVec;
use rose_data::{EffectBulletMoveType, EffectData, EffectDatabase, EffectFileId, EffectId, SoundId};
use serde::{Deserialize, Serialize};

use crate::Format;

pub const EFFECT_FILES_FILE: &str = "effect-files";
pub const ATTACK_EFFECTS_FILE: &str = "attack-effects";

/// The most effects an attack effect puts on the weapon.
pub const POINT_EFFECTS: usize = 4;

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

fn number<T: Copy>(id: Option<T>, get: fn(&T) -> u16) -> u16 {
    id.as_ref().map_or(0, get)
}

/// What two paths of one effect have in common: the client file (`3DData\Effect\levelup_01.eft`)
/// and its imported scene (`res://assets/rose/3ddata/effect/levelup_01.scn`) both give
/// `3ddata/effect/levelup_01`. Another Godot file keeps its whole path.
pub fn effect_stem(path: &str) -> String {
    crate::sounds::sound_stem(path)
}

/// The names of the bullet move types, as the export writes them.
fn move_name(move_type: Option<EffectBulletMoveType>) -> String {
    match move_type {
        None => String::new(),
        Some(EffectBulletMoveType::Linear) => "Linear".into(),
        Some(EffectBulletMoveType::Parabola) => "Parabola".into(),
        Some(EffectBulletMoveType::Immediate) => "Immediate".into(),
    }
}

fn move_type(name: &str) -> Result<Option<EffectBulletMoveType>, anyhow::Error> {
    Ok(match name {
        "" => None,
        "Linear" => Some(EffectBulletMoveType::Linear),
        "Parabola" => Some(EffectBulletMoveType::Parabola),
        "Immediate" => Some(EffectBulletMoveType::Immediate),
        other => bail!("bullet_move {other:?} is not Linear, Parabola or Immediate"),
    })
}

/// One effect file.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct EffectFileExport {
    /// The number (iROSE's FILE_EFFECT.STB row). Items, monsters, skills, status effects and
    /// the attack effects name it.
    pub id: u16,
    /// For finding it only (the file's name).
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    /// The effect scene; empty = nothing to show (iROSE names files the client doesn't have).
    #[serde(skip_serializing_if = "is_default")]
    pub scene: String,
}

/// How one attack looks (a weapon's `attack_effect`, a projectile's `bullet_effect` ...).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AttackEffectExport {
    /// The number (iROSE's LIST_EFFECT.STB row).
    pub id: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub name: String,
    /// Effects drawn on the weapon's effect points, in order (up to 4).
    #[serde(skip_serializing_if = "is_default")]
    pub point_effects: Vec<u16>,
    /// The colour of the swing's trail (iROSE's trail colour number, 0 = no trail).
    #[serde(skip_serializing_if = "is_default")]
    pub trail_colour: u32,
    /// How long the trail stays, in milliseconds.
    #[serde(skip_serializing_if = "is_default")]
    pub trail_milliseconds: u32,
    #[serde(skip_serializing_if = "is_default")]
    pub hit_effect: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub hit_effect_critical: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub bullet_effect: u16,
    /// `Linear`, `Parabola` or `Immediate` (empty = none, flies straight).
    #[serde(skip_serializing_if = "is_default")]
    pub bullet_move: String,
    /// Centimetres per second.
    #[serde(skip_serializing_if = "is_default")]
    pub bullet_speed: f32,
    #[serde(skip_serializing_if = "is_default")]
    pub fire_sound: u16,
    #[serde(skip_serializing_if = "is_default")]
    pub hit_sound: u16,
}

impl AttackEffectExport {
    pub fn from_data(effect: &EffectData) -> Self {
        Self {
            id: effect.id.get(),
            name: String::new(),
            point_effects: effect.point_effects.iter().map(|id| id.get()).collect(),
            trail_colour: effect.trail_colour_index.map_or(0, |c| c.get() as u32),
            trail_milliseconds: effect.trail_duration.as_millis() as u32,
            hit_effect: number(effect.hit_effect_normal, EffectFileId::get),
            hit_effect_critical: number(effect.hit_effect_critical, EffectFileId::get),
            bullet_effect: number(effect.bullet_effect, EffectFileId::get),
            bullet_move: move_name(effect.bullet_move_type),
            bullet_speed: effect.bullet_speed,
            fire_sound: number(effect.fire_sound_id, SoundId::get),
            hit_sound: number(effect.hit_sound_id, SoundId::get),
        }
    }

    pub fn to_data(&self) -> Result<EffectData, anyhow::Error> {
        if self.point_effects.len() > POINT_EFFECTS {
            bail!("attack effect {}: {} point effects, at most {POINT_EFFECTS}", self.id, self.point_effects.len());
        }
        let Some(id) = EffectId::new(self.id) else { bail!("attack effect 0: ids start at 1") };
        let mut point_effects = ArrayVec::new();
        for id in self.point_effects.iter().filter_map(|id| EffectFileId::new(*id)) {
            point_effects.push(id);
        }
        Ok(EffectData {
            id,
            point_effects,
            trail_colour_index: NonZeroUsize::new(self.trail_colour as usize),
            trail_duration: Duration::from_millis(self.trail_milliseconds as u64),
            hit_effect_normal: EffectFileId::new(self.hit_effect),
            hit_effect_critical: EffectFileId::new(self.hit_effect_critical),
            bullet_effect: EffectFileId::new(self.bullet_effect),
            bullet_move_type: move_type(&self.bullet_move)?,
            bullet_speed: self.bullet_speed,
            fire_sound_id: SoundId::new(self.fire_sound),
            hit_sound_id: SoundId::new(self.hit_sound),
        })
    }

    /// The effect files it names.
    pub fn effect_files(&self) -> impl Iterator<Item = u16> + '_ {
        self.point_effects
            .iter()
            .copied()
            .chain([self.hit_effect, self.hit_effect_critical, self.bullet_effect])
            .filter(|id| *id != 0)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EffectFileFile {
    #[serde(default)]
    effect_file: Vec<EffectFileExport>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AttackEffectFile {
    #[serde(default)]
    attack_effect: Vec<AttackEffectExport>,
}

/// Every effect file and attack effect, as exported.
#[derive(Debug, Default)]
pub struct EffectsExport {
    pub effect_files: Vec<EffectFileExport>,
    pub attack_effects: Vec<AttackEffectExport>,
}

/// An effect's name from its file: `3DData\Effect\levelup_01.eft` -> `levelup_01`.
fn file_name(path: &str) -> String {
    let file = path.rsplit(['/', '\\']).next().unwrap_or(path);
    file.rsplit_once('.').map_or(file, |(stem, _)| stem).to_string()
}

impl EffectsExport {
    /// The converter's starting point: every effect file with a file, every one something
    /// names (`used`, even without a file, so the link stays) and every attack effect row that
    /// has something in it or that something names (`used_attack_effects`). Paths stay as the
    /// database has them (the converter links the imported scenes); attack effects get their
    /// names from the converter.
    pub fn from_database(db: &EffectDatabase, used: &BTreeSet<u16>, used_attack_effects: &BTreeSet<u16>) -> Self {
        let mut effect_files: BTreeMap<u16, EffectFileExport> = db
            .iter_files()
            .map(|(id, path)| (id.get(), EffectFileExport { id: id.get(), name: file_name(path), scene: path.to_string() }))
            .collect();
        let attack_effects: Vec<AttackEffectExport> = db
            .iter()
            .map(AttackEffectExport::from_data)
            .filter(|effect| {
                let empty = AttackEffectExport { id: effect.id, ..Default::default() };
                *effect != empty || used_attack_effects.contains(&effect.id)
            })
            .collect();
        let named = attack_effects.iter().flat_map(|e| e.effect_files().collect::<Vec<_>>()).chain(used.iter().copied());
        for id in named.filter(|id| *id != 0) {
            effect_files.entry(id).or_insert_with(|| EffectFileExport { id, name: format!("effect {id}"), scene: String::new() });
        }
        Self { effect_files: effect_files.into_values().collect(), attack_effects }
    }

    /// The effect database the game reads.
    pub fn to_database(&self) -> Result<EffectDatabase, anyhow::Error> {
        self.check()?;
        let mut files: Vec<Option<String>> = Vec::new();
        for file in &self.effect_files {
            let at = file.id as usize;
            if files.len() <= at {
                files.resize_with(at + 1, || None);
            }
            files[at] = Some(file.scene.clone()).filter(|s| !s.is_empty());
        }
        let mut effects: Vec<Option<EffectData>> = Vec::new();
        for effect in &self.attack_effects {
            let at = effect.id as usize;
            if effects.len() <= at {
                effects.resize_with(at + 1, || None);
            }
            effects[at] = Some(effect.to_data()?);
        }
        Ok(EffectDatabase::new(effects, files))
    }

    /// Ids used twice, and links to effect files that aren't there. (Links to sounds are
    /// checked against the sound export by `--export-data`.)
    pub fn check(&self) -> Result<(), anyhow::Error> {
        let mut files = BTreeSet::new();
        for file in &self.effect_files {
            if file.id == 0 {
                bail!("effect file 0: ids start at 1");
            }
            if !files.insert(file.id) {
                bail!("{EFFECT_FILES_FILE}: effect file {} is there twice", file.id);
            }
        }
        let mut effects = BTreeSet::new();
        for effect in &self.attack_effects {
            if effect.id == 0 {
                bail!("attack effect 0: ids start at 1 (0 is no effect)");
            }
            if !effects.insert(effect.id) {
                bail!("{ATTACK_EFFECTS_FILE}: attack effect {} is there twice", effect.id);
            }
            if let Some(id) = effect.effect_files().find(|id| !files.contains(id)) {
                bail!("attack effect {} names effect file {id}, which isn't there", effect.id);
            }
            move_type(&effect.bullet_move).map_err(|e| e.context(format!("attack effect {}", effect.id)))?;
        }
        Ok(())
    }

    /// Whether a link names something that is there: an effect file or an attack effect.
    pub fn has_effect_file(&self, id: u16) -> bool {
        id == 0 || self.effect_files.iter().any(|e| e.id == id)
    }

    pub fn has_attack_effect(&self, id: u16) -> bool {
        id == 0 || self.attack_effects.iter().any(|e| e.id == id)
    }

    /// The export's files: (file name without extension, contents).
    pub fn to_files_as(&self, format: Format) -> Result<Vec<(&'static str, String)>, anyhow::Error> {
        let mut effect_file = self.effect_files.clone();
        effect_file.sort_by_key(|e| e.id);
        let mut attack_effect = self.attack_effects.clone();
        attack_effect.sort_by_key(|e| e.id);
        Ok(vec![
            (EFFECT_FILES_FILE, format.write(EFFECT_FILES_FILE, &EffectFileFile { effect_file })?),
            (ATTACK_EFFECTS_FILE, format.write(ATTACK_EFFECTS_FILE, &AttackEffectFile { attack_effect })?),
        ])
    }

    /// Reads the export's files through `read` (file name without extension -> its text, or
    /// None when it isn't there). None when there is none of them; one missing is an error.
    pub fn from_files_as(
        format: Format,
        mut read: impl FnMut(&str) -> Result<Option<String>, anyhow::Error>,
    ) -> Result<Option<Self>, anyhow::Error> {
        let files = [EFFECT_FILES_FILE, ATTACK_EFFECTS_FILE];
        let mut texts = Vec::new();
        for name in files {
            texts.push(read(name)?);
        }
        match texts.iter().filter(|t| t.is_some()).count() {
            0 => return Ok(None),
            n if n < files.len() => bail!(
                "the effect export has no {}",
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
            effect_files: format.parse::<EffectFileFile>(EFFECT_FILES_FILE, text(0))?.effect_file,
            attack_effects: format.parse::<AttackEffectFile>(ATTACK_EFFECTS_FILE, text(1))?.attack_effect,
        }))
    }
}

/// Every difference between two effect databases as the game uses them: each effect file
/// (by `effect_stem`, so a client file and its imported scene are the same; a file `has_file`
/// says isn't there counts as none, as the game shows nothing for it), and every value of
/// every attack effect.
pub fn compare_effect_databases(a: &EffectDatabase, b: &EffectDatabase, has_file: impl Fn(&str) -> bool) -> Vec<String> {
    let mut out = Vec::new();
    let files = |db: &EffectDatabase| {
        db.iter_files().filter(|(_, path)| has_file(path)).map(|(id, path)| (id.get(), effect_stem(path))).collect::<BTreeMap<_, _>>()
    };
    let (fa, fb) = (files(a), files(b));
    for id in fa.keys().chain(fb.keys()).collect::<BTreeSet<_>>() {
        match (fa.get(id), fb.get(id)) {
            (Some(x), Some(y)) if x == y => {}
            (x, y) => out.push(format!("effect file {id}: {} -> {}", x.map_or("none", |s| s.as_str()), y.map_or("none", |s| s.as_str()))),
        }
    }
    let effects = |db: &EffectDatabase| db.iter().map(|e| (e.id.get(), AttackEffectExport::from_data(e))).collect::<BTreeMap<_, _>>();
    let (ea, eb) = (effects(a), effects(b));
    let empty = |id: u16| AttackEffectExport { id, ..Default::default() };
    for id in ea.keys().chain(eb.keys()).collect::<BTreeSet<_>>() {
        let x = ea.get(id).cloned().unwrap_or_else(|| empty(*id));
        let y = eb.get(id).cloned().unwrap_or_else(|| empty(*id));
        if x != y {
            out.push(format!("attack effect {id}: {x:?} -> {y:?}"));
        }
    }
    out
}
