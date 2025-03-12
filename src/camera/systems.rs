use super::components::{CameraDriver, PlayerCamera};
use crate::components::Grip;
use bevy::prelude::*;

pub fn connect_player_cam(
    trigger: Trigger<OnAdd, CameraDriver>,
    mut commands: Commands,
    driver: Query<&Transform, With<CameraDriver>>,
    cam: Query<Entity, With<PlayerCamera>>,
) {
    let driver_poition = driver.get(trigger.entity()).unwrap().translation;
    let cam = commands
        .entity(cam.single())
        .insert((
            Grip {
                location_offset: Vec3::new(0.0, 2.0, 7.0),
                rotation_offset: Vec3::ZERO,
                tracking: (1.0, 1.0),
                near: 1.0,
                far: 15.0,
                smoothing_curve: EaseFunction::QuadraticIn,
                ..default()
            },
            Transform::from_translation(driver_poition + Vec3::new(0.0, 2.0, 7.0))
                .looking_at(driver_poition, Vec3::Y),
        ))
        .id();
    commands.entity(trigger.entity()).add_child(cam);
}

pub fn update_camera_position(
    mut cam: Query<(&mut Transform, &Parent, &Grip), With<CameraDriver>>,
    target: Query<(Entity, &Transform), Without<CameraDriver>>,
) {
    if let Ok((mut transform, parent, grip)) = cam.get_single_mut() {
        if let Ok(_) = target.get(**parent) {
            let future_position = grip.location_offset;

            transform.translation = transform.translation.lerp(
                future_position,
                EasingCurve::new(0.3, 1.0, grip.smoothing_curve)
                    .sample(grip.tracking.0)
                    .unwrap_or(0.0),
            );
        }
    }
}

pub fn update_camera_rotation(
    mut cam: Query<(&mut Transform, &Parent, &Grip), With<CameraDriver>>,
    target: Query<(Entity, &Transform), Without<CameraDriver>>,
) {
    if let Ok((mut transform, parent, grip)) = cam.get_single_mut() {
        if let Ok((_t, t_transform)) = target.get(**parent) {
            let rotation = transform
                .looking_at(
                    transform.translation + grip.rotation_offset.y,
                    t_transform.up(),
                )
                .rotation;
            if transform.rotation != rotation {
                transform.rotation = transform.rotation.lerp(rotation, grip.tracking.1);
            }
        }
    }
}
