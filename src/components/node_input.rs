use bevy::prelude::Entity;

#[derive(Debug)]
pub struct NodeInput {
    pub connected_to: Option<Entity>,
    pub field_name: String,
}

impl NodeInput {
    pub fn in_use(&self) -> bool {
        self.connected_to.is_some()
    }
}
