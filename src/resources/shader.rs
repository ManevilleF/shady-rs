use shady_generator::Shader;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct CurrentShader(pub Shader);

impl Default for CurrentShader {
    fn default() -> Self {
        Self(Shader::default())
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
