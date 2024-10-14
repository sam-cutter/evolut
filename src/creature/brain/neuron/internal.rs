use super::super::connection::Connection;

#[derive(Debug)]
pub struct InternalNeuron {
    inputs: Vec<Connection>,
}

impl InternalNeuron {
    pub fn new(inputs: Vec<Connection>) -> Self {
        Self { inputs }
    }
}
