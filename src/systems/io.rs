use crate::{CurrentShader, IOEvent};
use bevy::log;
use bevy::prelude::*;
use std::path::PathBuf;

macro_rules! get_path_or_continue {
    ($res:expr) => {
        match $res {
            Some(v) => v,
            None => continue,
        }
    };
}

fn handle_path(p: &str, shader: &CurrentShader, is_shader: bool) -> Option<String> {
    let mut path_builder = PathBuf::new();
    path_builder.push(p);
    if !path_builder.exists() {
        log::error!("Path `{}` does not exist", p);
        return None;
    }
    if path_builder.is_dir() {
        if is_shader {
            path_builder.push(&shader.shader_file_name());
        } else {
            path_builder.push(&shader.save_file_name());
        }
    }
    path_builder.to_str().map(|str| str.to_string())
}

pub fn handle_io_events(shader: Res<CurrentShader>, mut io_evr: EventReader<IOEvent>) {
    for event in io_evr.iter() {
        match event {
            IOEvent::Save(path) => {
                let path = get_path_or_continue!(handle_path(path, &shader, false));
                match shader.save_to(&path) {
                    Ok(()) => {
                        log::info!("Saved shader to {}", path);
                    }
                    Err(e) => {
                        log::error!("Failed to save shader: {}", e);
                    }
                };
            }
            IOEvent::Load(path) => {}
            IOEvent::Export(path) => {
                let path = get_path_or_continue!(handle_path(path, &shader, true));
                match shader.export_glsl_to(&path) {
                    Ok(()) => {
                        log::info!("Exported shader to {}", path);
                    }
                    Err(e) => {
                        log::error!("Failed to export shader: {}", e);
                    }
                };
            }
        }
    }
}
