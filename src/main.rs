use std::{collections::HashMap, sync::Arc};

use bevy::prelude::*;

use evolut::{
    creature::{
        brain::{ActionOutput, Activation, Brain, InternalNeuron, Neuron},
        genome::Genome,
    },
    simulation::{AngularVelocity, Velocity, GENERATION_ZERO_SIZE, GENOME_LENGTH},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_generation_zero)
        .add_systems(
            Update,
            (
                execute_creature_decisions,
                update_translations,
                update_rotations,
            ),
        )
        .run();
}

fn spawn_generation_zero(mut commands: Commands) {
    for _ in 0..GENERATION_ZERO_SIZE {
        let genome = Genome::random(GENOME_LENGTH);
        let brain = Brain::new(&genome);

        commands.spawn((
            SpatialBundle::default(),
            Velocity {
                value: Vec2::default(),
            },
            AngularVelocity { value: 0.0 },
            brain,
            genome,
        ));
    }
}

fn execute_creature_decisions(
    mut query: Query<(&Brain, &Transform, &mut Velocity, &mut AngularVelocity)>,
) {
    for (brain, transform, mut velocity, mut angular_velocity) in &mut query {
        let mut internal_activation_cache: HashMap<Arc<InternalNeuron>, f32> = HashMap::new();

        for action_neuron in brain.neurons().iter().filter_map(|neuron| match neuron {
            Neuron::Action(action_neuron) => Some(action_neuron),
            _ => None,
        }) {
            let activation = action_neuron.activation(&mut internal_activation_cache);

            match action_neuron.output() {
                ActionOutput::Acceleration => {
                    let angle = transform.rotation.to_euler(EulerRot::XYZ).2;

                    let acceleration =
                        Vec2::new(activation * angle.cos(), activation * angle.sin());

                    velocity.value += acceleration;

                    println!("accelerated by {}", activation);
                }
                ActionOutput::AngularAcceleration => {
                    angular_velocity.value += activation;
                    println!("angularly accelerated by {}", activation);
                }
            }
        }
    }
}

fn update_translations(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        println!("updated translation");
        transform.translation += velocity.value.extend(0.0) * time.delta_seconds();
    }
}

fn update_rotations(mut query: Query<(&mut Transform, &AngularVelocity)>, time: Res<Time>) {
    for (mut transform, angular_velocity) in &mut query {
        println!("updated rotation");
        transform.rotate_z(angular_velocity.value * time.delta_seconds());
    }
}

fn _print_brain_sizes(query: Query<&Brain>) {
    for brain in &query {
        println!("{}", brain.neurons().len());
    }
}

/*
How is this going to work? I think that I am going to have one system which updates positions and orientations every frame,
and one system which both computes output neurons, and applies the updates to position and velocity.

1) The system which updates positions every frame will simply add the velocity multiplied by the time since the previous update,
to the creature's position. The orientation update may be more difficult. I am going to have to take the angular velocity,
and somehow update the creature's orientation.

2) All updates that take place in the second system will happen while the creature's position and orientation are frozen.
First, the velocity will be updated using the acceleration output neuron, and the current creature orientation.
Then, the angular velocity will be updated.
 */
