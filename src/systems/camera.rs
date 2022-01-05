use crate::resources::CameraTranslation;
use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(CameraTranslation(Vec2::ZERO));
}

pub fn handle_camera_movement(
    mut camera_query: Query<&mut Transform, With<OrthographicProjection>>,
    camera_translation: Res<CameraTranslation>,
) {
    let mut transform = camera_query.single_mut().unwrap();
    transform.translation.x = camera_translation.0.x;
    transform.translation.y = camera_translation.0.y;
}
