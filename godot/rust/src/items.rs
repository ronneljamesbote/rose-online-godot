//! Items for the UI and the world: icons from ITEM1.TSI, ground models from
//! LIST_FIELDITEM.ZSC, and item descriptions for slots and tooltips.

use std::collections::HashMap;
use std::sync::OnceLock;

use godot::{
    classes::{AtlasTexture, MeshInstance3D, Texture2D},
    prelude::*,
};
use rose_data::{AbilityType, Item, ItemClass, ItemType};
use rose_file_readers::{TsiFile, ZscFile};

use crate::{data, material, mesh, texture};

/// The UI sprite sheets icons come from.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconSheet {
    Item,
    Skill,
    State,
}

fn icon_sheet(sheet: IconSheet) -> Option<&'static TsiFile> {
    static ITEM: OnceLock<Option<TsiFile>> = OnceLock::new();
    static SKILL: OnceLock<Option<TsiFile>> = OnceLock::new();
    static STATE: OnceLock<Option<TsiFile>> = OnceLock::new();
    let (lock, path) = match sheet {
        IconSheet::Item => (&ITEM, "3DDATA/CONTROL/RES/ITEM1.TSI"),
        IconSheet::Skill => (&SKILL, "3DDATA/CONTROL/RES/SKILLICON.TSI"),
        IconSheet::State => (&STATE, "3DDATA/CONTROL/RES/STATEICON.TSI"),
    };
    lock.get_or_init(|| data::read_file::<TsiFile>(path)).as_ref()
}

thread_local! {
    static ICONS: std::cell::RefCell<HashMap<(IconSheet, u32), Gd<AtlasTexture>>> = Default::default();
}

pub fn clear_cache() {
    ICONS.with(|icons| icons.borrow_mut().clear());
}

/// The inventory icon with this index (BaseItemData::icon_index).
pub fn item_icon(index: u32) -> Option<Gd<Texture2D>> {
    icon(IconSheet::Item, index)
}

/// A sprite from one of the icon sheets (items, skills, status effects).
pub fn icon(sheet_type: IconSheet, index: u32) -> Option<Gd<Texture2D>> {
    if let Some(icon) = ICONS.with(|icons| icons.borrow().get(&(sheet_type, index)).cloned()) {
        return Some(icon.upcast());
    }
    let sheet = icon_sheet(sheet_type)?;
    let sprite = sheet.sprites.get(index as usize)?;
    let file = sheet.textures.get(sprite.texture_id as usize)?;
    let atlas_texture = texture::load_texture(&format!("3DDATA/CONTROL/RES/{}", file.filename))?;
    let mut icon = AtlasTexture::new_gd();
    icon.set_atlas(&atlas_texture);
    icon.set_region(Rect2::new(
        Vector2::new(sprite.left as f32, sprite.top as f32),
        Vector2::new((sprite.right - sprite.left) as f32, (sprite.bottom - sprite.top) as f32),
    ));
    ICONS.with(|icons| icons.borrow_mut().insert((sheet_type, index), icon.clone()));
    Some(icon.upcast())
}

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
    d.set("model", base.field_model_index as i64);
    d.set("class", class_name(base.class).as_str());
    if let Some(icon) = item_icon(base.icon_index) {
        d.set("icon", &icon);
    }

    let mut lines: Vec<String> = vec![class_name(base.class)];
    match reference.item_type {
        ItemType::Weapon => {
            if let Some(w) = game.items.get_weapon_item(reference.item_number) {
                lines.push(format!("Attack {}   Range {:.1} m", w.attack_power, w.attack_range as f32 / 100.0));
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
        if e.grade > 0 {
            lines[0] = format!("{} +{}", lines[0], e.grade);
        }
        lines.push(format!("Durability {}   Life {}%", e.durability, e.life / 10));
        if e.is_crafted {
            lines.push("Crafted".to_string());
        }
        if e.has_socket {
            lines.push(if e.gem >= 300 { "Gem socketed".to_string() } else { "Empty gem socket".to_string() });
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

fn field_item_zsc() -> Option<&'static ZscFile> {
    static ZSC: OnceLock<Option<ZscFile>> = OnceLock::new();
    ZSC.get_or_init(|| data::read_file::<ZscFile>("3DDATA/ITEM/LIST_FIELDITEM.ZSC")).as_ref()
}

/// An item lying on the ground.
#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct RoseFieldItem {
    base: Base<Node3D>,
}

#[godot_api]
impl RoseFieldItem {
    /// Builds the ground model with this index (0 is the money bag).
    #[func]
    fn build(&mut self, model: i32) -> bool {
        let Some(zsc) = field_item_zsc() else { return false };
        let Some(object) = zsc.objects.get(model.max(0) as usize) else { return false };
        let mut built = false;
        for part in object.parts.iter() {
            let Some(mesh_path) = zsc.meshes.get(part.mesh_id as usize) else { continue };
            let Some(part_mesh) = mesh::load_mesh(&mesh_path.path().to_string_lossy(), false) else { continue };
            let Some(zsc_material) = zsc.materials.get(part.material_id as usize) else { continue };
            let mut instance = MeshInstance3D::new_alloc();
            instance.set_mesh(&part_mesh);
            instance.set_material_override(&material::object_material(zsc_material, None));
            instance.set_position(Vector3::new(part.position.x, part.position.z, -part.position.y) / 100.0);
            let q = Quaternion::new(part.rotation.x, part.rotation.z, -part.rotation.y, part.rotation.w);
            instance.set_quaternion(if q.length_squared() > 0.0 { q.normalized() } else { Quaternion::IDENTITY });
            instance.set_scale(Vector3::new(part.scale.x, part.scale.z, part.scale.y));
            self.base_mut().add_child(&instance);
            built = true;
        }
        built
    }
}
