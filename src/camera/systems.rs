use bevy::prelude::*;
use bevy_codex::components::MainCam;
use bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraTarget, Zoom};

use crate::game::{movement::components::CharacterController, player::components::Player};

pub fn add_player_camera(
    trigger: Trigger<OnAdd, CharacterController>,
    players: Query<(Entity, &Player)>,
    mut commands: Commands,
    camera: Query<Entity, With<MainCam>>,
) {
    for (ent, player) in players.iter() {
        if ent == trigger.entity() && player.is_local {
            let mut binding = commands.entity(camera.single());
            binding.insert(ThirdPersonCamera {
                aim_speed: 6.0,
                zoom: Zoom::new(5.0, 10.0),
                cursor_lock_active: false,
                sensitivity: Vec2::splat(1.0),
                ..default()
            });
        }
    }
}

pub fn update_camera_target(
    mut target: Query<&mut Transform, With<ThirdPersonCameraTarget>>,
    racer: Query<(Entity, &Player)>,
    suspensions: Query<(&GlobalTransform, &Parent)>,
) {
    for mut t in target.iter_mut() {
        for (ent, player) in racer.iter() {
            if player.is_local {
                for (gt, parent) in suspensions.iter() {
                    if **parent == ent {
                        t.translation = gt.translation();
                    }
                }
            }
        }
    }
}
