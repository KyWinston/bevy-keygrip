use crate::components::Grip;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Grip)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct CameraDriver;

#[cfg(feature = "third_person")]
#[derive(Component, Default)]
#[require(ThirdPersonCamera)]
pub struct ThirdPersonController;
