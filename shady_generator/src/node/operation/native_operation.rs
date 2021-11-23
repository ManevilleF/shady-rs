use crate::{Input, InputField, NativeType, Output, ScalarNativeType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NativeOperation {
    /// Increment operation: `a++`
    Inc(NativeType),
    /// Decrement operation: `a--`
    Dec(NativeType),
    /// Minus operation: `-a`
    Minus(NativeType),
    /// Add operation: `a + b`
    Add(NativeType),
    /// Sub operation: `a - b`
    Sub(NativeType),
    /// Mul operation: `a * b`
    Mul(NativeType),
    /// Div operation: `a / b`
    Div(NativeType),
    /// Selection operation: ` c ? a : b`, `c` is a boolean
    Selection(NativeType),
    /// Equals operation: `a == b`, returns a boolean
    Equals(NativeType),
    /// Greater than operation: `a > b`, returns a boolean
    GreaterThan(ScalarNativeType),
    /// Greater or equal operation: `a >= b`, returns a boolean
    GreaterThanEqual(ScalarNativeType),
    /// No operation: `!a`, takes and return booleans
    No,
    /// And operation: `a && b`, takes and return booleans
    And,
    /// Or operation: `a || b`, takes and return booleans
    Or,
    /// XOr operation: `a ^^ b`, takes and return booleans
    Xor,
}

impl NativeOperation {
    /// Retrieves the output data for the operation
    pub fn output(&self) -> Output {
        match self {
            NativeOperation::Inc(t)
            | NativeOperation::Dec(t)
            | NativeOperation::Minus(t)
            | NativeOperation::Add(t)
            | NativeOperation::Sub(t)
            | NativeOperation::Mul(t)
            | NativeOperation::Div(t)
            | NativeOperation::Selection(t) => Output::GlslType(*t),
            NativeOperation::And
            | NativeOperation::Or
            | NativeOperation::Xor
            | NativeOperation::No
            | NativeOperation::Equals(_)
            | NativeOperation::GreaterThan(_)
            | NativeOperation::GreaterThanEqual(_) => Output::GlslType(NativeType::Bool),
        }
    }

    /// Retrieves the input data for the operation
    pub fn input(&self) -> Input {
        match self {
            NativeOperation::Inc(t) | NativeOperation::Dec(t) | NativeOperation::Minus(t) => {
                Input {
                    fields: vec![("i".to_string(), InputField::new(*t))],
                }
            }
            NativeOperation::Add(t)
            | NativeOperation::Sub(t)
            | NativeOperation::Mul(t)
            | NativeOperation::Div(t)
            | NativeOperation::Equals(t) => Input {
                fields: vec![
                    ("a".to_string(), InputField::new(*t)),
                    ("b".to_string(), InputField::new(*t)),
                ],
            },
            NativeOperation::GreaterThan(t) | NativeOperation::GreaterThanEqual(t) => Input {
                fields: vec![
                    ("a".to_string(), InputField::new((*t).into())),
                    ("b".to_string(), InputField::new((*t).into())),
                ],
            },
            NativeOperation::No => Input {
                fields: vec![("i".to_string(), InputField::new(NativeType::Bool))],
            },
            NativeOperation::And | NativeOperation::Or | NativeOperation::Xor => Input {
                fields: vec![
                    ("a".to_string(), InputField::new(NativeType::Bool)),
                    ("b".to_string(), InputField::new(NativeType::Bool)),
                ],
            },
            NativeOperation::Selection(t) => Input {
                fields: vec![
                    ("c".to_string(), InputField::new(NativeType::Bool)),
                    ("a".to_string(), InputField::new(*t)),
                    ("b".to_string(), InputField::new(*t)),
                ],
            },
        }
    }

    /// Outputs the operation as GLSL code
    pub fn glsl_operation(&self, field_values: Vec<String>) -> String {
        match self {
            NativeOperation::Inc(_) => format!("{}++", field_values.first().unwrap()),
            NativeOperation::Dec(_) => format!("{}--", field_values.first().unwrap()),
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
            NativeOperation::Equals(_) => field_values.join(" == "),
            NativeOperation::GreaterThan(_) => field_values.join(" > "),
            NativeOperation::GreaterThanEqual(_) => field_values.join(" >= "),
        }
    }

    pub fn name(&self) -> String {
        match self {
            NativeOperation::Inc(t) => format!("{}++", t),
            NativeOperation::Dec(t) => format!("{}--", t),
            NativeOperation::Minus(t) => format!("-{}", t),
            NativeOperation::Add(t) => format!("{0} + {0}", t),
            NativeOperation::Sub(t) => format!("{0} - {0}", t),
            NativeOperation::Mul(t) => format!("{0} * {0}", t),
            NativeOperation::Div(t) => format!("{0} / {0}", t),
            NativeOperation::No => "NO".to_string(),
            NativeOperation::And => "AND".to_string(),
            NativeOperation::Or => "OR".to_string(),
            NativeOperation::Xor => "XOR".to_string(),
            NativeOperation::Selection(t) => format!("Select {}", t),
            NativeOperation::Equals(t) => format!("{0} == {0}", t),
            NativeOperation::GreaterThan(t) => format!("{0} > {0}", t),
            NativeOperation::GreaterThanEqual(t) => format!("{0} >= {0}", t),
        }
    }

    /// Retrieves a generic descriptive name for the operation
    pub fn descriptive_name(&self) -> &'static str {
        match self {
            NativeOperation::Inc(_) => "a++",
            NativeOperation::Dec(_) => "a--",
            NativeOperation::Minus(_) => "-a",
            NativeOperation::Add(_) => "a + b",
            NativeOperation::Sub(_) => "a - b",
            NativeOperation::Mul(_) => "a * b",
            NativeOperation::Div(_) => "a / b",
            NativeOperation::No => "NO",
            NativeOperation::And => "AND",
            NativeOperation::Or => "OR",
            NativeOperation::Xor => "XOR",
            NativeOperation::Selection(_) => "c ? a : b",
            NativeOperation::Equals(_) => "a == b",
            NativeOperation::GreaterThan(_) => " a > b",
            NativeOperation::GreaterThanEqual(_) => " a >= b",
        }
    }

    /// All enum variants with default values
    pub const VARIANTS: &'static [Self] = &[
        Self::Inc(NativeType::Float),
        Self::Dec(NativeType::Float),
        Self::Minus(NativeType::Float),
        Self::Add(NativeType::Float),
        Self::Sub(NativeType::Float),
        Self::Mul(NativeType::Float),
        Self::Div(NativeType::Float),
        Self::Selection(NativeType::Float),
        Self::Equals(NativeType::Float),
        Self::GreaterThan(ScalarNativeType::Float),
        Self::GreaterThanEqual(ScalarNativeType::Float),
        Self::No,
        Self::And,
        Self::Or,
        Self::Xor,
    ];
}
