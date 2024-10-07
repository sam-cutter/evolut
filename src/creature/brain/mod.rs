pub mod connection;
pub mod neuron;

use super::genome::Genome;
use crate::simulation::MAX_INTERNAL_NEURONS;
use connection::{Connection, DestinationNeuron, SourceNeuron};
use neuron::{ActionNeuron, InternalNeuron, LineOfSight, Neurons, SensoryNeuron};

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

        let neurons: Neurons = create_neurons(genome);
        let connections: Vec<Connection> = create_connections(genome, &neurons);
    }

    pub fn connections(&self) -> &Vec<Connection<'a>> {
        &self.connections
    }

    pub fn neurons(&self) -> &Neurons {
        &self.neurons
    }
}

/// Creates each connection specified in the [Genome] out [Neurons].
fn create_connections<'a>(genome: &Genome, neurons: &'a Neurons) -> Vec<Connection<'a>> {
    let mut connections: Vec<Connection> = Vec::new();

    for gene in genome {
        let source_is_sensory_neuron = gene.source_id() < 128;
        let source_neuron: SourceNeuron = match source_is_sensory_neuron {
            true => SourceNeuron::Sensory(
                &neurons
                    .sensory()
                    .iter()
                    .filter(|(id, _)| *id == gene.source_id())
                    .collect::<Vec<&(u8, SensoryNeuron)>>()
                    .first()
                    .unwrap()
                    .1,
            ),
            false => SourceNeuron::Internal(
                &neurons
                    .internal()
                    .iter()
                    .filter(|(id, _)| *id == gene.source_id())
                    .collect::<Vec<&(u8, InternalNeuron)>>()
                    .first()
                    .unwrap()
                    .1,
            ),
        };

        let destination_is_action_neuron = gene.destination_id() < 128;
        let destination_neuron: DestinationNeuron = match destination_is_action_neuron {
            true => DestinationNeuron::Action(
                &neurons
                    .action()
                    .iter()
                    .filter(|(id, _)| *id == gene.destination_id())
                    .collect::<Vec<&(u8, ActionNeuron)>>()
                    .first()
                    .unwrap()
                    .1,
            ),
            false => DestinationNeuron::Internal(
                &neurons
                    .internal()
                    .iter()
                    .filter(|(id, _)| *id == gene.destination_id())
                    .collect::<Vec<&(u8, InternalNeuron)>>()
                    .first()
                    .unwrap()
                    .1,
            ),
        };

        let connection = Connection::new(source_neuron, destination_neuron, gene.weight());

        connections.push(connection);
    }

    connections
}

/// Creates all of the neurons required to build every connection specified in a [Genome].
fn create_neurons(genome: &Genome) -> Neurons {
    let mut neurons = Neurons::new();

    for gene in genome {
        // Source id
        let source_is_sensory_neuron = gene.source_id() < 128;

        if source_is_sensory_neuron {
            let source_id = gene.source_id() % 12;

            if neurons.sensory().iter().any(|(id, _)| *id == source_id) {
                continue;
            }

            let neuron = match_sensory_neuron_id(source_id);

            neurons.push_sensory(neuron);
        } else {
            create_internal_neuron(gene.source_id(), &mut neurons);
        }

        // Destination id
        let destination_is_action_neuron = gene.destination_id() < 128;

        if destination_is_action_neuron {
            let destination_id = gene.destination_id() % 2;

            if neurons.action().iter().any(|(id, _)| *id == destination_id) {
                continue;
            }

            let neuron = match_action_neuron_id(destination_id);

            neurons.push_action(neuron);
        } else {
            create_internal_neuron(gene.destination_id(), &mut neurons);
        }
    }

    neurons
}

/// Matches a sensory neuron id to the correct variant of [SensoryNeuron].
fn match_sensory_neuron_id(source_id: u8) -> (u8, SensoryNeuron) {
    match source_id {
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
    }
}

/// Matches an action neuron id to the correct variant of [ActionNeuron].
fn match_action_neuron_id(destination_id: u8) -> (u8, ActionNeuron) {
    match destination_id {
        0 => (destination_id, ActionNeuron::Acceleration),
        1 => (destination_id, ActionNeuron::AngularAcceleration),
        _ => unreachable!(),
    }
}

/// Creates an [InternalNeuron].
fn create_internal_neuron(neuron_id: u8, neurons: &mut Neurons) {
    let neuron_id = (neuron_id - 128) % MAX_INTERNAL_NEURONS;

    if neurons.internal().iter().any(|(id, _)| *id == neuron_id) {
        return;
    }

    let neuron = (neuron_id, InternalNeuron);

    neurons.push_internal(neuron);
}
