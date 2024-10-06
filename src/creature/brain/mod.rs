pub mod connection;
pub mod neuron;

use super::genome::Genome;
use connection::Connection;
use neuron::Neurons;
use neuron::{LineOfSight, SensoryNeuron};

pub struct Brain<'a> {
    connections: Vec<Connection<'a>>,
    neurons: Neurons,
}

// TODO: Implement constructor
impl<'a> Brain<'a> {
    pub fn new(genome: &Genome) {
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

        // Step 1
        let mut neurons = Neurons::new();

        for gene in genome {
            if gene.source_id() < 127 {
                let source_id = (gene.source_id() - 128) % 12;

                // TODO: export sensory neuron matching to a separate function
                let neuron: (u8, SensoryNeuron) = match source_id {
                    0 => (source_id, SensoryNeuron::Age),
                    1 => (source_id, SensoryNeuron::Speed),
                    2 => (source_id, SensoryNeuron::AngularVelocity),
                    3 => (
                        source_id,
                        SensoryNeuron::LineOfSight(LineOfSight::LeftCreature),
                    ),
                    4 => (source_id, SensoryNeuron::LineOfSight(LineOfSight::LeftFood)),
                    5 => (
                        source_id,
                        SensoryNeuron::LineOfSight(LineOfSight::LeftObstacle),
                    ),
                    6 => (
                        source_id,
                        SensoryNeuron::LineOfSight(LineOfSight::MiddleCreature),
                    ),
                    7 => (
                        source_id,
                        SensoryNeuron::LineOfSight(LineOfSight::MiddleFood),
                    ),
                    8 => (
                        source_id,
                        SensoryNeuron::LineOfSight(LineOfSight::MiddleObstacle),
                    ),
                    9 => (
                        source_id,
                        SensoryNeuron::LineOfSight(LineOfSight::RightCreature),
                    ),
                    10 => (
                        source_id,
                        SensoryNeuron::LineOfSight(LineOfSight::RightFood),
                    ),
                    11 => (
                        source_id,
                        SensoryNeuron::LineOfSight(LineOfSight::RightObstacle),
                    ),
                    12 => (source_id, SensoryNeuron::StoredEnergy),
                    _ => unreachable!(),
                };

                neurons.push_sensory(neuron);
            }
        }
    }
}
