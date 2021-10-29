use crate::components::NodeConnector;
use bevy::prelude::Entity;

#[derive(Debug)]
pub struct NodeConnectorCandidate {
    pub output_from: Entity,
    pub line_entity: Entity,
}

impl NodeConnectorCandidate {
    pub fn to_connector(&self, input_to: Entity) -> NodeConnector {
        NodeConnector {
            output_from: self.output_from,
            input_to,
        }
    }
}
