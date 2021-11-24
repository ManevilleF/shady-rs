use crate::resources::CameraTranslation;
use bevy::prelude::{Vec2, Windows};
use std::ops::Deref;

#[derive(Debug, Copy, Clone)]
pub struct WorldCursorPosition(pub Vec2);

impl WorldCursorPosition {
    pub fn new(windows: &Windows, camera_translation: &CameraTranslation) -> Option<Self> {
        let window = windows.get_primary()?;
        let pos = window.cursor_position()?;
        let mouse_position = Vec2::new(pos.x - window.width() / 2., pos.y - window.height() / 2.)
            + camera_translation.0;
        Some(Self(mouse_position))
    }
}

impl Deref for WorldCursorPosition {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
