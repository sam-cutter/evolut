use bevy::prelude::*;
use rand::Rng;

use super::INITIAL_FOOD;

#[derive(Bundle)]
pub struct FoodBundle {
    pub mesh: Mesh2d,
    pub mesh_material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub visibility: Visibility,
}

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, place_initial_food);
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
        });
    }
}
