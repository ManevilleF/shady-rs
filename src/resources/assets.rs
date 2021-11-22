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
    pub input_property_title_material: Handle<ColorMaterial>,
    pub output_property_title_material: Handle<ColorMaterial>,
    pub delete_icon_material: Handle<ColorMaterial>,
    pub node_body_material: Handle<ColorMaterial>,
    pub connector_color: Color,
    pub selected_connector_color: Color,
    pub glsl_type_materials: GlslTypeMaterials,
}

impl ShadyAssets {
    fn slot_material(texture: Handle<Texture>, color: Color) -> ColorMaterial {
        ColorMaterial {
            color,
            texture: Some(texture),
        }
    }

    pub fn load(assets: &mut Assets<ColorMaterial>, asset_server: &AssetServer) -> Self {
        let dot_texture = asset_server.load("sprites/2x/outline_circle_white_48dp.png");
        let close_texture = asset_server.load("sprites/2x/outline_close_white_48dp.png");
        Self {
            font: asset_server.load("fonts/AvenirNext-Regular.ttf"),
            node_title_material: assets.add(Color::CYAN.into()),
            input_property_title_material: assets.add(Color::OLIVE.into()),
            output_property_title_material: assets.add(Color::TURQUOISE.into()),
            delete_icon_material: assets.add(ColorMaterial {
                color: Color::RED,
                texture: Some(close_texture),
            }),
            node_body_material: assets.add(Color::GRAY.into()),
            connector_color: Color::WHITE,
            selected_connector_color: Color::GOLD,
            glsl_type_materials: GlslTypeMaterials {
                bool_material: assets.add(Self::slot_material(dot_texture.clone(), Color::CYAN)),
                int_material: assets
                    .add(Self::slot_material(dot_texture.clone(), Color::DARK_GREEN)),
                uint_material: assets.add(Self::slot_material(
                    dot_texture.clone(),
                    Color::YELLOW_GREEN,
                )),
                float_material: assets
                    .add(Self::slot_material(dot_texture.clone(), Color::LIME_GREEN)),
                double_material: assets.add(Self::slot_material(dot_texture.clone(), Color::GREEN)),
                vec2_material: assets.add(Self::slot_material(dot_texture.clone(), Color::BLUE)),
                ivec2_material: assets.add(Self::slot_material(
                    dot_texture.clone(),
                    Color::MIDNIGHT_BLUE,
                )),
                vec3_material: assets.add(Self::slot_material(dot_texture.clone(), Color::YELLOW)),
                ivec3_material: assets.add(Self::slot_material(dot_texture.clone(), Color::GOLD)),
                vec4_material: assets.add(Self::slot_material(dot_texture.clone(), Color::ORANGE)),
                ivec4_material: assets
                    .add(Self::slot_material(dot_texture.clone(), Color::ORANGE_RED)),
                sampler_2d_material: assets
                    .add(Self::slot_material(dot_texture.clone(), Color::PURPLE)),
                sampler_cube_material: assets.add(Self::slot_material(dot_texture, Color::PINK)),
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
