use super::neuron::{ActionNeuron, InternalNeuron, SensoryNeuron};
use std::rc::Rc;

pub struct Connection {
    source: SourceNeuron,
    destination: DestinationNeuron,
    weight: f64,
}

impl Connection {
    pub fn new(source: SourceNeuron, destination: DestinationNeuron, weight: f64) -> Self {
        Self {
            source,
            destination,
            weight,
        }
    }

    pub fn source(&self) -> &SourceNeuron {
        &self.source
    }

    pub fn destination(&self) -> &DestinationNeuron {
        &self.destination
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }
}

pub enum SourceNeuron {
    Sensory(Rc<SensoryNeuron>),
    Internal(Rc<InternalNeuron>),
}

pub enum DestinationNeuron {
    Internal(Rc<InternalNeuron>),
    Action(Rc<ActionNeuron>),
}
