//! Items for the UI and the world: icons and ground models by their path (the imported
//! resources, or without them the client's icon sheets and LIST_FIELDITEM.ZSC), and item
//! descriptions for slots and tooltips.

use std::collections::HashMap;

use godot::{
    classes::{AtlasTexture, MeshInstance3D, PackedScene, Texture2D},
    prelude::*,
};
use rose_data::{AbilityType, Item, ItemClass, ItemType};
use rose_file_readers::{TsiFile, ZscFile};

use crate::{assets, data, material, mesh, texture};

/// Icon sheets read from the client, by path (without the import, or for an icon the import
/// doesn't have).
fn icon_sheet(path: &str) -> Option<&'static TsiFile> {
    static SHEETS: std::sync::Mutex<Option<HashMap<String, Option<&'static TsiFile>>>> = std::sync::Mutex::new(None);
    let mut sheets = SHEETS.lock().ok()?;
    *sheets
        .get_or_insert_with(HashMap::new)
        .entry(path.to_ascii_uppercase())
        .or_insert_with(|| data::read_file::<TsiFile>(path).map(|sheet| &*Box::leak(Box::new(sheet))))
}

thread_local! {
    static ICONS: std::cell::RefCell<HashMap<String, Option<Gd<Texture2D>>>> = Default::default();
}

pub fn clear_cache() {
    ICONS.with(|icons| icons.borrow_mut().clear());
}

/// The item icon with this number in the item icon sheet (the interface sample).
pub fn item_icon(index: u32) -> Option<Gd<Texture2D>> {
    icon(&rose_data::art::client_part(rose_data::art::ITEM_ICONS, index))
}

/// An icon by its path (an item's, skill's or status effect's `icon`, see `rose_data::art`):
/// the Godot texture (ours or the imported one), else the sprite cut from the client's icon
/// sheet. None for an empty path or one that names nothing.
pub fn icon(path: &str) -> Option<Gd<Texture2D>> {
    if path.trim().is_empty() {
        return None;
    }
    if let Some(cached) = ICONS.with(|icons| icons.borrow().get(path).cloned()) {
        return cached;
    }
    let texture = load_icon(path);
    if texture.is_none() {
        godot_warn!("rose: no icon {path}");
    }
    ICONS.with(|icons| icons.borrow_mut().insert(path.to_string(), texture.clone()));
    texture
}

fn load_icon(path: &str) -> Option<Gd<Texture2D>> {
    let stem = rose_data::art::art_stem(path);
    if stem.starts_with("res://") {
        let mut loader = godot::classes::ResourceLoader::singleton();
        return loader.exists(&stem).then(|| loader.load(&stem)).flatten().and_then(|r| r.try_cast::<Texture2D>().ok());
    }
    if let Some(texture) = assets::load_stem::<Texture2D>(&stem, ICON_EXTENSIONS) {
        return Some(texture);
    }
    let (list, index) = rose_data::art::client_list(path)?;
    let sheet = icon_sheet(&format!("{list}.TSI"))?;
    let sprite = sheet.sprites.get(index as usize)?;
    let file = sheet.textures.get(sprite.texture_id as usize)?;
    let folder = list.rsplit_once('/').map_or("", |(folder, _)| folder);
    let atlas_texture = texture::load_texture(&format!("{folder}/{}", file.filename))?;
    let mut icon = AtlasTexture::new_gd();
    icon.set_atlas(&atlas_texture);
    icon.set_region(sprite_region(sprite));
    Some(icon.upcast())
}

/// The imported textures of the icon sheets. A caller loading many icons (the export) holds
/// these, so Godot doesn't free a sheet between two icons and decode it again.
pub fn icon_sheets() -> Vec<Gd<Texture2D>> {
    let mut out = Vec::new();
    for sheet_path in [rose_data::art::ITEM_ICONS, rose_data::art::SKILL_ICONS, rose_data::art::STATUS_EFFECT_ICONS] {
        let Some(sheet) = icon_sheet(sheet_path) else { continue };
        let folder = sheet_path.rsplit_once('/').map_or("", |(folder, _)| folder);
        out.extend(sheet.textures.iter().filter_map(|t| assets::load_texture(&format!("{folder}/{}", t.filename))));
    }
    out
}

/// Where an icon's picture sits on its sheet.
fn sprite_region(sprite: &rose_file_readers::TsiSprite) -> Rect2 {
    Rect2::new(
        Vector2::new(sprite.left as f32, sprite.top as f32),
        Vector2::new((sprite.right - sprite.left) as f32, (sprite.bottom - sprite.top) as f32),
    )
}

/// What an icon of ours may be: the imported ones are `.tres` (an AtlasTexture on its sheet).
pub const ICON_EXTENSIONS: &[&str] = &["tres", "png", "webp", "res"];

