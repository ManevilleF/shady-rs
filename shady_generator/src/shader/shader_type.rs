use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Shader types
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum ShaderType {
    /// Vertex shader
    Vertex,
    /// Fragment shader
    Fragment,
    // TODO: Geometry
}

impl Default for ShaderType {
    fn default() -> Self {
        Self::Vertex
    }
}

impl Display for ShaderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShaderType::Vertex => "Vertex",
                ShaderType::Fragment => "Fragment",
            }
        )
    }
}
