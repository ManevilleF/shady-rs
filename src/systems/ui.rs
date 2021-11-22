use crate::resources::{CandidateSelection, CreationCandidate};
use crate::{CurrentShader, UiState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use shady_generator::{NativeType, NodePreset};

pub fn setup(egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx().set_visuals(egui::Visuals {
        window_corner_radius: 0.0,
        ..Default::default()
    });
}

pub fn menu(
    mut commands: Commands,
    egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut shader: ResMut<CurrentShader>,
) {
    egui::SidePanel::left("Menu")
        .default_width(150.)
        .min_width(100.)
        .max_width(300.)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Shady");
            ui.separator();

            ui.label("Shader name:");
            ui.text_edit_singleline(&mut shader.name);
            ui.separator();

            if ui.button("Create Input Property").clicked() {
                ui_state.candidate_selection =
                    Some(CandidateSelection::InputNativeType(NativeType::default()));
            }
            if ui.button("Create Output Property").clicked() {
                ui_state.candidate_selection =
                    Some(CandidateSelection::OutputNativeType(NativeType::default()));
            }

            ui.separator();

            ui.label("Node presets:");

            for preset in NodePreset::VARIANTS.iter() {
                if ui.button(preset.name()).clicked() {
                    commands.insert_resource(CreationCandidate::Node(*preset))
                }
            }
        });
    let mut close = false;
    let mut picked = false;
    if let Some(candidate) = &mut ui_state.candidate_selection {
        match candidate {
            CandidateSelection::InputNativeType(t) | CandidateSelection::OutputNativeType(t) => {
                egui::SidePanel::left("Type Selection").show(egui_ctx.ctx(), |ui| {
                    ui.label("Select a type");
                    for native_type in NativeType::VARIANTS {
                        if ui
                            .selectable_value(t, *native_type, native_type.to_string())
                            .clicked()
                        {
                            picked = true;
                        }
                    }
                    ui.separator();
                    if ui.small_button("Cancel").clicked() {
                        close = true;
                    }
                });
            }
        }
        if picked {
            ui_state.creation_candidate = Some(candidate.creation_candidate());
        }
    }
    if close || picked {
        ui_state.candidate_selection = None;
    }
    let mut close = false;
    if let Some(candidate) = &mut ui_state.creation_candidate {
        egui::Window::new("Create input Property")
            .collapsible(false)
            .show(egui_ctx.ctx(), |ui| {
                match candidate {
                    CreationCandidate::Node(_) => {}
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
                if ui.small_button("Cancel").clicked() {
                    close = true;
                }
                if ui.small_button("Create").clicked() {
                    close = true;
                    commands.insert_resource(candidate.clone());
                }
            });
    }
    if close {
        ui_state.creation_candidate = None;
    }
}
