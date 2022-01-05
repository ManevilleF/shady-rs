use bevy::prelude::Entity;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default)]
pub struct SelectedEntities(pub Vec<Entity>);

impl Deref for SelectedEntities {
    type Target = Vec<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SelectedEntities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
