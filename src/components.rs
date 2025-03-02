use bevy::{math::VectorSpace, prelude::*};

#[derive(Component)]
pub struct KeyGrip;

#[derive(Default, Reflect)]
pub enum GripType {
    #[default]
    ThirdPerson,
    Racing,
    Fps,
}

#[derive(Component, Reflect)]
pub struct Grip {
    pub grip_type: GripType,
    pub location_offset: Vec3,
    pub rotation_offset: Vec3,
    pub tracking: (f32, f32),
}

impl Default for Grip {
    fn default() -> Self {
        Self {
            grip_type: GripType::default(),
            location_offset: Vec3::ZERO,
            rotation_offset: Vec3::ZERO,
            tracking: (1.0, 1.0),
        }
    }
}

impl Curve<Vec2> for Grip {
    fn domain(&self) -> Interval {
        match self.grip_type {
            GripType::Racing => Interval::EVERYWHERE,
            _ => Interval::EVERYWHERE,
        }
    }
    fn sample_unchecked(&self, t: f32) -> Vec2 {
        match self.grip_type {
            GripType::Racing => Vec2::new(
                t.cos() * self.location_offset.x,
                t.sin() * self.location_offset.z,
            ),
            _ => Vec2::new(
                t.cos() * self.location_offset.x,
                t.sin() * self.location_offset.z,
            ),
        }
    }
}
impl Grip {
    pub fn set_cam_to_t(self, focus: Option<Vec3>, _t: f32) {
        if focus.is_some() {}
    }
}
