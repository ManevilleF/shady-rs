use crate::{make_native_type_enum, Input, InputField, NativeType, ScalarNativeType};

make_native_type_enum!(NonScalarNativeType {
    Vec2,
    IVec2,
    Vec3,
    IVec3,
    Vec4,
    IVec4,
});

impl NonScalarNativeType {
    pub(crate) fn type_construction_fields(self) -> Vec<(String, NativeType)> {
        match self {
            Self::Vec2 => vec![
                ("x".to_string(), ScalarNativeType::Float.into()),
                ("y".to_string(), ScalarNativeType::Float.into()),
            ],
            Self::IVec2 => vec![
                ("x".to_string(), ScalarNativeType::Int.into()),
                ("y".to_string(), ScalarNativeType::Int.into()),
            ],
            Self::Vec3 => vec![
                ("x".to_string(), ScalarNativeType::Float.into()),
                ("y".to_string(), ScalarNativeType::Float.into()),
                ("z".to_string(), ScalarNativeType::Float.into()),
            ],
            Self::IVec3 => vec![
                ("x".to_string(), ScalarNativeType::Int.into()),
                ("y".to_string(), ScalarNativeType::Int.into()),
                ("z".to_string(), ScalarNativeType::Int.into()),
            ],
            Self::Vec4 => vec![
                ("x".to_string(), ScalarNativeType::Float.into()),
                ("y".to_string(), ScalarNativeType::Float.into()),
                ("z".to_string(), ScalarNativeType::Float.into()),
                ("w".to_string(), ScalarNativeType::Float.into()),
            ],
            Self::IVec4 => vec![
                ("x".to_string(), ScalarNativeType::Int.into()),
                ("y".to_string(), ScalarNativeType::Int.into()),
                ("z".to_string(), ScalarNativeType::Int.into()),
                ("w".to_string(), ScalarNativeType::Int.into()),
            ],
        }
    }

    pub(crate) fn type_construction_input(self) -> Input {
        Input {
            fields: self
                .type_construction_fields()
                .into_iter()
                .map(|(f, t)| (f, InputField::new(t)))
                .collect(),
        }
    }
}

impl Default for NonScalarNativeType {
    fn default() -> Self {
        Self::Vec2
    }
}
