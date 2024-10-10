pub mod connection;
pub mod neuron;

use neuron::Neuron;

struct Brain {
    neurons: Vec<Neuron>,
}
