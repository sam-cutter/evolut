use super::super::connection::Connection;

pub struct ActionNeuron {
    inputs: Vec<Connection>,
    output: ActionOutput,
}

impl ActionNeuron {
    pub fn new(action_neuron_id: u8, inputs: Vec<Connection>) -> Self {
        let output = match action_neuron_id {
            0 => ActionOutput::Acceleration,
            1 => ActionOutput::AngularAcceleration,
            _ => unreachable!(),
        };

        Self { inputs, output }
    }
}

pub enum ActionOutput {
    Acceleration,
    AngularAcceleration,
}
