use crate::common::get_current_dir;
use crate::components::{LogElement, LogLevel};
use crate::resources::{Candidate, CreationCandidate, IOState, OperationSelection, TypeSelection};
use crate::{CurrentShader, IOEvent, UiState, VERSION};
use bevy::prelude::*;
use bevy_egui::egui::{Button, Color32, ComboBox, Frame, Label, Rgba, Ui, Widget};
use bevy_egui::{egui, EguiContext};
use shady_generator::node_operation::*;
use shady_generator::{
    ConstantValue, FloatingNativeType, GraphicLibrary, NativeType, NonScalarNativeType,
    NumericScalarNativeType, ShaderType,
};
use std::fmt::Display;

pub fn setup(egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx().set_visuals(egui::Visuals {
        window_corner_radius: 0.0,
        ..Default::default()
    });
}

fn type_selection<T: Copy + Display + PartialEq>(
    ui: &mut Ui,
    variants: &[T],
    value: &mut T,
    picked: &mut bool,
) {
    for native_type in variants {
        if ui
            .selectable_value(value, *native_type, native_type.to_string())
            .clicked()
        {
            *picked = true;
        }
    }
}

fn swizzle_selection<T: FieldToGlsl, const SIZE: usize>(
    ui: &mut Ui,
    variants: &[[T; SIZE]],
    value: &mut [T; SIZE],
    picked: &mut bool,
) {
    ui.horizontal_wrapped(|ui| {
        for native_type in variants {
            if ui
                .selectable_value(
                    value,
                    *native_type,
                    native_type
                        .iter()
                        .map(FieldToGlsl::to_glsl)
                        .collect::<Vec<&str>>()
                        .join(""),
                )
                .clicked()
            {
                *picked = true;
            }
        }
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
            ui.label("Create Property:");
            if ui.button("Input Property").clicked() {
                ui_state.candidate = Some(Candidate::TypeSelection(TypeSelection::InputProperty(
                    NativeType::default(),
                )));
            }
            if ui.button("Output Property").clicked() {
                ui_state.candidate = Some(Candidate::TypeSelection(TypeSelection::OutputProperty(
                    NativeType::default(),
                )));
            }
            ui.separator();
            if ui.button("Create Constant").clicked() {
                ui_state.candidate = Some(Candidate::TypeSelection(TypeSelection::Constant(
                    ConstantValue::default(),
                )))
            }
            ui.separator();
            ui.label("Create Node:");
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
                    OperationSelection::NativeOperation(NativeOperation::Inc(NativeType::Float)),
                ));
            }
            if ui.button("Native function").clicked() {
                ui_state.candidate = Some(Candidate::OperationSelection(
                    OperationSelection::NativeFunction(NativeFunction::Absolute(
                        FloatingNativeType::Float,
                    )),
                ));
            }

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

