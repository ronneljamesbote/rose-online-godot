//! Player characters: ZMD skeleton, ZSC body parts and ZMO motions.
//! Ported from rose-offline-client's model_loader.rs and zmo_asset_loader.rs.

use std::collections::{BTreeMap, HashMap, HashSet};

use enum_map::Enum;
use godot::{
    classes::{
        animation::{InterpolationType, LoopMode, TrackType},
        Animation, AnimationLibrary, AnimationPlayer, ArrayMesh, BoneAttachment3D, MeshInstance3D, Node, Node3D,
        PackedScene, Skeleton3D, Skin,
    },
    prelude::*,
};
use rose_data::{AnimationEventFlags, CharacterMotionAction, NpcId, NpcMotionAction, VehicleMotionAction, VehicleType};
use rose_file_readers::{ZmdFile, ZmoChannel, ZmoFile, ZscFile};

use crate::{assets, data, material, mesh};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Part {
    Face,
    Hair,
    Head,
    Body,
    Hands,
    Feet,
    Weapon,
    SubWeapon,
    FaceItem,
    Back,
}

const PARTS: [Part; 10] = [
    Part::Face, Part::Hair, Part::Head, Part::Body, Part::Hands, Part::Feet, Part::Weapon, Part::SubWeapon, Part::FaceItem, Part::Back,
];

fn skeleton_file(male: bool) -> &'static str {
    if male { "3DDATA/AVATAR/MALE.ZMD" } else { "3DDATA/AVATAR/FEMALE.ZMD" }
}

/// Monsters and NPCs: skeletons, parts and motions per model id.
const NPC_CHR: &str = "3DDATA/NPC/LIST_NPC.CHR";
const NPC_ZSC: &str = "3DDATA/NPC/PART_NPC.ZSC";
/// Cart and castle gear parts.
const VEHICLE_ZSC: &str = "3DDATA/PAT/LIST_PAT.ZSC";

fn vehicle_skeleton_file(is_cart: bool) -> &'static str {
    if is_cart { "3DDATA/PAT/CART/CART01.ZMD" } else { "3DDATA/PAT/CASTLEGEAR/CASTLEGEAR02/CASTLEGEAR02.ZMD" }
}

/// Where the imported scene of a monster or NPC model lives: `3ddata/npc/list_npc/12` is
/// LIST_NPC.CHR entry 12, the model of NPC 12.
pub fn npc_scene_stem(npc_id: u16) -> String {
    format!("{}/{npc_id}", assets::stem(NPC_CHR))
}

impl Part {
    fn list(self, male: bool) -> &'static str {
        let (m, w) = match self {
            Part::Face => ("3DDATA/AVATAR/LIST_MFACE.ZSC", "3DDATA/AVATAR/LIST_WFACE.ZSC"),
            Part::Hair => ("3DDATA/AVATAR/LIST_MHAIR.ZSC", "3DDATA/AVATAR/LIST_WHAIR.ZSC"),
            Part::Head => ("3DDATA/AVATAR/LIST_MCAP.ZSC", "3DDATA/AVATAR/LIST_WCAP.ZSC"),
            Part::Body => ("3DDATA/AVATAR/LIST_MBODY.ZSC", "3DDATA/AVATAR/LIST_WBODY.ZSC"),
            Part::Hands => ("3DDATA/AVATAR/LIST_MARMS.ZSC", "3DDATA/AVATAR/LIST_WARMS.ZSC"),
            Part::Feet => ("3DDATA/AVATAR/LIST_MFOOT.ZSC", "3DDATA/AVATAR/LIST_WFOOT.ZSC"),
            Part::Weapon => ("3DDATA/WEAPON/LIST_WEAPON.ZSC", "3DDATA/WEAPON/LIST_WEAPON.ZSC"),
            Part::SubWeapon => ("3DDATA/WEAPON/LIST_SUBWPN.ZSC", "3DDATA/WEAPON/LIST_SUBWPN.ZSC"),
            // Masks, glasses, capes and backpacks are one list for both (the iROSE file name
            // is spelt FACEIEM).
            Part::FaceItem => ("3DDATA/AVATAR/LIST_FACEIEM.ZSC", "3DDATA/AVATAR/LIST_FACEIEM.ZSC"),
            Part::Back => ("3DDATA/AVATAR/LIST_BACK.ZSC", "3DDATA/AVATAR/LIST_BACK.ZSC"),
        };
        if male { m } else { w }
    }

    /// DefaultBoneId in model_loader.rs; dummy bones come after the regular bones.
    fn default_bone(self, dummy_offset: usize) -> Option<usize> {
        match self {
            Part::Face | Part::Hair => Some(4),
            Part::Head => Some(dummy_offset + 6),
            Part::FaceItem => Some(dummy_offset + 4),
            Part::Back => Some(dummy_offset + 3),
            _ => None,
        }
    }
}

#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct RoseCharacter {
    base: Base<Node3D>,
    gender: usize,
    weapon_motion_type: usize,
    dummy_offset: usize,
    /// The mesh instances of each equipped part, in the model's part order, with its item
    /// number and model path.
    parts: Vec<(Part, usize, String, Vec<Gd<MeshInstance3D>>)>,
    /// Gem effects shown on the weapons.
    gem_effects: Vec<Gd<Node3D>>,
}

fn bone_name(index: usize, dummy_offset: usize) -> String {
    if index < dummy_offset { format!("b{index}") } else { format!("d{}", index - dummy_offset) }
}

fn build_skeleton(zmd: &ZmdFile) -> Gd<Skeleton3D> {
    let mut skeleton = Skeleton3D::new_alloc();
    skeleton.set_name("Skeleton3D");
    let dummy_offset = zmd.bones.len();
    let all: Vec<_> = zmd.bones.iter().chain(zmd.dummy_bones.iter()).collect();
    for (i, _) in all.iter().enumerate() {
        skeleton.add_bone(&bone_name(i, dummy_offset));
    }
    for (i, bone) in all.iter().enumerate() {
        let parent = bone.parent as usize;
        if parent != i && parent < all.len() {
            skeleton.set_bone_parent(i as i32, parent as i32);
        }
        let rest = Transform3D::new(
            Basis::from_quaternion(
                Quaternion::new(bone.rotation.x, bone.rotation.z, -bone.rotation.y, bone.rotation.w).normalized(),
            ),
            Vector3::new(bone.position.x, bone.position.z, -bone.position.y) / 100.0,
        );
        skeleton.set_bone_rest(i as i32, rest);
        skeleton.set_bone_pose(i as i32, rest);
    }
    skeleton
}

/// Converts a ZMO bone motion to an Animation with position/rotation tracks on the skeleton.
fn build_animation(zmo: &ZmoFile, dummy_offset: usize, looping: bool) -> Gd<Animation> {
    let mut animation = Animation::new_gd();
    let fps = zmo.fps.max(1) as f64;
    animation.set_length(zmo.num_frames as f64 / fps);
    animation.set_loop_mode(if looping { LoopMode::LINEAR } else { LoopMode::NONE });

    for (bone_id, channel) in zmo.channels.iter() {
        let path = format!("Skeleton3D:{}", bone_name(*bone_id as usize, dummy_offset));
        match channel {
            ZmoChannel::Position(positions) => {
                let track = animation.add_track(TrackType::POSITION_3D);
                animation.track_set_path(track, &NodePath::from(path.as_str()));
                animation.track_set_interpolation_type(track, InterpolationType::LINEAR);
                for (frame, p) in positions.iter().enumerate() {
                    animation.position_track_insert_key(
                        track,
                        frame as f64 / fps,
                        Vector3::new(p.x, p.z, -p.y) / 100.0,
                    );
                }
            }
            ZmoChannel::Rotation(rotations) => {
                let track = animation.add_track(TrackType::ROTATION_3D);
                animation.track_set_path(track, &NodePath::from(path.as_str()));
                animation.track_set_interpolation_type(track, InterpolationType::LINEAR);
                for (frame, r) in rotations.iter().enumerate() {
                    animation.rotation_track_insert_key(
                        track,
                        frame as f64 / fps,
                        Quaternion::new(r.x, r.z, -r.y, r.w).normalized(),
                    );
                }
            }
            _ => {}
        }
    }

    // Keep the frame events (hit frames, footsteps) for gameplay code.
    let events: PackedInt32Array = zmo.frame_events.iter().map(|&e| e as i32).collect();
    animation.set_meta("frame_events", &events.to_variant());
    animation.set_meta("fps", &fps.to_variant());
    animation
}

