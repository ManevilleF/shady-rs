use shady_generator::node_operation::NodeOperation;
use shady_generator::{InputProperty, OutputProperty};

#[derive(Debug, Clone)]
pub enum CreationCandidate {
    Node {
        name: String,
        operation: NodeOperation,
    },
    InputProperty(InputProperty),
    OutputProperty(OutputProperty),
}
