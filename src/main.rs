mod common;
mod components;
mod events;
mod resources;
mod systems;

use crate::events::*;
use crate::resources::{CurrentShader, SelectedEntities, UiState};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init();
    let mut app = App::build();
    app.insert_resource(ClearColor(Color::DARK_GRAY))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Shady".to_string(),
            ..Default::default()
        })
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
                .with_system(
                    systems::input::handle_mouse_interaction
                        .system()
                        .after("cursor"),
                )
                .with_system(systems::input::handle_dragging.system().after("cursor")),
        )
        // Nodes
        .add_system(systems::shader::handle_shader_event.system())
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
        .add_system_set(
            SystemSet::new()
                .with_system(systems::ui::menu.system().label("ui_setup"))
                .with_system(
                    systems::ui::creation_menu
                        .system()
                        .after("ui_setup")
                        .label("ui_menu"),
                )
                .with_system(systems::ui::io.system().after("ui_menu").label("ui_io"))
                .with_system(systems::ui::handle_log_elements.system().after("ui_io")),
        )
        .add_system(systems::io::handle_io_events.system())
        .add_event::<ShaderEvent>()
        .add_event::<IOEvent>()
        .insert_resource(CurrentShader::default())
        .insert_resource(UiState::default());
    // Debug hierarchy inspector
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.run()
}
