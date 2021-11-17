use bevy::prelude::Entity;

#[derive(Debug)]
pub struct NodeInput {
    pub connected_to: Option<Entity>,
}

impl Default for NodeInput {
    fn default() -> Self {
        Self { connected_to: None }
    }
}
