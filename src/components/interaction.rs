use crate::common::Bounds;
use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum BoxInteraction {
    Drag,
    ConnectionStart,
    ConnectionEnd,
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

    pub fn new_drag_box(size: Vec2) -> Self {
        Self {
            size,
            interaction: BoxInteraction::Drag,
        }
    }

    pub fn new_connect_box(size: Vec2) -> Self {
        Self {
            size,
            interaction: BoxInteraction::ConnectionStart,
        }
    }

    pub fn get_interaction(&self, self_translation: Vec2, pos: Vec2) -> Option<BoxInteraction> {
        let bounds = Bounds::centered(self_translation, self.size);
        if bounds.in_bounds(pos) {
            Some(self.interaction)
        } else {
            None
        }
    }
}
