use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

// TODO: Rename to `NativeType`
/// Available native types for input and output properties and nodes.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum GlslType {
    /// Boolean type
    Bool,
    /// Signed Integer type
    Int,
    /// Unsigned Integer type
    UInt,
    /// Floating number type
    Float,
    /// Double floating number type (long)
    Double,
    /// 2D Float Vector (x, y)
    Vec2,
    /// 2D Integer Vector (x, y)
    IVec2,
    /// 3D Float Vector (x, y, z)
    Vec3,
    /// 3D Integer Vector (x, y, z)
    IVec3,
    /// 4D Float Vector (x, y, z, w)
    Vec4,
    /// 4D Integer Vector (x, y, z, w)
    IVec4,
}

impl Display for GlslType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_glsl_type())
    }
}

impl GlslType {
    /// Returns the GLSL type declaration
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

    /// Default GLSL value
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
