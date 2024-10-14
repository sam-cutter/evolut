pub mod connection;
pub mod neuron;

use std::rc::Rc;

use super::genome::{Gene, Genome};
use crate::simulation::MAX_INTERNAL_NEURONS;
use connection::{Connection, InputNeuron};
use neuron::{ActionNeuron, InternalNeuron, Neuron, SensoryNeuron};

pub struct Brain {
    neurons: Vec<Neuron>,
}

impl Brain {
    pub fn new(genome: &Genome) -> Self {
        let mut working_genome: Vec<Option<Gene>> = genome
            .iter()
            .map(|gene| {
                let source_is_sensory_neuron = gene.source_id() < 128;

                let source_id = if source_is_sensory_neuron {
                    calculate_sensory_neuron_id(gene.source_id())
                } else {
                    calculate_internal_neuron_id(gene.source_id())
                };

                let destination_is_action_neuron = gene.destination_id() < 128;

                let destination_id = if destination_is_action_neuron {
                    calculate_action_neuron_id(gene.destination_id())
                } else {
                    calculate_internal_neuron_id(gene.destination_id())
                };

                Some(Gene::new(source_id, destination_id, gene.weight()))
            })
            .collect();

        let mut working_neurons: Vec<(u8, Neuron)> = Vec::new();

        let mut gene_index = 0;

        while gene_index < working_genome.len() {
            if working_genome[gene_index]
                .as_ref()
                .is_some_and(|g| g.destination_id() < 128)
            {
                let neuron_id = working_genome[gene_index]
                    .as_ref()
                    .unwrap()
                    .destination_id();

                let mut visited_neurons = vec![neuron_id];

                let input = build_tree(
                    neuron_id,
                    &mut working_genome,
                    &mut working_neurons,
                    &mut visited_neurons,
                );

                if let Some(neuron) = input {
                    match neuron {
                        Neuron::Action(_) => working_neurons.push((neuron_id, neuron)),
                        _ => unreachable!(),
                    }
                }
            }

            gene_index += 1;
        }

        let neurons: Vec<Neuron> = working_neurons
            .into_iter()
            .map(|(_, neuron)| neuron)
            .collect();

        Self { neurons }
    }

    pub fn neurons(&self) -> &Vec<Neuron> {
        &self.neurons
    }
}

fn build_tree(
    neuron_id: u8,
    working_genome: &mut Vec<Option<Gene>>,
    working_neurons: &mut Vec<(u8, Neuron)>,
    visited_neurons: &mut Vec<u8>,
) -> Option<Neuron> {
    let mut inputs: Vec<Connection> = Vec::new();

    let mut gene_index = 0;

    while gene_index < working_genome.len() {
        if !working_genome[gene_index]
            .as_ref()
            .is_some_and(|g| g.destination_id() == neuron_id)
        {
            gene_index += 1;
            continue;
        }

        let source_id = working_genome[gene_index].as_ref().unwrap().source_id();
        let weight = working_genome[gene_index].as_ref().unwrap().weight();

        let source_is_sensory_neuron = source_id < 128;
        let source_is_internal_neuron = !source_is_sensory_neuron;

        working_genome[gene_index] = None;

        let mut source_neuron_search = working_neurons.iter().filter(|(id, _)| *id == source_id);

        if let Some((_, source_neuron)) = source_neuron_search.next() {
            let input = Connection::new(
                match source_neuron {
                    Neuron::Sensory(sensory_neuron) => {
                        InputNeuron::Sensory(Rc::clone(sensory_neuron))
                    }
                    Neuron::Internal(internal_neuron) => {
                        InputNeuron::Internal(Rc::clone(internal_neuron))
                    }
                    Neuron::Action(_) => unreachable!(),
                },
                weight,
            );

            inputs.push(input);
        } else if source_neuron_search.next().is_none() && source_is_sensory_neuron {
            let sensory_neuron = Rc::new(SensoryNeuron::new(source_id));

            working_neurons.push((source_id, Neuron::Sensory(Rc::clone(&sensory_neuron))));

            let input = Connection::new(InputNeuron::Sensory(Rc::clone(&sensory_neuron)), weight);

            inputs.push(input);
        } else if source_neuron_search.next().is_none() && source_is_internal_neuron {
            if visited_neurons.contains(&source_id) {
                continue;
            }

            visited_neurons.push(source_id);

            let neuron = build_tree(source_id, working_genome, working_neurons, visited_neurons);

            if let Some(neuron) = neuron {
                match neuron {
                    Neuron::Internal(internal_neuron) => {
                        let input = Connection::new(
                            InputNeuron::Internal(Rc::clone(&internal_neuron)),
                            weight,
                        );

                        working_neurons.push((source_id, Neuron::Internal(internal_neuron)));

                        inputs.push(input);
                    }
                    _ => unreachable!(),
                }
            }

            visited_neurons.pop();
        }

        gene_index += 1;
    }

    if inputs.len() == 0 {
        return None;
    }

    let neuron_is_action_neuron = neuron_id < 128;

    if neuron_is_action_neuron {
        return Some(Neuron::Action(Rc::new(ActionNeuron::new(
            neuron_id, inputs,
        ))));
    } else {
        return Some(Neuron::Internal(Rc::new(InternalNeuron::new(inputs))));
    }
}

fn calculate_sensory_neuron_id(source_id: u8) -> u8 {
    source_id % 12
}

fn calculate_action_neuron_id(destination_id: u8) -> u8 {
    destination_id % 2
}

fn calculate_internal_neuron_id(id: u8) -> u8 {
    (id - 128) % MAX_INTERNAL_NEURONS + 128
}
