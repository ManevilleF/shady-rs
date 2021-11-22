use shady_generator::{InputProperty, NodeOperation, OutputProperty};

#[derive(Debug, Clone)]
pub enum CreationCandidate {
    Node {
        name: String,
        operation: NodeOperation,
    },
    InputProperty(InputProperty),
    OutputProperty(OutputProperty),
}
