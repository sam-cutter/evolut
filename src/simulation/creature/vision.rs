use bevy::prelude::*;
use std::collections::HashMap;
use std::f32::consts::{E, PI};

use crate::model::creature::brain::{Brain, LinesOfSight};
use crate::simulation::SEEING_DISTANCE;

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
        (cell_x - SEEING_DISTANCE * 2, cell_y),
        (cell_x + SEEING_DISTANCE * 2, cell_y),
        (cell_x, cell_y - SEEING_DISTANCE * 2),
        (cell_x, cell_y + SEEING_DISTANCE * 2),
        (cell_x - SEEING_DISTANCE * 2, cell_y - SEEING_DISTANCE * 2),
        (cell_x + SEEING_DISTANCE * 2, cell_y - SEEING_DISTANCE * 2),
        (cell_x - SEEING_DISTANCE * 2, cell_y + SEEING_DISTANCE * 2),
        (cell_x + SEEING_DISTANCE * 2, cell_y + SEEING_DISTANCE * 2),
    ];

    for cell in cells_to_search {
        let creatures = spatial_index.get(&cell);

        if let Some(creatures) = creatures {
            for creature in creatures {
                let (creature_x, creature_y) = (creature.0, creature.1);

                if creature_x == x && creature_y == y {
                    continue;
                }

                for eye_angle in EYE_ANGLES {
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

                        let delta_x = intersection_x - x;
                        let delta_y = intersection_y - y;

                        let creature_vector = Vec2::new(delta_x, delta_y);
                        let eyeline_vector =
                            Vec2::new(global_eye_angle.cos(), global_eye_angle.sin());

                        if creature_vector.dot(eyeline_vector) <= 0.0 {
                            continue;
                        }

                        let distance = (delta_x.powi(2) + delta_y.powi(2)).sqrt();

                        if distance > SEEING_DISTANCE as f32 {
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
                    }
                }
            }
        }
    }

    return lines_of_sight;
}

pub fn build_spatial_index(
    query: Query<&Transform, With<Brain>>,
) -> HashMap<(i32, i32), Vec<(f32, f32)>> {
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

fn get_cell_coordinates(x: f32, y: f32) -> (i32, i32) {
    (
        (x - x.rem_euclid((SEEING_DISTANCE * 2) as f32)) as i32,
        (y - y.rem_euclid((SEEING_DISTANCE * 2) as f32)) as i32,
    )
}
