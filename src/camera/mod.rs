use bevy::prelude::*;
use systems::{add_player_camera, update_camera_target};

pub mod systems;
pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera_target)
            .add_observer(add_player_camera);
    }
}
