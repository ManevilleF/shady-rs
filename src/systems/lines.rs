use crate::components::{ConnectorBox, NodeConnector, NodeInput};
use crate::resources::{NodeConnectorCandidate, ShadyAssets, WorldCursorPosition};
use bevy::ecs::query::QueryEntityError;
use bevy::log;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

fn draw_bezier_line((start, end): (Vec2, Vec2), lines: &mut DebugLines, color: Color) {
    let start = Vec3::new(start.x, start.y, 1.);
    let end = Vec3::new(end.x, end.y, 1.);
    lines.line_colored(start, end, 0., color);
}

pub fn handle_candidate_line(
    cursor_position: Option<Res<WorldCursorPosition>>,
    connector_candidate: Option<Res<NodeConnectorCandidate>>,
    connector_query: Query<&ConnectorBox, With<NodeInput>>,
    assets: Res<ShadyAssets>,
    mut lines: ResMut<DebugLines>,
) {
    let connector_candidate = match connector_candidate {
        None => return,
        Some(c) => c,
    };
    let position = match cursor_position {
        None => return,
        Some(p) => p,
    };
    let start_pos = match connector_query.get(connector_candidate.output_from) {
        Ok(connector) => connector.center(),
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

macro_rules! get_box {
    ($res:expr) => {
        match $res {
            Ok(c) => c.center(),
            Err(e) => {
                log::error!("Failed to retrieve node connector entity: {}", e);
                continue;
            }
        }
    };
}

pub fn handle_connector_lines(
    connector_query: Query<&NodeConnector>,
    connector_box_query: Query<&ConnectorBox>,
    mut lines: ResMut<DebugLines>,
    assets: Res<ShadyAssets>,
) {
    for node_connector in connector_query.iter() {
        let from = get_box!(connector_box_query.get(node_connector.output_from));
        let to = get_box!(connector_box_query.get(node_connector.output_from));
        draw_bezier_line((from, to), &mut lines, assets.connector_color);
    }
}
