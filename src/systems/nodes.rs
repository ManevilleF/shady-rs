use crate::components::{NodeConnector, ShadyInputSlot, ShadyNode, ShadyOutputSlot};
use crate::events::ShaderEvent;
use crate::resources::{NodeConnectorCandidate, ShadyAssets};
use crate::CurrentShader;
use bevy::log;
use bevy::prelude::*;

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
            ShaderEvent::CreateInputProperty {
                target_position,
                property,
            } => {
                log::info!(
                    "Creating Input Node {} at {}, {}",
                    property.name,
                    target_position.x,
                    target_position.y
                );
                let property = current_shader.add_input_property(property.clone());
                let id = property.reference.clone();
                let entity =
                    ShadyInputSlot::spawn(&mut commands, &assets, *target_position, property);
                current_shader.input_property_entities.insert(id, entity);
            }
            ShaderEvent::CreateOutputProperty {
                target_position,
                property,
            } => {
                log::info!(
                    "Creating Output Node {} at {}, {}",
                    property.name,
                    target_position.x,
                    target_position.y
                );
                let property = current_shader.add_output_property(property.clone());
                let id = property.reference.clone();
                let entity =
                    ShadyOutputSlot::spawn(&mut commands, &assets, *target_position, property);
                current_shader.output_property_entities.insert(id, entity);
            }
            ShaderEvent::DeleteNode { id } => {
                log::info!("Deleting node {}", id);
                if current_shader.remove_node(&id).is_none() {
                    log::error!("Shader did not have a Node with id {}", id);
                }
                current_shader.delete_node_entity(id, &mut commands);
            }
            ShaderEvent::DeleteInputProperty { id } => {
                log::info!("Deleting input property {}", id);
                if current_shader.remove_input_property(&id).is_none() {
                    log::error!("Shader did not have an input with id {}", id);
                }
                current_shader.delete_input_property_entity(&id, &mut commands);
            }
            ShaderEvent::DeleteOutputProperty { id } => {
                log::info!("Deleting output property {}", id);
                if current_shader.remove_output_property(&id).is_none() {
                    log::error!("Shader did not have an output property with id {}", id);
                }
                current_shader.delete_output_property_entity(&id, &mut commands);
            }
            ShaderEvent::Connect { from, to, attempt } => {
                match current_shader.connect(attempt.clone()) {
                    Ok(c) => {
                        if let Some(c) = c {
                            let id = CurrentShader::unique_connector_id(&attempt.connection_to, &c);
                            log::info!("Detected connection reset, removing {:?} ({})", c, id);
                            match current_shader.connection_entities.get(&id) {
                                Some(entity) => commands.entity(*entity).despawn_recursive(),
                                None => log::error!("Failed to remove connection {:?}", c),
                            }
                        }
                        let id = CurrentShader::unique_connector_id(
                            &attempt.connection_to,
                            &attempt.connection_from,
                        );
                        let connector_id = commands
                            .spawn()
                            .insert(NodeConnector {
                                output_from: *from,
                                input_to: *to,
                            })
                            .insert(Name::new(format!("{} connector", id)))
                            .id();
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
