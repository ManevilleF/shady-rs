use crate::{Input, InputField, NativeType, ScalarNativeType};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

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

impl From<NonScalarNativeType> for NativeType {
    fn from(t: NonScalarNativeType) -> Self {
        match t {
            NonScalarNativeType::Vec2 => NativeType::Vec2,
            NonScalarNativeType::IVec2 => NativeType::IVec2,
            NonScalarNativeType::Vec3 => NativeType::Vec3,
            NonScalarNativeType::IVec3 => NativeType::IVec3,
            NonScalarNativeType::Vec4 => NativeType::Vec4,
            NonScalarNativeType::IVec4 => NativeType::IVec4,
        }
    }
}

impl Display for NonScalarNativeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", NativeType::from(*self))
    }
}

impl Default for NonScalarNativeType {
    fn default() -> Self {
        Self::Vec2
    }
}
