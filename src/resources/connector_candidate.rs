use bevy::prelude::Entity;
use shady_generator::Connection;

#[derive(Debug)]
pub struct NodeConnectorCandidate {
    pub line_entity: Entity,
    pub output_from: Entity,
    pub connection: Connection,
}
