use bevy::prelude::*;

use crate::{
    components::{Grip, GripRig},
    events::SwitchActive,
};

pub fn focus_cameras(mut cam_q: Query<(&mut Transform, &GripRig), With<Grip>>) {
    for (mut cam, grip) in cam_q.iter_mut() {
        cam.look_at(grip.focus, Vec3::Y);
    }
}

pub fn update_active(trigger: Trigger<SwitchActive>, mut cam_q: Query<(Entity, &mut Camera)>) {
    for (ent, mut cam) in cam_q.iter_mut() {
        if trigger.0 == ent {
            cam.is_active = true;
        } else {
            cam.is_active = false;
        }
    }
}
