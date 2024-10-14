use std::rc::Rc;

use super::neuron::{InternalNeuron, SensoryNeuron};

#[derive(Debug)]
pub struct Connection {
    input: InputNeuron,
    weight: f64,
}

impl Connection {
    pub fn new(input: InputNeuron, weight: f64) -> Self {
        Self { input, weight }
    }
}

#[derive(Debug)]
pub enum InputNeuron {
    Sensory(Rc<SensoryNeuron>),
    Internal(Rc<InternalNeuron>),
}
