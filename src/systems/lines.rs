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
    cursor_position: Option<Res<WorldCursorPosition>>,
    assets: Res<ShadyAssets>,
    connector_candidate: Option<Res<NodeConnectorCandidate>>,
    connector_query: Query<&GlobalTransform, With<NodeOutput>>,
    mut lines: ResMut<DebugLines>,
) {
    let connector_candidate = match connector_candidate {
        None => return,
        Some(c) => c,
    };
    let position = get_cursor_position!(cursor_position);
    let start_pos = match connector_query.get(connector_candidate.output_from) {
        Ok(transform) => transform.translation.xy(),
        Err(e) => {
            log::error!(
                "Failed to retrieve connector candidate entity {:?}: {}",
                connector_candidate.output_from,
                e
            );
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
    ($res:expr) => {
        match $res {
            Ok(t) => t.translation.xy(),
            Err(e) => {
                log::error!("Failed to retrieve node connector entity: {}", e);
                continue;
            }
        }
    };
}

pub fn handle_connector_lines(
    connector_query: Query<&NodeConnector>,
    connector_box_query: Query<&GlobalTransform>, // , Or<(NodeInput, NodeOutput)>>,
    mut lines: ResMut<DebugLines>,
    assets: Res<ShadyAssets>,
) {
    for node_connector in connector_query.iter() {
        let from = get_vec2_transform!(connector_box_query.get(node_connector.output_from));
        let to = get_vec2_transform!(connector_box_query.get(node_connector.input_to));
        draw_bezier_line((from, to), &mut lines, assets.connector_color);
    }
}
