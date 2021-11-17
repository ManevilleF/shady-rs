use serde::{Deserialize, Serialize};

/// Supported graphics libraries
#[derive(Debug, Serialize, Deserialize)]
pub enum GraphicLibrary {
    /// OpenGL (GLSL code)
    OpenGl,
    /// OpenGL ES (GLSL code)
    OpenGlEs,
    /// WebGPU (WGSL code)
    WebGPU,
    // TODO: Add DirectX and Vulkan
}

impl Default for GraphicLibrary {
    fn default() -> Self {
        Self::OpenGl
    }
}
