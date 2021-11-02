#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TransferValue {
    Float(f32),
    BigFloat(f64),
    Int(i32),
    BigInt(i64),
    Vec3(f32, f32, f32),
}