/// Frame event ids the server and `hit_time` count as the moment a swing or skill lands
/// (`attack_hit_ms` on the server).
fn is_hit_event(id: u16) -> bool {
    id == 10 || (20..=28).contains(&id) || matches!(id, 56 | 57 | 66 | 67)
}

/// Turns the ZMO frame events into animation markers: `hit1:21` is the first hit marker and
/// carries event id 21; footsteps are `step`, everything else (sounds, effects) `effect`.
fn add_event_markers(animation: &mut Gd<Animation>, events: &[u16], fps: f64) {
    let flags = data::get().map(|g| g.event_flags.as_slice()).unwrap_or_default();
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for (frame, &id) in events.iter().enumerate() {
        if id == 0 {
            continue;
        }
        let footstep = flags.get(id as usize).is_some_and(|f| f.contains(AnimationEventFlags::SOUND_FOOTSTEP));
        let (kind, color) = if is_hit_event(id) {
            ("hit", Color::from_rgb(0.9, 0.2, 0.2))
        } else if footstep {
            ("step", Color::from_rgb(0.3, 0.6, 1.0))
        } else {
            ("effect", Color::from_rgb(1.0, 0.75, 0.3))
        };
        let count = counts.entry(kind).or_default();
        *count += 1;
        let name = format!("{kind}{count}:{id}");
        animation.add_marker(name.as_str(), frame as f64 / fps);
        animation.set_marker_color(name.as_str(), color);
    }
}

/// The per-frame `frame_events` that gameplay reads, rebuilt from the animation's markers
/// (the `:ID` at the end of each marker name), so moving a marker in the editor moves the
/// event. Done once per animation resource.
fn events_from_markers(animation: &mut Gd<Animation>) {
    if animation.has_meta("frame_events") {
        return;
    }
    let fps: f64 = animation.get_meta("fps").try_to().unwrap_or(30.0);
    let frames = ((animation.get_length() as f64 * fps).round() as usize).max(1);
    let mut events = vec![0i32; frames];
    for name in animation.get_marker_names().as_slice() {
        let Some(id) = name.to_string().rsplit_once(':').and_then(|(_, id)| id.parse::<i32>().ok()) else { continue };
        let frame = (animation.get_marker_time(&StringName::from(name)) * fps).round() as usize;
        events[frame.min(frames - 1)] = id;
    }
    animation.set_meta("frame_events", &PackedInt32Array::from(events.as_slice()).to_variant());
}

/// A motion: the imported animation for the ZMO when there is one made for a skeleton with
/// this many bones (its `dummy_offset`), with its markers as frame events; else converted now.
fn load_motion(zmo_path: &str, dummy_offset: usize, looping: bool) -> Option<Gd<Animation>> {
    let wanted = if looping { LoopMode::LINEAR } else { LoopMode::NONE };
    if let Some(mut animation) = assets::load_stem::<Animation>(&assets::stem(zmo_path), &["anim", "res", "tres"]) {
        let offset = animation.get_meta_ex("dummy_offset").default(&(dummy_offset as i64).to_variant()).done();
        if offset.try_to::<i64>().ok() == Some(dummy_offset as i64) {
            events_from_markers(&mut animation);
            if animation.get_loop_mode() != wanted {
                // Shared resource: a copy with the other loop mode.
                let mut copy = animation.duplicate_resource();
                copy.set_loop_mode(wanted);
                return Some(copy);
            }
            return Some(animation);
        }
    }
    let zmo = data::read_file::<ZmoFile>(zmo_path)?;
    Some(build_animation(&zmo, dummy_offset, looping))
}

/// Moves the children of an instantiated scene under `node` (clearing their owners) and
/// frees the scene's root. Returns the Skeleton3D among them.
fn adopt_scene(node: &mut Gd<Node3D>, mut root: Gd<Node>) -> Option<Gd<Skeleton3D>> {
    fn clear_owners(node: &Gd<Node>) {
        for mut child in node.get_children().iter_shared() {
            child.set_owner(Gd::null_arg());
            clear_owners(&child);
        }
    }
    clear_owners(&root);
    let mut skeleton = None;
    for child in root.get_children().iter_shared() {
        root.remove_child(&child);
        node.add_child(&child);
        if let Ok(found) = child.try_cast::<Skeleton3D>() {
            skeleton = Some(found);
        }
    }
    root.free();
    skeleton
}

/// The imported meshes keep their bone weights; parts that hang on a bone (`skinned` meta
/// false) get the rigid copy, as the run-time loader gives them.
fn use_rigid_meshes(node: &Gd<Node>) {
    for child in node.get_children().iter_shared() {
        if let Ok(mut instance) = child.clone().try_cast::<MeshInstance3D>() {
            if instance.get_meta_ex("skinned").default(&true.to_variant()).done().try_to::<bool>().ok() == Some(false) {
                if let Some(rigid) = instance.get_mesh().and_then(|m| m.try_cast::<ArrayMesh>().ok()).map(|m| mesh::rigid(&m)) {
                    instance.set_mesh(&rigid);
                }
            }
        }
        use_rigid_meshes(&child);
    }
}

/// A model's own copy of an imported motion library: the markers as frame events and each
/// animation with the loop mode `looping` gives its name.
fn own_library(library: &Gd<AnimationLibrary>, looping: impl Fn(&str) -> bool) -> Gd<AnimationLibrary> {
    let mut copy = library.duplicate_resource();
    for name in copy.get_animation_list().iter_shared() {
        let Some(mut animation) = copy.get_animation(&name) else { continue };
        events_from_markers(&mut animation);
        let wanted = if looping(&name.to_string()) { LoopMode::LINEAR } else { LoopMode::NONE };
        if animation.get_loop_mode() != wanted {
            let mut looped = animation.duplicate_resource();
            looped.set_loop_mode(wanted);
            copy.add_animation(&name, &looped);
        }
    }
    copy
}

/// One ZSC object's parts as mesh instances, each with whether it is skinned and the bone
/// it hangs on (spawn_model in model_loader.rs). For the import the meshes are the imported
/// resources, so scenes refer to them instead of embedding a copy.
fn part_meshes(
    zsc: &ZscFile,
    object_id: usize,
    dummy_offset: usize,
    default_bone: Option<usize>,
    imported: bool,
) -> Vec<(Gd<MeshInstance3D>, bool, Option<usize>)> {
    let mut parts = Vec::new();
    let Some(object) = zsc.objects.get(object_id) else { return parts };
    for object_part in object.parts.iter() {
        let Some(zsc_material) = zsc.materials.get(object_part.material_id as usize) else { continue };
        let Some(mesh_path) = zsc.meshes.get(object_part.mesh_id as usize) else { continue };
        let mesh_path = mesh_path.path().to_string_lossy();
        let part_mesh =
            if imported { assets::load_mesh(&mesh_path) } else { mesh::load_mesh(&mesh_path, zsc_material.is_skin) };
        let Some(part_mesh) = part_mesh else { continue };
        let mut instance = MeshInstance3D::new_alloc();
        instance.set_mesh(&part_mesh);
        let part_material = material::object_material(zsc_material, material::Light::Character);
        if !material::casts_shadow(&part_material) {
            instance.set_cast_shadows_setting(godot::classes::geometry_instance_3d::ShadowCastingSetting::OFF);
        }
        instance.set_material_override(&part_material);
        let link_bone = object_part
            .bone_index
            .map(|b| b as usize)
            .or(object_part.dummy_index.map(|d| d as usize + dummy_offset))
            .or(default_bone);
        parts.push((instance, zsc_material.is_skin, link_bone));
    }
    parts
}

