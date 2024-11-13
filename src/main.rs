use std::{collections::HashMap, sync::Arc};

use bevy::prelude::*;
use rand;

use evolut::{
    creature::{
        brain::{ActionOutput, Activation, Brain, InternalNeuron, Neuron},
        genome::{Gene, Genome},
    },
    simulation::{GENERATION_ZERO_SIZE, MAX_GENES},
};

fn main() {
    App::new()
        .add_systems(Startup, spawn_generation_zero)
        .add_systems(Update, compute_output_neurons)
        .run();
}

fn spawn_generation_zero(mut commands: Commands) {
    for _ in 0..GENERATION_ZERO_SIZE {
        let mut genes: Vec<Gene> = Vec::new();

        for _ in 0..MAX_GENES {
            let gene = Gene::new(rand::random(), rand::random(), rand::random());
            genes.push(gene);
        }

        let genome = Genome::new(genes);
        let brain = Brain::new(&genome);

        commands.spawn((brain, genome));
    }
}

fn compute_output_neurons(query: Query<&Brain>) {
    for brain in &query {
        let mut internal_activation_cache: HashMap<Arc<InternalNeuron>, f64> = HashMap::new();

        for action_neuron in brain
            .neurons()
            .iter()
            .filter(|n| match n {
                Neuron::Action(_) => true,
                _ => false,
            })
            .map(|n| {
                if let Neuron::Action(an) = n {
                    an
                } else {
                    unreachable!()
                }
            })
        {
            let activation = action_neuron.activation(&mut internal_activation_cache);

            match action_neuron.output() {
                ActionOutput::Acceleration => println!("accelerated by {}", activation),
                ActionOutput::AngularAcceleration => {
                    println!("angularly accelerated by {}", activation)
                }
            }
        }
    }
}

fn print_brain_sizes(query: Query<&Brain>) {
    for brain in &query {
        println!("{}", brain.neurons().len());
    }
}
