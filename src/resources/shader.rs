use shady_generator::{NodePreset, Shader};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct CurrentShader(pub Shader);

#[derive(Debug, Clone)]
pub struct SelectedNodePreset(pub Option<NodePreset>);

impl Default for CurrentShader {
    fn default() -> Self {
        Self(Shader::default())
    }
}

impl Default for SelectedNodePreset {
    fn default() -> Self {
        Self(None)
    }
}

impl Deref for CurrentShader {
    type Target = Shader;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CurrentShader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
