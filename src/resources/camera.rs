use crate::Vec2;

#[derive(Debug)]
pub struct CameraDragging {
    pub previous_cursor_position: Vec2,
}

#[derive(Debug, Clone)]
pub struct CameraTranslation(pub Vec2);
