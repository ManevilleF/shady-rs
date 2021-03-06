use bevy::prelude::{Color, Component};
#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Clone, Default, Component)]
pub struct ShadyOutputSlot {
    pub color: Color,
}

impl ShadyOutputSlot {
    pub const fn new(color: Color) -> Self {
        Self { color }
    }
}
