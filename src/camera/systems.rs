use bevy::prelude::*;

use crate::components::Grip;

use super::components::{CameraDriver, PlayerCamera};

pub fn update_camera_position(
    mut cam: Query<(&mut Transform, &Grip), With<PlayerCamera>>,
    target: Query<&GlobalTransform, With<CameraDriver>>,
) {
    if let Ok((mut transform, grip)) = cam.get_single_mut() {
        if let Ok(t) = target.get_single() {
            let distance = transform.translation.distance(t.translation());
            let future_position = t.translation()
                + *t.back() * grip.location_offset.z
                + *t.up() * grip.location_offset.y;
            transform.translation = transform.translation.lerp(
                future_position,
                EasingCurve::new(0.2, 1.0, grip.smoothing_curve)
                    .sample(grip.tracking.0)
                    .unwrap_or(0.0),
            );
        }
    }
}

pub fn update_camera_rotation(
    mut cam: Query<(&mut Transform, &Grip), With<PlayerCamera>>,
    target: Query<&GlobalTransform, With<CameraDriver>>,
) {
    if let Ok((mut transform, grip)) = cam.get_single_mut() {
        if let Ok(t) = target.get_single() {
            let rotation = transform
                .looking_at(t.translation() + t.up() * grip.rotation_offset.y, t.up())
                .rotation;
            if transform.rotation != rotation {
                transform.rotation = transform.rotation.lerp(rotation, grip.tracking.1);
            }
        }
    }
}
