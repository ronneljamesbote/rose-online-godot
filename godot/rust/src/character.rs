//! Player characters: ZMD skeleton, ZSC body parts and ZMO motions.
//! Ported from rose-offline-client's model_loader.rs and zmo_asset_loader.rs.

use enum_map::Enum;
use godot::{
    classes::{
        animation::{InterpolationType, LoopMode, TrackType},
        Animation, AnimationLibrary, AnimationPlayer, BoneAttachment3D, MeshInstance3D, Node3D, Skeleton3D,
    },
    prelude::*,
};
use rose_data::CharacterMotionAction;
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
            let Some(object) = zsc.objects.get(id as usize) else { continue };
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
                    .or(part.default_bone(dummy_offset));
                if zsc_material.is_skin {
                    if let Some(skin) = skin.as_ref() {
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
                    skeleton.add_child(&instance);
                }
            }
        }

        // Motions for the equipped weapon type, with the same fallbacks as load_character_action_motions.
        let weapon_motion_type = if weapon > 0 {
            game_data.items.get_weapon_item(weapon as usize).map_or(0, |item| item.motion_type as usize)
        } else {
            0
        };
        let gender = if male { 0 } else { 1 };
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

        let mut player = AnimationPlayer::new_alloc();
        player.set_name("AnimationPlayer");
        player.add_animation_library("", &library);
        self.base_mut().add_child(&player);
        true
    }
}
