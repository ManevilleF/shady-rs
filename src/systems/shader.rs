use crate::components::{LogElement, LogLevel, NodeConnector};
use crate::events::ShaderEvent;
use crate::resources::{CreationCandidate, NodeConnectorCandidate, ShadyAssets};
use crate::systems::spawner::{spawn_element, SpawnType};
use crate::CurrentShader;
use bevy::log;
use bevy::prelude::*;
use shady_generator::{Connection, ConnectionTo, Node};

pub fn handle_shader_event(
    mut commands: Commands,
    mut spawn_evr: EventReader<ShaderEvent>,
    mut current_shader: ResMut<CurrentShader>,
    assets: Res<ShadyAssets>,
) {
    for event in spawn_evr.iter() {
        match event {
            ShaderEvent::CreateElement {
                target_position,
                candidate,
            } => match candidate {
                CreationCandidate::Node { name, operation } => {
                    let node = current_shader.create_node(Node::new(name, operation.clone()));
                    let id = node.unique_id().clone();
                    let response = spawn_element(
                        &mut commands,
                        &assets,
                        *target_position,
                        (node.unique_id(), node.name()),
                        SpawnType::Node {
                            input_fields: node.input_fields().into_iter().map(Into::into).collect(),
                            output_fields: node.output_fields(),
                        },
                    );
                    current_shader.node_entities.insert(id, response.entity);
                }
                CreationCandidate::InputProperty(property) => {
                    let property = current_shader.add_input_property(property.clone());
                    let id = property.reference.clone();
                    let response = spawn_element(
                        &mut commands,
                        &assets,
                        *target_position,
                        (&property.reference, &property.name),
                        SpawnType::InputProperty {
                            output_fields: vec![(property.reference.clone(), property.native_type)],
                        },
                    );
                    current_shader
                        .input_property_entities
                        .insert(id, response.entity);
                }
                CreationCandidate::OutputProperty(property) => {
                    let property = current_shader.add_output_property(property.clone());
                    let id = property.reference.clone();
                    let response = spawn_element(
                        &mut commands,
                        &assets,
                        *target_position,
                        (&property.reference, &property.name),
                        SpawnType::OutputProperty {
                            input_fields: vec![(
                                property.reference.clone(),
                                property.native_type,
                                false,
                            )
                                .into()],
                        },
                    );
                    current_shader
                        .output_property_entities
                        .insert(id, response.entity);
                }
            },
            ShaderEvent::DeleteNode { id } => {
                LogElement::new(LogLevel::Info, format!("Deleting node {}", id))
                    .spawn(&mut commands);
                if current_shader.remove_node(id).is_none() {
                    LogElement::new(
                        LogLevel::Error,
                        format!("Shader did not have a Node with id {}", id),
                    )
                    .spawn(&mut commands);
                }
                current_shader.delete_node_entity(id, &mut commands);
            }
            ShaderEvent::DeleteInputProperty { id } => {
                LogElement::new(LogLevel::Info, format!("Deleting input property {}", id))
                    .spawn(&mut commands);
                if current_shader.remove_input_property(id).is_none() {
                    LogElement::new(
                        LogLevel::Error,
                        format!("Shader did not have an input with id {}", id),
                    )
                    .spawn(&mut commands);
                }
                current_shader.delete_input_property_entity(id, &mut commands);
            }
            ShaderEvent::DeleteOutputProperty { id } => {
                LogElement::new(LogLevel::Info, format!("Deleting output property {}", id))
                    .spawn(&mut commands);
                if current_shader.remove_output_property(id).is_none() {
                    // TODO: add UI logger
                    LogElement::new(
                        LogLevel::Error,
                        format!("Shader did not have an output property with id {}", id),
                    )
                    .spawn(&mut commands);
                }
                current_shader.delete_output_property_entity(id, &mut commands);
            }
            ShaderEvent::Connect { from, to, attempt } => {
                match current_shader.connect(attempt.clone()) {
                    Ok(c) => {
                        if let Some(c) = c {
                            let id = CurrentShader::unique_connector_id(&attempt.connection_to, &c);
                            log::info!("Detected connection reset, removing {:?} ({})", c, id);
                            match current_shader.connection_entities.get(&id) {
                                Some(entity) => commands.entity(*entity).despawn_recursive(),
                                None => {
                                    LogElement::new(
                                        LogLevel::Error,
                                        format!("Failed to remove connection {:?}", c),
                                    )
                                    .spawn(&mut commands);
                                }
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
                        LogElement::new(
                            LogLevel::Error,
                            format!("Failed apply connection: `{}`", e),
                        )
                        .spawn(&mut commands);
                    }
                };
            }
            ShaderEvent::Disconnect(connection_to) => {
                match current_shader.disconnect(connection_to.clone()) {
                    Ok(Some(connection_from)) => {
                        current_shader.delete_connection_entity(
                            connection_to,
                            &connection_from,
                            &mut commands,
                        );
                        LogElement::new(
                            LogLevel::Info,
                            format!(
                                "Removed connnection from {} to {}",
                                match connection_from {
                                    Connection::ComplexOutputNode { id, field_name } =>
                                        format!("Node {} field {}", id, field_name),
                                    Connection::SingleOutputNode { id } => format!("Node {}", id),
                                    Connection::InputProperty { id } =>
                                        format!("Input Property {}", id),
                                    Connection::Constant { id } => format!("Constant {}", id),
                                },
                                match &connection_to {
                                    ConnectionTo::Node { id, field_name } =>
                                        format!("Node {} field {}", id, field_name),
                                    ConnectionTo::OutputProperty { id } =>
                                        format!("Output Property {}", id),
                                }
                            ),
                        )
                        .spawn(&mut commands);
                    }
                    Err(e) => {
                        LogElement::new(LogLevel::Error, format!("Failed to disconnect: `{}`", e))
                            .spawn(&mut commands);
                    }
                    _ => {}
                }
            }
        }
    }
}
