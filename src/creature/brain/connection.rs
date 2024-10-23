use std::rc::Rc;

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
}

/// Represents the specific neuron which a connection depends upon.
#[derive(Debug)]
pub enum InputNeuron {
    Sensory(Rc<SensoryNeuron>),
    Internal(Rc<InternalNeuron>),
}
