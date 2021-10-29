use bevy::prelude::*;

#[derive(Debug)]
pub struct ShadyAssets {
    pub font: Handle<Font>,
    pub node_title_material: Handle<ColorMaterial>,
    pub node_body_material: Handle<ColorMaterial>,
    pub connector_color: Color,
    pub selected_connector_color: Color,
    pub input_slot_material: Handle<ColorMaterial>,
    pub output_slot_material: Handle<ColorMaterial>,
}

impl ShadyAssets {
    pub fn load(assets: &mut Assets<ColorMaterial>, asset_server: &AssetServer) -> Self {
        Self {
            font: asset_server.load("fonts/AvenirNext-Regular.ttf"),
            node_title_material: assets.add(Color::CYAN.into()),
            node_body_material: assets.add(Color::GRAY.into()),
            connector_color: Color::WHITE,
            selected_connector_color: Color::GOLD,
            input_slot_material: assets.add(Color::LIME_GREEN.into()),
            output_slot_material: assets.add(Color::BLUE.into()),
        }
    }
}
