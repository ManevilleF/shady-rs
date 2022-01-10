use crate::components::{LogElement, LogLevel};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, Frame, RichText},
    EguiContext,
};

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
                    let mut text = RichText::new(&log.message);
                    match log.log_level {
                        LogLevel::Info => text = text.small().color(Color32::GREEN),
                        LogLevel::Warn | LogLevel::Error => {
                            text = text.strong().color(Color32::RED);
                        }
                    };
                    ui.label(text);
                    log.alive_time -= delta_time;
                    if log.alive_time <= 0.0 {
                        commands.entity(entity).despawn();
                    }
                }
            });
        });
}
