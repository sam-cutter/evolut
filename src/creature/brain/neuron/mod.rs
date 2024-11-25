mod action;
mod internal;
mod sensory;

use std::{collections::HashMap, sync::Arc};

pub use action::{ActionNeuron, ActionOutput};
pub use internal::InternalNeuron;
pub use sensory::SensoryNeuron;

/// Has a variant for each type of neuron.
#[derive(Debug)]
pub enum Neuron {
    /// A sensory neuron.
    Sensory(Arc<SensoryNeuron>),
    /// An internal neuron.
    Internal(Arc<InternalNeuron>),
    /// An action neuron.
    Action(Arc<ActionNeuron>),
}

/// Contains functionality to retrieve the activation of neurons.
// TODO: make implementation of Activation the same for ActionNeuron and InternalNeuron
pub trait Activation {
    /// Computes the activation of a neuron, given a cache of internal neuron activations.
    fn activation(&self, internal_activation_cache: &mut HashMap<Arc<InternalNeuron>, f64>) -> f64;
}
