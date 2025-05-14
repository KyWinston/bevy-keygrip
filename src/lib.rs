use bevy::prelude::*;
use camera::FollowCameraPlugin;
use components::Grip;
use events::SwitchActive;
use systems::update_active;

pub mod camera;
pub mod components;
pub mod events;
pub mod systems;
pub struct KeyGripPlugin;

impl Plugin for KeyGripPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FollowCameraPlugin)
            .register_type::<Grip>()
            .add_event::<SwitchActive>()
            .add_observer(update_active);
    }
}
