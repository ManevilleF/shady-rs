use crate::NativeType;
use thiserror::Error;

/// Shady Generator Error types
#[derive(Debug, Error)]
pub enum ShadyError {
    /// Shader Native type mismatch
    #[error("Wrong input value type, got {input_type} expected a type in {expected_types:?}")]
    WrongNativeType {
        /// Received native type
        input_type: NativeType,
        /// Expected native type
        expected_types: Vec<NativeType>,
    },
    /// Wrong Node field
    #[error("Could not find a field with key `{0}`")]
    WrongFieldKey(String),

    #[error("Node {0} doesn't have complex output fields")]
    SingleOutput(String),
    #[error("Node {0} has complex output fields")]
    ComplexOutput(String),
    /// Missing Node
    #[error("Could not find node with uuid `{0}`")]
    MissingNode(String),
    /// Missing input Property
    #[error("Could not find input property with uuid `{0}`")]
    MissingInputProperty(String),
    /// Missing output Property
    #[error("Could not find output property with uuid `{0}`")]
    MissingOutputProperty(String),
    /// Node generation processing reached its max depth
    #[error("Node processing reached depth {0}, check your nodes for potential loops")]
    MaxDepthReached(usize),
    /// Node generation detected a Node loop in the shader
    #[error("Detected a loop for nodes {}", .0.join(", "))]
    NodeLoopDetected(Vec<String>),
    /// A Node can't be connected to itself
    #[error("Tried to connect Node {0} to itself")]
    SameNodeConnection(String),
    /// I/O Error from `std::io::Error`
    #[error("I/O Error: {0}")]
    IOError(
        #[source]
        #[from]
        std::io::Error,
    ),
    /// File not found from `std::io::Error`
    #[error("File `{file}` not found")]
    FileNotFound {
        /// File path
        file: String,
        /// Source error
        #[source]
        source: std::io::Error,
    },
    /// Serialization error for Shader from `serde_yaml::Error`
    #[error("Failed to parse Shader file: {0}")]
    WrongShaderSave(
        #[from]
        #[source]
        serde_yaml::Error,
    ),
}
