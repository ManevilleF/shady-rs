use bevy::prelude::{Entity, Vec2};
use shady_generator::{ConnectionAttempt, InputProperty, NodePreset, OutputProperty};

#[derive(Debug, Clone)]
pub enum ShaderEvent {
    CreateNode {
        target_position: Vec2,
        node_preset: NodePreset,
    },
    CreateInputProperty {
        target_position: Vec2,
        property: InputProperty,
    },
    CreateOutputProperty {
        target_position: Vec2,
        property: OutputProperty,
    },
    DeleteNode {
        id: String,
    },
    DeleteInputProperty {
        id: String,
    },
    DeleteOutputProperty {
        id: String,
    },
    Connect {
        attempt: ConnectionAttempt,
        from: Entity,
        to: Entity,
    },
}
