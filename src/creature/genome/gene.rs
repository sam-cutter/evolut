use fstr::FStr;
use std::{num::ParseIntError, str::FromStr};

/// # Gene
///
/// A gene represents one neural connection in a creature's brain.
///
/// See also: [crate::creature::brain::Brain] and [crate::creature::brain::Connection].
pub struct Gene {
    /// The source of the connection.
    /// If the most significant bit of the source id is a 0 (i.e. less than 128), the source is a sensory neuron.
    source_id: u8,
    /// If the most significant bit of the destination id is a 0 (i.e. less than 128), the destination is an action neuron..
    destination_id: u8,
    /// The weight of the connection.
    weight: f64,
}

impl Gene {
    pub fn source_id(&self) -> u8 {
        self.source_id
    }

    pub fn destination_id(&self) -> u8 {
        self.destination_id
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }
}

impl Gene {
    pub fn new(source_id: u8, destination_id: u8, weight: f64) -> Self {
        Self {
            source_id,
            destination_id,
            weight,
        }
    }
    pub fn from_hex(hex: FStr<20>) -> Result<Self, ParseIntError> {
        let source_id = &hex[0..2];
        let destination_id = &hex[2..4];
        let weight = &hex[4..20];

        let source_id = u8::from_str_radix(source_id, 16)?;
        let destination_id = u8::from_str_radix(destination_id, 16)?;
        let weight = f64::from_bits(u64::from_str_radix(weight, 16)?);

        Ok(Gene::new(source_id, destination_id, weight))
    }

    pub fn as_hex(&self) -> FStr<20> {
        let hex = format!(
            "{:02x}{:02x}{:016x}",
            self.source_id,
            self.destination_id,
            self.weight.to_bits(),
        );

        FStr::from_str(&hex).unwrap()
    }
}
