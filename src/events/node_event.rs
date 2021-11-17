use bevy::prelude::Vec2;
use shady_generator::NodePreset;

#[derive(Debug, Clone)]
pub enum NodeEvent {
    CreateNode {
        target_position: Vec2,
        node_preset: NodePreset,
    },
    DeleteNode {
        id: String,
    },
}
