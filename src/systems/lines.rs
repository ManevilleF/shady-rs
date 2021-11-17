use crate::components::{InteractionBox, NodeConnector, NodeInput, NodeOutput};
use crate::get_cursor_position;
use crate::resources::{NodeConnectorCandidate, ShadyAssets, WorldCursorPosition};
use bevy::ecs::query::QueryEntityError;
use bevy::log;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

fn draw_bezier_line((start, end): (Vec2, Vec2), lines: &mut DebugLines, color: Color) {
    let start = Vec3::new(start.x, start.y, 1.);
    let end = Vec3::new(end.x, end.y, 1.);
    lines.line_colored(start, end, 0., color);
}

pub fn handle_candidate_line(
    mut commands: Commands,
    cursor_position: Option<Res<WorldCursorPosition>>,
    assets: Res<ShadyAssets>,
    connector_candidate: Option<Res<NodeConnectorCandidate>>,
    connector_query: Query<&GlobalTransform, With<NodeOutput>>,
    mut lines: ResMut<DebugLines>,
) {
    let candidate = match connector_candidate {
        None => return,
        Some(c) => c,
    };
    let position = get_cursor_position!(cursor_position);
    let start_pos = match connector_query.get(candidate.output_from) {
        Ok(transform) => transform.translation.xy(),
        Err(e) => {
            log::warn!(
                "Failed to retrieve connector candidate entity {:?}, deleting it: {}",
                candidate.output_from,
                e
            );
            commands.remove_resource::<NodeConnectorCandidate>();
            return;
        }
    };
    draw_bezier_line(
        (start_pos, position.0),
        &mut lines,
        assets.selected_connector_color,
    );
}

macro_rules! get_vec2_transform {
    ($res:expr, $entity:ident, $cmd:ident) => {
        match $res {
            Ok(t) => t.translation.xy(),
            Err(e) => {
                log::warn!(
                    "Failed to retrieve node connector entity: {}, deleting line.",
                    e
                );
                $cmd.entity($entity).despawn_recursive();
                continue;
            }
        }
    };
}

pub fn handle_connector_lines(
    mut commands: Commands,
    connector_query: Query<(Entity, &NodeConnector)>,
    connector_box_query: Query<&GlobalTransform, Or<(With<NodeInput>, With<NodeOutput>)>>,
    mut lines: ResMut<DebugLines>,
    assets: Res<ShadyAssets>,
) {
    for (entity, node_connector) in connector_query.iter() {
        let from = get_vec2_transform!(
            connector_box_query.get(node_connector.output_from),
            entity,
            commands
        );
        let to = get_vec2_transform!(
            connector_box_query.get(node_connector.input_to),
            entity,
            commands
        );
        draw_bezier_line((from, to), &mut lines, assets.connector_color);
    }
}
