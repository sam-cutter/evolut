//! Houses all gene and genome-related code.

mod gene;

pub use gene::Gene;

/// Represents a list of a creature's genes. This genome is required to build a creature's brain.
pub type Genome = Vec<Gene>;
