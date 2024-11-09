use bevy::prelude::*;
use rand;

use evolut::{
    creature::{
        brain::Brain,
        genome::{Gene, Genome},
    },
    simulation::{GENERATION_ZERO_SIZE, MAX_GENES},
};

fn main() {
    App::new()
        .add_systems(Startup, spawn_generation_zero)
        .add_systems(Update, print_brain_sizes)
        .run();

    /*
    TODO:
    We need some way to compute each brain. I think that there needs to be one system which queries for brains. This will compute the
    outputs of the action neurons, and update the creature's velocity and rotational velocity. When calculating the outputs of the neurons,
    the order in which the neurons are stored in the brain is perfect: we can iterate over each neuron, and calculate its output. Maybe
    we store outputs in another array, where the indexes match up to the indexes of the neurons array in the brain.
     */
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

fn print_brain_sizes(query: Query<&Brain>) {
    for brain in &query {
        println!("{}", brain.neurons().len());
    }
}
