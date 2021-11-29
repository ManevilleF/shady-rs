use crate::resources::{BuiltinValue, PreviewValue};
use crate::systems::ui::constants::constant_value_selection;
use crate::PreviewMaterial;
use bevy_egui::egui::{CollapsingHeader, ComboBox, Ui};

pub fn handle_preview(ui: &mut Ui, preview_material: &mut PreviewMaterial) {
    for (key, input_value) in preview_material.input_values.iter_mut() {
        ui.horizontal(|ui| {
            ComboBox::from_label(key)
                .selected_text(input_value.preview_value.selection_name())
                .show_ui(ui, |ui| {
                    let available_values =
                        PreviewValue::available_values(input_value.expected_type);
                    for available_value in available_values {
                        ui.selectable_value(
                            &mut input_value.preview_value,
                            available_value.clone(),
                            available_value.selection_name(),
                        );
                    }
                    ui.selectable_value(
                        &mut input_value.preview_value,
                        PreviewValue::Unset,
                        "Unset",
                    );
                });
        });
        match &mut input_value.preview_value {
            PreviewValue::ConstantValue(value) => {
                CollapsingHeader::new("Value")
                    .id_source(format!("{} value", key))
                    .show(ui, |ui| {
                        constant_value_selection(ui, value);
                    });
            }
            PreviewValue::BuiltinValue(BuiltinValue::Color(c)) => {
                ui.color_edit_button_rgba_unmultiplied(c);
            }
            _ => (),
        }
    }
}
