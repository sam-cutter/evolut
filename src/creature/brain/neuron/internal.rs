use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

use super::{super::connection::Connection, Activation};

/// Neurons which exist to facilitate more complexity in the neural network.
#[derive(Debug)]
pub struct InternalNeuron {
    inputs: Vec<Connection>,
}

impl InternalNeuron {
    /// Creates a new internal neuron.
    pub fn new(inputs: Vec<Connection>) -> Self {
        Self { inputs }
    }

    pub fn inputs(&self) -> &Vec<Connection> {
        &self.inputs
    }
}

impl Activation for InternalNeuron {
    fn activation(&self, internal_activation_cache: &mut HashMap<Arc<InternalNeuron>, f64>) -> f64 {
        let cached_activation = internal_activation_cache.get(self);

        if let Some(activation) = cached_activation {
            return *activation;
        }

        let activation = self
            .inputs()
            .iter()
            .map(|connection| {
                connection.weight() * connection.input().activation(internal_activation_cache)
            })
            .sum::<f64>()
            .tanh();

        return activation;
    }
}

impl PartialEq for InternalNeuron {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for InternalNeuron {}

impl Hash for InternalNeuron {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let pointer = self as *const _ as usize;
        pointer.hash(state);
    }
}
