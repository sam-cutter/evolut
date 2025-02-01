//! Houses all code related to a creature's brain.

mod connection;
mod neuron;

use bevy::prelude::Component;
use std::sync::Arc;

use super::genome::{Gene, Genome};
use crate::simulation::MAX_INTERNAL_NEURONS;
pub use connection::{Connection, InputNeuron};
pub use neuron::Activation;
pub use neuron::{
    ActionNeuron, ActionOutput, InternalNeuron, LinesOfSight, Neuron, SensoryInputs, SensoryNeuron,
};

/// A collection of neurons.
///
/// The brain is a neural network, where the sensory neurons are the inputs to the network, and the action neurons
/// are the outputs, directly modifying the behaviour of the creature.

#[derive(Component)]
pub struct Brain {
    neurons: Vec<Neuron>,
}

impl Brain {
    /// Builds a new brain from a genome.
    pub fn new(genome: &Genome) -> Self {
        // Build the working genome
        let mut working_genome: Vec<Option<Gene>> = genome
            .genes()
            .iter()
            .map(|gene| {
                // Calculating new source/destination ids is essential in order to know whether two neurons are the same.

                let source_is_sensory_neuron = gene.source_id() < 128;

                // Calculate the global source id
                let source_id = if source_is_sensory_neuron {
                    calculate_sensory_neuron_id(gene.source_id())
                } else {
                    calculate_internal_neuron_id(gene.source_id())
                };

                let destination_is_action_neuron = gene.destination_id() < 128;

                // Calculate the global destination id
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
            // TODO: update this logic to use a while let pattern, then the un-wrappings can be removed.

            if working_genome[gene_index]
                .as_ref()
                .is_some_and(|g| g.destination_id() < 128)
            {
                // For each action neuron, build its tree

                let neuron_id = working_genome[gene_index]
                    .as_ref()
                    .unwrap()
                    .destination_id();

                let mut visited_neurons = vec![neuron_id];

                let action_neuron = build_tree(
                    neuron_id,
                    &mut working_genome,
                    &mut working_neurons,
                    &mut visited_neurons,
                );

                if let Some(neuron) = action_neuron {
                    match neuron {
                        Neuron::Action(_) => working_neurons.push((neuron_id, neuron)),
                        // TODO: update this to use a nested if let statement
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

    /// Returns a reference to its neurons.
    pub fn neurons(&self) -> &Vec<Neuron> {
        &self.neurons
    }
}

/* Dear my very confused future self.

This function doesn't seem like it should work, but it does.

The first place where you're likely to be confused is the visited_neurons.contains(&source_id) condition. This seems to be in the wrong place.
It is not. It seems like we should be checking this in the branch of the control flow which runs if the source_neuron_search yielded something.
However, it would be useless here: assuming that the neuron we found was built correctly, it cannot possibly be part of any cycles;
in the neuron's creation, any cycles would have been detected and genes appropriately discarded.

The next thing that may confuse you is the fact that the function still seems to work fine, even if the visited_neurons.contains(&source_id)
condition is removed. Here's why. While it's impossible for the same gene to be built multiple times, it is possible for the same neuron
to be built more than once. This can happen when a cycle occurs (but we don't want that to happen!).
Let's assume that we are currently building internal neuron A. The first of the genes relating to internal neuron A actually forms
a self-referencing connection. Seeing as internal neuron A has not yet been built (a neuron is not added to the working_neurons list
until all of the trees of its inputs have been built), we will move to the following branch when evaluating the gene which connects A to itself.
    else if source_neuron_search.next().is_none() && source_is_internal_neuron
Let's now imagine that the visited_neurons.contains(&source_id) condition was removed. We would call upon the build_tree function again,
and we would once again be at internal neuron A (I will call this A' from now on). All of the genes relating to internal neuron A
would instead be built for A', and there would now be two of internal neuron A. You can see how, in reality, no cycles or loops
will ever be built (***the key thing to remember is that a neuron is not built until all of its input trees are built, therefore it is impossible
for a neuron to ever actually reference itself***), neurons will just be pulled apart into multiple copies.

This logic extends to if there are longer cycles.

Intended structure:
= A
-> A (IGNORE)
-> B
-> C

Structure which occurs when the condition is removed:
= A
-> A'
    -> B
    -> C

Please believe me, and don't try to understand this ever again.

Sam, quarter past 11 on a Saturday night :)
*/

fn build_tree(
    neuron_id: u8, // The id of the neuron whose tree is to be built
    working_genome: &mut Vec<Option<Gene>>, // The list of genes which have not already been used/discarded
    working_neurons: &mut Vec<(u8, Neuron)>, // The list of neurons whose trees have been built
    visited_neurons: &mut Vec<u8>,          // The ids of the neurons who have already been visited
) -> Option<Neuron> {
    // The list of connection inputs for the current neuron
    let mut inputs: Vec<Connection> = Vec::new();

    let mut gene_index = 0;

    while gene_index < working_genome.len() {
        // TODO: update this logic to use a while let pattern, then the un-wrappings can be removed.

        // Ignore any genes whose destination ids aren't the current neuron id
        if !working_genome[gene_index]
            .as_ref()
            .is_some_and(|g| g.destination_id() == neuron_id)
        {
            gene_index += 1;
            continue;
        }

        // Get the source id and weight of the gene
        let source_id = working_genome[gene_index].as_ref().unwrap().source_id();
        let weight = working_genome[gene_index].as_ref().unwrap().weight();

        // If the most significant bit of the source id is 0 (i.e. the source id is less than 128), the source is a sensory neuron
        let source_is_sensory_neuron = source_id < 128;
        let source_is_internal_neuron = !source_is_sensory_neuron;

        // Mark the gene as used, so it won't be built again
        working_genome[gene_index] = None;

        // See whether the source neuron has already been created (i.e. its tree has already been built)
        let mut source_neuron_search = working_neurons.iter().filter(|(id, neuron)| {
            *id == source_id
                && if let Neuron::Action(_) = neuron {
                    false
                } else {
                    true
                }
        });

        if let Some((_, source_neuron)) = source_neuron_search.next() {
            // If the source neuron has already been created, create a new connection and add it to the list of inputs
            let input = Connection::new(
                match source_neuron {
                    Neuron::Sensory(sensory_neuron) => {
                        InputNeuron::Sensory(Arc::clone(sensory_neuron))
                    }
                    Neuron::Internal(internal_neuron) => {
                        InputNeuron::Internal(Arc::clone(internal_neuron))
                    }
                    Neuron::Action(_) => unreachable!(),
                },
                weight,
            );

            inputs.push(input);
        } else if source_neuron_search.next().is_none() && source_is_sensory_neuron {
            // If the source neuron hasn't yet been created, and the source is a sensory neuron, create it
            let sensory_neuron = Arc::new(SensoryNeuron::new(source_id));

            // Add the sensory neuron to the list of neurons whose trees have been built
            working_neurons.push((source_id, Neuron::Sensory(Arc::clone(&sensory_neuron))));

            // Create a connection to this new neuron and add it to the list of inputs
            let input = Connection::new(InputNeuron::Sensory(Arc::clone(&sensory_neuron)), weight);

            inputs.push(input);
        } else if source_neuron_search.next().is_none() && source_is_internal_neuron {
            if visited_neurons.contains(&source_id) {
                // If the source neuron has already been visited while building an upstream tree, this means that the genome
                // is coding for a loop or cycle. I am not allowing this, so we should discard this gene.
                continue;
            }

            // Mark the source neuron as visited
            visited_neurons.push(source_id);

            // Build the tree of the source neuron
            let neuron = build_tree(source_id, working_genome, working_neurons, visited_neurons);

            if let Some(neuron) = neuron {
                // If the source neuron was actually created (i.e. a valid tree could be built),
                // create a connection sourcing the returned neuron. Since, at this point, we know that we are working with an internal
                // neuron, we know that something has gone wrong if anything else is returned.
                match neuron {
                    Neuron::Internal(internal_neuron) => {
                        working_neurons
                            .push((source_id, Neuron::Internal(Arc::clone(&internal_neuron))));

                        let input = Connection::new(
                            InputNeuron::Internal(Arc::clone(&internal_neuron)),
                            weight,
                        );

                        inputs.push(input);
                    }
                    // TODO: change this to use if let somehow
                    _ => unreachable!(),
                }
            }

            // Remove this neuron from the visited neurons stack
            visited_neurons.pop();
        }

        gene_index += 1;
    }

    if inputs.len() == 0 {
        return None;
    }

    let neuron_is_action_neuron = neuron_id < 128;

    if neuron_is_action_neuron {
        return Some(Neuron::Action(Arc::new(ActionNeuron::new(
            neuron_id, inputs,
        ))));
    } else {
        return Some(Neuron::Internal(Arc::new(InternalNeuron::new(inputs))));
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
