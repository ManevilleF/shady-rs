use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Supported graphics libraries
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
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

impl Display for GraphicLibrary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GraphicLibrary::OpenGl => "OpenGl",
                GraphicLibrary::OpenGlEs => "OpenGlEs",
                GraphicLibrary::WebGPU => "WebGPU",
            }
        )
    }
}