/// Puts a part on the skeleton: skinned parts under it with its skin, rigid parts on their
/// bone or dummy bone, or on the model's own transform when they have none.
fn place_part(skeleton: &mut Gd<Skeleton3D>, skin: Option<&Gd<Skin>>, mut instance: Gd<MeshInstance3D>, skinned: bool, bone: Option<String>) -> Gd<MeshInstance3D> {
    if skinned {
        if let Some(skin) = skin {
            instance.set_skin(skin);
        }
        instance.set_skeleton_path(&NodePath::from(".."));
        skeleton.add_child(&instance);
    } else if let Some(bone) = bone {
        let mut attachment = BoneAttachment3D::new_alloc();
        attachment.set_name(&format!("On_{bone}"));
        attachment.set_bone_name(bone.as_str());
        skeleton.add_child(&attachment);
        attachment.add_child(&instance);
    } else {
        skeleton.add_child(&instance);
    }
    instance
}

/// Adds one ZSC object's parts to the skeleton, built from the ROSE files.
fn attach_object(
    skeleton: &mut Gd<Skeleton3D>,
    skin: Option<&Gd<Skin>>,
    zsc: &ZscFile,
    object_id: usize,
    dummy_offset: usize,
    default_bone: Option<usize>,
) -> Vec<Gd<MeshInstance3D>> {
    part_meshes(zsc, object_id, dummy_offset, default_bone, false)
        .into_iter()
        .map(|(instance, skinned, bone)| place_part(skeleton, skin, instance, skinned, bone.map(|b| bone_name(b, dummy_offset))))
        .collect()
}

/// A part's scene (`3ddata/avatar/list_mbody/12.tscn`, or one of our own): its mesh
/// instances, with `skinned` and `bone` metadata, go onto the skeleton.
fn attach_scene(skeleton: &mut Gd<Skeleton3D>, skin: Option<&Gd<Skin>>, scene: &Gd<PackedScene>) -> Option<Vec<Gd<MeshInstance3D>>> {
    let mut root = scene.instantiate()?;
    let mut placed = Vec::new();
    for child in root.get_children().iter_shared() {
        let Ok(mut instance) = child.try_cast::<MeshInstance3D>() else { continue };
        instance.set_owner(Gd::null_arg());
        root.remove_child(&instance);
        let skinned = instance.get_meta_ex("skinned").default(&false.to_variant()).done().try_to::<bool>().unwrap_or(false);
        let bone = instance.get_meta_ex("bone").default(&GString::new().to_variant()).done().try_to::<GString>().map(|b| b.to_string()).unwrap_or_default();
        if !skinned {
            if let Some(rigid) = instance.get_mesh().and_then(|m| m.try_cast::<ArrayMesh>().ok()).map(|m| mesh::rigid(&m)) {
                instance.set_mesh(&rigid);
            }
        }
        placed.push(place_part(skeleton, skin, instance, skinned, (!bone.is_empty()).then_some(bone)));
    }
    root.free();
    Some(placed)
}

/// Puts a model on the skeleton by its path (an item's model, see `rose_data::art`): its
/// scene, or without one the object of the client's model list. None when there is neither.
fn attach_model(
    skeleton: &mut Gd<Skeleton3D>,
    skin: Option<&Gd<Skin>>,
    path: &str,
    dummy_offset: usize,
    default_bone: Option<usize>,
) -> Option<Vec<Gd<MeshInstance3D>>> {
    if let Some(scene) = crate::items::model_scene(path) {
        return attach_scene(skeleton, skin, &scene);
    }
    let (list, object) = rose_data::art::client_list(path)?;
    let zsc = crate::items::zsc_list(&format!("{list}.ZSC"))?;
    Some(attach_object(skeleton, skin, zsc, object as usize, dummy_offset, default_bone))
}

/// The model path of an equipped item for a male or female character (empty when it has
/// none). A number with no item, like 1 (bare body, hands and feet), is the client list's
/// object of that number, as before items had models.
fn item_model(part: Part, number: usize, male: bool) -> String {
    let Some(game_data) = data::get() else { return String::new() };
    let items = &game_data.items;
    let base = match part {
        Part::Face | Part::Hair => return rose_data::art::client_part(part.list(male), number as u32),
        Part::Head => items.get_head_item(number).map(|i| &i.item_data),
        Part::Body => items.get_body_item(number).map(|i| &i.item_data),
        Part::Hands => items.get_hands_item(number).map(|i| &i.item_data),
        Part::Feet => items.get_feet_item(number).map(|i| &i.item_data),
        Part::Weapon => items.get_weapon_item(number).map(|i| &i.item_data),
        Part::SubWeapon => items.get_sub_weapon_item(number).map(|i| &i.item_data),
        Part::FaceItem => items.get_face_item(number).map(|i| &i.item_data),
        Part::Back => items.get_back_item(number).map(|i| &i.item_data),
    };
    match base {
        Some(b) if male => b.model_male.clone(),
        Some(b) => b.model_female.clone(),
        None => rose_data::art::client_part(part.list(male), number as u32),
    }
}

fn add_player(node: &mut Gd<Node3D>, library: Gd<AnimationLibrary>) {
    let mut player = AnimationPlayer::new_alloc();
    player.set_name("AnimationPlayer");
    player.add_animation_library("", &library);
    node.add_child(&player);
}

fn action_name(action: CharacterMotionAction) -> String {
    format!("{action:?}").to_lowercase()
}

/// Player motions that loop.
fn is_looping(action: CharacterMotionAction) -> bool {
    matches!(
        action,
        CharacterMotionAction::Stop1 | CharacterMotionAction::Stop2 | CharacterMotionAction::Stop3
            | CharacterMotionAction::Walk | CharacterMotionAction::Run | CharacterMotionAction::Sit
    )
}

/// The motion file for an action, with the same fallbacks as load_character_action_motions.
fn action_motion_path(action: CharacterMotionAction, weapon_motion_type: usize, gender: usize) -> Option<String> {
    let game_data = data::get()?;
    [(weapon_motion_type, gender), (weapon_motion_type, 0), (0, gender), (0, 0)]
        .into_iter()
        .find_map(|(w, g)| game_data.motions.get_character_action_motion(action, w, g))
        .map(|motion| motion.path.path().to_string_lossy().to_string())
}

/// Every action motion for one gender and weapon type, named after the action. With
/// `imported`, the animations are the imported `.anim` resources (for saving the library).
fn avatar_motion_library(gender: usize, weapon_motion_type: usize, dummy_offset: usize, imported: bool) -> Gd<AnimationLibrary> {
    let mut library = AnimationLibrary::new_gd();
    for index in 0..CharacterMotionAction::LENGTH {
        let action = CharacterMotionAction::from_usize(index);
        let Some(path) = action_motion_path(action, weapon_motion_type, gender) else { continue };
        let animation = if imported {
            assets::load_stem::<Animation>(&assets::stem(&path), &["anim"])
        } else {
            load_motion(&path, dummy_offset, is_looping(action))
        };
        if let Some(animation) = animation {
            library.add_animation(&action_name(action), &animation);
        }
    }
    library
}

