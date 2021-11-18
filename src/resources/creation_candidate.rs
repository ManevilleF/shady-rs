use shady_generator::{InputProperty, NodePreset, OutputProperty};

#[derive(Debug, Clone)]
pub enum CreationCandidate {
    Node(NodePreset),
    InputProperty(InputProperty),
    OutputProperty(OutputProperty),
}
