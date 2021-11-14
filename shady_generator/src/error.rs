use crate::glsl::GlslType;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShadyError {
    #[error("Wrong input value type, got {input_type} expected {expected_type}")]
    WrongGlslType {
        input_type: GlslType,
        expected_type: GlslType,
    },
    #[error("Could not find a field with key `{0}`")]
    WrongFieldKey(String),
    #[error("Could not find node with uuid `{0}`")]
    MissingNode(String),
    #[error("Could not find input property with uuid `{0}`")]
    MissingInputProperty(String),
    #[error("Could not find output property with uuid `{0}`")]
    MissingOutputProperty(String),
}
