use bevy::log;
use bevy::prelude::{Commands, Component, Entity};
#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug, Clone, Component)]
pub struct LogElement {
    pub log_level: LogLevel,
    pub message: String,
    pub alive_time: f32,
}

impl LogElement {
    pub const fn new(log_level: LogLevel, message: String) -> Self {
        Self {
            log_level,
            message,
            alive_time: 10.0,
        }
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        match &self.log_level {
            LogLevel::Info => log::info!("{}", self.message),
            LogLevel::Warn => log::warn!("{}", self.message),
            LogLevel::Error => log::error!("{}", self.message),
        }
        commands.spawn().insert(self).id()
    }
}
