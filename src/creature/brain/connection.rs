use super::neuron::{ActionNeuron, InternalNeuron, SensoryNeuron};

pub struct Connection<'a> {
    source: &'a SourceNeuron,
    destination: &'a DestinationNeuron,
    weight: f64,
}

impl<'a> Connection<'a> {
    pub fn new(source: &'a SourceNeuron, destination: &'a DestinationNeuron, weight: f64) -> Self {
        Self {
            source,
            destination,
            weight,
        }
    }
}

pub enum SourceNeuron {
    Sensory(SensoryNeuron),
    Internal(InternalNeuron),
}

pub enum DestinationNeuron {
    Internal(InternalNeuron),
    Action(ActionNeuron),
}
