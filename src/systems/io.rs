use crate::common::get_current_dir;
use crate::components::{LogElement, LogLevel};
use crate::resources::{CameraTranslation, IOState, ShadyAssets};
use crate::{CurrentShader, IOEvent, PreviewMaterial, UiState};
use bevy::prelude::*;
use bevy::tasks::{ComputeTaskPool, Task};
use futures_lite::future;
use rfd::{AsyncFileDialog, FileHandle};
use shady_generator::Shader;

pub fn handle_io_events(
    mut commands: Commands,
    mut shader: ResMut<CurrentShader>,
    mut io_evr: EventReader<IOEvent>,
    mut preview: ResMut<PreviewMaterial>,
    camera_translation: Res<CameraTranslation>,
    assets: Res<ShadyAssets>,
) {
    for event in io_evr.iter() {
        match event {
            IOEvent::Save(path) => {
                match shader.save_to(&path) {
                    Ok(()) => {
                        LogElement::new(LogLevel::Info, format!("Saved shader to {:?}", path))
                            .spawn(&mut commands);
                    }
                    Err(e) => {
                        LogElement::new(LogLevel::Error, format!("Failed to save shader: {}", e))
                            .spawn(&mut commands);
                    }
                };
            }
            IOEvent::Load(path) => {
                let new_shader = match Shader::load(path) {
                    Ok(s) => s,
                    Err(e) => {
                        LogElement::new(LogLevel::Error, format!("Failed to load shader: {}", e))
                            .spawn(&mut commands);
                        continue;
                    }
                };
                shader.load(
                    new_shader,
                    &mut commands,
                    &assets,
                    &mut preview,
                    camera_translation.0,
                );
            }
            IOEvent::Export(path) => {
                match shader.export_glsl_to(&path) {
                    Ok(()) => {
                        LogElement::new(LogLevel::Info, format!("Exported shader to {:?}", path))
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

pub fn handle_io_state(
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
    task_pool: Res<ComputeTaskPool>,
    shader: Res<CurrentShader>,
) {
    if let Some(state) = ui_state.io_state.take() {
        let shader_name = shader.shader_file_name();
        let save_name = shader.save_file_name();
        let task = task_pool.spawn(async move {
            let dialog_task = match state {
                IOState::Saving => {
                    AsyncFileDialog::new()
                        .set_file_name(&save_name)
                        .save_file()
                        .await
                }
                IOState::Exporting => {
                    AsyncFileDialog::new()
                        .set_directory(get_current_dir())
                        .set_file_name(&shader_name)
                        .save_file()
                        .await
                }
                IOState::Loading => {
                    AsyncFileDialog::new()
                        .set_directory(get_current_dir())
                        .add_filter("Save File", &["yaml"])
                        .pick_file()
                        .await
                }
            };
            dialog_task.map(|h| (h, state))
        });
        commands.spawn().insert(task);
    }
}

type IOTask = Task<Option<(FileHandle, IOState)>>;

pub fn handle_io_task(
    mut commands: Commands,
    mut query: Query<(Entity, &mut IOTask)>,
    mut io_evw: EventWriter<IOEvent>,
) {
    for (entity, mut task) in query.iter_mut() {
        if let Some(res) = future::block_on(future::poll_once(&mut *task)) {
            if let Some((handle, state)) = res {
                io_evw.send(state.event(handle.path().to_path_buf()));
            } else {
                // TODO: cancel task
                // task.cancel();
            }
            commands.entity(entity).despawn();
        }
    }
}
