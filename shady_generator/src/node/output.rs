use crate::glsl::GlslType;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Output {
    GlslType {
        glsl_type: GlslType,
        field_name: String,
    },
    CustomType(CustomOutput),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomOutput {
    pub struct_name: String,
    pub fields: Vec<(String, GlslType)>,
}

impl Output {
    pub fn glsl_type(&self) -> String {
        match self {
            Output::GlslType { glsl_type, .. } => glsl_type.get_glsl_type().to_string(),
            Output::CustomType(c) => c.struct_name.clone(),
        }
    }

    pub fn fields(&self) -> Vec<(String, GlslType)> {
        match self {
            Output::GlslType {
                glsl_type,
                field_name,
            } => vec![(field_name.clone(), *glsl_type)],
            Output::CustomType(c) => c.fields.clone(),
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
    type Target = Vec<(String, GlslType)>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_declaration_works() {
        let field = Output::CustomType(CustomOutput {
            struct_name: "MyStruct".to_string(),
            fields: vec![
                ("coords".to_string(), GlslType::IVec2),
                ("n".to_string(), GlslType::Float),
                ("matrix".to_string(), GlslType::Vec4),
                ("count".to_string(), GlslType::UInt),
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
