use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};

use super::{
    super::connection::{Connection, InputNeuron},
    Activation, SensoryInputs,
};

/// Neurons which exist to facilitate more complexity in the neural network.
#[derive(Debug)]
pub struct InternalNeuron {
    inputs: Vec<Connection>,
}

impl InternalNeuron {
    /// Creates a new internal neuron.
    pub fn new(inputs: Vec<Connection>) -> Self {
        Self { inputs }
    }

    /// Returns a reference to its inputs.
    pub fn inputs(&self) -> &Vec<Connection> {
        &self.inputs
    }
}

impl Activation for InternalNeuron {
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

impl PartialEq for InternalNeuron {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for InternalNeuron {}

impl Hash for InternalNeuron {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let pointer = self as *const _ as usize;
        pointer.hash(state);
    }
}
