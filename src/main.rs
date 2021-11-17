mod common;
mod components;
mod events;
mod resources;
mod systems;

use crate::events::*;
use crate::resources::{CurrentShader, SelectedEntities, SelectedNodePreset};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(DebugLinesPlugin)
        .insert_resource(SelectedEntities::default())
        .add_startup_system(systems::setup::setup_camera.system())
        .add_startup_system(systems::setup::setup_assets.system())
        // Mouse
        .add_system_set(
            SystemSet::new()
                .with_system(
                    systems::input::handle_mouse_position
                        .system()
                        .label("cursor"),
                )
                .with_system(systems::input::handle_mouse_input.system().after("cursor")),
        )
        // Nodes
        .add_system(systems::nodes::handle_node_spawn.system())
        // Lines
        .add_system_set(
            SystemSet::new()
                .with_system(systems::lines::handle_connector_lines.system())
                .with_system(
                    systems::lines::handle_candidate_line
                        .system()
                        .after("cursor"),
                ),
        )
        // UI
        .add_startup_system(systems::ui::setup.system())
        .add_system_set(SystemSet::new().with_system(systems::ui::menu.system()))
        .add_event::<ShaderEvent>()
        .insert_resource(CurrentShader::default())
        .insert_resource(SelectedNodePreset::default())
        .run()
}
