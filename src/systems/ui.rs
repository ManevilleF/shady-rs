use crate::resources::{Candidate, CreationCandidate, OperationSelection, TypeSelection};
use crate::{CurrentShader, UiState};
use bevy::prelude::*;
use bevy_egui::egui::Ui;
use bevy_egui::{egui, EguiContext};
use shady_generator::{
    FloatingNativeType, NativeFunction, NativeOperation, NativeType, NonScalarNativeType,
    ScalarNativeType,
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

pub fn menu(
    mut commands: Commands,
    egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut shader: ResMut<CurrentShader>,
) {
    egui::SidePanel::left("Menu")
        .default_width(200.)
        .min_width(150.)
        .max_width(300.)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Shady");
            ui.separator();

            ui.label("Shader name:");
            ui.text_edit_singleline(&mut shader.name);
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
            ui.label("Create Node:");
            if ui.button("Type Construction").clicked() {
                ui_state.candidate = Some(Candidate::TypeSelection(
                    TypeSelection::TypeConstruction(NonScalarNativeType::Vec2),
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
        });
    let mut close = false;
    let mut new_candidate = None;
    if let Some(candidate) = &mut ui_state.candidate {
        egui::SidePanel::left("Create element")
            .min_width(150.)
            .show(egui_ctx.ctx(), |ui| {
                match candidate {
                    Candidate::OperationSelection(selection) => {
                        let mut picked = false;
                        ui.label("Select an operation");
                        match selection {
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
                        }
                        if picked {
                            new_candidate = Some(Candidate::TypeSelection(
                                selection.type_selection_candidate(),
                            ));
                        }
                    }
                    Candidate::TypeSelection(intermediate_candidate) => {
                        let mut picked = false;
                        ui.label("Select a type");
                        match intermediate_candidate {
                            TypeSelection::InputProperty(t) | TypeSelection::OutputProperty(t) => {
                                type_selection(ui, NativeType::VARIANTS, t, &mut picked);
                            }
                            TypeSelection::TypeConstruction(t) => {
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
                                    type_selection(ui, ScalarNativeType::VARIANTS, t, &mut picked);
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
                        }
                        if picked {
                            new_candidate = Some(Candidate::Creation(
                                intermediate_candidate.creation_candidate(),
                            ));
                        }
                    }
                    Candidate::Creation(creation_candidate) => {
                        match creation_candidate {
                            CreationCandidate::Node { name, .. } => {
                                ui.horizontal(|ui| {
                                    ui.label("Name");
                                    ui.text_edit_singleline(name);
                                });
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
                        if ui.button("Create").clicked() {
                            commands.insert_resource(creation_candidate.clone());
                            close = true;
                        }
                    }
                }
                if ui.small_button("Cancel").clicked() {
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
