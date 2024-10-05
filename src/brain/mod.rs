pub mod connection;
pub mod neuron;

use connection::Connection;
use neuron::Neuron;

pub struct Brain<'a> {
    connections: Vec<Connection<'a>>,
    neurons: Vec<Neuron>,
}

// TODO: Implement constructor
impl<'a> Brain<'a> {
    pub fn new() {
        // This function will have a genome passed to it by reference

        /*
        Step 1: creating required neurons.
            For each specified connection in the genome, the source and destination neurons will be created.
        Step 2: creating required connections.
            Each connection can now be built with the newly created neurons.
        Step 3: connection pruning.
            The connections can now be pruned to ensure that no loops or cycles are introduced into the network.
        Step 4: neuron pruning.
            Any unused neurons can now be pruned.
        Step 5: return all connections and neurons.
        */
    }
}
