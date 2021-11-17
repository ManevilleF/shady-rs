use crate::components::{BoxInteraction, InteractionBox, NodeConnector};
use crate::events::SpawnNode;
use crate::resources::{DraggedEntities, NodeConnectorCandidate, ShadyAssets, WorldCursorPosition};
use crate::{get_cursor_position, get_or_continue, SelectedNodePreset};
use bevy::log;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::reflect::List;
use bevy::ui::node::NODE;
use shady_generator::NodePreset;

pub fn handle_mouse_position(mut commands: Commands, windows: Res<Windows>) {
    match WorldCursorPosition::world_cursor_position(&windows) {
        None => commands.remove_resource::<WorldCursorPosition>(),
        Some(p) => commands.insert_resource(p),
    }
}

fn get_interaction(
    box_query: &Query<(Entity, &GlobalTransform, &InteractionBox)>,
    position: Vec2,
) -> Option<(Entity, BoxInteraction)> {
    for (entity, transform, interaction_box) in box_query.iter() {
        if let Some(interaction) =
            interaction_box.get_interaction(transform.translation.xy(), position)
        {
            log::info!("Found interaction: {:?}", interaction);
            return Some((entity, interaction));
        }
    }
    None
}

pub fn handle_mouse_input(
    mut commands: Commands,
    cursor_position: Option<Res<WorldCursorPosition>>,
    connector_candidate: Option<Res<NodeConnectorCandidate>>,
    mut dragged_entities: Option<ResMut<DraggedEntities>>,
    mut spawn_node_evw: EventWriter<SpawnNode>,
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
    if mouse_input.just_pressed(MouseButton::Left) {
        match get_interaction(&box_query, position.0) {
            None => {
                if let Some(preset) = current_preset.0 {
                    spawn_node_evw.send(SpawnNode {
                        target_position: position.0,
                        node_preset: preset,
                    });
                    current_preset.0 = None;
                }
                commands.remove_resource::<NodeConnectorCandidate>();
            }
            Some((entity, interaction)) => match interaction {
                BoxInteraction::ConnectionStart => {
                    let candidate = NodeConnectorCandidate {
                        output_from: entity,
                    };
                    commands.insert_resource(candidate);
                }
                BoxInteraction::ConnectionEnd => {
                    if let Some(candidate) = connector_candidate {
                        if candidate.output_from != entity {
                            let connector = candidate.to_connector(entity);
                            commands.spawn().insert(connector);
                        }
                        commands.remove_resource::<NodeConnectorCandidate>();
                    }
                }
                BoxInteraction::Drag => commands.insert_resource(DraggedEntities {
                    entities: vec![entity],
                    previous_cursor_position: position.0,
                }),
                BoxInteraction::Ignore => {
                    commands.remove_resource::<NodeConnectorCandidate>();
                }
            },
        }
    }
}
