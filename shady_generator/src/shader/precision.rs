use crate::NativeType;
use serde::{Deserialize, Serialize};

/// Shader Precision
#[derive(Debug, Serialize, Deserialize)]
pub enum ShaderPrecision {
    /// High Precision
    /// Requires `GL_FRAGMENT_PRECISION_HIGH` macro set to 1
    High,
    /// Medium Precision
    Medium,
    /// Low Precision
    Low,
}

impl Default for ShaderPrecision {
    fn default() -> Self {
        Self::Medium
    }
}

impl ShaderPrecision {
    pub fn to_glsl(&self, glsl_type: NativeType) -> String {
        match self {
            Self::High => format!("precision highp {};", glsl_type),
            Self::Medium => format!("precision mediump {};", glsl_type),
            Self::Low => format!("precision lowp {};", glsl_type),
        }
    }
}
