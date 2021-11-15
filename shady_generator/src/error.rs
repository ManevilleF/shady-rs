use crate::GlslType;
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
    #[error("Node processing reached depth {0}, check your nodes for potential loops")]
    MaxDepthReached(usize),
    #[error("Detected a loop for nodes {}", .0.join(", "))]
    NodeLoopDetected(Vec<String>),
    #[error("Tried to connect Node {0} to itself")]
    SameNodeConnection(String),
    #[error("I/O Error: {0}")]
    IOError(
        #[source]
        #[from]
        std::io::Error,
    ),
    #[error("Failed to parse Shader file: {0}")]
    WrongShaderSave(
        #[from]
        #[source]
        serde_yaml::Error,
    ),
}
