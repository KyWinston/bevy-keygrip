use super::components::{CameraDriver, PlayerCamera};
use crate::components::Grip;
use bevy::prelude::*;

pub fn connect_player_cam(
    trigger: Trigger<OnAdd, CameraDriver>,
    mut commands: Commands,
    driver: Query<&Transform, With<CameraDriver>>,
    cam: Query<(Entity, &Grip), With<PlayerCamera>>,
) {
    if let Ok(driver_position) = driver.get(trigger.entity()) {
        let cam = commands
            .entity(cam.single().0)
            .insert((Transform::from_translation(
                driver_position.translation + cam.single().1.location_offset,
            )
            .looking_at(
                driver_position.translation + cam.single().1.rotation_offset,
                Vec3::Y,
            ),))
            .id();
        commands.entity(trigger.entity()).add_child(cam);
    }
}
pub fn update_camera_position(
    mut cam: Query<(&mut Transform, &Parent, &Grip), With<PlayerCamera>>,
    target: Query<Entity, With<CameraDriver>>,
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
    mut cam: Query<(&mut Transform, &Parent, &Grip), With<PlayerCamera>>,
    target: Query<(Entity, &GlobalTransform), With<CameraDriver>>,
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
