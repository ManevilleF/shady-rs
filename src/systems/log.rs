use crate::components::{LogElement, LogLevel};
use bevy::prelude::*;
use bevy_egui::egui::{Color32, Frame, Label};
use bevy_egui::{egui, EguiContext};

pub fn handle_log_elements(
    egui_ctx: ResMut<EguiContext>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut LogElement)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    egui::SidePanel::right("Logger")
        .min_width(500.)
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
