use std::rc::Rc;

use super::neuron::{InternalNeuron, SensoryNeuron};

pub struct Connection {
    input: InputNeuron,
    weight: f64,
}

pub enum InputNeuron {
    Sensory(Rc<SensoryNeuron>),
    Internal(Rc<InternalNeuron>),
}
