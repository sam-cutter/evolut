use bevy::prelude::*;
use std::collections::HashMap;
use std::f32::consts::{E, PI};

use crate::model::creature::brain::{Brain, LinesOfSight};
use crate::simulation::food::Food;
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

pub enum ObjectCategory {
    Creature,
    Food,
    Obstacle,
}

pub struct VisibleObject {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub category: ObjectCategory,
}

pub fn compute_vision(
    transform: &Transform,
    spatial_index: &HashMap<(i32, i32), Vec<VisibleObject>>,
) -> LinesOfSight {
    let mut lines_of_sight = LinesOfSight { ..default() };

    let (creature_x, creature_y) = (transform.translation.x, transform.translation.y);

    let (cell_x, cell_y) = get_cell_coordinates(creature_x, creature_y);

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
        let objects = spatial_index.get(&cell);

        if let Some(objects) = objects {
            for object in objects {
                if object.x == creature_x && object.y == creature_y {
                    continue;
                }

                for eye_angle in EYE_ANGLES {
                    let global_eye_angle =
                        eye_angle.1 + transform.rotation.to_euler(EulerRot::XYZ).2;

                    let eyeline_is_vertical =
                        (global_eye_angle.abs() % PI - PI / 2.0).abs() < f32::EPSILON;

                    let mut intersection_coordinates: Vec<(f32, f32)> = Vec::new();

                    if eyeline_is_vertical {
                        // The intersection of a vertical eyeline and the object can be re-arranged into a quadratic in the form ay^2 + by + c
                        let a = 1.0;
                        let b = -2.0 * object.y;
                        let c = object.x.powi(2) + object.y.powi(2) + creature_x.powi(2)
                            - 2.0 * object.x * creature_x
                            - object.radius;

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

                        intersection_coordinates.extend(
                            y_values
                                .iter()
                                .map(|intersection_y| (creature_x, *intersection_y)),
                        );
                    } else {
                        let eyeline_gradient = global_eye_angle.tan();
                        let eyeline_y_intercept =
                            transform.translation.y - eyeline_gradient * transform.translation.x;

                        // The intersection of the eyeline and the object can be re-arranged into a quadratic in the form ax^2 + bx + c
                        let a = eyeline_gradient.powi(2) + 1.0;
                        let b = 2.0 * eyeline_gradient * eyeline_y_intercept
                            - 2.0 * eyeline_gradient * object.y
                            - 2.0 * object.x;
                        let c = object.x.powi(2) + object.y.powi(2) + eyeline_y_intercept.powi(2)
                            - 2.0 * object.y * eyeline_y_intercept
                            - object.radius;

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

                        let delta_x = intersection_x - creature_x;
                        let delta_y = intersection_y - creature_y;

                        let object_vector = Vec2::new(delta_x, delta_y);
                        let eyeline_vector =
                            Vec2::new(global_eye_angle.cos(), global_eye_angle.sin());

                        if object_vector.dot(eyeline_vector) <= 0.0 {
                            continue;
                        }

                        let distance = (delta_x.powi(2) + delta_y.powi(2)).sqrt();

                        if distance > SEEING_DISTANCE as f32 {
                            continue;
                        }

                        let new_eye_value = E.powf(-0.5 * distance);

                        let eye_value = match eye_angle.0 {
                            EyeAngle::Left => match object.category {
                                ObjectCategory::Creature => &mut lines_of_sight.left_creature,
                                ObjectCategory::Food => &mut lines_of_sight.left_food,
                                ObjectCategory::Obstacle => &mut lines_of_sight.left_obstacle,
                            },
                            EyeAngle::Middle => match object.category {
                                ObjectCategory::Creature => &mut lines_of_sight.middle_creature,
                                ObjectCategory::Food => &mut lines_of_sight.middle_food,
                                ObjectCategory::Obstacle => &mut lines_of_sight.middle_obstacle,
                            },
                            EyeAngle::Right => match object.category {
                                ObjectCategory::Creature => &mut lines_of_sight.right_creature,
                                ObjectCategory::Food => &mut lines_of_sight.right_food,
                                ObjectCategory::Obstacle => &mut lines_of_sight.right_obstacle,
                            },
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
    creature_query: Query<&Transform, With<Brain>>,
    food_query: Query<&Transform, With<Food>>,
) -> HashMap<(i32, i32), Vec<VisibleObject>> {
    let mut spatial_index: HashMap<(i32, i32), Vec<VisibleObject>> = HashMap::new();

    for transform in &creature_query {
        let (creature_x, creature_y) = (transform.translation.x, transform.translation.y);

        let cell_coordinates = get_cell_coordinates(creature_x, creature_y);

        let creature = VisibleObject {
            x: creature_x,
            y: creature_y,
            radius: 1.0,
            category: ObjectCategory::Creature,
        };

        match spatial_index.get_mut(&cell_coordinates) {
            Some(objects) => objects.push(creature),
            None => {
                spatial_index.insert(cell_coordinates, vec![creature]);
            }
        };
    }

    for transform in &food_query {
        let (food_x, food_y) = (transform.translation.x, transform.translation.y);

        let cell_coordinates = get_cell_coordinates(food_x, food_y);

        let food = VisibleObject {
            x: food_x,
            y: food_y,
            radius: 0.5,
            category: ObjectCategory::Food,
        };

        match spatial_index.get_mut(&cell_coordinates) {
            Some(objects) => objects.push(food),
            None => {
                spatial_index.insert(cell_coordinates, vec![food]);
            }
        };
    }

    spatial_index
}

fn get_cell_coordinates(x: f32, y: f32) -> (i32, i32) {
    (
        (x - x.rem_euclid((SEEING_DISTANCE * 2) as f32)) as i32,
        (y - y.rem_euclid((SEEING_DISTANCE * 2) as f32)) as i32,
    )
}
