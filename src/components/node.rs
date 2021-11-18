use crate::resources::ShadyAssets;
use crate::systems::spawner::{spawn_element, SpawnType};
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;
use shady_generator::Node;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug)]
pub struct ShadyNode;

impl ShadyNode {
    pub fn spawn(commands: &mut Commands, assets: &ShadyAssets, pos: Vec2, node: &Node) -> Entity {
        spawn_element(
            commands,
            assets,
            pos,
            (node.unique_id(), node.name()),
            SpawnType::Node {
                input_fields: node.input_field_types(),
                output_fields: node.output_field_types(),
            },
        )
    }
}
