mod action;
mod internal;
mod sensory;

pub use action::ActionNeuron;
pub use internal::InternalNeuron;
pub use sensory::SensoryNeuron;

// TODO: Create function signatures for Neuron trait
// TODO: Implement Neuron trait for different Neuron types
pub trait Neuron {}

impl Neuron for SensoryNeuron {}

impl Neuron for InternalNeuron {}

impl Neuron for ActionNeuron {}
