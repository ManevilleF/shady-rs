use crate::resources::ShadyAssets;
use bevy::prelude::*;

pub fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ShadyAssets::load(&asset_server));
}
