mod action;
mod internal;
mod sensory;

use std::sync::Arc;

pub use action::ActionNeuron;
pub use internal::InternalNeuron;
pub use sensory::SensoryNeuron;

/// Has a variant for each type of neuron.
#[derive(Debug)]
pub enum Neuron {
    Sensory(Arc<SensoryNeuron>),
    Internal(Arc<InternalNeuron>),
    Action(Arc<ActionNeuron>),
}
