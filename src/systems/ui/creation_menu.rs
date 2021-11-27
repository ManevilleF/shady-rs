use crate::resources::{Candidate, CreationCandidate, OperationSelection, TypeSelection};
use crate::UiState;
use bevy::prelude::*;
use bevy_egui::egui::{Button, Rgba, Slider, Ui, Widget};
use bevy_egui::{egui, EguiContext};
use shady_generator::node_operation::*;
use shady_generator::{
    ConstantValue, FloatingNativeType, NativeType, NonScalarNativeType, NumericScalarNativeType,
};
use std::fmt::Display;

fn int_range(ui: &mut Ui, v: &mut i32, name: &str) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(Slider::new(v, i32::MIN..=i32::MAX));
    });
}

fn uint_range(ui: &mut Ui, v: &mut u32, name: &str) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(Slider::new(v, u32::MIN..=u32::MAX));
    });
}

fn f32_range(ui: &mut Ui, v: &mut f32, name: &str) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(Slider::new(v, f32::MIN..=f32::MAX));
    });
}

fn f64_range(ui: &mut Ui, v: &mut f64, name: &str) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(Slider::new(v, f64::MIN..=f64::MAX));
    });
}

fn constant_value_selection(ui: &mut Ui, constant: &mut ConstantValue) {
    match constant {
        ConstantValue::Bool(v) => {
            ui.checkbox(v, "Value");
        }
        ConstantValue::Int(v) => {
            int_range(ui, v, "Value");
        }
        ConstantValue::UInt(v) => {
            uint_range(ui, v, "Value");
        }
        ConstantValue::Float(v) => {
            f32_range(ui, v, "Value");
        }
        ConstantValue::Double(v) => {
            f64_range(ui, v, "Value");
        }
        ConstantValue::Vec2(v) => {
            for (i, value) in v.iter_mut().enumerate() {
                f32_range(ui, value, i.to_string().as_str());
            }
        }
        ConstantValue::IVec2(v) => {
            for (i, value) in v.iter_mut().enumerate() {
                int_range(ui, value, i.to_string().as_str());
            }
        }
        ConstantValue::Vec3(v) => {
            for (i, value) in v.iter_mut().enumerate() {
                f32_range(ui, value, i.to_string().as_str());
            }
        }
        ConstantValue::IVec3(v) => {
            for (i, value) in v.iter_mut().enumerate() {
                int_range(ui, value, i.to_string().as_str());
            }
        }
        ConstantValue::Vec4(v) => {
            for (i, value) in v.iter_mut().enumerate() {
                f32_range(ui, value, i.to_string().as_str());
            }
        }
        ConstantValue::IVec4(v) => {
            for (i, value) in v.iter_mut().enumerate() {
                int_range(ui, value, i.to_string().as_str());
            }
        }
    }
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
                                constant_value_selection(ui, &mut c.value);
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