fn ability_name(ability: AbilityType) -> String {
    match ability {
        AbilityType::Level => "Level".into(),
        AbilityType::Strength => "STR".into(),
        AbilityType::Dexterity => "DEX".into(),
        AbilityType::Intelligence => "INT".into(),
        AbilityType::Concentration => "CON".into(),
        AbilityType::Charm => "CHA".into(),
        AbilityType::Sense => "SEN".into(),
        AbilityType::Health => "HP".into(),
        AbilityType::Mana => "MP".into(),
        AbilityType::MaxHealth => "Max HP".into(),
        AbilityType::MaxMana => "Max MP".into(),
        other => format!("{other:?}"),
    }
}

fn class_name(class: ItemClass) -> String {
    // ItemClass names read well split at capitals: OneHandedSword -> "One Handed Sword".
    let raw = format!("{class:?}");
    let mut out = String::new();
    for (i, ch) in raw.chars().enumerate() {
        if i > 0 && ch.is_uppercase() {
            out.push(' ');
        }
        out.push(ch);
    }
    out
}

/// Everything a slot or tooltip shows for an item: name, quantity, icon, model (field model
/// index), type, class, and tooltip lines.
pub fn item_dict(item: &Item) -> VarDictionary {
    let mut d = VarDictionary::new();
    let reference = item.get_item_reference();
    d.set("type", format!("{:?}", reference.item_type).as_str());
    d.set("number", reference.item_number as i64);
    d.set("quantity", item.get_quantity() as i64);
    let Some(game) = data::get() else { return d };
    let Some(base) = game.items.get_base_item(reference) else {
        d.set("name", format!("Item {:?} {}", reference.item_type, reference.item_number).as_str());
        return d;
    };
    d.set("name", base.name);
    d.set("tradeable", base.trade_restriction & 0x02 == 0);
    d.set("model", base.field_model.as_str());
    d.set("class", class_name(base.class).as_str());
    if let Some(icon) = icon(&base.icon) {
        d.set("icon", &icon);
    }

    let mut lines: Vec<String> = vec![class_name(base.class)];
    match reference.item_type {
        ItemType::Weapon => {
            if let Some(w) = game.items.get_weapon_item(reference.item_number) {
                lines.push(format!("Attack {}   Range {:.1} m", w.attack_power, w.attack_range as f32 / 100.0));
            }
        }
        ItemType::Vehicle => {
            if let Some(v) = game.items.get_vehicle_item(reference.item_number) {
                if v.move_speed > 0 {
                    lines.push(format!("Speed {}", v.move_speed));
                }
                if v.attack_power > 0 {
                    lines.push(format!("Attack {}   Range {:.1} m", v.attack_power, v.attack_range as f32 / 100.0));
                }
                if v.fuel_use_rate > 0 {
                    lines.push(format!("Uses {} fuel", v.fuel_use_rate));
                }
            }
        }
        ItemType::Consumable | ItemType::Material | ItemType::Gem | ItemType::Quest => {}
        _ => {
            if base.defence > 0 || base.resistance > 0 {
                lines.push(format!("Defence {}   Magic resist {}", base.defence, base.resistance));
            }
        }
    }
    if let Item::Equipment(e) = item {
        d.set("grade", e.grade as i64);
        d.set("life", e.life as i64);
        d.set("socket", e.has_socket);
        d.set("bonus", e.gem > 0 && e.gem <= 300);
        if e.grade > 0 {
            lines[0] = format!("{} +{}", lines[0], e.grade);
        }
        let is_engine = game.items.get_vehicle_item(reference.item_number).is_some_and(|v| {
            reference.item_type == ItemType::Vehicle && v.vehicle_part == rose_data::VehiclePartIndex::Engine
        });
        if is_engine {
            lines.push(format!("Fuel {}%", e.life / 10));
        } else {
            lines.push(format!("Durability {}   Life {}%", e.durability, e.life / 10));
            if e.life == 0 {
                lines.push("Broken: no stats until repaired".to_string());
            }
        }
        d.set("durability", e.durability as i64);
        if e.is_crafted {
            lines.push("Crafted".to_string());
        }
        // Gem numbers above 300 are set gems; lower ones are a bonus option.
        if let Some(gem) = (e.gem > 0).then(|| game.items.get_gem_item(e.gem as usize)).flatten() {
            let bonus = gem_bonus(gem);
            lines.push(if e.gem > 300 { format!("Gem: {} ({bonus})", gem.item_data.name) } else { format!("Bonus: {bonus}") });
        }
        if e.has_socket && e.gem <= 300 {
            lines.push("Empty gem socket".to_string());
        }
    }
    if reference.item_type == ItemType::Gem {
        if let Some(gem) = game.items.get_gem_item(reference.item_number) {
            lines.push(gem_bonus(gem));
        }
    }
    for (ability, value) in base.add_ability.iter() {
        lines.push(format!("{} {:+}", ability_name(*ability), value));
    }
    for (ability, value) in base.equip_ability_requirement.iter() {
        lines.push(format!("Needs {} {}", ability_name(*ability), value));
    }
    if let Some(job_class) = base.equip_job_class_requirement.and_then(|id| game.job_classes.get(id)) {
        lines.push(format!("For {}", job_class.name));
    }
    if reference.item_type == ItemType::Consumable {
        if let Some(c) = game.items.get_consumable_item(reference.item_number) {
            if let Some((ability, value)) = c.ability_requirement {
                lines.push(format!("Needs {} {}", ability_name(ability), value));
            }
        }
    }
    if base.weight > 0 {
        lines.push(format!("Weight {}", base.weight));
    }
    if !base.description.is_empty() {
        lines.push(base.description.to_string());
    }
    d.set("tooltip", lines.join("\n").as_str());
    d
}

