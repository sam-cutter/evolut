mod vision;

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;
use std::{collections::HashMap, sync::Arc, time::Duration};

use super::*;
use crate::model::creature::{
    brain::{ActionOutput, Activation, Brain, InternalNeuron, Neuron, SensoryInputs},
    genome::Genome,
};

#[derive(Bundle)]
pub struct CreatureBundle {
    pub mesh: Mesh2d,
    pub mesh_material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub visibility: Visibility,
    pub velocity: Velocity,
    pub angular_velocity: AngularVelocity,
    pub energy: Energy,
    pub brain: Brain,
    pub genome: Genome,
    pub age: Age,
}

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_generation_zero);

        app.add_systems(
            FixedUpdate,
            (
                deduct_energy,
                kill_creatures,
                update_ages,
                update_translations,
                update_rotations,
            )
                .chain(),
        );

        app.add_systems(
            Update,
            execute_creature_decisions.run_if(on_timer(Duration::from_secs_f64(
                1.0 / BRAIN_UPDATE_FREQUENCY,
            ))),
        );
    }
}

fn spawn_generation_zero(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut generator = rand::thread_rng();

    for _ in 0..GENERATION_ZERO_SIZE {
        let circle = meshes.add(Circle::new(1.0));

        let genome = Genome::random(GENOME_LENGTH);
        let brain = Brain::new(&genome);

        commands.spawn(CreatureBundle {
            mesh: Mesh2d(circle),
            mesh_material: MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0))),
            transform: Transform {
                translation: Vec3::new(
                    generator.gen_range(-50.0..=50.0),
                    generator.gen_range(-50.0..=50.0),
                    0.0,
                ),
                ..default()
            },
            visibility: Visibility::Visible,
            velocity: Velocity {
                value: Vec2::default(),
            },
            angular_velocity: AngularVelocity { value: 0.0 },
            energy: Energy {
                value: INITIAL_ENERGY,
            },
            brain,
            genome,
            age: Age { value: 0.0 },
        });
    }
}

fn execute_creature_decisions(
    mut query: Query<(
        &Brain,
        &Transform,
        &mut Velocity,
        &mut AngularVelocity,
        &Energy,
        &Age,
    )>,
    transform_query: Query<&Transform, With<Brain>>,
) {
    let spatial_index = vision::build_spatial_index(transform_query);

    for (brain, transform, mut velocity, mut angular_velocity, energy, age) in &mut query {
        let mut internal_activation_cache: HashMap<Arc<InternalNeuron>, f32> = HashMap::new();

        let lines_of_sight = vision::compute_vision(transform, &spatial_index);

        let sensory_inputs = SensoryInputs {
            age: age.value,
            speed: velocity.value.length(),
            angular_velocity: angular_velocity.value,
            lines_of_sight,
            stored_energy: energy.value,
        };

        for action_neuron in brain.neurons().iter().filter_map(|neuron| match neuron {
            Neuron::Action(action_neuron) => Some(action_neuron),
            _ => None,
        }) {
            let activation =
                action_neuron.activation(&mut internal_activation_cache, &sensory_inputs);

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

fn update_ages(mut query: Query<&mut Age>, time: Res<Time<Fixed>>) {
    for mut age in &mut query {
        age.value += time.delta_secs();
    }
}

fn deduct_energy(
    mut query: Query<(&mut Energy, &Velocity, &AngularVelocity)>,
    time: Res<Time<Fixed>>,
) {
    for (mut energy, velocity, angular_velocity) in &mut query {
        // TODO: export constants for multipliers of the different terms in this function, fine tune.
        energy.value -=
            (10.0 + velocity.value.length() + angular_velocity.value.abs()) * time.delta_secs();
    }
}

fn kill_creatures(query: Query<(&Energy, Entity)>, mut commands: Commands) {
    for (energy, entity) in &query {
        if energy.value <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
pub struct Energy {
    pub value: f32,
}

#[derive(Component)]
pub struct Age {
    pub value: f32,
}
