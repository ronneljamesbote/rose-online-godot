//! Player characters: ZMD skeleton, ZSC body parts and ZMO motions.
//! Ported from rose-offline-client's model_loader.rs and zmo_asset_loader.rs.

use enum_map::Enum;
use godot::{
    classes::{
        animation::{InterpolationType, LoopMode, TrackType},
        Animation, AnimationLibrary, AnimationPlayer, BoneAttachment3D, MeshInstance3D, Node3D, Skeleton3D, Skin,
    },
    prelude::*,
};
use rose_data::{CharacterMotionAction, NpcId, NpcMotionAction, VehicleMotionAction, VehicleType};
use rose_file_readers::{ZmdFile, ZmoChannel, ZmoFile, ZscFile};

use crate::{data, material, mesh};

#[derive(Clone, Copy)]
enum Part {
    Face,
    Hair,
    Head,
    Body,
    Hands,
    Feet,
    Weapon,
    SubWeapon,
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
        };
        if male { m } else { w }
    }

    /// DefaultBoneId in model_loader.rs; dummy bones come after the regular bones.
    fn default_bone(self, dummy_offset: usize) -> Option<usize> {
        match self {
            Part::Face | Part::Hair => Some(4),
            Part::Head => Some(dummy_offset + 6),
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
    animation.set_length((zmo.num_frames as f64 / fps) as f32);
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

/// Adds one ZSC object's parts: skinned parts under the skeleton, rigid parts on their
/// bone or dummy bone (spawn_model in model_loader.rs).
fn attach_object(
    skeleton: &mut Gd<Skeleton3D>,
    skin: Option<&Gd<Skin>>,
    zsc: &ZscFile,
    object_id: usize,
    dummy_offset: usize,
    default_bone: Option<usize>,
) {
    let Some(object) = zsc.objects.get(object_id) else { return };
    for object_part in object.parts.iter() {
        let Some(zsc_material) = zsc.materials.get(object_part.material_id as usize) else { continue };
        let Some(mesh_path) = zsc.meshes.get(object_part.mesh_id as usize) else { continue };
        let Some(part_mesh) = mesh::load_mesh(&mesh_path.path().to_string_lossy(), zsc_material.is_skin) else { continue };
        let mut instance = MeshInstance3D::new_alloc();
        instance.set_mesh(&part_mesh);
        instance.set_material_override(&material::object_material(zsc_material, None));

        let link_bone = object_part
            .bone_index
            .map(|b| b as usize)
            .or(object_part.dummy_index.map(|d| d as usize + dummy_offset))
            .or(default_bone);
        if zsc_material.is_skin {
            if let Some(skin) = skin {
                instance.set_skin(skin);
            }
            instance.set_skeleton_path(&NodePath::from(".."));
            skeleton.add_child(&instance);
        } else if let Some(bone) = link_bone {
            let mut attachment = BoneAttachment3D::new_alloc();
            attachment.set_bone_name(&bone_name(bone, dummy_offset));
            skeleton.add_child(&attachment);
            attachment.add_child(&instance);
        } else {
            // Rigid parts with no bone sit on the model's own transform; inside the
            // skeleton node that is the same place.
            skeleton.add_child(&instance);
        }
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

#[godot_api]
impl RoseCharacter {
    /// Builds the character. Part ids are item numbers (0 = not equipped), as in the
    /// Bevy client's get_model_part_index. Body, hands and feet fall back to 1.
    #[func]
    #[allow(clippy::too_many_arguments)]
    fn build(&mut self, male: bool, face: i32, hair: i32, head: i32, body: i32, hands: i32, feet: i32, weapon: i32, sub_weapon: i32) -> bool {
        let Some(game_data) = data::get() else {
            godot_error!("rose: call RoseData.open() first");
            return false;
        };
        let Some(zmd) = data::read_file::<ZmdFile>(if male { "3DDATA/AVATAR/MALE.ZMD" } else { "3DDATA/AVATAR/FEMALE.ZMD" }) else {
            return false;
        };
        let dummy_offset = zmd.bones.len();
        let mut skeleton = build_skeleton(&zmd);
        let skin = skeleton.create_skin_from_rest_transforms();
        self.base_mut().add_child(&skeleton);

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
        ];
        for (part, id) in parts {
            if id <= 0 && !matches!(part, Part::Face | Part::Hair) {
                continue;
            }
            let Some(zsc) = data::read_file::<ZscFile>(part.list(male)) else { continue };
            attach_object(&mut skeleton, skin.as_ref(), &zsc, id as usize, dummy_offset, part.default_bone(dummy_offset));
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
        let mut library = AnimationLibrary::new_gd();
        for index in 0..CharacterMotionAction::LENGTH {
            let action = CharacterMotionAction::from_usize(index);
            let motion = [(weapon_motion_type, gender), (weapon_motion_type, 0), (0, gender), (0, 0)]
                .into_iter()
                .find_map(|(w, g)| game_data.motions.get_character_action_motion(action, w, g));
            let Some(motion) = motion else { continue };
            let Some(zmo) = data::read_file::<ZmoFile>(&motion.path.path().to_string_lossy()) else { continue };
            let looping = matches!(
                action,
                CharacterMotionAction::Stop1 | CharacterMotionAction::Stop2 | CharacterMotionAction::Stop3
                    | CharacterMotionAction::Walk | CharacterMotionAction::Run | CharacterMotionAction::Sit
            );
            library.add_animation(&action_name(action), &build_animation(&zmo, dummy_offset, looping));
        }

        add_player(&mut self.to_gd().upcast(), library);
        true
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
        let Some(zmo) = data::read_file::<ZmoFile>(&motion.path.path().to_string_lossy()) else { return GString::new() };
        let Some(mut library) = player.get_animation_library("") else { return GString::new() };
        library.add_animation(name.as_str(), &build_animation(&zmo, self.dummy_offset, false));
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
        let Some(zmo) = data::read_file::<ZmoFile>(&motion.path.path().to_string_lossy()) else { continue };
        let looping = matches!(action, VehicleMotionAction::Stop | VehicleMotionAction::Move);
        library.add_animation(
            &format!("{prefix}{}", format!("{action:?}").to_lowercase()),
            &build_animation(&zmo, dummy_offset, looping),
        );
    }
    library
}

/// Carts and castle gear (spawn_vehicle_model in model_loader.rs): the body's skeleton with
/// the four parts from LIST_PAT.ZSC; the driver sits on dummy bone 0.
#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct RoseVehicle {
    base: Base<Node3D>,
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
        let skeleton_path = if is_cart {
            "3DDATA/PAT/CART/CART01.ZMD"
        } else {
            "3DDATA/PAT/CASTLEGEAR/CASTLEGEAR02/CASTLEGEAR02.ZMD"
        };
        let Some(zmd) = data::read_file::<ZmdFile>(skeleton_path) else { return false };
        let Some(zsc) = data::read_file::<ZscFile>("3DDATA/PAT/LIST_PAT.ZSC") else { return false };
        let dummy_offset = zmd.bones.len();
        let mut skeleton = build_skeleton(&zmd);
        let skin = skeleton.create_skin_from_rest_transforms();
        self.base_mut().add_child(&skeleton);
        for part in [body, engine, leg, arms] {
            if part > 0 {
                attach_object(&mut skeleton, skin.as_ref(), &zsc, part as usize, dummy_offset, None);
            }
        }
        let mut seat = BoneAttachment3D::new_alloc();
        seat.set_name("Seat");
        seat.set_bone_name(&bone_name(dummy_offset, dummy_offset));
        skeleton.add_child(&seat);
        // A passenger sits on dummy bone 10 (CObjCART::Create(CObjCHAR*) links it there).
        if zmd.dummy_bones.len() > 10 {
            let mut back = BoneAttachment3D::new_alloc();
            back.set_name("BackSeat");
            back.set_bone_name(&bone_name(dummy_offset + 10, dummy_offset));
            skeleton.add_child(&back);
        }

        let weapon = game_data.items.get_vehicle_item(arms.max(0) as usize).map_or(0, |v| v.base_motion_index as usize);
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
}

/// Monsters and NPCs from LIST_NPC.CHR and PART_NPC.ZSC (spawn_npc_model in model_loader.rs).
#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct RoseNpc {
    base: Base<Node3D>,
    npc_id: u16,
    dummy_offset: usize,
}

#[godot_api]
impl RoseNpc {
    /// Builds the model for an NPC id, scaled as in LIST_NPC.STB. Animations are named after
    /// NpcMotionAction in lower case: stop, move, attack, hit, die, run.
    #[func]
    fn build(&mut self, npc_id: i32) -> bool {
        let Some(game_data) = data::get() else {
            godot_error!("rose: call RoseData.open() first");
            return false;
        };
        let Some(npc_id) = u16::try_from(npc_id).ok().and_then(NpcId::new) else { return false };
        let Some(model) = game_data.npc_chr.npcs.get(&npc_id.get()) else {
            godot_warn!("rose: no model for npc {}", npc_id.get());
            return false;
        };
        let zmd = game_data
            .npc_chr
            .skeleton_files
            .get(model.skeleton_index as usize)
            .and_then(|path| data::read_file::<ZmdFile>(path));
        let Some(zmd) = zmd else { return false };
        let dummy_offset = zmd.bones.len();
        self.npc_id = npc_id.get();
        self.dummy_offset = dummy_offset;
        let mut skeleton = build_skeleton(&zmd);
        let skin = skeleton.create_skin_from_rest_transforms();
        self.base_mut().add_child(&skeleton);

        let Some(zsc) = npc_zsc() else { return false };
        for model_id in model.model_ids.iter() {
            attach_object(&mut skeleton, skin.as_ref(), zsc, *model_id as usize, dummy_offset, None);
        }
        let npc = game_data.npcs.get_npc(npc_id);
        if let Some(npc) = npc {
            for (list, part) in [
                ("3DDATA/WEAPON/LIST_WEAPON.ZSC", npc.right_hand_part_index),
                ("3DDATA/WEAPON/LIST_SUBWPN.ZSC", npc.left_hand_part_index),
            ] {
                if part != 0 {
                    if let Some(weapon_zsc) = data::read_file::<ZscFile>(list) {
                        attach_object(&mut skeleton, skin.as_ref(), &weapon_zsc, part as usize, dummy_offset, None);
                    }
                }
            }
            self.base_mut().set_scale(Vector3::ONE * npc.scale);
        }

        let mut library = AnimationLibrary::new_gd();
        for index in 0..NpcMotionAction::LENGTH {
            let action = NpcMotionAction::from_usize(index);
            let Some(motion) = game_data.npcs.get_npc_action_motion(npc_id, action) else { continue };
            let Some(zmo) = data::read_file::<ZmoFile>(&motion.path.path().to_string_lossy()) else { continue };
            let looping = matches!(action, NpcMotionAction::Stop | NpcMotionAction::Move | NpcMotionAction::Run);
            library.add_animation(&format!("{action:?}").to_lowercase(), &build_animation(&zmo, dummy_offset, looping));
        }
        add_player(&mut self.to_gd().upcast(), library);
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
        let Some(motion) = game_data.npcs.get_npc_motion(npc_id, id) else { return GString::new() };
        let Some(zmo) = data::read_file::<ZmoFile>(&motion.path.path().to_string_lossy()) else { return GString::new() };
        let Some(mut library) = player.get_animation_library("") else { return GString::new() };
        library.add_animation(name.as_str(), &build_animation(&zmo, self.dummy_offset, false));
        GString::from(name.as_str())
    }
}

fn npc_zsc() -> Option<&'static ZscFile> {
    static ZSC: std::sync::OnceLock<Option<ZscFile>> = std::sync::OnceLock::new();
    ZSC.get_or_init(|| data::read_file::<ZscFile>("3DDATA/NPC/PART_NPC.ZSC")).as_ref()
}
