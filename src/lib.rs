use bevy::prelude::*;
use events::SwitchActive;
use systems::{focus_cameras, update_active};

pub mod components;
pub mod events;
pub mod systems;
pub struct KeyGripPlugin;

impl Plugin for KeyGripPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_event::<SwitchActive>()
            .add_systems(Update, focus_cameras)
            .add_observer(update_active);
    }
}