/// Where a gender's motion library for a weapon type lives: `3ddata/avatar/male_motions/weapon_1`.
fn motion_library_stem(male: bool, weapon_motion_type: usize) -> String {
    format!("3ddata/avatar/{}_motions/weapon_{weapon_motion_type}", if male { "male" } else { "female" })
}

#[godot_api]
impl RoseCharacter {
    /// Builds the character. Part ids are item numbers (0 = not equipped), as in the
    /// Bevy client's get_model_part_index. Body, hands and feet fall back to 1.
    #[func]
    #[allow(clippy::too_many_arguments)]
    fn build(
        &mut self, male: bool, face: i32, hair: i32, head: i32, body: i32, hands: i32, feet: i32, weapon: i32, sub_weapon: i32,
        face_item: i32, back: i32,
    ) -> bool {
        let Some(game_data) = data::get() else {
            godot_error!("rose: call RoseData.open() first");
            return false;
        };
        let Some(zmd) = data::read_file::<ZmdFile>(skeleton_file(male)) else {
            return false;
        };
        let dummy_offset = zmd.bones.len();
        // The imported skeleton scene (Skeleton3D and AnimationPlayer) when there is one.
        let scene_root = assets::load_stem::<PackedScene>(&assets::stem(skeleton_file(male)), &["tscn", "scn"])
            .and_then(|scene| scene.instantiate());
        let mut skeleton = match scene_root.and_then(|root| adopt_scene(&mut self.to_gd().upcast(), root)) {
            Some(skeleton) => skeleton,
            None => {
                let skeleton = build_skeleton(&zmd);
                self.base_mut().add_child(&skeleton);
                skeleton
            }
        };
        let skin = skeleton.create_skin_from_rest_transforms();

        let hair_offset = if head > 0 {
            game_data.items.get_head_item(head as usize).map_or(0, |item| item.hair_type as i32)
        } else {
            0
        };
        let parts = [
            (Part::Face, face),
            (Part::Hair, hair + hair_offset),
            (Part::Head, head),
            (Part::Body, body.max(1)),
            (Part::Hands, hands.max(1)),
            (Part::Feet, feet.max(1)),
            (Part::Weapon, weapon),
            (Part::SubWeapon, sub_weapon),
            (Part::FaceItem, face_item),
            (Part::Back, back),
        ];
        self.parts.clear();
        self.gem_effects.clear();
        for (part, id) in parts {
            if id <= 0 && !matches!(part, Part::Face | Part::Hair) {
                continue;
            }
            if id < 0 {
                continue;
            }
            let model = item_model(part, id as usize, male);
            let Some(placed) = attach_model(&mut skeleton, skin.as_ref(), &model, dummy_offset, part.default_bone(dummy_offset)) else {
                continue;
            };
            if id > 0 {
                self.parts.push((part, id as usize, model, placed));
            }
        }

        // Motions for the equipped weapon type, with the same fallbacks as load_character_action_motions.
        let weapon_motion_type = if weapon > 0 {
            game_data.items.get_weapon_item(weapon as usize).map_or(0, |item| item.motion_type as usize)
        } else {
            0
        };
        let gender = if male { 0 } else { 1 };
        self.gender = gender;
        self.weapon_motion_type = weapon_motion_type;
        self.dummy_offset = dummy_offset;
        // Each character gets its own copy of the library, since skill and drive motions are
        // added to it later.
        let library = assets::load_stem::<AnimationLibrary>(&motion_library_stem(male, weapon_motion_type), &["tres", "res"])
            .map(|library| library.duplicate_resource());
        let library = match library {
            Some(library) => {
                for name in library.get_animation_list().iter_shared() {
                    if let Some(mut animation) = library.get_animation(&name) {
                        events_from_markers(&mut animation);
                    }
                }
                library
            }
            None => avatar_motion_library(gender, weapon_motion_type, dummy_offset, false),
        };

        match self.base().try_get_node_as::<AnimationPlayer>("AnimationPlayer") {
            Some(mut player) => {
                if player.has_animation_library("") {
                    player.remove_animation_library("");
                }
                player.add_animation_library("", &library);
            }
            None => add_player(&mut self.to_gd().upcast(), library),
        }
        true
    }

    /// Refine glows and gem effects (CObjAVT::CreateGradeEffect and CreateGemmingEffect).
    /// `grades`: the grade of the head, body, hands, feet, weapon, sub weapon, face item and
    /// back item, as passed to `build`; every graded part glows in its grade's LIST_GRADE
    /// colour. `gems`: the gem set in the weapon and sub weapon (0 for none); its attack
    /// effect plays on the item's gem point. Call after `build`; calling again replaces them.
    #[func]
    fn show_item_effects(&mut self, grades: PackedInt32Array, gems: PackedInt32Array) {
        let Some(game_data) = data::get() else { return };
        for mut effect in std::mem::take(&mut self.gem_effects) {
            if effect.is_instance_valid() {
                effect.queue_free();
            }
        }
        const GRADED: [Part; 8] =
            [Part::Head, Part::Body, Part::Hands, Part::Feet, Part::Weapon, Part::SubWeapon, Part::FaceItem, Part::Back];
        let mut effects = Vec::new();
        for (part, id, model, instances) in self.parts.iter_mut() {
            let grade = GRADED.iter().position(|p| p == part).and_then(|i| grades.get(i)).unwrap_or(0);
            let colour = (grade > 0)
                .then(|| game_data.items.get_item_grade(grade as u8))
                .flatten()
                .map(|g| g.glow_colour)
                .filter(|c| *c != (0.0, 0.0, 0.0));
            for instance in instances.iter_mut() {
                let glow = colour.map(|colour| {
                    let base = instance.get_material_override().and_then(|m| m.try_cast::<godot::classes::ShaderMaterial>().ok());
                    material::grade_glow(base.as_ref(), colour)
                });
                instance.set_material_overlay(glow.as_ref());
            }

            let (gem, gem_position) = match part {
                Part::Weapon => (gems.get(0), game_data.items.get_weapon_item(*id).map(|w| w.gem_position)),
                Part::SubWeapon => (gems.get(1), game_data.items.get_sub_weapon_item(*id).map(|w| w.gem_position)),
                _ => continue,
            };
            let (Some(gem), Some(gem_position)) = (gem, gem_position) else { continue };
            if gem <= 300 {
                continue;
            }
            let Some(path) = game_data
                .items
                .get_gem_item(gem as usize)
                .and_then(|g| g.gem_effect_id)
                .and_then(|id| game_data.effects.get_effect(id))
                .and_then(|e| e.point_effects.first().copied())
                .and_then(|file| game_data.effects.get_effect_file(file))
            else {
                continue;
            };
            // The gem point is the model list's (a scene of our own has none yet).
            let Some((list, object)) = rose_data::art::client_list(model) else { continue };
            let Some(zsc) = crate::items::zsc_list(&format!("{list}.ZSC")) else { continue };
            let Some(point) = zsc.objects.get(object as usize).and_then(|o| o.effects.get(gem_position as usize)) else { continue };
            let Some(parent) = instances.get(point.parent.unwrap_or(0) as usize) else { continue };
            let mut effect = crate::effect::RoseEffect::new_alloc();
            if !effect.bind_mut().load_path(path) {
                effect.free();
                continue;
            }
            effect.set_position(Vector3::new(point.position.x, point.position.z, -point.position.y) / 100.0);
            effect.set_quaternion(Quaternion::new(point.rotation.x, point.rotation.z, -point.rotation.y, point.rotation.w));
            parent.clone().add_child(&effect);
            effects.push(effect.upcast::<Node3D>());
        }
        self.gem_effects = effects;
    }

