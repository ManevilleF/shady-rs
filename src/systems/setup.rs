use crate::resources::{CameraTranslation, ShadyAssets};
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(CameraTranslation(Vec2::ZERO))
}

pub fn setup_assets(
    mut commands: Commands,
    mut assets: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(ShadyAssets::load(&mut assets, &asset_server));
}
