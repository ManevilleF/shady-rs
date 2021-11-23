use bevy::prelude::*;
use shady_generator::NativeType;

#[derive(Debug)]
pub struct GlslTypeMaterials {
    bool_material: Handle<ColorMaterial>,
    int_material: Handle<ColorMaterial>,
    uint_material: Handle<ColorMaterial>,
    float_material: Handle<ColorMaterial>,
    double_material: Handle<ColorMaterial>,
    vec2_material: Handle<ColorMaterial>,
    ivec2_material: Handle<ColorMaterial>,
    vec3_material: Handle<ColorMaterial>,
    ivec3_material: Handle<ColorMaterial>,
    vec4_material: Handle<ColorMaterial>,
    ivec4_material: Handle<ColorMaterial>,
    sampler_2d_material: Handle<ColorMaterial>,
    sampler_cube_material: Handle<ColorMaterial>,
}

#[derive(Debug)]
pub struct ShadyAssets {
    pub font: Handle<Font>,
    pub node_title_material: Handle<ColorMaterial>,
    pub node_title_text_color: Color,
    pub node_id_text_color: Color,
    pub slot_text_color: Color,
    pub input_property_title_material: Handle<ColorMaterial>,
    pub output_property_title_material: Handle<ColorMaterial>,
    pub delete_icon_material: Handle<ColorMaterial>,
    pub node_body_material: Handle<ColorMaterial>,
    pub selected_connector_color: Color,
    pub glsl_type_materials: GlslTypeMaterials,
}

impl ShadyAssets {
    pub fn load(assets: &mut Assets<ColorMaterial>, asset_server: &AssetServer) -> Self {
        let dot_texture = Some(asset_server.load("sprites/2x/outline_circle_white_48dp.png"));
        let close_texture = asset_server.load("sprites/2x/outline_close_white_48dp.png");
        Self {
            font: asset_server.load("fonts/AvenirNext-Regular.ttf"),
            node_title_material: assets.add(Color::BLACK.into()),
            node_title_text_color: Color::WHITE,
            node_id_text_color: Color::GRAY,
            slot_text_color: Color::WHITE,
            input_property_title_material: assets.add(Color::LIME_GREEN.into()),
            output_property_title_material: assets.add(Color::ORANGE.into()),
            delete_icon_material: assets.add(ColorMaterial {
                color: Color::RED,
                texture: Some(close_texture),
            }),
            node_body_material: assets.add(Color::GRAY.into()),
            selected_connector_color: Color::GOLD,
            glsl_type_materials: GlslTypeMaterials {
                bool_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::Bool,
                    dot_texture.clone(),
                )),
                int_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::Int,
                    dot_texture.clone(),
                )),
                uint_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::UInt,
                    dot_texture.clone(),
                )),
                float_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::Float,
                    dot_texture.clone(),
                )),
                double_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::Double,
                    dot_texture.clone(),
                )),
                vec2_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::Vec2,
                    dot_texture.clone(),
                )),
                ivec2_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::IVec2,
                    dot_texture.clone(),
                )),
                vec3_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::Vec3,
                    dot_texture.clone(),
                )),
                ivec3_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::IVec3,
                    dot_texture.clone(),
                )),
                vec4_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::Vec4,
                    dot_texture.clone(),
                )),
                ivec4_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::IVec4,
                    dot_texture.clone(),
                )),
                sampler_2d_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::Sampler2d,
                    dot_texture.clone(),
                )),
                sampler_cube_material: assets.add(GlslTypeMaterials::glsl_type_material(
                    NativeType::SamplerCube,
                    dot_texture,
                )),
            },
        }
    }

    pub fn glsl_type_material(&self, glsl_type: NativeType) -> Handle<ColorMaterial> {
        match glsl_type {
            NativeType::Bool => self.glsl_type_materials.bool_material.clone(),
            NativeType::Int => self.glsl_type_materials.int_material.clone(),
            NativeType::UInt => self.glsl_type_materials.uint_material.clone(),
            NativeType::Float => self.glsl_type_materials.float_material.clone(),
            NativeType::Double => self.glsl_type_materials.double_material.clone(),
            NativeType::Vec2 => self.glsl_type_materials.vec2_material.clone(),
            NativeType::IVec2 => self.glsl_type_materials.ivec2_material.clone(),
            NativeType::Vec3 => self.glsl_type_materials.vec3_material.clone(),
            NativeType::IVec3 => self.glsl_type_materials.ivec3_material.clone(),
            NativeType::Vec4 => self.glsl_type_materials.vec4_material.clone(),
            NativeType::IVec4 => self.glsl_type_materials.ivec4_material.clone(),
            NativeType::Sampler2d => self.glsl_type_materials.sampler_2d_material.clone(),
            NativeType::SamplerCube => self.glsl_type_materials.sampler_cube_material.clone(),
        }
    }
}

impl GlslTypeMaterials {
    pub fn glsl_type_color(glsl_type: NativeType) -> Color {
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

    fn glsl_type_material(
        glsl_type: NativeType,
        texture: Option<Handle<Texture>>,
    ) -> ColorMaterial {
        ColorMaterial {
            color: Self::glsl_type_color(glsl_type),
            texture,
        }
    }
}
