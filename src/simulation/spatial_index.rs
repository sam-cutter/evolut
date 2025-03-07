use bevy::prelude::*;
use std::collections::HashMap;

use super::{SEEING_DISTANCE, creature::vision::VisibleObject, food::Food};
use crate::model::creature::brain::Brain;

#[derive(Resource)]
pub struct SpatialIndex {
    pub index: HashMap<(i32, i32), Vec<VisibleObject>>,
}

pub struct SpatialIndexPlugin;

impl Plugin for SpatialIndexPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpatialIndex {
            index: HashMap::new(),
        });

        app.add_systems(Startup, build_spatial_index);

        app.add_systems(FixedPreUpdate, build_spatial_index);
    }
}

#[derive(PartialEq, Eq)]
pub enum ObjectCategory {
    Creature,
    Food,
    Obstacle,
}

pub fn build_spatial_index(
    creature_query: Query<(&Transform, Entity), With<Brain>>,
    food_query: Query<(&Transform, Entity), With<Food>>,
    mut spatial_index_resource: ResMut<SpatialIndex>,
) {
    let creatures = creature_query.iter().collect();
    let food = food_query.iter().collect();

    let mut spatial_index: HashMap<(i32, i32), Vec<VisibleObject>> = HashMap::new();

    add_to_spatial_index(&mut spatial_index, creatures, ObjectCategory::Creature);
    add_to_spatial_index(&mut spatial_index, food, ObjectCategory::Food);

    *spatial_index_resource = SpatialIndex {
        index: spatial_index,
    };
}

fn add_to_spatial_index(
    spatial_index: &mut HashMap<(i32, i32), Vec<VisibleObject>>,
    objects: Vec<(&Transform, Entity)>,
    category: ObjectCategory,
) {
    for (transform, entity) in objects {
        let (object_x, object_y) = (transform.translation.x, transform.translation.y);

        let cell_coordinates = get_cell_coordinates(object_x, object_y);

        let (category, radius) = get_category_radius_pair(&category);

        let object = VisibleObject {
            x: object_x,
            y: object_y,
            radius,
            category,
            entity,
        };

        match spatial_index.get_mut(&cell_coordinates) {
            Some(objects) => objects.push(object),
            None => {
                spatial_index.insert(cell_coordinates, vec![object]);
            }
        };
    }
}

fn get_category_radius_pair(category: &ObjectCategory) -> (ObjectCategory, f32) {
    match category {
        ObjectCategory::Creature => (ObjectCategory::Creature, 1.0),
        ObjectCategory::Food => (ObjectCategory::Food, 0.5),
        ObjectCategory::Obstacle => (ObjectCategory::Obstacle, 0.0),
    }
}

pub fn get_cell_coordinates(x: f32, y: f32) -> (i32, i32) {
    (
        (x - x.rem_euclid((SEEING_DISTANCE * 2) as f32)) as i32,
        (y - y.rem_euclid((SEEING_DISTANCE * 2) as f32)) as i32,
    )
}
