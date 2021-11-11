use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum GlslType {
    Bool,
    Int,
    UInt,
    Float,
    Double,
    Vec2,
    IVec2,
    Vec3,
    IVec3,
    Vec4,
    IVec4,
}

impl Display for GlslType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_glsl_type())
    }
}

impl GlslType {
    pub fn get_glsl_type(&self) -> &'static str {
        match self {
            GlslType::Bool => "bool",
            GlslType::Int => "int",
            GlslType::UInt => "uint",
            GlslType::Float => "float",
            GlslType::Double => "double",
            GlslType::Vec2 => "vec2",
            GlslType::IVec2 => "ivec2",
            GlslType::Vec3 => "vec3",
            GlslType::IVec3 => "ivec3",
            GlslType::Vec4 => "vec4",
            GlslType::IVec4 => "ivec4",
        }
    }

    pub fn default_glsl_value(&self) -> &'static str {
        match self {
            GlslType::Bool => "false",
            GlslType::Int => "0",
            GlslType::UInt => "0",
            GlslType::Float => "0.0",
            GlslType::Double => "0.0",
            GlslType::Vec2 => "vec2(0.0, 0.0)",
            GlslType::IVec2 => "ivec2(0, 0)",
            GlslType::Vec3 => "vec3(0.0, 0.0, 0.0)",
            GlslType::IVec3 => "ivec3(0, 0, 0)",
            GlslType::Vec4 => "vec4(0.0, 0.0, 0.0, 0.0)",
            GlslType::IVec4 => "ivec4(0, 0, 0, 0)",
        }
    }
}
