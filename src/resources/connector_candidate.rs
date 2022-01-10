use crate::Commands;
use bevy::prelude::Entity;
use shady_generator::Connection;

#[derive(Debug)]
pub struct NodeConnectorCandidate {
    pub line_entity: Entity,
    pub output_from: Entity,
    pub connection: Connection,
}

impl NodeConnectorCandidate {
    pub fn remove_candidate(commands: &mut Commands, res: Option<&Self>) {
        if let Some(c) = res {
            commands.entity(c.line_entity).despawn();
        }
        commands.remove_resource::<Self>();
    }
}
