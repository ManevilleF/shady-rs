use crate::NativeType;
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

impl ScalarNativeType {
    pub const VARIANTS: &'static [Self] =
        &[Self::Bool, Self::Int, Self::UInt, Self::Float, Self::Double];
}

impl From<ScalarNativeType> for NativeType {
    fn from(t: ScalarNativeType) -> Self {
        match t {
            ScalarNativeType::Bool => NativeType::Bool,
            ScalarNativeType::Int => NativeType::Int,
            ScalarNativeType::UInt => NativeType::UInt,
            ScalarNativeType::Float => NativeType::Float,
            ScalarNativeType::Double => NativeType::Double,
        }
    }
}

impl Display for ScalarNativeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", NativeType::from(*self))
    }
}

impl Default for ScalarNativeType {
    fn default() -> Self {
        Self::Float
    }
}