    /// Adds a skill motion (a MotionId from LIST_SKILL.STB, looked up in TYPE_MOTION.STB for
    /// the equipped weapon) to the animation player. Returns its animation name ("motion_ID"),
    /// or "" when there is none.
    #[func]
    fn add_motion(&mut self, motion_id: i32) -> GString {
        let name = format!("motion_{motion_id}");
        let Some(player) = self.base().try_get_node_as::<AnimationPlayer>("AnimationPlayer") else {
            return GString::new();
        };
        if player.has_animation(name.as_str()) {
            return GString::from(name.as_str());
        }
        let (Some(game_data), Some(id)) = (data::get(), u16::try_from(motion_id).ok().map(rose_data::MotionId::new)) else {
            return GString::new();
        };
        let Some(motion) = game_data.motions.find_first_character_motion(id, self.weapon_motion_type, self.gender) else {
            return GString::new();
        };
        let Some(animation) = load_motion(&motion.path.path().to_string_lossy(), self.dummy_offset, false) else {
            return GString::new();
        };
        let Some(mut library) = player.get_animation_library("") else { return GString::new() };
        library.add_animation(name.as_str(), &animation);
        GString::from(name.as_str())
    }

    /// Adds the driver's motions for a vehicle body (its base_avatar_motion_index) as
    /// drive_stop, drive_move, drive_attack1 and so on. Returns false when there are none.
    #[func]
    fn add_drive_motions(&mut self, body: i32) -> bool {
        let Some(game_data) = data::get() else { return false };
        let Some(vehicle) = game_data.items.get_vehicle_item(body.max(0) as usize) else { return false };
        let Some(player) = self.base().try_get_node_as::<AnimationPlayer>("AnimationPlayer") else { return false };
        let Some(mut library) = player.get_animation_library("") else { return false };
        let motions = vehicle_motions(
            vehicle.base_avatar_motion_index as usize,
            0,
            matches!(vehicle.vehicle_type, VehicleType::Cart),
            self.dummy_offset,
            "drive_",
        );
        let names = motions.get_animation_list();
        for name in names.iter_shared() {
            if !library.has_animation(&name) {
                if let Some(animation) = motions.get_animation(&name) {
                    library.add_animation(&name, &animation);
                }
            }
        }
        !names.is_empty()
    }
}

/// Vehicle motions, as animation names: stop, move, attack1-3, die, special1-2. Carts only
/// have one attack.
fn vehicle_motions(base_motion_index: usize, weapon: usize, is_cart: bool, dummy_offset: usize, prefix: &str) -> Gd<AnimationLibrary> {
    let mut library = AnimationLibrary::new_gd();
    let Some(game_data) = data::get() else { return library };
    for index in 0..VehicleMotionAction::LENGTH {
        let action = VehicleMotionAction::from_usize(index);
        let lookup = if is_cart && matches!(action, VehicleMotionAction::Attack2 | VehicleMotionAction::Attack3) {
            VehicleMotionAction::Attack1
        } else {
            action
        };
        let Some(motion) = game_data.motions.get_vehicle_action_motion(lookup, base_motion_index, weapon) else { continue };
        let looping = matches!(action, VehicleMotionAction::Stop | VehicleMotionAction::Move);
        let Some(animation) = load_motion(&motion.path.path().to_string_lossy(), dummy_offset, looping) else { continue };
        library.add_animation(&format!("{prefix}{}", format!("{action:?}").to_lowercase()), &animation);
    }
    library
}

/// The driver's seat (dummy bone 0) and, where there is one, the passenger's (dummy bone 10,
/// where CObjCART::Create(CObjCHAR*) links it).
fn add_seats(skeleton: &mut Gd<Skeleton3D>, zmd: &ZmdFile) -> Vec<Gd<BoneAttachment3D>> {
    let dummy_offset = zmd.bones.len();
    let mut seats = Vec::new();
    for (name, dummy) in [("Seat", 0), ("BackSeat", 10)] {
        if dummy == 0 || zmd.dummy_bones.len() > dummy {
            let mut seat = BoneAttachment3D::new_alloc();
            seat.set_name(name);
            seat.set_bone_name(&bone_name(dummy_offset + dummy, dummy_offset));
            skeleton.add_child(&seat);
            seats.push(seat);
        }
    }
    seats
}

/// Carts and castle gear (spawn_vehicle_model in model_loader.rs): the body's skeleton with
/// the four parts from LIST_PAT.ZSC; the driver sits on dummy bone 0.
#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct RoseVehicle {
    base: Base<Node3D>,
    /// The arms part's motion column (PAT_RELATIVE_MOTION_POS), which picks skill motions.
    weapon_motion_type: usize,
    dummy_offset: usize,
}

#[godot_api]
impl RoseVehicle {
    /// Builds a vehicle from part item numbers (0 = none; a body is needed). Animations are
    /// stop, move, attack1-3, die, special1 and special2. Returns false without a body.
    #[func]
    fn build(&mut self, body: i32, engine: i32, leg: i32, arms: i32) -> bool {
        let Some(game_data) = data::get() else { return false };
        let Some(body_data) = game_data.items.get_vehicle_item(body.max(0) as usize) else { return false };
        let is_cart = matches!(body_data.vehicle_type, VehicleType::Cart);
        let skeleton_path = vehicle_skeleton_file(is_cart);
        let Some(zmd) = data::read_file::<ZmdFile>(skeleton_path) else { return false };
        let dummy_offset = zmd.bones.len();
        // The imported skeleton scene (with its seats) when there is one.
        let scene_root = assets::load_stem::<PackedScene>(&assets::stem(skeleton_path), &["tscn", "scn"])
            .and_then(|scene| scene.instantiate());
        let mut skeleton = match scene_root.and_then(|root| adopt_scene(&mut self.to_gd().upcast(), root)) {
            Some(skeleton) => skeleton,
            None => {
                let mut skeleton = build_skeleton(&zmd);
                add_seats(&mut skeleton, &zmd);
                self.base_mut().add_child(&skeleton);
                skeleton
            }
        };
        let skin = skeleton.create_skin_from_rest_transforms();
        for part in [body, engine, leg, arms] {
            let model = game_data.items.get_vehicle_item(part.max(0) as usize).map_or(String::new(), |v| v.item_data.model_male.clone());
            if part > 0 {
                attach_model(&mut skeleton, skin.as_ref(), &model, dummy_offset, None);
            }
        }

        let weapon = game_data.items.get_vehicle_item(arms.max(0) as usize).map_or(0, |v| v.base_motion_index as usize);
        self.weapon_motion_type = weapon;
        self.dummy_offset = dummy_offset;
        let library = vehicle_motions(body_data.base_motion_index as usize, weapon, is_cart, dummy_offset, "");
        add_player(&mut self.to_gd().upcast(), library);
        true
    }

    /// Where the driver goes (dummy bone 0), or null before build.
    #[func]
    fn seat(&self) -> Option<Gd<Node3D>> {
        self.base().try_get_node_as::<Node3D>("Skeleton3D/Seat")
    }

    /// Where a passenger goes (dummy bone 10), or null when the vehicle has none.
    #[func]
    fn back_seat(&self) -> Option<Gd<Node3D>> {
        self.base().try_get_node_as::<Node3D>("Skeleton3D/BackSeat")
    }

