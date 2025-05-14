use bevy::prelude::*;

use crate::events::SwitchActive;

pub fn update_active(trigger: Trigger<SwitchActive>, mut cam_q: Query<(Entity, &mut Camera)>) {
    for (ent, mut cam) in cam_q.iter_mut() {
        if trigger.0 == ent {
            cam.is_active = true;
        } else {
            cam.is_active = false;
        }
    }
}
