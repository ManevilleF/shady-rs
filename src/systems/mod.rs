pub mod input;
pub mod lines;
pub mod nodes;
pub mod setup;
pub mod ui;

#[macro_export]
macro_rules! get_cursor_position {
    ($res:expr) => {
        match $res {
            Some(p) => p,
            None => return,
        }
    };
}
