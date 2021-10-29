use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn setup(egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx().set_visuals(egui::Visuals {
        window_corner_radius: 0.0,
        ..Default::default()
    });
}

pub fn menu(egui_ctx: ResMut<EguiContext>) {
    egui::SidePanel::left("Menu")
        .default_width(150.)
        .min_width(100.)
        .max_width(300.)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Title");
        });
}
