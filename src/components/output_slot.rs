use bevy::prelude::Color;
#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Clone)]
pub struct ShadyOutputSlot {
    pub color: Color,
}

impl Default for ShadyOutputSlot {
    fn default() -> Self {
        Self {
            color: Default::default(),
        }
    }
}

impl ShadyOutputSlot {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
