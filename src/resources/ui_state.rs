use crate::resources::CreationCandidate;
use shady_generator::{
    InputProperty, NativeOperation, NativeType, NonScalarNativeType, OutputProperty,
};
use shady_generator::{NativeFunction, NodeOperation};

#[derive(Debug, Clone)]
pub enum TypeSelection {
    InputProperty(NativeType),
    OutputProperty(NativeType),
    TypeConstruction(NonScalarNativeType),
    TypeSplit(NonScalarNativeType),
    NativeOperation(NativeOperation),
    NativeFunction(NativeFunction),
}

#[derive(Debug, Clone)]
pub enum OperationSelection {
    NativeOperation(NativeOperation),
    NativeFunction(NativeFunction),
}

#[derive(Debug)]
pub enum Candidate {
    OperationSelection(OperationSelection),
    TypeSelection(TypeSelection),
    Creation(CreationCandidate),
}

#[derive(Debug)]
pub struct UiState {
    pub candidate: Option<Candidate>,
}

impl Default for UiState {
    fn default() -> Self {
        Self { candidate: None }
    }
}

impl OperationSelection {
    pub fn type_selection_candidate(&self) -> TypeSelection {
        match self {
            OperationSelection::NativeOperation(o) => TypeSelection::NativeOperation(o.clone()),
            OperationSelection::NativeFunction(f) => TypeSelection::NativeFunction(f.clone()),
        }
    }
}

impl TypeSelection {
    pub fn creation_candidate(&self) -> CreationCandidate {
        match self {
            Self::InputProperty(t) => {
                CreationCandidate::InputProperty(InputProperty::new(*t, false))
            }
            Self::OutputProperty(t) => CreationCandidate::OutputProperty(OutputProperty::new(*t)),
            Self::TypeConstruction(t) => CreationCandidate::Node {
                name: t.to_string(),
                operation: NodeOperation::TypeConstruction(*t),
            },
            Self::TypeSplit(t) => CreationCandidate::Node {
                name: format!("{} Split", t),
                operation: NodeOperation::TypeSplit(*t),
            },
            Self::NativeOperation(o) => CreationCandidate::Node {
                name: o.name(),
                operation: NodeOperation::NativeOperation(o.clone()),
            },
            Self::NativeFunction(f) => CreationCandidate::Node {
                name: f.function_name().to_string(),
                operation: NodeOperation::NativeFunction(f.clone()),
            },
        }
    }
}