pub fn creation_menu(
    mut commands: Commands,
    egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
) {
    let mut close = false;
    let mut new_candidate = None;
    if let Some(candidate) = &mut ui_state.candidate {
        egui::SidePanel::left("Create element")
            .min_width(150.)
            .show(egui_ctx.ctx(), |ui| {
                match candidate {
                    Candidate::OperationSelection(selection) => {
                        let mut picked = false;
                        ui.heading("Select an operation");
                        ui.separator();
                        ui.horizontal_wrapped(|ui| match selection {
                            OperationSelection::NativeOperation(operation) => {
                                for available_operation in NativeOperation::VARIANTS {
                                    if ui.button(available_operation.descriptive_name()).clicked() {
                                        *operation = available_operation.clone();
                                        picked = true;
                                    }
                                }
                            }
                            OperationSelection::NativeFunction(function) => {
                                for available_function in NativeFunction::VARIANTS {
                                    if ui.button(available_function.descriptive_name()).clicked() {
                                        *function = available_function.clone();
                                        picked = true;
                                    }
                                }
                            }
                            OperationSelection::TypeSwizzle(swizzle) => {
                                for available_function in NonScalarSwizzle::VARIANTS {
                                    if ui.button(available_function.descriptive_name()).clicked() {
                                        *swizzle = available_function.clone();
                                        picked = true;
                                    }
                                }
                            }
                        });
                        if picked {
                            new_candidate = Some(Candidate::TypeSelection(
                                selection.type_selection_candidate(),
                            ));
                        }
                    }
                    Candidate::TypeSelection(intermediate_candidate) => {
                        let mut picked = false;
                        ui.heading("Select a type");
                        ui.separator();
                        match intermediate_candidate {
                            TypeSelection::Constant(c) => {
                                type_selection(ui, ConstantValue::VARIANTS, c, &mut picked);
                            }
                            TypeSelection::InputProperty(t) | TypeSelection::OutputProperty(t) => {
                                type_selection(ui, NativeType::VARIANTS, t, &mut picked);
                            }
                            TypeSelection::TypeConstruction(t) | TypeSelection::TypeSplit(t) => {
                                type_selection(ui, NonScalarNativeType::VARIANTS, t, &mut picked);
                            }
                            TypeSelection::NativeOperation(o) => match o {
                                NativeOperation::Inc(t)
                                | NativeOperation::Dec(t)
                                | NativeOperation::Minus(t)
                                | NativeOperation::Add(t)
                                | NativeOperation::Sub(t)
                                | NativeOperation::Mul(t)
                                | NativeOperation::Div(t)
                                | NativeOperation::Selection(t)
                                | NativeOperation::Equals(t) => {
                                    type_selection(ui, NativeType::VARIANTS, t, &mut picked);
                                }
                                NativeOperation::GreaterThan(t)
                                | NativeOperation::GreaterThanEqual(t) => {
                                    type_selection(
                                        ui,
                                        NumericScalarNativeType::VARIANTS,
                                        t,
                                        &mut picked,
                                    );
                                }
                                _ => {
                                    picked = true;
                                }
                            },
                            TypeSelection::NativeFunction(f) => match f {
                                NativeFunction::Radians(t)
                                | NativeFunction::Degrees(t)
                                | NativeFunction::Sine(t)
                                | NativeFunction::Cosine(t)
                                | NativeFunction::Tangent(t)
                                | NativeFunction::ArcSine(t)
                                | NativeFunction::ArcCosine(t)
                                | NativeFunction::ArcTangent(t)
                                | NativeFunction::ArcTangent2(t)
                                | NativeFunction::Power(t)
                                | NativeFunction::Exponential(t)
                                | NativeFunction::Exponential2(t)
                                | NativeFunction::Logarithm(t)
                                | NativeFunction::Logarithm2(t)
                                | NativeFunction::SquareRoot(t)
                                | NativeFunction::InverseSquareRoot(t)
                                | NativeFunction::Absolute(t)
                                | NativeFunction::Sign(t)
                                | NativeFunction::Floor(t)
                                | NativeFunction::Ceiling(t)
                                | NativeFunction::FractionalPart(t)
                                | NativeFunction::Modulo(t)
                                | NativeFunction::FloatModulo(t)
                                | NativeFunction::Minimum(t)
                                | NativeFunction::FloatMinimum(t)
                                | NativeFunction::Maximum(t)
                                | NativeFunction::FloatMaximum(t)
                                | NativeFunction::Clamp(t)
                                | NativeFunction::FloatClamp(t)
                                | NativeFunction::Mix(t)
                                | NativeFunction::FloatMix(t)
                                | NativeFunction::Step(t)
                                | NativeFunction::FloatStep(t)
                                | NativeFunction::SmoothStep(t)
                                | NativeFunction::FloatSmoothStep(t)
                                | NativeFunction::Distance(t)
                                | NativeFunction::Length(t)
                                | NativeFunction::DotProduct(t)
                                | NativeFunction::Normalize(t)
                                | NativeFunction::FaceForward(t)
                                | NativeFunction::Reflect(t)
                                | NativeFunction::Refract(t) => {
                                    type_selection(
                                        ui,
                                        FloatingNativeType::VARIANTS,
                                        t,
                                        &mut picked,
                                    );
                                }
                                _ => {
                                    picked = true;
                                }
                            },
                            TypeSelection::TypeSwizzle(swizzle) => {
                                match swizzle {
                                    NonScalarSwizzle::Vec2ToVec2(v) => {
                                        swizzle_selection(
                                            ui,
                                            &Vec2Field::every_vec2_possibility(),
                                            v,
                                            &mut picked,
                                        );
                                    }
                                    NonScalarSwizzle::Vec2ToVec3(v) => {
                                        swizzle_selection(
                                            ui,
                                            &Vec2Field::every_vec3_possibility(),
                                            v,
                                            &mut picked,
                                        );
                                    }
                                    NonScalarSwizzle::Vec2ToVec4(v) => {
                                        swizzle_selection(
                                            ui,
                                            &Vec2Field::every_vec4_possibility(),
                                            v,
                                            &mut picked,
                                        );
                                    }
                                    NonScalarSwizzle::Vec3ToVec2(v) => {
                                        swizzle_selection(
                                            ui,
                                            &Vec3Field::every_vec2_possibility(),
                                            v,
                                            &mut picked,
                                        );
                                    }
                                    NonScalarSwizzle::Vec3ToVec3(v) => {
                                        swizzle_selection(
                                            ui,
                                            &Vec3Field::every_vec3_possibility(),
                                            v,
                                            &mut picked,
                                        );
                                    }
                                    NonScalarSwizzle::Vec3ToVec4(v) => {
                                        swizzle_selection(
                                            ui,
                                            &Vec3Field::every_vec4_possibility(),
                                            v,
                                            &mut picked,
                                        );
                                    }
                                    NonScalarSwizzle::Vec4ToVec2(v) => {
                                        swizzle_selection(
                                            ui,
                                            &Vec4Field::every_vec2_possibility(),
                                            v,
                                            &mut picked,
                                        );
                                    }
                                    NonScalarSwizzle::Vec4ToVec3(v) => {
                                        swizzle_selection(
                                            ui,
                                            &Vec4Field::every_vec3_possibility(),
                                            v,
                                            &mut picked,
                                        );
                                    }
                                    NonScalarSwizzle::Vec4ToVec4(v) => {
                                        swizzle_selection(
                                            ui,
                                            &Vec4Field::every_vec4_possibility(),
                                            v,
                                            &mut picked,
                                        );
                                    }
                                };
                            }
                        }
                        if picked {
                            new_candidate = Some(Candidate::Creation(
                                intermediate_candidate.creation_candidate(),
                            ));
                        }
                    }
                    Candidate::Creation(creation_candidate) => {
                        ui.heading("Options");
                        match creation_candidate {
                            CreationCandidate::Node { name, .. } => {
                                ui.horizontal(|ui| {
                                    ui.label("Name");
                                    ui.text_edit_singleline(name);
                                });
                            }
                            CreationCandidate::Constant(c) => {
                                ui.horizontal(|ui| {
                                    ui.label("Name");
                                    ui.text_edit_singleline(&mut c.name);
                                });
                                // TODO: Check value
                            }
                            CreationCandidate::InputProperty(p) => {
                                ui.horizontal(|ui| {
                                    ui.label("Name");
                                    ui.text_edit_singleline(&mut p.name);
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Reference");
                                    ui.text_edit_singleline(&mut p.reference);
                                });
                                ui.checkbox(&mut p.uniform, "Uniform");
                            }
                            CreationCandidate::OutputProperty(p) => {
                                ui.horizontal(|ui| {
                                    ui.label("Name");
                                    ui.text_edit_singleline(&mut p.name);
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Reference");
                                    ui.text_edit_singleline(&mut p.reference);
                                });
                            }
                        }
                        ui.separator();
                        if Button::new("Create")
                            .fill(Rgba::from_rgb(0.2, 0.6, 0.2))
                            .text_color(Rgba::from_rgb(1., 1., 1.).into())
                            .ui(ui)
                            .clicked()
                        {
                            commands.insert_resource(creation_candidate.clone());
                            close = true;
                        }
                    }
                }
                ui.add_space(10.);
                if ui.button("Cancel").clicked() {
                    close = true;
                }
            });
    }
    if close {
        ui_state.candidate = None;
    } else if let Some(c) = new_candidate {
        ui_state.candidate = Some(c);
    }
}
