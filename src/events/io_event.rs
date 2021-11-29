use std::path::PathBuf;

#[derive(Debug)]
pub enum IOEvent {
    Save(PathBuf),
    Load(PathBuf),
    Export(PathBuf),
}
