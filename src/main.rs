mod common;
mod components;
mod resources;
mod systems;

use crate::resources::SelectedEntities;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(DebugLinesPlugin)
        .insert_resource(SelectedEntities::default())
        .add_startup_system(systems::ui::setup.system())
        .add_startup_system(systems::setup::setup_camera.system())
        .add_startup_system(systems::setup::setup_assets.system())
        .add_system(
            systems::input::handle_mouse_position
                .system()
                .label("cursor"),
        )
        .add_system(systems::nodes::handle_node_spawn.system())
        .add_system_set(
            SystemSet::new()
                .with_system(systems::lines::handle_connector_lines.system())
                .with_system(
                    systems::lines::handle_candidate_line
                        .system()
                        .after("cursor"),
                ),
        )
        .add_system(systems::ui::menu.system())
        .run()
}
