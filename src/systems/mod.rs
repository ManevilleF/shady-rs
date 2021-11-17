pub mod input;
pub mod lines;
pub mod nodes;
pub mod setup;
pub mod ui;

use bevy::log;

#[macro_export]
macro_rules! get_cursor_position {
    ($res:expr) => {
        match $res {
            Some(p) => p,
            None => return,
        }
    };
}

#[macro_export]
macro_rules! get_or_return {
    ($res:expr) => {
        match $res {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to retrieve component: {}", e);
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! get_or_continue {
    ($res:expr) => {
        match $res {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to retrieve component: {}", e);
                continue;
            }
        }
    };
}
