#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::nursery,
    clippy::pedantic,
    nonstandard_style
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::default_trait_access,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::needless_pass_by_value,
    clippy::cast_precision_loss
)]

mod common;
mod components;
mod events;
mod resources;
mod systems;

use crate::events::{IOEvent, ShaderEvent};
use crate::resources::{CurrentShader, PreviewMaterial, SelectedEntities, UiState};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::DARK_GRAY))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Shady".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin)
        .insert_resource(SelectedEntities::default())
        .add_startup_system(systems::camera::setup_camera)
        .add_startup_system(systems::assets::setup_assets)
        // Mouse
        .add_system_set(
            SystemSet::new()
                .with_system(systems::input::handle_mouse_position.label("cursor"))
                .with_system(systems::input::handle_mouse_interaction.after("cursor"))
                .with_system(systems::input::handle_element_dragging.after("cursor"))
                .with_system(systems::input::handle_camera_dragging),
        )
        // Nodes
        .add_system(systems::shader::handle_shader_event)
        // Lines
        .add_system_set(
            SystemSet::new()
                .with_system(systems::lines::handle_connector_lines)
                .with_system(systems::lines::handle_candidate_line.after("cursor")),
        )
        // UI
        .add_startup_system(systems::ui::setup)
        .add_system_set(
            SystemSet::new()
                .with_system(systems::ui::menu.label("ui_setup"))
                .with_system(
                    systems::ui::creation_menu::creation_menu
                        .after("ui_setup")
                        .label("ui_menu"),
                )
                .with_system(systems::ui::log::handle_log_elements.after("ui_menu")),
        )
        .add_system(systems::preview::handle_shader_event)
        .add_system_set(
            SystemSet::new()
                .with_system(systems::io::handle_io_events)
                .with_system(systems::io::handle_io_state)
                .with_system(systems::io::handle_io_task),
        )
        .add_system(systems::camera::handle_camera_movement)
        .add_event::<ShaderEvent>()
        .add_event::<IOEvent>()
        .insert_resource(CurrentShader::default())
        .insert_resource(UiState::default())
        .insert_resource(PreviewMaterial::default());
    // Debug hierarchy inspector
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.run();
}
