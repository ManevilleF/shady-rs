#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style
)]

mod common;
mod components;
mod events;
mod resources;
mod systems;

use crate::events::*;
use crate::resources::{CurrentShader, PreviewMaterial, SelectedEntities, UiState};
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
        .add_startup_system(systems::camera::setup_camera.system())
        .add_startup_system(systems::assets::setup_assets.system())
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
                .with_system(
                    systems::input::handle_element_dragging
                        .system()
                        .after("cursor"),
                )
                .with_system(systems::input::handle_camera_dragging.system()),
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
                    systems::ui::creation_menu::creation_menu
                        .system()
                        .after("ui_setup")
                        .label("ui_menu"),
                )
                .with_system(systems::ui::handle_log_elements.system().after("ui_menu")),
        )
        .add_system(systems::preview::handle_shader_event.system())
        .add_system_set(
            SystemSet::new()
                .with_system(systems::io::handle_io_events.system())
                .with_system(systems::io::handle_io_state.system())
                .with_system(systems::io::handle_io_task.system()),
        )
        .add_system(systems::camera::handle_camera_movement.system())
        .add_event::<ShaderEvent>()
        .add_event::<IOEvent>()
        .insert_resource(CurrentShader::default())
        .insert_resource(UiState::default())
        .insert_resource(PreviewMaterial::default());
    // Debug hierarchy inspector
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.run()
}
