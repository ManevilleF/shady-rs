use crate::glsl_type::NonScalarNativeType;
use crate::{GlslType, Input, InputField, Node, Output, ScalarNativeType, ShadyError};
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
    /// Non scalar type construction
    TypeConstruction(NonScalarNativeType),
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NativeOperation {
    /// Increment operation: `a++`
    Inc(GlslType),
    /// Minus operation: `-a`
    Minus(GlslType),
    /// Add operation: `a + b`
    Add(GlslType),
    /// Sub operation: `a - b`
    Sub(GlslType),
    /// Mul operation: `a * b`
    Mul(GlslType),
    /// Div operation: `a / b`
    Div(GlslType),
    /// Selection operation: ` c ? a : b`, `c` is a boolean
    Selection(GlslType),
    /// No operation: `!a`, takes and return booleans
    No,
    /// And operation: `a && b`, takes and return booleans
    And,
    /// Or operation: `a || b`, takes and return booleans
    Or,
    /// XOr operation: `a ^^ b`, takes and return booleans
    Xor,
}

impl NodeOperation {
    pub fn input(&self) -> Input {
        match self {
            NodeOperation::CustomOperation { input, .. } => input.clone(),
            NodeOperation::NativeOperation(o) => o.input(),
            NodeOperation::TypeConstruction(t) => t.input(),
        }
    }

    pub fn output(&self) -> Output {
        match self {
            NodeOperation::CustomOperation { output, .. } => output.clone(),
            NodeOperation::NativeOperation(o) => o.output(),
            NodeOperation::TypeConstruction(t) => Output::GlslType {
                glsl_type: GlslType::NonScalar(*t),
                field_name: "v".to_string(),
            },
        }
    }
}

impl NativeOperation {
    pub fn output(&self) -> Output {
        match self {
            NativeOperation::Inc(t)
            | NativeOperation::Minus(t)
            | NativeOperation::Add(t)
            | NativeOperation::Sub(t)
            | NativeOperation::Mul(t)
            | NativeOperation::Div(t)
            | NativeOperation::Selection(t) => Output::GlslType {
                glsl_type: *t,
                field_name: "o".to_string(),
            },
            NativeOperation::And
            | NativeOperation::Or
            | NativeOperation::Xor
            | NativeOperation::No => Output::GlslType {
                glsl_type: GlslType::Scalar(ScalarNativeType::Bool),
                field_name: "o".to_string(),
            },
        }
    }

    pub fn input(&self) -> Input {
        match self {
            NativeOperation::Inc(t) => Input {
                fields: vec![("i".to_string(), InputField::new(*t))],
            },
            NativeOperation::Minus(t)
            | NativeOperation::Add(t)
            | NativeOperation::Sub(t)
            | NativeOperation::Mul(t)
            | NativeOperation::Div(t) => Input {
                fields: vec![
                    ("a".to_string(), InputField::new(*t)),
                    ("b".to_string(), InputField::new(*t)),
                ],
            },
            NativeOperation::No => Input {
                fields: vec![(
                    "i".to_string(),
                    InputField::new(GlslType::Scalar(ScalarNativeType::Bool)),
                )],
            },
            NativeOperation::And | NativeOperation::Or | NativeOperation::Xor => Input {
                fields: vec![
                    (
                        "a".to_string(),
                        InputField::new(GlslType::Scalar(ScalarNativeType::Bool)),
                    ),
                    (
                        "b".to_string(),
                        InputField::new(GlslType::Scalar(ScalarNativeType::Bool)),
                    ),
                ],
            },
            NativeOperation::Selection(t) => Input {
                fields: vec![
                    (
                        "c".to_string(),
                        InputField::new(GlslType::Scalar(ScalarNativeType::Bool)),
                    ),
                    ("a".to_string(), InputField::new(*t)),
                    ("b".to_string(), InputField::new(*t)),
                ],
            },
        }
    }

    pub fn glsl_operation(&self, node: &Node) -> String {
        let field_values = node.input_field_glsl_values();
        match self {
            NativeOperation::Inc(_) => format!("{}++", field_values.first().unwrap()),
            NativeOperation::Minus(_) => format!("-{}", field_values.first().unwrap()),
            NativeOperation::No => format!("!{}", field_values.first().unwrap()),
            NativeOperation::Add(_) => field_values.join(" + "),
            NativeOperation::Sub(_) => field_values.join(" - "),
            NativeOperation::Mul(_) => field_values.join(" * "),
            NativeOperation::Div(_) => field_values.join(" / "),
            NativeOperation::And => field_values.join(" && "),
            NativeOperation::Or => field_values.join(" || "),
            NativeOperation::Xor => field_values.join(" ^^ "),
            NativeOperation::Selection(_) => format!(
                "{} ? {} : {}",
                field_values[0], field_values[1], field_values[2]
            ),
        }
    }
}

impl InternalNodeOperation {
    pub fn to_glsl(&self, node: &Node) -> String {
        match self {
            Self::CustomOperation(function_name) => {
                format!(
                    "{}({})",
                    function_name,
                    node.input_field_glsl_values().join(", ")
                )
            }
            Self::TypeConstruction(t) => {
                format!("{}({})", t, node.input_field_glsl_values().join(", "))
            }
            Self::NativeOperation(o) => o.glsl_operation(node),
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
        }
    }
}