    /// Adds a skill's casting or action motion for this vehicle ("motion_ID"), or returns "".
    /// As CObjCART::Get_MOTION, the motion table's column is the arms part's motion column
    /// (column 0 when that has none), so castle gear swings the arms it carries.
    #[func]
    fn add_motion(&mut self, motion_id: i32) -> GString {
        let name = format!("motion_{motion_id}");
        let Some(player) = self.base().try_get_node_as::<AnimationPlayer>("AnimationPlayer") else {
            return GString::new();
        };
        if player.has_animation(name.as_str()) {
            return GString::from(name.as_str());
        }
        let (Some(game_data), Some(id)) = (data::get(), u16::try_from(motion_id).ok().map(rose_data::MotionId::new)) else {
            return GString::new();
        };
        let Some(motion_file) = game_data.motions.find_first_character_motion(id, self.weapon_motion_type, 0) else {
            return GString::new();
        };
        let Some(animation) = load_motion(&motion_file.path.path().to_string_lossy(), self.dummy_offset, false) else {
            return GString::new();
        };
        let Some(mut library) = player.get_animation_library("") else { return GString::new() };
        library.add_animation(name.as_str(), &animation);
        GString::from(name.as_str())
    }
}

/// Monsters and NPCs from LIST_NPC.CHR and PART_NPC.ZSC (spawn_npc_model in model_loader.rs).
#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct RoseNpcModel {
    base: Base<Node3D>,
    npc_id: u16,
    dummy_offset: usize,
}

#[godot_api]
impl RoseNpcModel {
    /// Builds the model of a monster or NPC (its `model`: the scene, or without one the
    /// LIST_NPC.CHR entry it names), scaled as in its data. Animations are named after
    /// NpcMotionAction in lower case: stop, move, attack, hit, die, run.
    #[func]
    fn build(&mut self, npc_id: i32) -> bool {
        let Some(game_data) = data::get() else {
            godot_error!("rose: call RoseData.open() first");
            return false;
        };
        let Some(npc_id) = u16::try_from(npc_id).ok().and_then(NpcId::new) else { return false };
        let Some(npc) = game_data.npcs.get_npc(npc_id) else { return false };
        if npc.model.is_empty() {
            godot_warn!("rose: no model for npc {}", npc_id.get());
            return false;
        }
        self.npc_id = npc_id.get();
        let entry = rose_data::art::npc_model_entry(&npc.model).and_then(|entry| game_data.npc_chr.npcs.get(&entry));
        let zmd = || {
            let entry = entry?;
            game_data.npc_chr.skeleton_files.get(entry.skeleton_index as usize).and_then(|path| data::read_file::<ZmdFile>(path))
        };

        // The model scene (skeleton, parts and action motions) when there is one.
        let scene_root = crate::items::model_scene(&npc.model).and_then(|scene| scene.instantiate());
        let mut node = self.to_gd().upcast::<Node3D>();
        let (mut skeleton, imported, dummy_offset) = match scene_root.and_then(|root| adopt_scene(&mut node, root)) {
            Some(skeleton) => {
                use_rigid_meshes(&skeleton.clone().upcast());
                // Dummy bones (`d0` ...) come after the regular ones (`b0` ...).
                let regular = (0..skeleton.get_bone_count()).filter(|&i| !skeleton.get_bone_name(i).to_string().starts_with('d')).count();
                (skeleton, true, regular)
            }
            None => {
                let (Some(entry), Some(zmd)) = (entry, zmd()) else {
                    godot_warn!("rose: no model {} for npc {}", npc.model, npc_id.get());
                    return false;
                };
                let mut skeleton = build_skeleton(&zmd);
                let skin = skeleton.create_skin_from_rest_transforms();
                self.base_mut().add_child(&skeleton);
                let Some(zsc) = npc_zsc() else { return false };
                for model_id in entry.model_ids.iter() {
                    attach_object(&mut skeleton, skin.as_ref(), zsc, *model_id as usize, zmd.bones.len(), None);
                }
                (skeleton, false, zmd.bones.len())
            }
        };
        self.dummy_offset = dummy_offset;
        let skin = skeleton.create_skin_from_rest_transforms();
        for (list, part) in [
            ("3DDATA/WEAPON/LIST_WEAPON.ZSC", npc.right_hand_part_index),
            ("3DDATA/WEAPON/LIST_SUBWPN.ZSC", npc.left_hand_part_index),
        ] {
            if part != 0 {
                attach_model(&mut skeleton, skin.as_ref(), &rose_data::art::client_part(list, part), dummy_offset, None);
            }
        }
        self.base_mut().set_scale(Vector3::ONE * npc.scale);

        let looping = |name: &str| matches!(name, "stop" | "move" | "run");
        let player = imported.then(|| self.base().try_get_node_as::<AnimationPlayer>("AnimationPlayer")).flatten();
        match player.and_then(|player| player.get_animation_library("").map(|library| (player, library))) {
            Some((mut player, library)) => {
                player.remove_animation_library("");
                player.add_animation_library("", &own_library(&library, looping));
            }
            None => {
                let mut library = AnimationLibrary::new_gd();
                for (name, path) in npc_action_motions(npc_id) {
                    if let Some(animation) = load_motion(&path, dummy_offset, looping(&name)) {
                        library.add_animation(name.as_str(), &animation);
                    }
                }
                add_player(&mut self.to_gd().upcast(), library);
            }
        }
        true
    }

    /// Adds one of the NPC's own motions (by its index in LIST_NPC.CHR, as monster skills
    /// name them) to the animation player. Returns its animation name ("motion_ID"), or "".
    #[func]
    fn add_motion(&mut self, motion_id: i32) -> GString {
        let name = format!("motion_{motion_id}");
        let Some(player) = self.base().try_get_node_as::<AnimationPlayer>("AnimationPlayer") else {
            return GString::new();
        };
        if player.has_animation(name.as_str()) {
            return GString::from(name.as_str());
        }
        let (Some(game_data), Some(npc_id), Some(id)) =
            (data::get(), NpcId::new(self.npc_id), u16::try_from(motion_id).ok().map(rose_data::MotionId::new))
        else {
            return GString::new();
        };
        let Some(motion_file) = game_data.npcs.get_npc_motion(npc_id, id) else { return GString::new() };
        let Some(animation) = load_motion(&motion_file.path.path().to_string_lossy(), self.dummy_offset, false) else {
            return GString::new();
        };
        let Some(mut library) = player.get_animation_library("") else { return GString::new() };
        library.add_animation(name.as_str(), &animation);
        GString::from(name.as_str())
    }
}

fn npc_zsc() -> Option<&'static ZscFile> {
    static ZSC: std::sync::OnceLock<Option<ZscFile>> = std::sync::OnceLock::new();
    ZSC.get_or_init(|| data::read_file::<ZscFile>(NPC_ZSC)).as_ref()
}

/// An NPC's action motions as (animation name, ZMO path): stop, move, attack, hit, die, run.
fn npc_action_motions(npc_id: NpcId) -> Vec<(String, String)> {
    let Some(game_data) = data::get() else { return Vec::new() };
    (0..NpcMotionAction::LENGTH)
        .map(NpcMotionAction::from_usize)
        .filter_map(|action| {
            let motion = game_data.npcs.get_npc_action_motion(npc_id, action)?;
            Some((format!("{action:?}").to_lowercase(), motion.path.path().to_string_lossy().to_string()))
        })
        .collect()
}

/// Bumped when the imported characters, monsters, NPCs or vehicles change, so the next import redoes them.
pub const AVATAR_IMPORT_VERSION: i64 = 4;

#[derive(Default)]
pub struct AvatarImport {
    pub saved: i64,
    pub kept: i64,
    pub failed: i64,
    pub bytes: u64,
}

impl AvatarImport {
    /// Saves (or with `redo` false, keeps an existing) `res://assets/rose/<stem>.<ext>`.
    pub fn save(&mut self, redo: bool, stem: &str, extension: &str, make: impl FnOnce() -> Option<Gd<godot::classes::Resource>>) {
        let target = format!("{}{stem}.{extension}", assets::ROSE);
        let disk = godot::classes::ProjectSettings::singleton().globalize_path(&target).to_string();
        if !redo && std::path::Path::new(&disk).is_file() {
            self.kept += 1;
        } else if make().is_some_and(|resource| assets::save(resource, &target)) {
            self.saved += 1;
        } else {
            godot_warn!("rose: could not make {target}");
            self.failed += 1;
            return;
        }
        self.bytes += std::fs::metadata(&disk).map_or(0, |m| m.len());
    }
}

