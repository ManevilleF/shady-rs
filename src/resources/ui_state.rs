use crate::resources::CreationCandidate;
use crate::IOEvent;
use shady_generator::node_operation::*;
use shady_generator::{
    Constant, ConstantValue, InputProperty, NativeType, NonScalarNativeType, OutputProperty,
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum IOState {
    Saving,
    Loading,
    Exporting,
}

#[derive(Debug, Clone)]
pub enum TypeSelection {
    Constant(ConstantValue),
    InputProperty(NativeType),
    OutputProperty(NativeType),
    TypeConstruction(NonScalarNativeType),
    TypeSplit(NonScalarNativeType),
    NativeOperation(NativeOperation),
    NativeFunction(NativeFunction),
    TypeSwizzle(NonScalarSwizzle),
}

#[derive(Debug, Clone)]
pub enum OperationSelection {
    NativeOperation(NativeOperation),
    NativeFunction(NativeFunction),
    TypeSwizzle(NonScalarSwizzle),
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
    pub io_state: Option<IOState>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            candidate: None,
            io_state: None,
        }
    }
}

impl IOState {
    pub const fn event(&self, path: PathBuf) -> IOEvent {
        match self {
            IOState::Saving => IOEvent::Save(path),
            IOState::Loading => IOEvent::Load(path),
            IOState::Exporting => IOEvent::Export(path),
        }
    }
}

impl OperationSelection {
    pub fn type_selection_candidate(&self) -> TypeSelection {
        match self {
            OperationSelection::NativeOperation(o) => TypeSelection::NativeOperation(o.clone()),
            OperationSelection::NativeFunction(f) => TypeSelection::NativeFunction(f.clone()),
            OperationSelection::TypeSwizzle(n) => TypeSelection::TypeSwizzle(n.clone()),
        }
    }
}

impl TypeSelection {
    pub fn creation_candidate(&self) -> CreationCandidate {
        match self {
            Self::Constant(v) => CreationCandidate::Constant(Constant {
                name: format!("My {}", v.native_type()),
                value: *v,
            }),
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
            Self::TypeSwizzle(s) => CreationCandidate::Node {
                name: s.complete_name(),
                operation: NodeOperation::NonScalarSwizzle(s.clone()),
            },
        }
    }
}
