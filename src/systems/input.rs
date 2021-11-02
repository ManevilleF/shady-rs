use crate::events::SpawnNode;
use crate::get_cursor_position;
use crate::resources::{ShadyAssets, WorldCursorPosition};
use bevy::prelude::*;

pub fn handle_mouse_position(mut commands: Commands, windows: Res<Windows>) {
    match WorldCursorPosition::world_cursor_position(&windows) {
        None => commands.remove_resource::<WorldCursorPosition>(),
        Some(p) => commands.insert_resource(p),
    }
}

pub fn handle_mouse_input(
    cursor_position: Option<Res<WorldCursorPosition>>,
    mut spawn_node_evw: EventWriter<SpawnNode>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    let position = get_cursor_position!(cursor_position);
    spawn_node_evw.send(SpawnNode {
        target_position: position.0,
    })
}
