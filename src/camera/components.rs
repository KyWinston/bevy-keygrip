use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera(pub Entity);

#[derive(Component)]
pub struct CameraDriver;

#[cfg(feature = "third_person")]
#[derive(Component, Default)]
#[require(ThirdPersonCamera)]
pub struct ThirdPersonController;
