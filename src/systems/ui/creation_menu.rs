use crate::resources::{Candidate, CreationCandidate, OperationSelection, TypeSelection};
use crate::UiState;
use bevy::prelude::*;
use bevy_egui::egui::{Button, Color32, Response, Rgba, RichText, Ui, Widget};
use bevy_egui::{egui, EguiContext};
use shady_generator::node_operation::{
    FieldToGlsl, NativeFunction, NativeOperation, NonScalarSwizzle,
};
use shady_generator::{
    ConstantValue, FloatingNativeType, NativeType, NonScalarNativeType, NumericScalarNativeType,
};
use std::fmt::Display;

fn create_button(ui: &mut Ui) -> Response {
    Button::new(RichText::new("Create").color(Color32::WHITE))
        .fill(Rgba::from_rgb(0.2, 0.6, 0.2))
        .ui(ui)
}

fn type_selection<T: Copy + Display + PartialEq>(
    ui: &mut Ui,
    variants: &[T],
    value: &mut T,
    picked: &mut bool,
) {
    for variant in variants {
        if ui
            .selectable_value(value, *variant, variant.to_string())
            .clicked()
        {
            *picked = true;
        }
    }
}

fn swizzle_selection<T: FieldToGlsl, const SIZE: usize>(
    ui: &mut Ui,
    value: &mut [T; SIZE],
    picked: &mut bool,
) {
    ui.vertical_centered_justified(|ui| {
        for (i, v) in value.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(i.to_string());
                for variant in T::all_variants() {
                    ui.selectable_value(v, variant, variant.to_glsl());
                }
            });
        }
        if create_button(ui).clicked() {
            *picked = true;
        }
    });
}

#[allow(clippy::too_many_lines)]
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
                                        swizzle_selection(ui, v, &mut picked);
                                    }
                                    NonScalarSwizzle::Vec2ToVec3(v) => {
                                        swizzle_selection(ui, v, &mut picked);
                                    }
                                    NonScalarSwizzle::Vec2ToVec4(v) => {
                                        swizzle_selection(ui, v, &mut picked);
                                    }
                                    NonScalarSwizzle::Vec3ToVec2(v) => {
                                        swizzle_selection(ui, v, &mut picked);
                                    }
                                    NonScalarSwizzle::Vec3ToVec3(v) => {
                                        swizzle_selection(ui, v, &mut picked);
                                    }
                                    NonScalarSwizzle::Vec3ToVec4(v) => {
                                        swizzle_selection(ui, v, &mut picked);
                                    }
                                    NonScalarSwizzle::Vec4ToVec2(v) => {
                                        swizzle_selection(ui, v, &mut picked);
                                    }
                                    NonScalarSwizzle::Vec4ToVec3(v) => {
                                        swizzle_selection(ui, v, &mut picked);
                                    }
                                    NonScalarSwizzle::Vec4ToVec4(v) => {
                                        swizzle_selection(ui, v, &mut picked);
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
                        if create_button(ui).clicked() {
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
