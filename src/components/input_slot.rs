use bevy::prelude::{Color, Component, Entity};

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Clone, Component)]
pub struct ShadyInputSlot {
    pub connected_to: Option<Entity>,
    pub color: Color,
}

impl Default for ShadyInputSlot {
    fn default() -> Self {
        Self {
            connected_to: None,
            color: Default::default(),
        }
    }
}

impl ShadyInputSlot {
    pub const fn new(color: Color) -> Self {
        Self {
            connected_to: None,
            color,
        }
    }
}
