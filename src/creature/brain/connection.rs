use std::{collections::HashMap, sync::Arc};

use super::neuron::{Activation, InternalNeuron, SensoryNeuron};

/// Represents a dependency on another neuron.
#[derive(Debug)]
pub struct Connection {
    input: InputNeuron,
    weight: f64,
}

impl Connection {
    /// Creates a new connection.
    pub fn new(input: InputNeuron, weight: f64) -> Self {
        Self { input, weight }
    }

    /// Returns a reference to the connection input.
    pub fn input(&self) -> &InputNeuron {
        &self.input
    }

    /// Returns the weight of the connection.
    pub fn weight(&self) -> f64 {
        self.weight
    }
}

/// Represents the specific neuron which a connection depends upon.
#[derive(Debug)]
pub enum InputNeuron {
    /// A sensory neuron.
    Sensory(Arc<SensoryNeuron>),
    /// An internal neuron.
    Internal(Arc<InternalNeuron>),
}

impl Activation for InputNeuron {
    fn activation(&self, internal_activation_cache: &mut HashMap<Arc<InternalNeuron>, f64>) -> f64 {
        match self {
            InputNeuron::Sensory(sensory_neuron) => {
                sensory_neuron.activation(internal_activation_cache)
            }
            InputNeuron::Internal(internal_neuron) => {
                internal_neuron.activation(internal_activation_cache)
            }
        }
    }
}
