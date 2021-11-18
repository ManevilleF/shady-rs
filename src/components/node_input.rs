use bevy::prelude::Entity;

#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug)]
pub struct NodeInput {
    pub connected_to: Option<Entity>,
}

impl Default for NodeInput {
    fn default() -> Self {
        Self { connected_to: None }
    }
}
