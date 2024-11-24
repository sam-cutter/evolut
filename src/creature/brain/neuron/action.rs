use std::{collections::HashMap, sync::Arc};

use bevy::a11y::accesskit::Action;

use super::{
    super::{connection::Connection, InputNeuron},
    Activation, InternalNeuron,
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
    fn activation(&self, internal_activation_cache: &mut HashMap<Arc<InternalNeuron>, f64>) -> f64 {
        let activation = self
            .inputs()
            .iter()
            .map(|connection| match connection.input() {
                InputNeuron::Internal(internal_neuron) => {
                    let cached_activation = internal_activation_cache.get(internal_neuron);

                    if let Some(activation) = cached_activation {
                        connection.weight() * activation
                    } else {
                        let activation = internal_neuron.activation(internal_activation_cache);

                        internal_activation_cache.insert(Arc::clone(internal_neuron), activation);

                        connection.weight() * activation
                    }
                }
                InputNeuron::Sensory(sensory_neuron) => {
                    connection.weight() * sensory_neuron.activation(internal_activation_cache)
                }
            })
            .sum::<f64>()
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
}