/// A gem's stat bonuses, like "Attack +4, Hit +2".
fn gem_bonus(gem: &rose_data::GemItemData) -> String {
    gem.gem_add_ability.iter().map(|(ability, value)| format!("{} {:+}", ability_name(*ability), value)).collect::<Vec<_>>().join(", ")
}

/// An item lying on the ground.
#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct RoseFieldItem {
    base: Base<Node3D>,
}

#[godot_api]
impl RoseFieldItem {
    /// Builds a ground model by its path (an item's `model`, see `item_dict`; money's is
    /// object 0 of the ground model list). Returns false for none (empty) or a missing one.
    #[func]
    fn build(&mut self, model: GString) -> bool {
        let path = model.to_string();
        let Some(scene) = model_scene(&path) else { return self.build_from_client(&path) };
        let Some(root) = scene.instantiate() else { return false };
        self.base_mut().add_child(&root);
        true
    }
}

impl RoseFieldItem {
    /// Without an imported scene: the object of the client's ground model list.
    fn build_from_client(&mut self, path: &str) -> bool {
        let Some((list, object)) = rose_data::art::client_list(path) else { return false };
        let Some(zsc) = zsc_list(&format!("{list}.ZSC")) else { return false };
        let Some(object) = zsc.objects.get(object as usize) else { return false };
        let mut built = false;
        for instance in object_meshes(zsc, object, false) {
            self.base_mut().add_child(&instance);
            built = true;
        }
        built
    }
}

/// A model list read from the client, kept.
pub fn zsc_list(path: &str) -> Option<&'static ZscFile> {
    static LISTS: std::sync::Mutex<Option<HashMap<String, Option<&'static ZscFile>>>> = std::sync::Mutex::new(None);
    let mut lists = LISTS.lock().ok()?;
    *lists
        .get_or_insert_with(HashMap::new)
        .entry(path.to_ascii_uppercase())
        .or_insert_with(|| data::read_file::<ZscFile>(path).map(|zsc| &*Box::leak(Box::new(zsc))))
}

/// A ground model object's parts as mesh instances, placed as the ZSC places them. With
/// `imported`, the meshes are the imported resources (for saving a scene that links them).
pub fn object_meshes(zsc: &ZscFile, object: &rose_file_readers::ZscObject, imported: bool) -> Vec<Gd<MeshInstance3D>> {
    let mut out = Vec::new();
    for part in object.parts.iter() {
        let Some(mesh_path) = zsc.meshes.get(part.mesh_id as usize) else { continue };
        let mesh_path = mesh_path.path().to_string_lossy();
        let part_mesh = if imported { assets::load_mesh(&mesh_path) } else { mesh::load_mesh(&mesh_path, false) };
        let Some(part_mesh) = part_mesh else { continue };
        let Some(zsc_material) = zsc.materials.get(part.material_id as usize) else { continue };
        let mut instance = MeshInstance3D::new_alloc();
        instance.set_mesh(&part_mesh);
        instance.set_material_override(&material::object_material(zsc_material, None));
        instance.set_position(Vector3::new(part.position.x, part.position.z, -part.position.y) / 100.0);
        let q = Quaternion::new(part.rotation.x, part.rotation.z, -part.rotation.y, part.rotation.w);
        instance.set_quaternion(if q.length_squared() > 0.0 { q.normalized() } else { Quaternion::IDENTITY });
        instance.set_scale(Vector3::new(part.scale.x, part.scale.z, part.scale.y));
        out.push(instance);
    }
    out
}

