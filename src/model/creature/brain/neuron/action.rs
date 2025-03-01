use std::{collections::HashMap, sync::Arc};

use super::{
    super::{InputNeuron, connection::Connection},
    Activation, InternalNeuron, SensoryInputs,
};

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

    /// Returns a reference to its inputs.
    pub fn inputs(&self) -> &Vec<Connection> {
        &self.inputs
    }

    /// Returns a reference to its output type.
    pub fn output(&self) -> &ActionOutput {
        &self.output
    }
}

impl Activation for ActionNeuron {
    // TODO: make implementation of Activation the same for ActionNeuron and InternalNeuron
    fn activation(
        &self,
        internal_activation_cache: &mut HashMap<Arc<InternalNeuron>, f32>,
        sensory_inputs: &SensoryInputs,
    ) -> f32 {
        let activation = self
            .inputs()
            .iter()
            .map(|connection| match connection.input() {
                InputNeuron::Internal(internal_neuron) => {
                    let cached_activation = internal_activation_cache.get(internal_neuron);

                    if let Some(activation) = cached_activation {
                        connection.weight() * activation
                    } else {
                        let activation =
                            internal_neuron.activation(internal_activation_cache, sensory_inputs);

                        internal_activation_cache.insert(Arc::clone(internal_neuron), activation);

                        connection.weight() * activation
                    }
                }
                InputNeuron::Sensory(sensory_neuron) => {
                    connection.weight()
                        * sensory_neuron.activation(internal_activation_cache, sensory_inputs)
                }
            })
            .sum::<f32>()
            .tanh();

        return activation;
    }
}

/// The output type of an action neuron.
#[derive(Debug)]
pub enum ActionOutput {
    /// The acceleration to be applied to a creature's velocity.
    Acceleration,
    /// The angular acceleration to be applied to a creature's angular velocity.
    AngularAcceleration,
    // TODO: Add a neuron for reproduction.
}
