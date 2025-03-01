use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Camera)]
pub struct PlayerCamera {
   pub  offset: Vec2,
   pub  tracking: f32,
}

#[derive(Component)]
pub struct KeyGrip;

#[derive(Component, Default)]
pub struct Grip(pub String);

#[derive(Component, Default)]
#[require(Grip, Camera, Transform)]
pub struct GripRig {
    pub path: Vec<Vec3>,
    pub focus: Vec3,
}

impl GripRig {
    pub fn slide_cam_on_track(self, focus: Option<Vec3>, timing: Timer) {
        if focus.is_some() {}
    }
}
