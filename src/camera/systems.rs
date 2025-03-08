use bevy::prelude::*;

use crate::components::Grip;

use super::components::PlayerCamera;

pub fn update_camera_position(
    mut cam: Query<(&mut Transform, &Parent, &Grip), With<PlayerCamera>>,
    target: Query<(Entity, &Transform), Without<PlayerCamera>>,
) {
    if let Ok((mut transform, parent, grip)) = cam.get_single_mut() {
        if let Ok(t) = target.get(**parent) {
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
    target: Query<(Entity, &Transform), Without<PlayerCamera>>,
) {
    if let Ok((mut transform, parent, grip)) = cam.get_single_mut() {
        if let Ok((t, t_transform)) = target.get(**parent) {
            let rotation = transform
                .looking_at(
                    t_transform.translation + t_transform.up() * grip.rotation_offset.y,
                    t_transform.up(),
                )
                .rotation;
            if transform.rotation != rotation {
                transform.rotation = transform.rotation.lerp(rotation, grip.tracking.1);
            }
        }
    }
}
