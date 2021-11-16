use crate::{Input, InputField};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

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
    Scalar(ScalarNativeType),
    NonScalar(NonScalarNativeType),
}

impl Display for GlslType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_glsl_type())
    }
}

impl Display for ScalarNativeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_glsl_type())
    }
}

impl Display for NonScalarNativeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_glsl_type())
    }
}

impl GlslType {
    /// Returns the GLSL type declaration
    pub fn get_glsl_type(&self) -> &'static str {
        match self {
            GlslType::Scalar(t) => t.get_glsl_type(),
            GlslType::NonScalar(t) => t.get_glsl_type(),
        }
    }

    /// Default GLSL value
    pub fn default_glsl_value(&self) -> &'static str {
        match self {
            GlslType::Scalar(t) => t.default_glsl_value(),
            GlslType::NonScalar(t) => t.default_glsl_value(),
        }
    }
}

impl ScalarNativeType {
    /// Returns the GLSL type declaration
    pub fn get_glsl_type(&self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::Int => "int",
            Self::UInt => "uint",
            Self::Float => "float",
            Self::Double => "double",
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
        }
    }
}

impl NonScalarNativeType {
    /// Returns the GLSL type declaration
    pub fn get_glsl_type(&self) -> &'static str {
        match self {
            Self::Vec2 => "vec2",
            Self::IVec2 => "ivec2",
            Self::Vec3 => "vec3",
            Self::IVec3 => "ivec3",
            Self::Vec4 => "vec4",
            Self::IVec4 => "ivec4",
        }
    }

    /// Default GLSL value
    pub fn default_glsl_value(&self) -> &'static str {
        match self {
            Self::Vec2 => "vec2(0.0, 0.0)",
            Self::IVec2 => "ivec2(0, 0)",
            Self::Vec3 => "vec3(0.0, 0.0, 0.0)",
            Self::IVec3 => "ivec3(0, 0, 0)",
            Self::Vec4 => "vec4(0.0, 0.0, 0.0, 0.0)",
            Self::IVec4 => "ivec4(0, 0, 0, 0)",
        }
    }

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
        Self::Scalar(t)
    }
}

impl From<NonScalarNativeType> for GlslType {
    fn from(t: NonScalarNativeType) -> Self {
        Self::NonScalar(t)
    }
}
