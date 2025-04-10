use super::components::{CameraDriver, PlayerCamera};
use crate::components::Grip;
use bevy::prelude::*;

pub fn connect_player_cam(
    trigger: Trigger<OnAdd, CameraDriver>,
    mut commands: Commands,
    driver: Query<&Transform, With<CameraDriver>>,
    cam: Query<(Entity, &Grip), With<PlayerCamera>>,
) {
    println!("add driver");
    let driver_position = driver.get(trigger.target()).unwrap().translation;
    let cam = commands
        .entity(cam.single().expect("camera not found").0)
        .insert((Transform::from_translation(
            driver_position + cam.single().expect("no camera found").1.location_offset,
        )
        .looking_at(
            driver_position + cam.single().expect("no camera found").1.rotation_offset,
            Vec3::Y,
        ),))
        .id();
    commands.entity(trigger.target()).add_child(cam);
}

pub fn update_camera_position(
    mut cam: Query<(&mut Transform, &ChildOf, &Grip), With<CameraDriver>>,
    target: Query<(Entity, &Transform), Without<CameraDriver>>,
) {
    if let Ok((mut transform, child_of, grip)) = cam.single_mut() {
        if let Ok(_) = target.get(child_of.parent) {
            let future_position = grip.location_offset;

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
    mut cam: Query<(&mut Transform, &ChildOf, &Grip), With<CameraDriver>>,
    target: Query<(Entity, &Transform), Without<CameraDriver>>,
) {
    if let Ok((mut transform, child_of, grip)) = cam.single_mut() {
        if let Ok((_t, t_transform)) = target.get(child_of.parent) {
            let rotation = transform
                .looking_at(
                    transform.translation + grip.rotation_offset.y,
                    t_transform.up(),
                )
                .rotation;
            if transform.rotation != rotation {
                transform.rotation.x = transform.rotation.lerp(rotation, grip.tracking.1.x).x;
                transform.rotation.y = transform.rotation.lerp(rotation, grip.tracking.1.y).y;
                transform.rotation.z = transform.rotation.lerp(rotation, grip.tracking.1.z).z;
            }
        }
    }
}
