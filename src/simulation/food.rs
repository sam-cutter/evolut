use bevy::prelude::*;
use rand::Rng;

use super::{
    INITIAL_FOOD, SEEING_DISTANCE,
    creature::Energy,
    spatial_index::{ObjectCategory, SpatialIndex, get_cell_coordinates},
};
use crate::model::creature::brain::Brain;

#[derive(Component)]
pub struct Food;

#[derive(Bundle)]
pub struct FoodBundle {
    pub mesh: Mesh2d,
    pub mesh_material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub visibility: Visibility,
    pub food: Food,
}

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, place_initial_food);
        app.add_systems(FixedUpdate, check_consumption);
    }
}

fn place_initial_food(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut generator = rand::thread_rng();

    for _ in 0..INITIAL_FOOD {
        let circle = meshes.add(Circle::new(0.5));

        commands.spawn(FoodBundle {
            mesh: Mesh2d(circle),
            mesh_material: MeshMaterial2d(materials.add(Color::linear_rgb(0.0, 1.0, 0.0))),
            transform: Transform {
                translation: Vec3::new(
                    generator.gen_range(-50.0..=50.0),
                    generator.gen_range(-50.0..=50.0),
                    -1.0,
                ),
                ..default()
            },
            visibility: Visibility::Visible,
            food: Food,
        });
    }
}

fn check_consumption(
    mut creature_query: Query<(&Transform, Entity, &mut Energy), With<Brain>>,
    mut commands: Commands,
    spatial_index: Res<SpatialIndex>,
) {
    let spatial_index = &spatial_index.index;

    for mut creature in &mut creature_query {
        let transform = creature.0;

        let (creature_x, creature_y) = (transform.translation.x, transform.translation.y);

        let (cell_x, cell_y) = get_cell_coordinates(creature_x, creature_y);

        // This forms a 3 by 3 grid, centred at the current cell.
        let cells_to_search = [
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
                let food = objects
                    .iter()
                    .filter(|object| object.category == ObjectCategory::Food);

                for food_piece in food {
                    if let Some(mut entity) = commands.get_entity(food_piece.entity) {
                        let delta_x = creature_x - food_piece.x;
                        let delta_y = creature_y - food_piece.y;

                        if delta_x.powi(2) + delta_y.powi(2) > 1.0 {
                            continue;
                        }

                        creature.2.value += 1000.0;

                        entity.despawn();
                    }
                }
            }
        }
    }
}
