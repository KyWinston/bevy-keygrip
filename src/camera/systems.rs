use bevy::prelude::*;

use crate::components::Grip;

use super::components::{CameraDriver, PlayerCamera};

pub fn update_camera_position(
    mut cam: Query<(&mut Transform, &Grip), With<PlayerCamera>>,
    target: Query<&GlobalTransform, With<CameraDriver>>,
    time: Res<Time<Fixed>>,
) {
    let a = time.overstep_fraction();
    if let Ok((mut transform, grip)) = cam.get_single_mut() {
        if let Ok(t) = target.get_single() {
            let future_position = t.translation()
                + *t.back() * grip.location_offset.z
                + *t.up() * grip.location_offset.y;
            transform.translation = transform
                .translation
                .lerp(future_position, grip.tracking.0 * a);
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
