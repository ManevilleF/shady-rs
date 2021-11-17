use crate::{CurrentShader, SelectedNodePreset};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use shady_generator::NodePreset;

pub fn setup(egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx().set_visuals(egui::Visuals {
        window_corner_radius: 0.0,
        ..Default::default()
    });
}

pub fn menu(
    egui_ctx: ResMut<EguiContext>,
    mut selected_preset: ResMut<SelectedNodePreset>,
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

            ui.label("Node presets:");
            for preset in NodePreset::VARIANTS.iter() {
                if ui.button(preset.name()).clicked() {
                    selected_preset.0 = Some(*preset);
                }
            }
        });
}
