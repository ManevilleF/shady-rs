use bevy::prelude::Vec2;

#[derive(Debug, Clone)]
pub struct Bounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl Bounds {
    pub fn new(min: Vec2, size: Vec2) -> Self {
        Self {
            min,
            max: min + size,
        }
    }

    pub fn in_bounds(&self, pos: Vec2) -> bool {
        pos.x >= self.min.x && pos.x <= self.max.x && pos.y >= self.min.y && pos.y <= self.max.y
    }

    pub fn center(&self) -> Vec2 {
        self.min + (self.max - self.min) / 2.
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Bounds;
    use bevy::math::Vec2;

    #[test]
    fn bounds_center() {
        let bounds = Bounds::new(Vec2::ZERO, Vec2::new(2., 2.));
        assert_eq!(bounds.min, Vec2::ZERO);
        assert_eq!(bounds.max, Vec2::new(2., 2.));
        assert_eq!(bounds.center(), Vec2::new(1., 1.));
        let bounds = Bounds::new(Vec2::new(3., 1.), Vec2::new(2., 2.));
        assert_eq!(bounds.min, Vec2::new(3., 1.));
        assert_eq!(bounds.max, Vec2::new(5., 3.));
        assert_eq!(bounds.center(), Vec2::new(4., 2.));
    }
}
