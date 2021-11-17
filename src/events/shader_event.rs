use bevy::prelude::{Entity, Vec2};
use shady_generator::{ConnectionAttempt, NodePreset};

#[derive(Debug, Clone)]
pub enum ShaderEvent {
    CreateNode {
        target_position: Vec2,
        node_preset: NodePreset,
    },
    DeleteNode {
        id: String,
    },
    Connect {
        attempt: ConnectionAttempt,
        from: Entity,
        to: Entity,
    },
}
