use bevy::prelude::Entity;
use shady_generator::Connection;

#[derive(Debug)]
pub struct NodeConnectorCandidate {
    pub output_from: Entity,
    pub connection: Connection,
}
