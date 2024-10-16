mod gene;

pub use gene::Gene;

/// # Genome
///
/// A genome represents a list of a creature's genes. This genome is required to build a creature's brain.
///
/// See also: [Gene], [crate::creature::brain::Brain].
pub type Genome = Vec<Gene>;
