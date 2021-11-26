use crate::{make_native_type_enum, NativeType};

make_native_type_enum!(FloatingNativeType {
    Float,
    Vec2,
    Vec3,
    Vec4,
});

impl Default for FloatingNativeType {
    fn default() -> Self {
        Self::Float
    }
}
