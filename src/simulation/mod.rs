//! Contains code related to running the simulation.

use bevy::{math::Vec2, prelude::Component};

/// The maximum number of internal neurons a creature's brain can contain.
pub const MAX_INTERNAL_NEURONS: u8 = 5;
/// The maximum number of genes that may exist in a creature's genome.
pub const GENOME_LENGTH: usize = 10;
/// The number of creatures in the first generation.
pub const GENERATION_ZERO_SIZE: u32 = 1000;

#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
}

#[derive(Component)]
pub struct AngularVelocity {
    pub value: f32,
}
