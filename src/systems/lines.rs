use crate::components::{NodeConnector, ShadyInputSlot, ShadyOutputSlot};
use crate::get_cursor_position;
use crate::resources::{NodeConnectorCandidate, ShadyAssets, WorldCursorPosition};
use crate::systems::spawner::SLOT_STEP;
use bevy::log;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::Path as LinePath;
use bevy_prototype_lyon::prelude::{DrawMode, FillMode};
use lyon_path::Path;

fn draw_line(
    path: &mut LinePath,
    draw_mode: &mut DrawMode,
    (start, end): (Vec2, Vec2),
    color: Color,
) {
    let mut builder = Path::builder();
    builder.begin([start.x, start.y].into());
    builder.line_to([start.x + SLOT_STEP, start.y].into());
    builder.line_to([end.x, end.y].into());
    builder.line_to([end.x, -SLOT_STEP, end.y].into());
    builder.end(false);
    *path = LinePath(builder.build());
    *draw_mode = DrawMode::Fill(FillMode::color(color));
}

pub fn handle_candidate_line(
    mut commands: Commands,
    cursor_position: Option<Res<WorldCursorPosition>>,
    assets: Res<ShadyAssets>,
    connector_candidate: Option<Res<NodeConnectorCandidate>>,
    connector_query: Query<&GlobalTransform, With<ShadyOutputSlot>>,
    mut line_query: Query<(&mut LinePath, &mut DrawMode)>,
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
            commands.entity(candidate.line_entity).despawn_recursive();
            commands.remove_resource::<NodeConnectorCandidate>();
            return;
        }
    };
    let (mut path, mut mode) = line_query.get(candidate.line_entity);
    draw_line(
        &mut path,
        &mut mode,
        (start_pos, position.0),
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
    mut connector_query: Query<(Entity, &NodeConnector, &mut LinePath, &mut DrawMode)>,
    input_slot_query: Query<(&GlobalTransform, &ShadyInputSlot)>,
    output_slot_query: Query<(&GlobalTransform, &ShadyOutputSlot)>,
) {
    for (entity, node_connector, mut path, mut mode) in connector_query.iter_mut() {
        let (from, _from_color) = get_vec2_color!(
            output_slot_query.get(node_connector.output_from),
            entity,
            commands
        );
        let (to, to_color) = get_vec2_color!(
            input_slot_query.get(node_connector.input_to),
            entity,
            commands
        );
        draw_line(&mut path, &mut mode, (from, to), to_color);
    }
}
