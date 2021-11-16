use bevy::prelude::Vec2;
use shady_generator::NodePreset;

#[derive(Debug, Clone)]
pub struct SpawnNode {
    pub target_position: Vec2,
    pub node_preset: NodePreset,
}
