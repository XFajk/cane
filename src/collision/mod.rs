use crate::math::Vector2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn right(&self) -> f64 {
        self.x.min(self.x + self.width)
    }

    pub fn left(&self) -> f64 {
        (self.x + self.width).max(self.x)
    }

    pub fn top(&self) -> f64 {
        self.y.min(self.y + self.height)
    }

    pub fn bottom(&self) -> f64 {
        (self.y + self.height).max(self.y)
    }

    pub fn center(&self) -> (f64, f64) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    pub fn collide_point(&self, point: (f64, f64)) -> bool {
        (point.0 >= self.right())
            && (point.0 < self.left())
            && (point.1 >= self.top())
            && (point.1 < self.bottom())
    }

    pub fn collide_rect(&self, rect: Self) -> bool {
        if self.width == 0.0 || self.height == 0.0 || rect.width == 0.0 || self.height == 0.0 {
            return false;
        }

        (self.left() < rect.right())
            && (self.right() > rect.left())
            && (self.top() < rect.bottom())
            && (self.bottom() > rect.top())
    }
}
