mod action;
mod internal;
mod sensory;

use std::rc::Rc;

pub use action::ActionNeuron;
pub use internal::InternalNeuron;
pub use sensory::SensoryNeuron;

/// Has a variant for each type of neuron.
#[derive(Debug)]
pub enum Neuron {
    Sensory(Rc<SensoryNeuron>),
    Internal(Rc<InternalNeuron>),
    Action(Rc<ActionNeuron>),
}
