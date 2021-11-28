use bevy_egui::egui::{DragValue, Ui};
use shady_generator::ConstantValue;

fn int_range(ui: &mut Ui, v: &mut i32, name: &str) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(DragValue::new(v).clamp_range(i32::MIN..=i32::MAX));
    });
}

fn uint_range(ui: &mut Ui, v: &mut u32, name: &str) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(DragValue::new(v).clamp_range(u32::MIN..=u32::MAX));
    });
}

fn f32_range(ui: &mut Ui, v: &mut f32, name: &str) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(DragValue::new(v).clamp_range(f32::MIN..=f32::MAX));
    });
}

fn f64_range(ui: &mut Ui, v: &mut f64, name: &str) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(DragValue::new(v).clamp_range(f64::MIN..=f64::MAX));
    });
}

pub fn constant_value_selection(ui: &mut Ui, constant: &mut ConstantValue) {
    match constant {
        ConstantValue::Bool(v) => {
            ui.checkbox(v, "Value");
        }
        ConstantValue::Int(v) => {
            int_range(ui, v, "Value");
        }
        ConstantValue::UInt(v) => {
            uint_range(ui, v, "Value");
        }
        ConstantValue::Float(v) => {
            f32_range(ui, v, "Value");
        }
        ConstantValue::Double(v) => {
            f64_range(ui, v, "Value");
        }
        ConstantValue::Vec2(v) => {
            for (i, value) in v.iter_mut().enumerate() {
                f32_range(ui, value, i.to_string().as_str());
            }
        }
        ConstantValue::IVec2(v) => {
            for (i, value) in v.iter_mut().enumerate() {
                int_range(ui, value, i.to_string().as_str());
            }
        }
        ConstantValue::Vec3(v) => {
            for (i, value) in v.iter_mut().enumerate() {
                f32_range(ui, value, i.to_string().as_str());
            }
        }
        ConstantValue::IVec3(v) => {
            for (i, value) in v.iter_mut().enumerate() {
                int_range(ui, value, i.to_string().as_str());
            }
        }
        ConstantValue::Vec4(v) => {
            for (i, value) in v.iter_mut().enumerate() {
                f32_range(ui, value, i.to_string().as_str());
            }
        }
        ConstantValue::IVec4(v) => {
            for (i, value) in v.iter_mut().enumerate() {
                int_range(ui, value, i.to_string().as_str());
            }
        }
    }
}
