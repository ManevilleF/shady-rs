use crate::{make_native_type_enum, NativeType};

make_native_type_enum!(ScalarNativeType {
    Bool,
    Int,
    UInt,
    Float,
    Double,
});

make_native_type_enum!(NumericScalarNativeType {
    Int,
    UInt,
    Float,
    Double,
});

impl Default for ScalarNativeType {
    fn default() -> Self {
        Self::Float
    }
}

impl Default for NumericScalarNativeType {
    fn default() -> Self {
        Self::Float
    }
}
