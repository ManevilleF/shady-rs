use crate::common::Bounds;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum BoxInteraction {
    ConnectionStart,
    ConnectionEnd,
    DeleteNode(String),
    Drag,
    Ignore,
}

#[derive(Debug, Clone)]
pub struct InteractionBox {
    pub size: Vec2,
    pub interaction: BoxInteraction,
}

impl InteractionBox {
    pub fn new(size: Vec2, interaction: BoxInteraction) -> Self {
        Self { size, interaction }
    }

    // TODO: fix this
    pub fn get_interaction(&self, self_translation: Vec2, pos: Vec2) -> Option<BoxInteraction> {
        let bounds = Bounds::centered(self_translation, self.size);
        if bounds.in_bounds(pos) {
            Some(self.interaction.clone())
        } else {
            None
        }
    }
}
