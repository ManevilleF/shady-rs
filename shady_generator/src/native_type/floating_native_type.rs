use crate::NativeType;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// Shader native floating point types
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

impl FloatingNativeType {
    /// All enum variants
    pub const VARIANTS: &'static [Self] = &[Self::Float, Self::Vec2, Self::Vec3, Self::Vec4];
}

impl From<FloatingNativeType> for NativeType {
    fn from(t: FloatingNativeType) -> Self {
        match t {
            FloatingNativeType::Float => NativeType::Float,
            FloatingNativeType::Vec2 => NativeType::Vec2,
            FloatingNativeType::Vec3 => NativeType::Vec3,
            FloatingNativeType::Vec4 => NativeType::Vec4,
        }
    }
}

impl Display for FloatingNativeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", NativeType::from(*self))
    }
}

impl Default for FloatingNativeType {
    fn default() -> Self {
        Self::Float
    }
}
