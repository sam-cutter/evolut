pub struct Genome(Vec<Gene>);

// TODO: Implement a function which converts a hex string to a Gene.
pub struct Gene {
    source_id: u8,
    destination_id: u8,
    weight: f64,
}