/// Packs `root` and its children (owned by it) as a scene.
pub(crate) fn pack(root: Gd<Node3D>) -> Option<Gd<godot::classes::Resource>> {
    let mut scene = PackedScene::new_gd();
    let packed = scene.pack(&root) == godot::global::Error::OK;
    root.free();
    packed.then(|| scene.upcast())
}

pub(crate) fn own<T: Inherits<Node>>(root: &Gd<Node3D>, node: Gd<T>) {
    node.upcast::<Node>().set_owner(root);
}

/// The import of player characters (step 2 of the Godot-native assets plan):
/// - every player motion (3DDATA/MOTION/AVATAR) as an Animation `.anim` beside its ZMO path,
///   with the frame events as `hit`/`step`/`effect` markers;
/// - per gender and weapon motion type, an AnimationLibrary of the action motions
///   (`3ddata/avatar/male_motions/weapon_N.tres`);
/// - per gender, a skeleton scene at the ZMD's path (`3ddata/avatar/male.tscn`): Skeleton3D
///   and an AnimationPlayer with the no-weapon library;
/// - every object of the part lists (faces, hair, caps, bodies, arms, feet, weapons, shields,
///   face and back items)
///   as a scene of mesh instances (`3ddata/avatar/list_mbody/12.tscn`).
/// Needs the meshes and textures imported first.
pub fn import_avatars(redo: bool) -> AvatarImport {
    let mut out = AvatarImport::default();
    let Some(game_data) = data::get() else { return out };
    let (Some(male_zmd), Some(female_zmd)) = (data::read_file::<ZmdFile>(skeleton_file(true)), data::read_file::<ZmdFile>(skeleton_file(false)))
    else {
        out.failed += 1;
        return out;
    };
    let dummy_offset = male_zmd.bones.len();
    if female_zmd.bones.len() != dummy_offset {
        godot_warn!("rose: male and female skeletons differ in bone count; female motions may move the wrong dummies");
    }

    // Which motions loop: those used for a looping action anywhere.
    let weapon_types = game_data.motions.weapon_type_count();
    let mut looping: HashSet<String> = HashSet::new();
    for gender in 0..2 {
        for weapon in 0..weapon_types {
            for index in 0..CharacterMotionAction::LENGTH {
                let action = CharacterMotionAction::from_usize(index);
                if is_looping(action) {
                    if let Some(path) = action_motion_path(action, weapon, gender) {
                        looping.insert(assets::stem(&path));
                    }
                }
            }
        }
    }

    // Every motion with the skeleton it plays on (its bone count, which names the dummy
    // bones) and whether it loops: player motions on the player skeleton, vehicle motions on
    // the cart or castle gear one, monster and NPC motions on their model's. A file used on
    // two skeletons with different bone counts is saved for the first; the game converts
    // it for the other.
    let mut motions: BTreeMap<String, (usize, bool)> = BTreeMap::new();
    let mut shared = 0;
    let mut claim = |motions: &mut BTreeMap<String, (usize, bool)>, path: String, offset: usize, loops: bool, replace: bool| {
        match motions.get_mut(&path) {
            Some(entry) if replace => *entry = (offset, loops),
            Some(entry) => {
                if entry.0 != offset {
                    shared += 1;
                }
                entry.1 |= loops;
            }
            None => {
                motions.insert(path, (offset, loops));
            }
        }
    };
    for path in (0..3).flat_map(|gender| game_data.motions.motion_files(gender)).map(|m| m.path.path().to_string_lossy().to_string()) {
        let loops = looping.contains(&assets::stem(&path));
        claim(&mut motions, path, dummy_offset, loops, false);
    }
    for (is_cart, body) in vehicle_bodies() {
        let Some(zmd) = data::read_file::<ZmdFile>(vehicle_skeleton_file(is_cart)) else { continue };
        for weapon in 0..weapon_types {
            for (action, path) in vehicle_action_motions(body, weapon, is_cart) {
                let loops = matches!(action, VehicleMotionAction::Stop | VehicleMotionAction::Move);
                claim(&mut motions, path, zmd.bones.len(), loops, true);
            }
        }
    }
    let mut npc_skeletons: HashMap<u16, Option<ZmdFile>> = HashMap::new();
    let mut npc_ids: Vec<u16> = game_data.npc_chr.npcs.keys().copied().collect();
    npc_ids.sort();
    for &id in &npc_ids {
        let model = &game_data.npc_chr.npcs[&id];
        let zmd = npc_skeletons
            .entry(model.skeleton_index)
            .or_insert_with(|| game_data.npc_chr.skeleton_files.get(model.skeleton_index as usize).and_then(|p| data::read_file::<ZmdFile>(p)));
        let Some(zmd) = zmd else { continue };
        let actions: HashMap<String, String> =
            NpcId::new(id).map(npc_action_motions).unwrap_or_default().into_iter().map(|(name, path)| (path, name)).collect();
        for &(_, file) in &model.motion_ids {
            let Some(path) = game_data.npc_chr.motion_files.get(file as usize) else { continue };
            let path = rose_file_readers::VfsPathBuf::new(path).path().to_string_lossy().to_string();
            let loops = actions.get(&path).is_some_and(|name| matches!(name.as_str(), "stop" | "move" | "run"));
            claim(&mut motions, path, zmd.bones.len(), loops, false);
        }
    }
    if shared > 0 {
        godot_print!("rose: {shared} motions play on skeletons with different bone counts; the game converts those at run time");
    }
    for (path, &(offset, loops)) in &motions {
        if !game_data.vfs.exists(path.as_str()) {
            continue;
        }
        out.save(redo, &assets::stem(path), "anim", || {
            let zmo = data::read_file::<ZmoFile>(path)?;
            let mut animation = build_animation(&zmo, offset, loops);
            animation.remove_meta("frame_events");
            animation.set_meta("dummy_offset", &(offset as i64).to_variant());
            add_event_markers(&mut animation, &zmo.frame_events, zmo.fps.max(1) as f64);
            Some(animation.upcast())
        });
    }

    for (male, zmd) in [(true, &male_zmd), (false, &female_zmd)] {
        let gender = if male { 0 } else { 1 };
        for weapon in 0..weapon_types {
            let library = avatar_motion_library(gender, weapon, dummy_offset, true);
            if library.get_animation_list().is_empty() {
                continue;
            }
            out.save(redo, &motion_library_stem(male, weapon), "tres", || Some(library.upcast()));
        }

        out.save(redo, &assets::stem(skeleton_file(male)), "tscn", || {
            let mut root = Node3D::new_alloc();
            root.set_name(if male { "Male" } else { "Female" });
            let skeleton = build_skeleton(zmd);
            root.add_child(&skeleton);
            own(&root, skeleton);
            let mut player = AnimationPlayer::new_alloc();
            player.set_name("AnimationPlayer");
            if let Some(library) = assets::load_stem::<AnimationLibrary>(&motion_library_stem(male, 0), &["tres"]) {
                player.add_animation_library("", &library);
            }
            root.add_child(&player);
            own(&root, player);
            pack(root)
        });

        for part in PARTS {
            // Weapons, shields, face and back items are the same list for both.
            if !male && matches!(part, Part::Weapon | Part::SubWeapon | Part::FaceItem | Part::Back) {
                continue;
            }
            let list = part.list(male);
            let Some(zsc) = data::read_file::<ZscFile>(list) else { continue };
            for object_id in 0..zsc.objects.len() {
                let meshes = part_meshes(&zsc, object_id, dummy_offset, part.default_bone(dummy_offset), true);
                if meshes.is_empty() {
                    continue;
                }
                out.save(redo, &format!("{}/{object_id}", assets::stem(list)), "tscn", || {
                    let mut root = Node3D::new_alloc();
                    root.set_name(&format!("{part:?}{object_id}"));
                    for (i, (mut instance, skinned, bone)) in meshes.into_iter().enumerate() {
                        instance.set_name(&format!("Part{i}"));
                        instance.set_meta("skinned", &skinned.to_variant());
                        instance.set_meta("bone", &bone.map_or(String::new(), |b| bone_name(b, dummy_offset)).to_variant());
                        root.add_child(&instance);
                        own(&root, instance);
                    }
                    pack(root)
                });
            }
        }
    }

    // Monster and NPC models: skeleton, parts and an AnimationPlayer with the action motions.
    if let Some(zsc) = npc_zsc() {
        for &id in &npc_ids {
            let model = &game_data.npc_chr.npcs[&id];
            let Some(Some(zmd)) = npc_skeletons.get(&model.skeleton_index) else { continue };
            let offset = zmd.bones.len();
            out.save(redo, &npc_scene_stem(id), "tscn", || {
                let mut root = Node3D::new_alloc();
                root.set_name(&format!("Npc{id}"));
                let mut skeleton = build_skeleton(zmd);
                let skin = skeleton.create_skin_from_rest_transforms();
                root.add_child(&skeleton);
                let mut part = 0;
                for model_id in model.model_ids.iter() {
                    for (mut instance, skinned, bone) in part_meshes(zsc, *model_id as usize, offset, None, true) {
                        instance.set_name(&format!("Part{part}"));
                        part += 1;
                        instance.set_meta("skinned", &skinned.to_variant());
                        place_part(&mut skeleton, skin.as_ref(), instance, skinned, bone.map(|b| bone_name(b, offset)));
                    }
                }
                own_all(&root, &skeleton.upcast());
                let mut library = AnimationLibrary::new_gd();
                for (name, path) in NpcId::new(id).map(npc_action_motions).unwrap_or_default() {
                    if let Some(animation) = assets::load_stem::<Animation>(&assets::stem(&path), &["anim"]) {
                        library.add_animation(name.as_str(), &animation);
                    }
                }
                let mut player = AnimationPlayer::new_alloc();
                player.set_name("AnimationPlayer");
                player.add_animation_library("", &library);
                root.add_child(&player);
                own(&root, player);
                pack(root)
            });
        }
    }

    // Carts and castle gear: the two skeletons with their seats, and every part.
    for is_cart in [true, false] {
        let path = vehicle_skeleton_file(is_cart);
        let Some(zmd) = data::read_file::<ZmdFile>(path) else { continue };
        out.save(redo, &assets::stem(path), "tscn", || {
            let mut root = Node3D::new_alloc();
            root.set_name(if is_cart { "Cart" } else { "CastleGear" });
            let mut skeleton = build_skeleton(&zmd);
            add_seats(&mut skeleton, &zmd);
            root.add_child(&skeleton);
            own_all(&root, &skeleton.upcast());
            pack(root)
        });
    }
    if let (Some(zsc), Some(cart)) = (data::read_file::<ZscFile>(VEHICLE_ZSC), data::read_file::<ZmdFile>(vehicle_skeleton_file(true))) {
        let offset = cart.bones.len();
        for object_id in 0..zsc.objects.len() {
            let meshes = part_meshes(&zsc, object_id, offset, None, true);
            if meshes.is_empty() {
                continue;
            }
            out.save(redo, &format!("{}/{object_id}", assets::stem(VEHICLE_ZSC)), "tscn", || {
                let mut root = Node3D::new_alloc();
                root.set_name(&format!("Vehicle{object_id}"));
                for (i, (mut instance, skinned, bone)) in meshes.into_iter().enumerate() {
                    instance.set_name(&format!("Part{i}"));
                    instance.set_meta("skinned", &skinned.to_variant());
                    instance.set_meta("bone", &bone.map_or(String::new(), |b| bone_name(b, offset)).to_variant());
                    root.add_child(&instance);
                    own(&root, instance);
                }
                pack(root)
            });
        }
    }
    out
}

