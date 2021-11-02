use crate::components::{BoxInteraction, InteractionBox};
use crate::events::SpawnNode;
use crate::resources::{DraggedEntities, ShadyAssets, WorldCursorPosition};
use crate::{get_cursor_position, get_or_continue};
use bevy::log;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::reflect::List;

pub fn handle_mouse_position(mut commands: Commands, windows: Res<Windows>) {
    match WorldCursorPosition::world_cursor_position(&windows) {
        None => commands.remove_resource::<WorldCursorPosition>(),
        Some(p) => commands.insert_resource(p),
    }
}

pub fn handle_mouse_input(
    mut commands: Commands,
    cursor_position: Option<Res<WorldCursorPosition>>,
    mut spawn_node_evw: EventWriter<SpawnNode>,
    mouse_input: Res<Input<MouseButton>>,
    box_query: Query<(Entity, &GlobalTransform, &InteractionBox)>,
    mut transform_query: Query<&mut Transform, With<InteractionBox>>,
    mut dragged_entities: Option<ResMut<DraggedEntities>>,
) {
    let position = get_cursor_position!(cursor_position);
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
        for (entity, transform, interaction_box) in box_query.iter() {
            if let Some(interaction) =
                interaction_box.get_interaction(transform.translation.xy(), position.0)
            {
                log::info!("Found interaction: {:?}", interaction);
                match interaction {
                    BoxInteraction::Connect => {}
                    BoxInteraction::Drag => commands.insert_resource(DraggedEntities {
                        entities: vec![entity],
                        previous_cursor_position: position.0,
                    }),
                    BoxInteraction::Ignore => (),
                }
                return;
            }
        }
        spawn_node_evw.send(SpawnNode {
            target_position: position.0,
        })
    }
}
