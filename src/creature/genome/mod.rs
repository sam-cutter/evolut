pub struct Genome(Vec<Gene>);

use fstr::FStr;
use std::num::ParseIntError;

pub struct Gene {
    pub source_id: u8,
    pub destination_id: u8,
    pub weight: f64,
}

impl Gene {
    pub fn from_hex(hex: FStr<20>) -> Result<Self, ParseIntError> {
        let source_id = &hex[0..2];
        let destination_id = &hex[2..4];
        let weight = &hex[4..20];

        let source_id = u8::from_str_radix(source_id, 16)?;
        let destination_id = u8::from_str_radix(destination_id, 16)?;
        let weight = f64::from_bits(u64::from_str_radix(weight, 16)?);

        Ok(Self {
            source_id,
            destination_id,
            weight,
        })
    }
}
