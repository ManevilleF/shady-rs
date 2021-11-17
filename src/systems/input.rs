use crate::components::{BoxInteraction, InteractionBox};
use crate::events::ShaderEvent;
use crate::resources::{DraggedEntities, NodeConnectorCandidate, WorldCursorPosition};
use crate::{get_cursor_position, get_or_continue, SelectedNodePreset};
use bevy::log;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use shady_generator::ConnectionAttempt;

pub fn handle_mouse_position(mut commands: Commands, windows: Res<Windows>) {
    match WorldCursorPosition::new(&windows) {
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
            if let Some(interaction) =
                interaction_box.get_interaction(transform.translation.xy(), position)
            {
                log::info!("Found interaction: {:?}", interaction);
                Some((entity, interaction))
            } else {
                None
            }
        })
        .collect();
    interactions.sort_by_key(|(_e, b)| b.clone());
    interactions.first().cloned()
}

#[allow(clippy::too_many_arguments)]
pub fn handle_mouse_input(
    mut commands: Commands,
    cursor_position: Option<Res<WorldCursorPosition>>,
    connector_candidate: Option<Res<NodeConnectorCandidate>>,
    dragged_entities: Option<ResMut<DraggedEntities>>,
    mut node_evw: EventWriter<ShaderEvent>,
    mouse_input: Res<Input<MouseButton>>,
    box_query: Query<(Entity, &GlobalTransform, &InteractionBox)>,
    mut current_preset: ResMut<SelectedNodePreset>,
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
        return;
    }
    // Interaction
    if mouse_input.just_pressed(MouseButton::Left) {
        match get_interaction(&box_query, position.0) {
            None => {
                if let Some(preset) = current_preset.0 {
                    node_evw.send(ShaderEvent::CreateNode {
                        target_position: position.0,
                        node_preset: preset,
                    });
                    current_preset.0 = None;
                }
                commands.remove_resource::<NodeConnectorCandidate>();
            }
            Some((entity, interaction)) => match interaction {
                BoxInteraction::ConnectionStart(connection) => {
                    let candidate = NodeConnectorCandidate {
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
                    }
                }
                BoxInteraction::Drag => commands.insert_resource(DraggedEntities {
                    entities: vec![entity],
                    previous_cursor_position: position.0,
                }),
                BoxInteraction::Ignore => {
                    commands.remove_resource::<NodeConnectorCandidate>();
                }
                BoxInteraction::DeleteNode(id) => node_evw.send(ShaderEvent::DeleteNode { id }),
            },
        }
    }
}
