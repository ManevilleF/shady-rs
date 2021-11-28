use bevy_egui::egui::{emath, DragValue, Ui};
use shady_generator::ConstantValue;

const ITER_VALUE_NAMES: [&str; 4] = ["x", "y", "z", "w"];

fn show_range<T: emath::Numeric>(ui: &mut Ui, v: &mut T, name: &str) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(DragValue::new(v).clamp_range(T::MIN..=T::MAX));
    });
}

fn show_iter_range<T: emath::Numeric>(ui: &mut Ui, values: &mut [T]) {
    for (i, value) in values.iter_mut().enumerate() {
        show_range(
            ui,
            value,
            ITER_VALUE_NAMES.get(i).unwrap_or(&i.to_string().as_str()),
        );
    }
}

pub fn constant_value_selection(ui: &mut Ui, constant: &mut ConstantValue) {
    match constant {
        ConstantValue::Bool(v) => {
            ui.checkbox(v, "Value");
        }
        ConstantValue::Int(v) => {
            show_range(ui, v, "Value");
        }
        ConstantValue::UInt(v) => {
            show_range(ui, v, "Value");
        }
        ConstantValue::Float(v) => {
            show_range(ui, v, "Value");
        }
        ConstantValue::Double(v) => {
            show_range(ui, v, "Value");
        }
        ConstantValue::Vec2(v) => {
            show_iter_range(ui, v);
        }
        ConstantValue::IVec2(v) => {
            show_iter_range(ui, v);
        }
        ConstantValue::Vec3(v) => {
            show_iter_range(ui, v);
        }
        ConstantValue::IVec3(v) => {
            show_iter_range(ui, v);
        }
        ConstantValue::Vec4(v) => {
            show_iter_range(ui, v);
        }
        ConstantValue::IVec4(v) => {
            show_iter_range(ui, v);
        }
    }
}
