mod action;
mod internal;
mod sensory;

pub use action::ActionNeuron;
pub use internal::InternalNeuron;
pub use sensory::SensoryNeuron;

pub enum Neuron {
    Action(ActionNeuron),
    Internal(InternalNeuron),
    Sensory(SensoryNeuron),
}
