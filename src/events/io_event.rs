#[derive(Debug)]
pub enum IOEvent {
    Save(String),
    Load(String),
    Export(String),
    // TODO: Add presets (bevy, Unity, etc)
    // LoadPreset
}
