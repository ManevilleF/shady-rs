use crate::Color;
use shady_generator::GlslType;

pub struct ColorScheme;

impl ColorScheme {
    pub fn resolution_blue() -> Color {
        Color::hex("002082").unwrap()
    }

    pub fn royal_blue() -> Color {
        Color::hex("3057E1").unwrap()
    }

    pub fn light_royal_blue() -> Color {
        Color::hex("4A6DE5").unwrap()
    }

    pub fn lavender_blue() -> Color {
        Color::hex("CED8F7").unwrap()
    }
}
