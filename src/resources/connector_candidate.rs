use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use shady_generator::Connection;

#[derive(Debug)]
pub struct NodeConnectorCandidate {
    pub line_entity: Entity,
    pub output_from: Entity,
    pub connection: Connection,
}

impl NodeConnectorCandidate {
    pub fn spawn(commands: &mut Commands, output_from: Entity, connection: Connection) -> Self {
        Self {
            line_entity: commands
                .spawn_bundle(ShapeBundle {
                    transform: Transform::from_xyz(0., 0., 1.),
                    ..Default::default()
                })
                .id(),
            output_from,
            connection,
        }
    }

    pub fn remove_candidate(commands: &mut Commands, res: Option<&Self>) {
        if let Some(c) = res {
            commands.entity(c.line_entity).despawn();
            commands.remove_resource::<Self>();
        }
    }
}
