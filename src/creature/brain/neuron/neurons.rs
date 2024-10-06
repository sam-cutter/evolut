use super::ActionNeuron;
use super::InternalNeuron;
use super::SensoryNeuron;

pub struct Neurons {
    sensory: Vec<(u8, SensoryNeuron)>,
    internal: Vec<(u8, InternalNeuron)>,
    action: Vec<(u8, ActionNeuron)>,
}

impl Neurons {
    pub fn new() -> Self {
        Self {
            sensory: Vec::new(),
            internal: Vec::new(),
            action: Vec::new(),
        }
    }

    pub fn sensory(&self) -> &Vec<(u8, SensoryNeuron)> {
        &self.sensory
    }

    pub fn internal(&self) -> &Vec<(u8, InternalNeuron)> {
        &self.internal
    }

    pub fn action(&self) -> &Vec<(u8, ActionNeuron)> {
        &self.action
    }

    pub fn push_sensory(&mut self, neuron: (u8, SensoryNeuron)) {
        self.sensory.push(neuron);
    }

    pub fn push_internal(&mut self, neuron: (u8, InternalNeuron)) {
        self.internal.push(neuron);
    }

    pub fn push_action(&mut self, neuron: (u8, ActionNeuron)) {
        self.action.push(neuron);
    }
}
