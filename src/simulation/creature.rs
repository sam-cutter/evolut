use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;
use std::{
    collections::HashMap,
    f32::consts::{E, PI},
    sync::Arc,
    time::Duration,
};

use super::{
    Age, AngularVelocity, Energy, Velocity, BRAIN_UPDATE_FREQUENCY, GENERATION_ZERO_SIZE,
    GENOME_LENGTH, INITIAL_ENERGY,
};
use crate::model::creature::{
    brain::{ActionOutput, Activation, Brain, InternalNeuron, LinesOfSight, Neuron, SensoryInputs},
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
                update_ages,
                update_translations,
                update_rotations,
                deduct_energy,
                kill_creatures,
            )
                .chain(),
        );

        app.add_systems(
            Update,
            execute_creature_decisions.run_if(on_timer(Duration::from_secs_f64(
                1. / BRAIN_UPDATE_FREQUENCY,
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
        let circle = meshes.add(Circle::new(1.));

        let genome = Genome::random(GENOME_LENGTH);
        let brain = Brain::new(&genome);

        commands.spawn(CreatureBundle {
            mesh: Mesh2d(circle),
            mesh_material: MeshMaterial2d(materials.add(Color::linear_rgb(0., 1., 0.))),
            transform: Transform {
                translation: Vec3::new(
                    generator.gen_range(-50.0..=50.0),
                    generator.gen_range(-50.0..=50.0),
                    0.,
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
            age: Age { value: 0. },
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
) {
    // MUST DO: build the query for the transforms
    // let spatial_index = build_spatial_index(todo!());

    for (brain, transform, mut velocity, mut angular_velocity, energy, age) in &mut query {
        let mut internal_activation_cache: HashMap<Arc<InternalNeuron>, f32> = HashMap::new();

        // MUST DO: compute lines of sight
        // let lines_of_sight = compute_vision(transform, &spatial_index);

        let lines_of_sight = LinesOfSight { ..default() };

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
            (10. + velocity.value.length() + angular_velocity.value.abs()) * time.delta_secs();
    }
}

fn kill_creatures(query: Query<(&Energy, Entity)>, mut commands: Commands) {
    for (energy, entity) in &query {
        if energy.value < 0. {
            commands.entity(entity).despawn();
        }
    }
}

enum EyeAngle {
    Left,
    Middle,
    Right,
}

const EYE_ANGLES: [(EyeAngle, f32); 3] = [
    (EyeAngle::Left, PI / 4.0),
    (EyeAngle::Middle, 0.0),
    (EyeAngle::Right, PI * 7.0 / 4.0),
];

pub fn compute_vision(
    transform: &Transform,
    spatial_index: &HashMap<(i32, i32), Vec<(f32, f32)>>,
) -> LinesOfSight {
    let mut lines_of_sight = LinesOfSight { ..default() };

    let (x, y) = (transform.translation.x, transform.translation.y);

    let (cell_x, cell_y) = get_cell_coordinates(x, y);

    // This forms a 3 by 3 grid, centred at the current cell.
    let cells_to_search: Vec<(i32, i32)> = vec![
        (cell_x, cell_y),
        (cell_x - 5, cell_y),
        (cell_x + 5, cell_y),
        (cell_x, cell_y - 5),
        (cell_x, cell_y + 5),
        (cell_x - 5, cell_y - 5),
        (cell_x + 5, cell_y - 5),
        (cell_x - 5, cell_y + 5),
        (cell_x + 5, cell_y + 5),
    ];

    for cell in cells_to_search {
        let creatures = spatial_index.get(&cell);

        if let Some(creatures) = creatures {
            for creature in creatures {
                let (creature_x, creature_y) = (creature.0, creature.1);

                if creature_x == x && creature_y == y {
                    continue;
                }

                println!("Creature located at ({creature_x}, {creature_y})");

                for eye_angle in EYE_ANGLES {
                    println!("Eye: {} degrees", eye_angle.1.to_degrees());

                    let global_eye_angle =
                        eye_angle.1 + transform.rotation.to_euler(EulerRot::XYZ).2;

                    let eyeline_is_vertical =
                        (global_eye_angle.abs() % PI - PI / 2.0).abs() < f32::EPSILON;

                    let mut intersection_coordinates: Vec<(f32, f32)> = Vec::new();

                    if eyeline_is_vertical {
                        // The intersection of a vertical eyeline and the creature can be re-arranged into a quadratic in the form ay^2 + by + c
                        let a = 1.0;
                        let b = -2.0 * creature_y;
                        let c = creature_x.powi(2) + creature_y.powi(2) + x.powi(2)
                            - 2.0 * creature_x * x
                            - 1.0;

                        let discriminant = b.powi(2) - 4.0 * a * c;

                        let mut y_values: Vec<f32> = Vec::new();

                        if discriminant < 0.0 {
                            continue;
                        } else if discriminant.abs() < 1e-6 {
                            y_values.push(-b / (2.0 * a));
                        } else if discriminant > 0.0 {
                            y_values.push((-b + discriminant.sqrt()) / (2.0 * a));
                            y_values.push((-b - discriminant.sqrt()) / (2.0 * a));
                        }

                        intersection_coordinates
                            .extend(y_values.iter().map(|intersection_y| (x, *intersection_y)));
                    } else {
                        let eyeline_gradient = global_eye_angle.tan();
                        let eyeline_y_intercept =
                            transform.translation.y - eyeline_gradient * transform.translation.x;

                        // The intersection of the eyeline and the creature can be re-arranged into a quadratic in the form ax^2 + bx + c
                        let a = eyeline_gradient.powi(2) + 1.0;
                        let b = 2.0 * eyeline_gradient * eyeline_y_intercept
                            - 2.0 * eyeline_gradient * creature_y
                            - 2.0 * creature_x;
                        let c =
                            creature_x.powi(2) + creature_y.powi(2) + eyeline_y_intercept.powi(2)
                                - 2.0 * creature_y * eyeline_y_intercept
                                - 1.0;

                        let discriminant = b.powi(2) - 4.0 * a * c;

                        let mut x_values: Vec<f32> = Vec::new();

                        if discriminant < 0.0 {
                            continue;
                        } else if discriminant.abs() < 1e-6 {
                            x_values.push(-b / (2.0 * a));
                        } else if discriminant > 0.0 {
                            x_values.push((-b + discriminant.sqrt()) / (2.0 * a));
                            x_values.push((-b - discriminant.sqrt()) / (2.0 * a));
                        }

                        intersection_coordinates.extend(x_values.iter().map(|intersection_x| {
                            (
                                *intersection_x,
                                eyeline_gradient * intersection_x + eyeline_y_intercept,
                            )
                        }));
                    }

                    for intersection in intersection_coordinates {
                        let intersection_x = intersection.0;
                        let intersection_y = intersection.1;

                        print!("Intersection at ({intersection_x}, {intersection_y}): ");

                        let delta_x = intersection_x - x;
                        let delta_y = intersection_y - y;

                        let creature_vector = Vec2::new(delta_x, delta_y);
                        let eyeline_vector =
                            Vec2::new(global_eye_angle.cos(), global_eye_angle.sin());

                        if creature_vector.dot(eyeline_vector) <= 0.0 {
                            println!("was not in the right direction.");
                            continue;
                        }

                        let distance = (delta_x.powi(2) + delta_y.powi(2)).sqrt();

                        if distance > 5.0 {
                            println!("was too far away.");
                            continue;
                        }

                        let new_eye_value = E.powf(-0.5 * distance);

                        let eye_value = match eye_angle.0 {
                            EyeAngle::Left => &mut lines_of_sight.left_creature,
                            EyeAngle::Middle => &mut lines_of_sight.middle_creature,
                            EyeAngle::Right => &mut lines_of_sight.right_creature,
                        };

                        if new_eye_value > *eye_value {
                            *eye_value = new_eye_value
                        }

                        println!();
                    }
                }
            }
        }
    }

    return lines_of_sight;
}

fn build_spatial_index(query: Query<&Transform>) -> HashMap<(i32, i32), Vec<(f32, f32)>> {
    let mut spatial_index: HashMap<(i32, i32), Vec<(f32, f32)>> = HashMap::new();

    for transform in &query {
        let (x, y) = (transform.translation.x, transform.translation.y);

        let cell_coordinates = get_cell_coordinates(x, y);

        spatial_index
            .entry(cell_coordinates)
            .and_modify(|cell| cell.push((x, y)))
            .or_insert(vec![(x, y)]);
    }

    spatial_index
}

pub fn get_cell_coordinates(x: f32, y: f32) -> (i32, i32) {
    ((x - x.rem_euclid(5.)) as i32, (y - y.rem_euclid(5.)) as i32)
}
