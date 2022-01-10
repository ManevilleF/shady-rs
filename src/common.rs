#![allow(dead_code)]

use bevy::prelude::{Transform, Vec2};
use bevy_prototype_lyon::entity::ShapeBundle;
use std::env::current_dir;

#[derive(Debug, Clone)]
pub struct Bounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl Bounds {
    pub fn centered(center: Vec2, extents: Vec2) -> Self {
        Self {
            min: center - extents,
            max: center + extents,
        }
    }

    pub fn in_bounds(&self, pos: Vec2) -> bool {
        pos.x >= self.min.x && pos.x <= self.max.x && pos.y >= self.min.y && pos.y <= self.max.y
    }

    pub fn center(&self) -> Vec2 {
        self.min + (self.max - self.min) / 2.
    }
}

pub fn get_current_dir() -> String {
    current_dir()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn shape_bundle() -> ShapeBundle {
    ShapeBundle {
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Bounds;
    use bevy::math::Vec2;

    #[test]
    fn bounds() {
        let bounds = Bounds::centered(Vec2::new(1., 1.), Vec2::new(1., 1.));
        assert_eq!(bounds.min, Vec2::ZERO);
        assert_eq!(bounds.max, Vec2::new(2., 2.));
        assert_eq!(bounds.center(), Vec2::new(1., 1.));
        let bounds = Bounds::centered(Vec2::new(4., 2.), Vec2::new(1., 1.));
        assert_eq!(bounds.min, Vec2::new(3., 1.));
        assert_eq!(bounds.max, Vec2::new(5., 3.));
        assert_eq!(bounds.center(), Vec2::new(4., 2.));
    }
}
