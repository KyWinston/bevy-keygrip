use bevy::prelude::*;
use bevy_keygrip::{components::{Grip, GripRig}, KeyGripPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(KeyGripPlugin)
        .add_systems(Startup, add_camera)
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        GripRig::default(),
        Grip("main".to_string()),
        Transform::from_translation(Vec3::new(0.0, 5.0, 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
