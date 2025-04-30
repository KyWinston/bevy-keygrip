use bevy::prelude::*;
use systems::{connect_player_cam, update_camera_position, update_camera_rotation};

pub mod components;
pub mod systems;
pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (update_camera_position, update_camera_rotation)
                .before(TransformSystem::TransformPropagate),
        )
        .add_observer(connect_player_cam);
    }
}
