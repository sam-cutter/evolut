use std::rc::Rc;

use super::neuron::{InternalNeuron, SensoryNeuron};

pub struct Connection {
    input: InputNeuron,
    weight: f64,
}

impl Connection {
    pub fn new(input: InputNeuron, weight: f64) -> Self {
        Self { input, weight }
    }
}

pub enum InputNeuron {
    Sensory(Rc<SensoryNeuron>),
    Internal(Rc<InternalNeuron>),
}
