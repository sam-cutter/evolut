//! Houses all gene and genome-related code.

mod gene;

use bevy::prelude::Component;

pub use gene::Gene;

/// Represents a list of a creature's genes. This genome is required to build a creature's brain.
#[derive(Component)]
pub struct Genome {
    genes: Vec<Gene>,
}

impl Genome {
    /// Builds a new genome.
    pub fn new(genes: Vec<Gene>) -> Self {
        Self { genes }
    }

    /// Gets the creatures genes.
    pub fn genes(&self) -> &Vec<Gene> {
        &self.genes
    }
}