/// A model scene by its path (an item's or monster's model, see `rose_data::art`): ours or
/// the imported one, else None (the caller builds it from the client's files).
pub fn model_scene(path: &str) -> Option<Gd<PackedScene>> {
    if path.trim().is_empty() {
        return None;
    }
    let stem = rose_data::art::art_stem(path);
    if stem.starts_with("res://") {
        let mut loader = godot::classes::ResourceLoader::singleton();
        let scene = loader.exists(&stem).then(|| loader.load(&stem)).flatten().and_then(|r| r.try_cast::<PackedScene>().ok());
        if scene.is_none() {
            godot_warn!("rose: no model {path}");
        }
        return scene;
    }
    assets::load_stem::<PackedScene>(&stem, &["tscn", "scn"])
}

/// Bumped when the imported icons or ground models change, so the next import redoes them.
pub const ART_IMPORT_VERSION: i64 = 1;

/// The import of the icons and ground models (step 9 of the own game data plan):
/// - every sprite of the item, skill and status icon sheets as an `AtlasTexture` on its
///   imported sheet (`3ddata/control/res/item1/919.tres` is sprite 919 of ITEM1.TSI);
/// - every object of LIST_FIELDITEM.ZSC as a scene of mesh instances
///   (`3ddata/item/list_fielditem/310.tscn`);
/// - the loading picture (`3ddata/control/res/loading.rtex`) and the mouse cursors
///   (`res/attack.res`, an ImageTexture of the client's RES/ATTACK.CUR).
/// The data resources link these. Needs the textures and meshes imported first.
pub fn import_art(redo: bool) -> crate::character::AvatarImport {
    let mut out = crate::character::AvatarImport::default();
    for sheet_path in [rose_data::art::ITEM_ICONS, rose_data::art::SKILL_ICONS, rose_data::art::STATUS_EFFECT_ICONS] {
        let Some(sheet) = data::read_file::<TsiFile>(sheet_path) else {
            out.failed += 1;
            continue;
        };
        let folder = sheet_path.rsplit_once('/').map_or("", |(folder, _)| folder);
        // The sheets' textures (the import leaves the rest of the interface out), loaded once
        // and linked by path.
        let atlases: Vec<Option<Gd<Texture2D>>> = sheet
            .textures
            .iter()
            .map(|t| {
                let client = format!("{folder}/{}", t.filename);
                let target = format!("{}{}.rtex", assets::ROSE, assets::stem(&client));
                if (redo || !godot::classes::FileAccess::file_exists(&target)) && !assets::save_texture(&client, &target) {
                    out.failed += 1;
                    return None;
                }
                assets::load_texture(&client).filter(|t| t.get_path().to_string().starts_with("res://"))
            })
            .collect();
        for (index, sprite) in sheet.sprites.iter().enumerate() {
            let Some(Some(atlas)) = atlases.get(sprite.texture_id as usize) else { continue };
            let stem = rose_data::art::art_stem(&rose_data::art::client_part(sheet_path, index as u32));
            out.save(redo, &stem, "tres", || {
                let mut icon = AtlasTexture::new_gd();
                icon.set_atlas(atlas);
                icon.set_region(sprite_region(sprite));
                Some(icon.upcast())
            });
        }
    }
    // The interface's own pictures and mouse cursors (the interface settings link them).
    let loading = rose_data_export::interface::LOADING_PICTURE;
    let target = format!("{}{}.rtex", assets::ROSE, assets::stem(loading));
    if !redo && godot::classes::FileAccess::file_exists(&target) {
        out.kept += 1;
    } else if assets::save_texture(loading, &target) {
        out.saved += 1;
    } else {
        out.failed += 1;
    }
    for (_, file, _) in rose_data_export::interface::CURSORS {
        out.save(redo, &assets::stem(file), "res", || {
            let image = crate::cursor::load(file)?;
            godot::classes::ImageTexture::create_from_image(&image).map(|t| t.upcast())
        });
    }
    if let Some(zsc) = zsc_list(rose_data::art::FIELD_ITEM_MODELS) {
        for (index, object) in zsc.objects.iter().enumerate() {
            let meshes = object_meshes(zsc, object, true);
            if meshes.is_empty() {
                continue;
            }
            let stem = rose_data::art::art_stem(&rose_data::art::client_part(rose_data::art::FIELD_ITEM_MODELS, index as u32));
            out.save(redo, &stem, "tscn", || {
                let mut root = Node3D::new_alloc();
                root.set_name(&format!("FieldItem{index}"));
                for (i, mut instance) in meshes.into_iter().enumerate() {
                    instance.set_name(&format!("Part{i}"));
                    root.add_child(&instance);
                    crate::character::own(&root, instance);
                }
                crate::character::pack(root)
            });
        }
    }
    out
}
