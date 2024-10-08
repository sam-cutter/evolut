pub mod connection;
pub mod neuron;

use super::genome::Genome;
use crate::simulation::MAX_INTERNAL_NEURONS;
use connection::{Connection, DestinationNeuron, SourceNeuron};
use neuron::{ActionNeuron, InternalNeuron, LineOfSight, Neurons, SensoryNeuron};
use std::rc::Rc;

pub struct Brain {
    connections: Vec<Connection>,
    neurons: Neurons,
}

// TODO: Implement constructor
impl Brain {
    pub fn new(genome: &Genome) -> Self {
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

        Self {
            connections,
            neurons,
        }
    }

    pub fn connections(&self) -> &Vec<Connection> {
        &self.connections
    }

    pub fn neurons(&self) -> &Neurons {
        &self.neurons
    }
}

/// Creates each connection specified in the [Genome] out [Neurons].
fn create_connections<'a>(genome: &Genome, neurons: &'a Neurons) -> Vec<Connection> {
    let mut connections: Vec<Connection> = Vec::new();

    for gene in genome {
        let source_neuron: SourceNeuron;
        let source_is_sensory_neuron = gene.source_id() < 128;

        if source_is_sensory_neuron {
            source_neuron = SourceNeuron::Sensory(Rc::clone(
                &neurons
                    .sensory()
                    .iter()
                    .filter(|(id, _)| *id == calculate_sensory_neuron_id(gene.source_id()))
                    .collect::<Vec<&(u8, Rc<SensoryNeuron>)>>()
                    .first()
                    .unwrap()
                    .1,
            ));
        } else {
            source_neuron = SourceNeuron::Internal(Rc::clone(
                &neurons
                    .internal()
                    .iter()
                    .filter(|(id, _)| *id == calculate_internal_neuron_id(gene.source_id()))
                    .collect::<Vec<&(u8, Rc<InternalNeuron>)>>()
                    .first()
                    .unwrap()
                    .1,
            ));
        }

        let destination_neuron: DestinationNeuron;
        let destination_is_action_neuron = gene.destination_id() < 128;

        if destination_is_action_neuron {
            destination_neuron = DestinationNeuron::Action(Rc::clone(
                &neurons
                    .action()
                    .iter()
                    .filter(|(id, _)| *id == calculate_action_neuron_id(gene.destination_id()))
                    .collect::<Vec<&(u8, Rc<ActionNeuron>)>>()
                    .first()
                    .unwrap()
                    .1,
            ));
        } else {
            destination_neuron = DestinationNeuron::Internal(Rc::clone(
                &neurons
                    .internal()
                    .iter()
                    .filter(|(id, _)| *id == calculate_internal_neuron_id(gene.destination_id()))
                    .collect::<Vec<&(u8, Rc<InternalNeuron>)>>()
                    .first()
                    .unwrap()
                    .1,
            ));
        }

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
            let sensory_neuron_id = calculate_sensory_neuron_id(gene.source_id());

            if neurons
                .sensory()
                .iter()
                .any(|(id, _)| *id == sensory_neuron_id)
            {
                continue;
            }

            let neuron = match_sensory_neuron_id(sensory_neuron_id);

            neurons.push_sensory(neuron);
        } else {
            create_internal_neuron(gene.source_id(), &mut neurons);
        }

        // Destination id
        let destination_is_action_neuron = gene.destination_id() < 128;

        if destination_is_action_neuron {
            let action_neuron_id = calculate_action_neuron_id(gene.destination_id());

            if neurons
                .action()
                .iter()
                .any(|(id, _)| *id == action_neuron_id)
            {
                continue;
            }

            let neuron = match_action_neuron_id(action_neuron_id);

            neurons.push_action(neuron);
        } else {
            create_internal_neuron(gene.destination_id(), &mut neurons);
        }
    }

    neurons
}

/// Matches a sensory neuron id to the correct variant of [SensoryNeuron].
fn match_sensory_neuron_id(sensory_neuron_id: u8) -> (u8, Rc<SensoryNeuron>) {
    match sensory_neuron_id {
        0 => (sensory_neuron_id, Rc::new(SensoryNeuron::Age)),
        1 => (sensory_neuron_id, Rc::new(SensoryNeuron::Speed)),
        2 => (sensory_neuron_id, Rc::new(SensoryNeuron::AngularVelocity)),
        3 => (
            sensory_neuron_id,
            Rc::new(SensoryNeuron::LineOfSight(LineOfSight::LeftCreature)),
        ),
        4 => (
            sensory_neuron_id,
            Rc::new(SensoryNeuron::LineOfSight(LineOfSight::LeftFood)),
        ),
        5 => (
            sensory_neuron_id,
            Rc::new(SensoryNeuron::LineOfSight(LineOfSight::LeftObstacle)),
        ),
        6 => (
            sensory_neuron_id,
            Rc::new(SensoryNeuron::LineOfSight(LineOfSight::MiddleCreature)),
        ),
        7 => (
            sensory_neuron_id,
            Rc::new(SensoryNeuron::LineOfSight(LineOfSight::MiddleFood)),
        ),
        8 => (
            sensory_neuron_id,
            Rc::new(SensoryNeuron::LineOfSight(LineOfSight::MiddleObstacle)),
        ),
        9 => (
            sensory_neuron_id,
            Rc::new(SensoryNeuron::LineOfSight(LineOfSight::RightCreature)),
        ),
        10 => (
            sensory_neuron_id,
            Rc::new(SensoryNeuron::LineOfSight(LineOfSight::RightFood)),
        ),
        11 => (
            sensory_neuron_id,
            Rc::new(SensoryNeuron::LineOfSight(LineOfSight::RightObstacle)),
        ),
        12 => (sensory_neuron_id, Rc::new(SensoryNeuron::StoredEnergy)),
        _ => unreachable!(),
    }
}

/// Matches an action neuron id to the correct variant of [ActionNeuron].
fn match_action_neuron_id(action_neuron_id: u8) -> (u8, Rc<ActionNeuron>) {
    match action_neuron_id {
        0 => (action_neuron_id, Rc::new(ActionNeuron::Acceleration)),
        1 => (action_neuron_id, Rc::new(ActionNeuron::AngularAcceleration)),
        _ => unreachable!(),
    }
}

/// Creates an [InternalNeuron].
fn create_internal_neuron(destination_id: u8, neurons: &mut Neurons) {
    let internal_neuron_id = calculate_internal_neuron_id(destination_id);

    if neurons
        .internal()
        .iter()
        .any(|(id, _)| *id == internal_neuron_id)
    {
        return;
    }

    let neuron = (internal_neuron_id, Rc::new(InternalNeuron));

    neurons.push_internal(neuron);
}

fn calculate_sensory_neuron_id(source_id: u8) -> u8 {
    source_id % 12
}

fn calculate_action_neuron_id(destination_id: u8) -> u8 {
    destination_id % 2
}

fn calculate_internal_neuron_id(id: u8) -> u8 {
    (id - 128) % MAX_INTERNAL_NEURONS
}
