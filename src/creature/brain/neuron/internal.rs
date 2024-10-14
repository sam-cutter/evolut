use super::super::connection::Connection;

pub struct InternalNeuron {
    inputs: Vec<Connection>,
}

impl InternalNeuron {
    pub fn new(inputs: Vec<Connection>) -> Self {
        Self { inputs }
    }
}
