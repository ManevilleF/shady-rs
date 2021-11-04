use bevy::prelude::Entity;

#[derive(Debug)]
pub struct NodeInput {
    pub connected_to: Option<Entity>,
}

impl NodeInput {
    pub fn in_use(&self) -> bool {
        self.connected_to.is_some()
    }
}

impl Default for NodeInput {
    fn default() -> Self {
        Self { connected_to: None }
    }
}
