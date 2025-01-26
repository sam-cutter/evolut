use bevy::{prelude::*, time::common_conditions::on_timer};
use std::{collections::HashMap, sync::Arc, time::Duration};

use super::{
    AngularVelocity, Velocity, BRAIN_UPDATE_FREQUENCY, GENERATION_ZERO_SIZE, GENOME_LENGTH,
};
use crate::model::creature::{
    brain::{ActionOutput, Activation, Brain, InternalNeuron, Neuron},
    genome::Genome,
};

#[derive(Bundle)]
pub struct CreatureBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub visibility: Visibility,
    pub velocity: Velocity,
    pub angular_velocity: AngularVelocity,
    pub brain: Brain,
    pub genome: Genome,
}

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_generation_zero);

        app.add_systems(FixedUpdate, (update_translations, update_rotations));

        app.add_systems(
            Update,
            execute_creature_decisions.run_if(on_timer(Duration::from_secs_f64(
                1. / BRAIN_UPDATE_FREQUENCY,
            ))),
        );
    }
}

fn spawn_generation_zero(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..GENERATION_ZERO_SIZE {
        let genome = Genome::random(GENOME_LENGTH);
        let brain = Brain::new(&genome);

        commands.spawn(CreatureBundle {
            sprite: Sprite::from_image(asset_server.load("creature.png")),
            transform: Transform {
                scale: Vec3 {
                    x: 0.05,
                    y: 0.05,
                    z: 1.0,
                },
                ..default()
            },
            visibility: Visibility::Visible,
            velocity: Velocity {
                value: Vec2::default(),
            },
            angular_velocity: AngularVelocity { value: 0.0 },
            brain,
            genome,
        });
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
                }
                ActionOutput::AngularAcceleration => {
                    angular_velocity.value += activation;
                }
            }
        }
    }
}

fn update_translations(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time<Fixed>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.value.extend(0.0) * time.delta_secs();
    }
}

fn update_rotations(mut query: Query<(&mut Transform, &AngularVelocity)>, time: Res<Time<Fixed>>) {
    for (mut transform, angular_velocity) in &mut query {
        transform.rotate_z(angular_velocity.value * time.delta_secs());
    }
}
