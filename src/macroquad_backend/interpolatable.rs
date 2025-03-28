use crate::animation::Interpolatable;
use macroquad::prelude::*;

impl Interpolatable for Rect {
    fn lerp(&self, end: &Self, t: f32) -> Self {
        Rect::new(
            self.x.lerp(end.x, t),
            self.y.lerp(end.y, t),
            self.w.lerp(end.w, t),
            self.h.lerp(end.h, t)
        )
    }
}

impl Interpolatable for Vec2 {
    fn lerp(&self, end: &Self, t: f32) -> Self {
        Vec2::lerp(*self, *end, t)
    }
}

impl Interpolatable for Vec3 {
    fn lerp(&self, end: &Self, t: f32) -> Self {
        Vec3::lerp(*self, *end, t)
    }
}

