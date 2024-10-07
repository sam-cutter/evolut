use super::neuron::{ActionNeuron, InternalNeuron, SensoryNeuron};

pub struct Connection<'a> {
    source: SourceNeuron<'a>,
    destination: DestinationNeuron<'a>,
    weight: f64,
}

impl<'a> Connection<'a> {
    pub fn new(source: SourceNeuron<'a>, destination: DestinationNeuron<'a>, weight: f64) -> Self {
        Self {
            source,
            destination,
            weight,
        }
    }
}

pub enum SourceNeuron<'a> {
    Sensory(&'a SensoryNeuron),
    Internal(&'a InternalNeuron),
}

pub enum DestinationNeuron<'a> {
    Internal(&'a InternalNeuron),
    Action(&'a ActionNeuron),
}
