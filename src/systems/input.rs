use crate::resources::WorldCursorPosition;
use bevy::prelude::*;

pub fn handle_mouse_position(mut commands: Commands, windows: Res<Windows>) {
    match WorldCursorPosition::world_cursor_position(&windows) {
        None => commands.remove_resource::<WorldCursorPosition>(),
        Some(p) => commands.insert_resource(p),
    }
}
