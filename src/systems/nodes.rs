use crate::components::{NodeConnector, ShadyNode};
use crate::events::ShaderEvent;
use crate::resources::{NodeConnectorCandidate, ShadyAssets};
use crate::CurrentShader;
use bevy::log;
use bevy::prelude::*;
use shady_generator::{Connection, ConnectionTo};

pub fn handle_node_spawn(
    mut commands: Commands,
    mut spawn_evr: EventReader<ShaderEvent>,
    mut current_shader: ResMut<CurrentShader>,
    assets: Res<ShadyAssets>,
) {
    for event in spawn_evr.iter() {
        match event {
            ShaderEvent::CreateNode {
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
                let entity = ShadyNode::spawn(&mut commands, &assets, *target_position, node);
                current_shader.node_entities.insert(id, entity);
            }
            ShaderEvent::DeleteNode { id } => {
                log::info!("Deleting node {}", id);
                if current_shader.remove_node(id).is_none() {
                    log::warn!("Shader did not have a Node with id {}", id);
                }
                current_shader.delete_node_entity(id, &mut commands);
            }
            ShaderEvent::Connect { from, to, attempt } => {
                match current_shader.connect(attempt.clone()) {
                    Ok(c) => {
                        if let Some(c) = c {
                            let id = unique_id(&attempt.connection_to, &c);
                            log::info!("Detected connection reset, removing {:?} ({})", c, id);
                            match current_shader.connection_entities.get(&id) {
                                Some(entity) => commands.entity(*entity).despawn_recursive(),
                                None => log::error!("Failed to remove connection {:?}", c),
                            }
                        }
                        let connector_id = commands
                            .spawn()
                            .insert(NodeConnector {
                                output_from: *from,
                                input_to: *to,
                            })
                            .id();
                        let id = unique_id(&attempt.connection_to, &attempt.connection_from);
                        current_shader.connection_entities.insert(id, connector_id);
                        commands.remove_resource::<NodeConnectorCandidate>();
                    }
                    Err(e) => {
                        // TODO: add UI logger
                        log::error!("Failed apply connection: `{}`", e);
                    }
                };
            }
        }
    }
}

fn unique_id(to: &ConnectionTo, from: &Connection) -> String {
    format!(
        "{}_{}",
        match from {
            Connection::PropertyConnection { property_id } => property_id.clone(),
            Connection::NodeConnection {
                node_id,
                field_name,
            } => format!("{}::{}", node_id, field_name),
        },
        match to {
            ConnectionTo::ToNode { id, field } => format!("{}::{}", id, field),
            ConnectionTo::OutputProperty { id } => id.clone(),
        }
    )
}
