use anyhow::Result;
use rand::Rng;
use std::{error::Error, fmt::Display};

/// Represents one neural connection in a creature's brain.
#[derive(Clone)]
pub struct Gene {
    /// The source of the connection.
    /// If the most significant bit of the source id is a 0 (i.e. less than 128), the source is a sensory neuron.
    source_id: u8,
    /// If the most significant bit of the destination id is a 0 (i.e. less than 128), the destination is an action neuron..
    destination_id: u8,
    /// The weight of the connection.
    weight: f32,
}

impl Gene {
    /// Returns the source id.
    pub fn source_id(&self) -> u8 {
        self.source_id
    }

    /// Returns the destination id.
    pub fn destination_id(&self) -> u8 {
        self.destination_id
    }

    /// Returns the weight.
    pub fn weight(&self) -> f32 {
        self.weight
    }

    /// Returns a gene with a random source id, destination id and weight.
    pub fn random() -> Self {
        let mut generator = rand::thread_rng();

        Gene::new(
            rand::random(),
            rand::random(),
            generator.gen_range(-1.0..=1.0),
        )
    }

    /// Creates a new gene.
    pub fn new(source_id: u8, destination_id: u8, weight: f32) -> Self {
        Self {
            source_id,
            destination_id,
            weight,
        }
    }

    /// Creates a new gene from a given hex string.
    pub fn from_hex(hex: &str) -> Result<Self> {
        if hex.len() != 12 {
            return Err(InvalidHexLength.into());
        }

        let source_id = &hex[0..2];
        let destination_id = &hex[2..4];
        let weight = &hex[4..12];

        let source_id = u8::from_str_radix(source_id, 16)?;
        let destination_id = u8::from_str_radix(destination_id, 16)?;
        let weight = f32::from_bits(u32::from_str_radix(weight, 16)?);

        Ok(Gene::new(source_id, destination_id, weight))
    }

    /// Returns the hex representation of a gene.
    pub fn as_hex(&self) -> String {
        format!(
            "{:02x}{:02x}{:012x}",
            self.source_id(),
            self.destination_id(),
            self.weight().to_bits(),
        )
    }
}

/// An error returned when a hex string to be converted into a Gene is of invalid length .
#[derive(Debug)]
struct InvalidHexLength;

impl Display for InvalidHexLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The length of the provided hex string was not 12.")
    }
}

impl Error for InvalidHexLength {}
