use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum GraphicLibrary {
    // GLSL
    OpenGl,
    // GLSL
    OpenGlEs,
    // WGSL
    WebGPU,
    // TODO: Add DirectX
}

impl Default for GraphicLibrary {
    fn default() -> Self {
        Self::OpenGl
    }
}
