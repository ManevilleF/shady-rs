use crate::components::ShadyNode;
use crate::events::NodeEvent;
use crate::resources::ShadyAssets;
use crate::CurrentShader;
use bevy::log;
use bevy::prelude::*;
use shady_generator::{ShaderOperation, ShadyError};

pub fn handle_node_spawn(
    mut commands: Commands,
    mut spawn_evr: EventReader<NodeEvent>,
    mut current_shader: ResMut<CurrentShader>,
    assets: Res<ShadyAssets>,
) {
    for event in spawn_evr.iter() {
        match event {
            NodeEvent::CreateNode {
                target_position,
                node_preset,
            } => {
                log::info!(
                    "Creating Node from `{}` preset at {}, {}",
                    node_preset.name(),
                    target_position.x,
                    target_position.y
                );
                let node = current_shader.create_node_from_preset(*node_preset);
                let id = node.unique_id().clone();
                let entity = ShadyNode::spawn(&mut commands, &assets, *target_position, &node);
                current_shader.node_entities.insert(id, entity);
            }
            NodeEvent::DeleteNode { id } => {
                log::info!("Deleting node {}", id);
                if current_shader.remove_node(&id).is_none() {
                    log::warn!("Shader did not have a Node with id {}", id);
                }
                current_shader.delete_node_entity(&id, &mut commands);
            }
        }
    }
}
