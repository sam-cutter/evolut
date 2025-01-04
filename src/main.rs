use std::{collections::HashMap, sync::Arc};

use bevy::prelude::*;

use evolut::{
    creature::{
        brain::{ActionOutput, Activation, Brain, InternalNeuron, Neuron},
        genome::Genome,
    },
    simulation::{GENERATION_ZERO_SIZE, GENOME_LENGTH},
};

fn main() {
    App::new()
        .add_systems(Startup, spawn_generation_zero)
        .add_systems(Update, compute_output_neurons)
        .run();
}

fn spawn_generation_zero(mut commands: Commands) {
    for _ in 0..GENERATION_ZERO_SIZE {
        let genome = Genome::random(GENOME_LENGTH);
        let brain = Brain::new(&genome);

        commands.spawn((brain, genome));
    }
}

fn compute_output_neurons(query: Query<&Brain>) {
    for brain in &query {
        let mut internal_activation_cache: HashMap<Arc<InternalNeuron>, f64> = HashMap::new();

        for action_neuron in brain.neurons().iter().filter_map(|neuron| match neuron {
            Neuron::Action(action_neuron) => Some(action_neuron),
            _ => None,
        }) {
            let activation = action_neuron.activation(&mut internal_activation_cache);

            match action_neuron.output() {
                ActionOutput::Acceleration => {
                    println!("accelerated by {}", activation)
                }
                ActionOutput::AngularAcceleration => {
                    println!("angularly accelerated by {}", activation)
                }
            }
        }
    }
}

fn _print_brain_sizes(query: Query<&Brain>) {
    for brain in &query {
        println!("{}", brain.neurons().len());
    }
}
