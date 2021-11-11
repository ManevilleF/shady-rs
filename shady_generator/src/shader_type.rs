use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ShaderType {
    Vertex,
    Fragment,
    // TODO: Geometry
}

impl Default for ShaderType {
    fn default() -> Self {
        Self::Vertex
    }
}
