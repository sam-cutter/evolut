mod action;
mod internal;
mod neurons;
mod sensory;

pub use action::ActionNeuron;
pub use internal::InternalNeuron;
pub use neurons::Neurons;
// TODO: figure out how LineOfSight should be accessed
pub use sensory::LineOfSight;
pub use sensory::SensoryNeuron;

pub enum Neuron {
    Action(ActionNeuron),
    Internal(InternalNeuron),
    Sensory(SensoryNeuron),
}
