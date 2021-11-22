use crate::resources::CreationCandidate;
use shady_generator::{InputProperty, NativeType, OutputProperty};

#[derive(Debug)]
pub enum CandidateSelection {
    InputNativeType(NativeType),
    OutputNativeType(NativeType),
}

#[derive(Debug)]
pub struct UiState {
    pub creation_candidate: Option<CreationCandidate>,
    pub candidate_selection: Option<CandidateSelection>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            creation_candidate: None,
            candidate_selection: None,
        }
    }
}

impl CandidateSelection {
    pub fn creation_candidate(&self) -> CreationCandidate {
        match self {
            CandidateSelection::InputNativeType(t) => {
                CreationCandidate::InputProperty(InputProperty::new(*t, false))
            }
            CandidateSelection::OutputNativeType(t) => {
                CreationCandidate::OutputProperty(OutputProperty::new(*t))
            }
        }
    }
}
