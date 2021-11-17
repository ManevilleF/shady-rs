use serde::{Deserialize, Serialize};

/// Shader types
#[derive(Debug, Serialize, Deserialize)]
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
