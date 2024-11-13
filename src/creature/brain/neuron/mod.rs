mod action;
mod internal;
mod sensory;

use std::{collections::HashMap, sync::Arc};

pub use action::ActionNeuron;
pub use action::ActionOutput;
pub use internal::InternalNeuron;
pub use sensory::SensoryNeuron;

/// Has a variant for each type of neuron.
#[derive(Debug)]
pub enum Neuron {
    Sensory(Arc<SensoryNeuron>),
    Internal(Arc<InternalNeuron>),
    Action(Arc<ActionNeuron>),
}

pub trait Activation {
    fn activation(&self, internal_activation_cache: &mut HashMap<Arc<InternalNeuron>, f64>) -> f64;
}
