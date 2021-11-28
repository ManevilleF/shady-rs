pub mod creation_menu;
mod live_edit;

use crate::common::get_current_dir;
use crate::components::{LogElement, LogLevel};
use crate::resources::{Candidate, IOState, OperationSelection, TypeSelection};
use crate::systems::ui::live_edit::constant_value_selection;
use crate::{CurrentShader, IOEvent, UiState, VERSION};
use bevy::prelude::*;
use bevy_egui::egui::{Color32, ComboBox, Frame, Label, Rgba, Widget};
use bevy_egui::{egui, EguiContext};
use shady_generator::node_operation::*;
use shady_generator::{
    ConstantValue, FloatingNativeType, GraphicLibrary, NativeType, NonScalarNativeType, ShaderType,
};

pub fn setup(egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx().set_visuals(egui::Visuals {
        window_corner_radius: 0.0,
        ..Default::default()
    });
}

pub fn menu(
    egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut shader: ResMut<CurrentShader>,
) {
    egui::SidePanel::left("Menu")
        .default_width(200.)
        .min_width(150.)
        .max_width(300.)
        .show(egui_ctx.ctx(), |ui| {
            ui.label(
                Label::new("Shady-rs")
                    .text_color(Rgba::from_rgb(1., 1., 1.))
                    .strong()
                    .heading(),
            );
            ui.separator();

            ui.label("Shader name:");
            ui.text_edit_singleline(&mut shader.name);

            ComboBox::from_label("Type")
                .selected_text(shader.shader_type.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut shader.shader_type, ShaderType::Vertex, "Vertex");
                    ui.selectable_value(&mut shader.shader_type, ShaderType::Fragment, "Fragment");
                });

            ComboBox::from_label("Target Lib")
                .selected_text(shader.library.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut shader.library, GraphicLibrary::OpenGl, "OpenGl");
                    ui.selectable_value(&mut shader.library, GraphicLibrary::OpenGlEs, "OpenGlEs");
                    ui.selectable_value(&mut shader.library, GraphicLibrary::WebGPU, "WebGPU");
                });

            ui.separator();
            ui.label("Properties");
            ui.vertical_centered_justified(|ui| {
                if ui.button("Create Input Property").clicked() {
                    ui_state.candidate = Some(Candidate::TypeSelection(
                        TypeSelection::InputProperty(NativeType::default()),
                    ));
                }
                if ui.button("Create Output Property").clicked() {
                    ui_state.candidate = Some(Candidate::TypeSelection(
                        TypeSelection::OutputProperty(NativeType::default()),
                    ));
                }
            });
            ui.separator();
            ui.label("Constants");
            ui.vertical_centered_justified(|ui| {
                if ui.button("Create Constant").clicked() {
                    ui_state.candidate = Some(Candidate::TypeSelection(TypeSelection::Constant(
                        ConstantValue::default(),
                    )))
                }
                ui.collapsing("Constants", |ui| {
                    for (key, constant) in shader.constants_mut() {
                        ui.collapsing(key, |ui| {
                            constant_value_selection(ui, &mut constant.value);
                        });
                    }
                });
            });

            ui.separator();
            ui.label("Node Operations");
            ui.vertical_centered_justified(|ui| {
                if ui.button("Type Construction").clicked() {
                    ui_state.candidate = Some(Candidate::TypeSelection(
                        TypeSelection::TypeConstruction(NonScalarNativeType::Vec2),
                    ));
                }
                if ui.button("Type Split").clicked() {
                    ui_state.candidate = Some(Candidate::TypeSelection(TypeSelection::TypeSplit(
                        NonScalarNativeType::Vec2,
                    )));
                }
                if ui.button("Type Swizzle").clicked() {
                    ui_state.candidate = Some(Candidate::OperationSelection(
                        OperationSelection::TypeSwizzle(NonScalarSwizzle::default()),
                    ));
                }
                if ui.button("Native Operation").clicked() {
                    ui_state.candidate = Some(Candidate::OperationSelection(
                        OperationSelection::NativeOperation(NativeOperation::Inc(
                            NativeType::Float,
                        )),
                    ));
                }
                if ui.button("Native function").clicked() {
                    ui_state.candidate = Some(Candidate::OperationSelection(
                        OperationSelection::NativeFunction(NativeFunction::Absolute(
                            FloatingNativeType::Float,
                        )),
                    ));
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Hyperlink::new("https://github.com/ManevilleF/shady-rs")
                        .text("Shady-rs by ManevilleF"),
                );
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        ui_state.io_state = Some(IOState::Saving(get_current_dir()))
                    }
                    if ui.button("Load").clicked() {
                        ui_state.io_state = Some(IOState::Loading(get_current_dir()))
                    }
                    if ui.button("Export").clicked() {
                        ui_state.io_state = Some(IOState::Exporting(get_current_dir()))
                    }
                });
                ui.label("I/O");
            });
        });
    egui::TopBottomPanel::bottom("Build info")
        .frame(Frame::none())
        .resizable(false)
        .show(egui_ctx.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                let label = Label::new(format!(
                    "App version {} - Lib version {}",
                    VERSION,
                    shady_generator::VERSION
                ))
                .small();
                ui.label(label);
            });
        });
}

pub fn handle_log_elements(
    egui_ctx: ResMut<EguiContext>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut LogElement)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    egui::SidePanel::right("Logger")
        .min_width(200.)
        .frame(Frame::none())
        .resizable(false)
        .show(egui_ctx.ctx(), |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Max), |ui| {
                for (entity, mut log) in query.iter_mut() {
                    let mut label = Label::new(&log.message).small();
                    match log.log_level {
                        LogLevel::Info => label = label.text_color(Color32::GREEN),
                        LogLevel::Warn => label = label.strong().text_color(Color32::RED),
                        LogLevel::Error => label = label.strong().text_color(Color32::RED),
                    };
                    ui.label(label);
                    log.alive_time -= delta_time;
                    if log.alive_time <= 0.0 {
                        commands.entity(entity).despawn();
                    }
                }
            });
        });
}

pub fn io(
    egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut io_ewr: EventWriter<IOEvent>,
) {
    let mut open = true;
    let mut done = false;
    if let Some(state) = &mut ui_state.io_state {
        egui::Window::new(state.title())
            .default_size((500., 200.))
            .open(&mut open)
            .show(egui_ctx.ctx(), |ui| {
                ui.label(state.message());
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Directory");
                    egui::TextEdit::singleline(state.path_mut())
                        .desired_width(500.)
                        .ui(ui)
                });
                if ui.button(state.title()).clicked() {
                    io_ewr.send(state.event());
                    done = true;
                }
            });
    }
    if !open || done {
        ui_state.io_state = None;
    }
}
