use bevy::prelude::*;
use shady_generator::NativeType;

#[derive(Debug)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

#[derive(Debug)]
pub struct ShadyAssets {
    pub font: Handle<Font>,
    pub connector_image: Handle<Image>,
    pub node_title_color: Color,
    pub node_title_text_color: Color,
    pub node_id_text_color: Color,
    pub slot_text_color: Color,
    pub constant_title_color: Color,
    pub input_property_title_color: Color,
    pub output_property_title_color: Color,
    pub delete_icon_material: SpriteMaterial,
    pub node_body_color: Color,
    pub selected_connector_color: Color,
    pub tolerant_slot_color: Color,
}

impl ShadyAssets {
    pub fn load(asset_server: &AssetServer) -> Self {
        let connector_image = asset_server.load("sprites/2x/outline_circle_white_48dp.png");
        let close_texture = asset_server.load("sprites/2x/outline_close_white_48dp.png");
        Self {
            font: asset_server.load("fonts/AvenirNext-Regular.ttf"),
            connector_image,
            node_title_color: Color::BLACK,
            node_title_text_color: Color::WHITE,
            node_id_text_color: Color::GRAY,
            slot_text_color: Color::WHITE,
            constant_title_color: Color::MIDNIGHT_BLUE,
            input_property_title_color: Color::LIME_GREEN,
            output_property_title_color: Color::ORANGE,
            delete_icon_material: SpriteMaterial {
                color: Color::RED,
                texture: close_texture,
            },
            node_body_color: Color::GRAY,
            selected_connector_color: Color::WHITE,
            tolerant_slot_color: Color::WHITE,
        }
    }

    pub const fn glsl_type_color(glsl_type: NativeType) -> Color {
        match glsl_type {
            NativeType::Bool => Color::CYAN,
            NativeType::Int => Color::DARK_GREEN,
            NativeType::UInt => Color::YELLOW_GREEN,
            NativeType::Float => Color::LIME_GREEN,
            NativeType::Double => Color::GREEN,
            NativeType::Vec2 => Color::BLUE,
            NativeType::IVec2 => Color::MIDNIGHT_BLUE,
            NativeType::Vec3 => Color::YELLOW,
            NativeType::IVec3 => Color::GOLD,
            NativeType::Vec4 => Color::ORANGE,
            NativeType::IVec4 => Color::ORANGE_RED,
            NativeType::Sampler2d => Color::PURPLE,
            NativeType::SamplerCube => Color::PINK,
        }
    }
}
