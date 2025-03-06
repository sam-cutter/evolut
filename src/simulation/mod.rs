//! Contains code related to running the simulation.

mod creature;
mod food;
mod setup;
mod spatial_index;

use bevy::{math::Vec2, prelude::Component};

pub use creature::CreaturePlugin;
pub use food::FoodPlugin;
pub use setup::SetupPlugin;

/// The maximum number of internal neurons a creature's brain can contain.
pub const MAX_INTERNAL_NEURONS: u8 = 5;
/// The maximum number of genes that may exist in a creature's genome.
pub const GENOME_LENGTH: usize = 10;
/// The number of creatures in the first generation.
pub const GENERATION_ZERO_SIZE: u32 = 1000;
/// The frequency, measured in Hz, at which the physics system should be updated.
pub const FIXED_UPDATE_FREQUENCY: f64 = 1000.0;
/// The frequency, measured in Hz, at which the creatures should recalculate their brain state;
pub const BRAIN_UPDATE_FREQUENCY: f64 = 10.0;
/// The initial energy a creature should have.
pub const INITIAL_ENERGY: f32 = 1000.0;
/// The distance that a creature is able to see
pub const SEEING_DISTANCE: i32 = 10;
/// The initial quantity of food to spawn.
pub const INITIAL_FOOD: i32 = 1000;

#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
}

#[derive(Component)]
pub struct AngularVelocity {
    pub value: f32,
}
