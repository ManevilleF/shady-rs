mod native_function;
mod native_operation;

pub use {native_function::*, native_operation::*};

use crate::{Input, InputField, NonScalarNativeType, Output, ShadyError};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

lazy_static::lazy_static! {
    static ref FUNCTIONS_PATH: String = std::env::var("CUSTOM_FUNCTIONS_PATH").unwrap_or_else(|_| "functions".to_string());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum InternalNodeOperation {
    /// Custom function operation, with custom input and output
    CustomOperation(String),
    /// Native operation
    NativeOperation(NativeOperation),
    /// Non scalar type split
    TypeSplit(NonScalarNativeType),
    /// Non scalar type construction
    TypeConstruction(NonScalarNativeType),
    /// Native Function
    NativeFunction(NativeFunction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeOperation {
    /// Custom function operation, with custom input and output
    CustomOperation {
        /// Custom function name
        /// Must match a `functions` file
        function_name: String,
        /// Input fields
        input: Input,
        /// Output fields
        output: Output,
    },
    /// Native operation
    NativeOperation(NativeOperation),
    /// Non scalar type construction
    TypeConstruction(NonScalarNativeType),
    /// Non scalar type split
    TypeSplit(NonScalarNativeType),
    /// Native Function
    NativeFunction(NativeFunction),
}

impl NodeOperation {
    /// Retrieves the input data for the operation
    pub fn input(&self) -> Input {
        match self {
            NodeOperation::CustomOperation { input, .. } => input.clone(),
            NodeOperation::NativeOperation(o) => o.input(),
            NodeOperation::TypeConstruction(t) => t.input(),
            NodeOperation::TypeSplit(t) => Input {
                fields: vec![("in".to_string(), InputField::new((*t).into()))],
            },
            NodeOperation::NativeFunction(f) => f.input(),
        }
    }

    /// Retrieves the output data for the operation
    pub fn output(&self) -> Output {
        match self {
            NodeOperation::CustomOperation { output, .. } => output.clone(),
            NodeOperation::NativeOperation(o) => o.output(),
            NodeOperation::NativeFunction(f) => f.output(),
            NodeOperation::TypeConstruction(t) => Output::GlslType {
                glsl_type: (*t).into(),
                field_name: "out".to_string(),
            },
            NodeOperation::TypeSplit(t) => t.output(),
        }
    }
}

impl InternalNodeOperation {
    pub fn to_glsl(&self, input_fields: Vec<String>) -> String {
        match self {
            Self::CustomOperation(function_name) => {
                format!("{}({})", function_name, input_fields.join(", "))
            }
            Self::TypeConstruction(t) => {
                format!("{}({})", t, input_fields.join(", "))
            }
            Self::TypeSplit(t) => {
                format!("{}({})", t, input_fields.join(", "))
            }
            Self::NativeOperation(o) => o.glsl_operation(input_fields),
            Self::NativeFunction(f) => {
                format!("{}({})", f.function_name(), input_fields.join(", "))
            }
        }
    }

    pub fn function_declaration(&self) -> Result<Option<String>, ShadyError> {
        match self {
            Self::CustomOperation(function_name) => {
                let path = format!("{}/{}.glsl", *FUNCTIONS_PATH, function_name);
                log::info!("Loading function from {} file", path);
                let buff = match read_to_string(path.as_str()) {
                    Ok(b) => b,
                    Err(e) => {
                        return Err(ShadyError::FileNotFound {
                            file: path,
                            source: e,
                        })
                    }
                };
                Ok(Some(buff))
            }
            _ => Ok(None),
        }
    }
}

impl From<NodeOperation> for InternalNodeOperation {
    fn from(o: NodeOperation) -> Self {
        match o {
            NodeOperation::CustomOperation { function_name, .. } => {
                Self::CustomOperation(function_name)
            }
            NodeOperation::NativeOperation(t) => Self::NativeOperation(t),
            NodeOperation::TypeConstruction(t) => Self::TypeConstruction(t),
            NodeOperation::TypeSplit(t) => Self::TypeSplit(t),
            NodeOperation::NativeFunction(f) => Self::NativeFunction(f),
        }
    }
}
