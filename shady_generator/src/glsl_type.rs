use crate::{Input, InputField};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum FloatingNativeType {
    /// Floating number type
    Float,
    /// 2D Float vector (x, y)
    Vec2,
    /// 3D Float Vector (x, y, z)
    Vec3,
    /// 4D Float Vector (x, y, z, w)
    Vec4,
}

/// Scalar native types
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ScalarNativeType {
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
}

/// Complex native types like vectors and matrices,
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum NonScalarNativeType {
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
}

// TODO: Rename to `NativeType`
// TODO: Add Sampler types
/// Available native types for input and output properties and nodes.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
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

impl Display for GlslType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_glsl_type())
    }
}

impl GlslType {
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
}

impl NonScalarNativeType {
    pub fn input(&self) -> Input {
        match self {
            Self::Vec2 => Input {
                fields: vec![
                    (
                        "x".to_string(),
                        InputField::new(ScalarNativeType::Float.into()),
                    ),
                    (
                        "y".to_string(),
                        InputField::new(ScalarNativeType::Float.into()),
                    ),
                ],
            },
            Self::IVec2 => Input {
                fields: vec![
                    (
                        "x".to_string(),
                        InputField::new(ScalarNativeType::Int.into()),
                    ),
                    (
                        "y".to_string(),
                        InputField::new(ScalarNativeType::Int.into()),
                    ),
                ],
            },
            Self::Vec3 => Input {
                fields: vec![
                    (
                        "x".to_string(),
                        InputField::new(ScalarNativeType::Float.into()),
                    ),
                    (
                        "y".to_string(),
                        InputField::new(ScalarNativeType::Float.into()),
                    ),
                    (
                        "z".to_string(),
                        InputField::new(ScalarNativeType::Float.into()),
                    ),
                ],
            },
            Self::IVec3 => Input {
                fields: vec![
                    (
                        "x".to_string(),
                        InputField::new(ScalarNativeType::Int.into()),
                    ),
                    (
                        "y".to_string(),
                        InputField::new(ScalarNativeType::Int.into()),
                    ),
                    (
                        "z".to_string(),
                        InputField::new(ScalarNativeType::Int.into()),
                    ),
                ],
            },
            Self::Vec4 => Input {
                fields: vec![
                    (
                        "x".to_string(),
                        InputField::new(ScalarNativeType::Float.into()),
                    ),
                    (
                        "y".to_string(),
                        InputField::new(ScalarNativeType::Float.into()),
                    ),
                    (
                        "z".to_string(),
                        InputField::new(ScalarNativeType::Float.into()),
                    ),
                    (
                        "w".to_string(),
                        InputField::new(ScalarNativeType::Float.into()),
                    ),
                ],
            },
            Self::IVec4 => Input {
                fields: vec![
                    (
                        "x".to_string(),
                        InputField::new(ScalarNativeType::Int.into()),
                    ),
                    (
                        "y".to_string(),
                        InputField::new(ScalarNativeType::Int.into()),
                    ),
                    (
                        "z".to_string(),
                        InputField::new(ScalarNativeType::Int.into()),
                    ),
                    (
                        "w".to_string(),
                        InputField::new(ScalarNativeType::Int.into()),
                    ),
                ],
            },
        }
    }
}

impl From<ScalarNativeType> for GlslType {
    fn from(t: ScalarNativeType) -> Self {
        match t {
            ScalarNativeType::Bool => GlslType::Bool,
            ScalarNativeType::Int => GlslType::Int,
            ScalarNativeType::UInt => GlslType::UInt,
            ScalarNativeType::Float => GlslType::Float,
            ScalarNativeType::Double => GlslType::Double,
        }
    }
}

impl From<NonScalarNativeType> for GlslType {
    fn from(t: NonScalarNativeType) -> Self {
        match t {
            NonScalarNativeType::Vec2 => GlslType::Vec2,
            NonScalarNativeType::IVec2 => GlslType::IVec2,
            NonScalarNativeType::Vec3 => GlslType::Vec3,
            NonScalarNativeType::IVec3 => GlslType::IVec3,
            NonScalarNativeType::Vec4 => GlslType::Vec4,
            NonScalarNativeType::IVec4 => GlslType::IVec4,
        }
    }
}

impl From<FloatingNativeType> for GlslType {
    fn from(t: FloatingNativeType) -> Self {
        match t {
            FloatingNativeType::Float => GlslType::Float,
            FloatingNativeType::Vec2 => GlslType::Vec2,
            FloatingNativeType::Vec3 => GlslType::Vec3,
            FloatingNativeType::Vec4 => GlslType::Vec4,
        }
    }
}
