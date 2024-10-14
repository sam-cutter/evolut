pub mod action;
pub mod internal;
pub mod sensory;

use std::rc::Rc;

pub use action::ActionNeuron;
pub use internal::InternalNeuron;
pub use sensory::SensoryNeuron;

#[derive(Debug)]
pub enum Neuron {
    Sensory(Rc<SensoryNeuron>),
    Internal(Rc<InternalNeuron>),
    Action(Rc<ActionNeuron>),
}
