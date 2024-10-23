use super::super::connection::Connection;

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
}
