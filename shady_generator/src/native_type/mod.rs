mod floating_native_type;
mod non_scalar_native_type;
mod scalar_native_type;

pub use {floating_native_type::*, non_scalar_native_type::*, scalar_native_type::*};

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

// TODO: Rename to `NativeType`
/// Available native types for input and output properties and nodes.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum NativeType {
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
    /// 2D Float vector (x, y)
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
    /// 4D Integer Vector (x, y, z, w)
    Sampler2d,
    /// 4D Integer Vector (x, y, z, w)
    SamplerCube,
}

impl Display for NativeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_glsl_type())
    }
}

impl NativeType {
    /// Returns the GLSL type declaration
    pub fn get_glsl_type(&self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::Int => "int",
            Self::UInt => "uint",
            Self::Float => "float",
            Self::Double => "double",
            Self::Vec2 => "vec2",
            Self::IVec2 => "ivec2",
            Self::Vec3 => "vec3",
            Self::IVec3 => "ivec3",
            Self::Vec4 => "vec4",
            Self::IVec4 => "ivec4",
            Self::Sampler2d => "sampler2D",
            Self::SamplerCube => "samplerCube",
        }
    }

    /// Default GLSL value
    pub fn default_glsl_value(&self) -> &'static str {
        match self {
            Self::Bool => "false",
            Self::Int => "0",
            Self::UInt => "0",
            Self::Float => "0.0",
            Self::Double => "0.0",
            Self::Vec2 => "vec2(0.0, 0.0)",
            Self::IVec2 => "ivec2(0, 0)",
            Self::Vec3 => "vec3(0.0, 0.0, 0.0)",
            Self::IVec3 => "ivec3(0, 0, 0)",
            Self::Vec4 => "vec4(0.0, 0.0, 0.0, 0.0)",
            Self::IVec4 => "ivec4(0, 0, 0, 0)",
            _ => {
                log::warn!("There is no default value available for {}", self);
                "__UNSET__"
            }
        }
    }

    pub const VARIANTS: &'static [Self] = &[
        Self::Bool,
        Self::Int,
        Self::UInt,
        Self::Float,
        Self::Double,
        Self::Vec2,
        Self::IVec2,
        Self::Vec3,
        Self::IVec3,
        Self::Vec4,
        Self::IVec4,
        Self::Sampler2d,
        Self::SamplerCube,
    ];
}

impl Default for NativeType {
    fn default() -> Self {
        Self::Float
    }
}