/// Makes `root` the owner of `node` and everything under it, so packing keeps them.
fn own_all(root: &Gd<Node3D>, node: &Gd<Node>) {
    node.clone().set_owner(root);
    for child in node.get_children().iter_shared() {
        own_all(root, &child);
    }
}

/// Every vehicle body item, with whether it is a cart.
fn vehicle_bodies() -> Vec<(bool, u32)> {
    let Some(game_data) = data::get() else { return Vec::new() };
    (0..4096)
        .filter_map(|id| game_data.items.get_vehicle_item(id))
        .filter(|v| matches!(v.vehicle_part, rose_data::VehiclePartIndex::Body))
        .map(|v| (matches!(v.vehicle_type, VehicleType::Cart), v.base_motion_index))
        .collect()
}

/// A vehicle body's motions for an arms motion column, as vehicle_motions picks them.
fn vehicle_action_motions(base_motion_index: u32, weapon: usize, is_cart: bool) -> Vec<(VehicleMotionAction, String)> {
    let Some(game_data) = data::get() else { return Vec::new() };
    (0..VehicleMotionAction::LENGTH)
        .map(VehicleMotionAction::from_usize)
        .filter_map(|action| {
            let lookup = if is_cart && matches!(action, VehicleMotionAction::Attack2 | VehicleMotionAction::Attack3) {
                VehicleMotionAction::Attack1
            } else {
                action
            };
            let motion = game_data.motions.get_vehicle_action_motion(lookup, base_motion_index as usize, weapon)?;
            Some((action, motion.path.path().to_string_lossy().to_string()))
        })
        .collect()
}

/// `export/motions.toml` from the player, monster and NPC animations in the asset folders (`ours/` first):
/// each motion's frame rate, length in frames and the frames of its `hit` markers. The
/// server times hits from this file, so it has to match the animations; `--export-data`
/// writes it and `--export-data=check` fails when it is out of date.
pub fn motion_timings() -> Option<rose_data_export::MotionTimings> {
    let game_data = data::get()?;
    let mut motions: Vec<String> = (0..3)
        .flat_map(|gender| game_data.motions.motion_files(gender))
        .map(|motion| motion.path.path().to_string_lossy().to_string())
        .chain(game_data.npcs.iter().flat_map(|npc| npc.motion_data.iter().map(|(_, m)| m.path.path().to_string_lossy().to_string())))
        .collect();
    motions.sort();
    motions.dedup();
    let mut timings = rose_data_export::MotionTimings::default();
    for path in motions {
        let Some(animation) = assets::load_stem::<Animation>(&assets::stem(&path), &["anim"]) else { continue };
        let fps: f64 = animation.get_meta("fps").try_to().unwrap_or(30.0);
        let mut hit_frames: Vec<u32> = animation
            .get_marker_names()
            .as_slice()
            .iter()
            .filter(|name| {
                let name = name.to_string();
                name.starts_with("hit") && name.rsplit_once(':').and_then(|(_, id)| id.parse::<u16>().ok()).is_some_and(is_hit_event)
            })
            .map(|name| (animation.get_marker_time(&StringName::from(name)) * fps).round() as u32)
            .collect();
        hit_frames.sort();
        timings.motion.push(rose_data_export::MotionTiming {
            file: path,
            fps: fps.round() as u32,
            frames: (animation.get_length() as f64 * fps).round() as u32,
            hit_frames,
        });
    }
    Some(timings)
}
