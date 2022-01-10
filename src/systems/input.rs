use crate::common::shape_bundle;
use crate::components::{BoxInteraction, InteractionBox};
use crate::events::ShaderEvent;
use crate::resources::{
    CameraDragging, CameraTranslation, CreationCandidate, DraggedEntities, NodeConnectorCandidate,
    WorldCursorPosition,
};
use crate::{get_cursor_position, get_or_continue};
use bevy::log;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use shady_generator::ConnectionAttempt;

pub fn handle_mouse_position(
    mut commands: Commands,
    windows: Res<Windows>,
    camera_translation: Res<CameraTranslation>,
) {
    match WorldCursorPosition::new(&windows, &camera_translation) {
        None => commands.remove_resource::<WorldCursorPosition>(),
        Some(p) => commands.insert_resource(p),
    }
}

fn get_interaction(
    box_query: &Query<(Entity, &GlobalTransform, &InteractionBox)>,
    position: Vec2,
) -> Option<(Entity, BoxInteraction)> {
    let mut interactions: Vec<(Entity, BoxInteraction)> = box_query
        .iter()
        .filter_map(|(entity, transform, interaction_box)| {
            interaction_box
                .get_interaction(transform.translation.xy(), position)
                .map(|interaction| (entity, interaction))
        })
        .collect();
    interactions.sort_by_key(|(_e, b)| b.clone());
    interactions.first().cloned()
}

pub fn handle_element_dragging(
    mut commands: Commands,
    cursor_position: Option<Res<WorldCursorPosition>>,
    dragged_entities: Option<ResMut<DraggedEntities>>,
    mouse_input: Res<Input<MouseButton>>,
    mut transform_query: Query<&mut Transform, With<InteractionBox>>,
) {
    let position = get_cursor_position!(cursor_position);
    // Dragging
    if let Some(mut dragged_entities) = dragged_entities {
        if !mouse_input.pressed(MouseButton::Left) {
            commands.remove_resource::<DraggedEntities>();
            return;
        }
        let delta_pos = position.0 - dragged_entities.previous_cursor_position;
        for dragged_entity in &dragged_entities.entities {
            let mut transform = get_or_continue!(transform_query.get_mut(*dragged_entity));
            transform.translation += Vec3::new(delta_pos.x, delta_pos.y, 0.);
        }
        dragged_entities.previous_cursor_position = position.0;
    }
}

pub fn handle_camera_dragging(
    mut commands: Commands,
    camera_dragging: Option<Res<CameraDragging>>,
    mut camera_translation: ResMut<CameraTranslation>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>, //
) {
    if !mouse_input.pressed(MouseButton::Right) {
        commands.remove_resource::<CameraDragging>();
        return;
    }
    let position = match WorldCursorPosition::new(&windows, &camera_translation) {
        None => return,
        Some(p) => p,
    };

    match camera_dragging {
        None => commands.insert_resource(CameraDragging {
            previous_cursor_position: position.0,
        }),
        Some(drag) => {
            let delta = position.0 - drag.previous_cursor_position;
            camera_translation.0 -= delta;
        }
    }
}

pub fn handle_mouse_interaction(
    mut commands: Commands,
    cursor_position: Option<Res<WorldCursorPosition>>,
    connector_candidate: Option<Res<NodeConnectorCandidate>>,
    mut node_evw: EventWriter<ShaderEvent>,
    mouse_input: Res<Input<MouseButton>>,
    box_query: Query<(Entity, &GlobalTransform, &InteractionBox)>,
    creation_candidate: Option<Res<CreationCandidate>>,
) {
    let position = get_cursor_position!(cursor_position);

    // Interaction
    if mouse_input.just_pressed(MouseButton::Left) {
        match get_interaction(&box_query, position.0) {
            None => {
                if let Some(candidate) = creation_candidate {
                    node_evw.send(ShaderEvent::CreateElement {
                        target_position: position.0,
                        candidate: candidate.clone(),
                    });
                    log::debug!("Creating {:?}", *candidate);
                    commands.remove_resource::<CreationCandidate>();
                }
                NodeConnectorCandidate::remove_candidate(
                    &mut commands,
                    connector_candidate.as_deref(),
                );
            }
            Some((entity, interaction)) => match interaction {
                BoxInteraction::ConnectionStart(connection) => {
                    let candidate = NodeConnectorCandidate {
                        line_entity: commands.spawn_bundle(shape_bundle()).id(),
                        output_from: entity,
                        connection,
                    };
                    commands.insert_resource(candidate);
                }
                BoxInteraction::ConnectionEnd(connection_to) => {
                    if let Some(candidate) = connector_candidate {
                        node_evw.send(ShaderEvent::Connect {
                            attempt: ConnectionAttempt {
                                connection_from: candidate.connection.clone(),
                                connection_to,
                            },
                            from: candidate.output_from,
                            to: entity,
                        });
                    } else {
                        // Disconnect
                        node_evw.send(ShaderEvent::Disconnect(connection_to));
                    }
                }
                BoxInteraction::Drag => commands.insert_resource(DraggedEntities {
                    entities: vec![entity],
                    previous_cursor_position: position.0,
                }),
                BoxInteraction::Ignore => {
                    NodeConnectorCandidate::remove_candidate(
                        &mut commands,
                        connector_candidate.as_deref(),
                    );
                }
                BoxInteraction::DeleteNode(id) => node_evw.send(ShaderEvent::DeleteNode { id }),
                BoxInteraction::DeleteOutput(id) => {
                    node_evw.send(ShaderEvent::DeleteOutputProperty { id });
                }
                BoxInteraction::DeleteInput(id) => {
                    node_evw.send(ShaderEvent::DeleteInputProperty { id });
                }
                BoxInteraction::DeleteConstant(id) => {
                    node_evw.send(ShaderEvent::DeleteConstant { id });
                }
            },
        }
    }
}
