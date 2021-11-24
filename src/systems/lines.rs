use crate::components::{NodeConnector, ShadyInputSlot, ShadyOutputSlot};
use crate::get_cursor_position;
use crate::resources::{NodeConnectorCandidate, ShadyAssets, WorldCursorPosition};
use crate::systems::spawner::SLOT_STEP;
use bevy::log;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

fn draw_straight_line((start, end): (Vec2, Vec2), lines: &mut DebugLines, color: Color) {
    let start = Vec3::new(start.x, start.y, 1.);
    let end = Vec3::new(end.x, end.y, 1.);
    lines.line_colored(start, end, 0., color);
}

fn draw_pretty_line(
    (start, end): (Vec2, Vec2),
    lines: &mut DebugLines,
    start_color: Color,
    end_color: Color,
) {
    let start = Vec3::new(start.x, start.y, 1.);
    let start_b = start + Vec3::new(SLOT_STEP, 0., 0.);
    let end = Vec3::new(end.x, end.y, 1.);
    let end_b = end + Vec3::new(-SLOT_STEP, 0., 0.);
    lines.line_colored(start, start_b, 0., start_color);
    lines.line_gradient(start_b, end_b, 0., start_color, end_color);
    lines.line_colored(end_b, end, 0., end_color);
}

pub fn handle_candidate_line(
    mut commands: Commands,
    cursor_position: Option<Res<WorldCursorPosition>>,
    assets: Res<ShadyAssets>,
    connector_candidate: Option<Res<NodeConnectorCandidate>>,
    connector_query: Query<&GlobalTransform, With<ShadyOutputSlot>>,
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
    draw_straight_line(
        (start_pos, position.0),
        &mut lines,
        assets.selected_connector_color,
    );
}

macro_rules! get_vec2_color {
    ($res:expr, $entity:ident, $cmd:ident) => {
        match $res {
            Ok((t, s)) => (t.translation.xy(), s.color),
            Err(e) => {
                log::warn!(
                    "Failed to retrieve node connector entity {:?}, deleting it : {}",
                    $entity,
                    e
                );
                $cmd.entity($entity).despawn_recursive();
                continue;
            }
        }
    };
}

#[allow(clippy::type_complexity)]
pub fn handle_connector_lines(
    mut commands: Commands,
    connector_query: Query<(Entity, &NodeConnector)>,
    input_slot_query: Query<(&GlobalTransform, &ShadyInputSlot)>,
    output_slot_query: Query<(&GlobalTransform, &ShadyOutputSlot)>,
    mut lines: ResMut<DebugLines>,
) {
    for (entity, node_connector) in connector_query.iter() {
        let (from, from_color) = get_vec2_color!(
            output_slot_query.get(node_connector.output_from),
            entity,
            commands
        );
        let (to, to_color) = get_vec2_color!(
            input_slot_query.get(node_connector.input_to),
            entity,
            commands
        );
        draw_pretty_line((from, to), &mut lines, from_color, to_color);
    }
}
