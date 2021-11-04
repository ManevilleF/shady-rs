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
}

impl CustomOutput {
    pub fn glsl_struct_declaration(&self) -> String {
        let mut buff = format!("struct {} {{\n", self.struct_name);
        for (field_name, glsl_type) in self.fields.iter() {
            buff = format!("{}{} {};\n", buff, glsl_type.get_glsl_type(), field_name);
        }
        format!("{}\n}};\n", buff)
    }
}

impl Deref for CustomOutput {
    type Target = Vec<(String, GlslType)>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}
