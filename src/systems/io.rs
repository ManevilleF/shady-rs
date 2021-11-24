use crate::components::{LogElement, LogLevel};
use crate::resources::{CameraTranslation, ShadyAssets};
use crate::{CurrentShader, IOEvent};
use bevy::prelude::*;
use shady_generator::Shader;
use std::path::PathBuf;

macro_rules! get_path_or_continue {
    ($res:expr, $cmd:expr) => {
        match $res {
            Ok(v) => v,
            Err(e) => {
                LogElement::new(LogLevel::Error, format!("{}", e)).spawn($cmd);
                continue;
            }
        }
    };
}

fn handle_path(p: &str, shader: &CurrentShader, is_shader: bool) -> Result<String, String> {
    let mut path_builder = PathBuf::new();
    path_builder.push(p);
    if !path_builder.exists() {
        return Err(format!("Path `{}` does not exist", p));
    }
    if path_builder.is_dir() {
        if is_shader {
            path_builder.push(&shader.shader_file_name());
        } else {
            path_builder.push(&shader.save_file_name());
        }
    }
    path_builder
        .to_str()
        .map(|str| str.to_string())
        .ok_or_else(|| format!("Failed to format path {:?}", path_builder))
}

pub fn handle_io_events(
    mut commands: Commands,
    mut shader: ResMut<CurrentShader>,
    mut io_evr: EventReader<IOEvent>,
    camera_translation: Res<CameraTranslation>,
    assets: Res<ShadyAssets>,
) {
    for event in io_evr.iter() {
        match event {
            IOEvent::Save(path) => {
                let path = get_path_or_continue!(handle_path(path, &shader, false), &mut commands);
                match shader.save_to(&path) {
                    Ok(()) => {
                        LogElement::new(LogLevel::Info, format!("Saved shader to {}", path))
                            .spawn(&mut commands);
                    }
                    Err(e) => {
                        LogElement::new(LogLevel::Error, format!("Failed to save shader: {}", e))
                            .spawn(&mut commands);
                    }
                };
            }
            IOEvent::Load(path) => {
                let new_shader = get_path_or_continue!(Shader::load(path), &mut commands);
                shader.load(new_shader, &mut commands, &assets, camera_translation.0);
            }
            IOEvent::Export(path) => {
                let path = get_path_or_continue!(handle_path(path, &shader, true), &mut commands);
                match shader.export_glsl_to(&path) {
                    Ok(()) => {
                        LogElement::new(LogLevel::Info, format!("Exported shader to {}", path))
                            .spawn(&mut commands);
                    }
                    Err(e) => {
                        LogElement::new(LogLevel::Error, format!("Failed to export shader: {}", e))
                            .spawn(&mut commands);
                    }
                };
            }
        }
    }
}
