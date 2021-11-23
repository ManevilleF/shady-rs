use crate::{NativeType, NonScalarNativeType};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Output {
    GlslType {
        glsl_type: NativeType,
        field_name: String,
    },
    CustomType(CustomOutput),
    Split(NonScalarNativeType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomOutput {
    pub struct_name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<(String, NativeType)>,
}

impl Output {
    pub fn glsl_type(&self) -> String {
        match self {
            Output::GlslType { glsl_type, .. } => glsl_type.get_glsl_type().to_string(),
            Output::CustomType(c) => c.struct_name.clone(),
            Output::Split(t) => NativeType::from(*t).get_glsl_type().to_string(),
        }
    }

    pub fn fields(&self) -> Vec<(String, NativeType)> {
        match self {
            Output::GlslType {
                glsl_type,
                field_name,
            } => vec![(field_name.clone(), *glsl_type)],
            Output::CustomType(c) => c.fields.clone(),
            Output::Split(t) => t.fields(),
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
