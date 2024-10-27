use std::sync::Arc;

use super::neuron::{InternalNeuron, SensoryNeuron};

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

    pub fn input(&self) -> &InputNeuron {
        &self.input
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }
}

/// Represents the specific neuron which a connection depends upon.
#[derive(Debug)]
pub enum InputNeuron {
    Sensory(Arc<SensoryNeuron>),
    Internal(Arc<InternalNeuron>),
}
