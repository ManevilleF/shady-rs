use bevy::prelude::*;
use std::ops::Deref;

#[derive(Debug)]
pub struct DraggedEntities {
    pub entities: Vec<Entity>,
    pub previous_cursor_position: Vec2,
}

impl Deref for DraggedEntities {
    type Target = Vec<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.entities
    }
}
