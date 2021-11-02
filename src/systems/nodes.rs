use crate::components::ShadyNode;
use crate::get_cursor_position;
use crate::resources::{ShadyAssets, WorldCursorPosition};
use bevy::prelude::*;

pub fn handle_node_spawn(
    mut commands: Commands,
    cursor_position: Option<Res<WorldCursorPosition>>,
    assets: Res<ShadyAssets>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    let position = get_cursor_position!(cursor_position);
    ShadyNode::spawn(&mut commands, &assets, position.0, "test");
}
