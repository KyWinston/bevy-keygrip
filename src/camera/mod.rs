use bevy::prelude::*;
use systems::update_camera;

pub mod components;
pub mod systems;
pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera);
    }
}
