use bevy::prelude::*;

use crate::components::PlayerCamera;

use super::components::CameraDriver;

pub fn update_camera(
    mut cam: Query<(&mut Transform, &PlayerCamera)>,
    target: Query<&GlobalTransform, With<CameraDriver>>,
) {
    if let Ok((mut transform, c)) = cam.get_single_mut() {
        if let Ok(t) = target.get_single() {
            let delta = t.translation() - transform.translation;
            if delta != Vec3::ZERO {
                transform.translation += delta * c.tracking;
            }
        }
    }
}
