use crate::{NativeType, NonScalarNativeType};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Output {
    NativeType(NativeType),
    CustomType(CustomOutput),
    Split(NonScalarNativeType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomOutput {
    pub struct_name: String,
    pub fields: Vec<(String, NativeType)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFields {
    SingleOutput(NativeType),
    Fields(Vec<(String, NativeType)>),
}

impl Output {
    pub fn glsl_type(&self) -> String {
        match self {
            Output::NativeType(glsl_type) => glsl_type.get_glsl_type().to_string(),
            Output::CustomType(c) => c.struct_name.clone(),
            Output::Split(t) => NativeType::from(*t).get_glsl_type().to_string(),
        }
    }

    pub fn fields(&self) -> OutputFields {
        match self {
            Output::NativeType(glsl_type) => OutputFields::SingleOutput(*glsl_type),
            Output::CustomType(c) => OutputFields::Fields(c.fields.clone()),
            Output::Split(t) => OutputFields::Fields(t.type_construction_fields()),
        }
    }

    pub fn custom_declaration(&self) -> Option<String> {
        if let Output::CustomType(c) = &self {
            Some(c.glsl_struct_declaration())
        } else {
            None
        }
    }
}

impl OutputFields {
    pub const SINGLE_FIELD_NAME: &'static str = "out";

    pub fn field_names(&self) -> Vec<(String, NativeType)> {
        match self {
            OutputFields::SingleOutput(t) => vec![(Self::SINGLE_FIELD_NAME.to_string(), *t)],
            OutputFields::Fields(f) => f.clone(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            OutputFields::SingleOutput(_t) => 1,
            OutputFields::Fields(f) => f.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            OutputFields::SingleOutput(_t) => false,
            OutputFields::Fields(f) => f.is_empty(),
        }
    }
}

impl CustomOutput {
    pub fn glsl_struct_declaration(&self) -> String {
        let mut buff = format!("struct {} {{\n", self.struct_name);
        for (field_name, glsl_type) in self.fields.iter() {
            buff = format!("{}  {} {};\n", buff, glsl_type.get_glsl_type(), field_name);
        }
        format!("{}}};\n", buff)
    }
}

impl Deref for CustomOutput {
    type Target = Vec<(String, NativeType)>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NonScalarNativeType, ScalarNativeType};

    #[test]
    fn custom_declaration_works() {
        let field = Output::CustomType(CustomOutput {
            struct_name: "MyStruct".to_string(),
            fields: vec![
                ("coords".to_string(), NonScalarNativeType::IVec2.into()),
                ("n".to_string(), ScalarNativeType::Float.into()),
                ("matrix".to_string(), NonScalarNativeType::Vec4.into()),
                ("count".to_string(), ScalarNativeType::UInt.into()),
            ],
        });
        assert_eq!(field.glsl_type(), "MyStruct".to_string());
        assert!(field.custom_declaration().is_some());
        assert_eq!(
            field.custom_declaration().unwrap(),
            formatdoc! {"
                struct MyStruct {{
                  ivec2 coords;
                  float n;
                  vec4 matrix;
                  uint count;
                }};
            "}
        )
    }
}
