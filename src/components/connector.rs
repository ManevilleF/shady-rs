use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;
use bevy_prototype_lyon::entity::ShapeBundle;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Component)]
pub struct NodeConnector {
    pub output_from: Entity,
    pub input_to: Entity,
}

impl NodeConnector {
    pub fn spawn(
        commands: &mut Commands,
        output_from: Entity,
        input_to: Entity,
        id: &str,
    ) -> Entity {
        commands
            .spawn_bundle(ShapeBundle {
                transform: Transform::from_xyz(0., 0., 1.),
                ..Default::default()
            })
            .insert(Self {
                output_from,
                input_to,
            })
            .insert(Name::new(format!("{} Connector", id)))
            .id()
    }
}
