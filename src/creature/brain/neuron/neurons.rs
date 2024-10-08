use super::ActionNeuron;
use super::InternalNeuron;
use super::SensoryNeuron;
use std::rc::Rc;

pub struct Neurons {
    sensory: Vec<(u8, Rc<SensoryNeuron>)>,
    internal: Vec<(u8, Rc<InternalNeuron>)>,
    action: Vec<(u8, Rc<ActionNeuron>)>,
}

impl Neurons {
    pub fn new() -> Self {
        Self {
            sensory: Vec::new(),
            internal: Vec::new(),
            action: Vec::new(),
        }
    }

    pub fn sensory(&self) -> &Vec<(u8, Rc<SensoryNeuron>)> {
        &self.sensory
    }

    pub fn internal(&self) -> &Vec<(u8, Rc<InternalNeuron>)> {
        &self.internal
    }

    pub fn action(&self) -> &Vec<(u8, Rc<ActionNeuron>)> {
        &self.action
    }

    pub fn push_sensory(&mut self, neuron: (u8, Rc<SensoryNeuron>)) {
        self.sensory.push(neuron);
    }

    pub fn push_internal(&mut self, neuron: (u8, Rc<InternalNeuron>)) {
        self.internal.push(neuron);
    }

    pub fn push_action(&mut self, neuron: (u8, Rc<ActionNeuron>)) {
        self.action.push(neuron);
    }
}
