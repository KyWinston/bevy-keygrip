use super::components::CameraDriver;
use crate::components::Grip;
use bevy::prelude::*;

pub fn update_camera_position(
    mut cam: Query<(&mut Transform, &Grip), Without<CameraDriver>>,
    target: Query<&GlobalTransform, With<CameraDriver>>,
) {
    if let Ok((mut transform, grip)) = cam.single_mut() {
        if let Ok(target_transform) = target.single() {
            let future_position = target_transform.translation() + grip.location_offset;

            transform.translation.x = transform
                .translation
                .lerp(
                    future_position,
                    EasingCurve::new(0.3, 1.0, grip.smoothing_curve)
                        .sample(grip.tracking.0.x)
                        .unwrap_or(0.0),
                )
                .x;
            transform.translation.y = transform
                .translation
                .lerp(
                    future_position,
                    EasingCurve::new(0.3, 1.0, grip.smoothing_curve)
                        .sample(grip.tracking.0.y)
                        .unwrap_or(0.0),
                )
                .y;
            transform.translation.z = transform
                .translation
                .lerp(
                    future_position,
                    EasingCurve::new(0.3, 1.0, grip.smoothing_curve)
                        .sample(grip.tracking.0.z)
                        .unwrap_or(0.0),
                )
                .z;
        }
    }
}

pub fn update_camera_rotation(
    mut cam: Query<(&mut Transform, &Grip), Without<CameraDriver>>,
    target: Query<(Entity, &GlobalTransform), With<CameraDriver>>,
) {
    if let Ok((mut transform, grip)) = cam.single_mut() {
        if let Ok((_t, t_transform)) = target.single() {
            let t_rotation = transform
                .looking_at(
                    t_transform.translation() + grip.rotation_offset.y,
                    t_transform.up(),
                )
                .rotation;
            transform.rotation.x = transform.rotation.slerp(t_rotation, grip.tracking.1.x).x;
            transform.rotation.y = transform.rotation.slerp(t_rotation, grip.tracking.1.y).y;
            transform.rotation.z = transform.rotation.slerp(t_rotation, grip.tracking.1.z).z;
        }
    }
}
