use crate::{GlslType, Input, InputField, Output, ScalarNativeType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NativeOperation {
    /// Increment operation: `a++`
    Inc(GlslType),
    /// Decrement operation: `a--`
    Dec(GlslType),
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
    /// Equals operation: `a == b`, returns a boolean
    Equals(GlslType),
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
    pub fn output(&self) -> Output {
        match self {
            NativeOperation::Inc(t)
            | NativeOperation::Dec(t)
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
            | NativeOperation::No
            | NativeOperation::Equals(_)
            | NativeOperation::GreaterThan(_)
            | NativeOperation::GreaterThanEqual(_) => Output::GlslType {
                glsl_type: GlslType::Bool,
                field_name: "o".to_string(),
            },
        }
    }

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
                fields: vec![("i".to_string(), InputField::new(GlslType::Bool))],
            },
            NativeOperation::And | NativeOperation::Or | NativeOperation::Xor => Input {
                fields: vec![
                    ("a".to_string(), InputField::new(GlslType::Bool)),
                    ("b".to_string(), InputField::new(GlslType::Bool)),
                ],
            },
            NativeOperation::Selection(t) => Input {
                fields: vec![
                    ("c".to_string(), InputField::new(GlslType::Bool)),
                    ("a".to_string(), InputField::new(*t)),
                    ("b".to_string(), InputField::new(*t)),
                ],
            },
        }
    }

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
}
