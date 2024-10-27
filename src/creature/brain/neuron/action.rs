use super::super::connection::Connection;

/// The outputs of a creature's neural network.
#[derive(Debug)]
pub struct ActionNeuron {
    inputs: Vec<Connection>,
    output: ActionOutput,
}

impl ActionNeuron {
    /// Creates a new action neuron.
    pub fn new(action_neuron_id: u8, inputs: Vec<Connection>) -> Self {
        let output = match action_neuron_id {
            0 => ActionOutput::Acceleration,
            1 => ActionOutput::AngularAcceleration,
            _ => unreachable!(),
        };

        Self { inputs, output }
    }

    pub fn inputs(&self) -> &Vec<Connection> {
        &self.inputs
    }

    pub fn output(&self) -> &ActionOutput {
        &self.output
    }
}

#[derive(Debug)]
pub enum ActionOutput {
    Acceleration,
    AngularAcceleration,
}
