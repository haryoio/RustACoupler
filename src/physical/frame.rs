use std::fmt::Display;

use crc::{Crc, CRC_16_IBM_SDLC};

use crate::{
    bytes::{u16_from_bytes, u16_to_bytes},
    error::Result,
    PREAMBLE,
    USFD,
};

pub const CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);

/// +----------+--------+------------------+--------+--------+---------+
/// | Preamble |  SFD   |  Modulation Type | length |  CRC   | Payload |
/// +----------+--------+------------------+--------+--------+---------+
/// | 1 byte   | 1 byte |  4 bit           | 2 byte | 2 byte | ...     |
/// +----------+--------+------------------+--------+--------+---------+
#[derive(Debug)]
pub struct Physical {
    pub mod_type:        ModulationType,
    pub length:          u16,
    pub detect_checksum: bool,
}

impl Physical {
    pub fn new(length: u16, mod_type: ModulationType) -> Self {
        Self {
            mod_type,
            length,
            detect_checksum: true,
        }
    }

    pub fn to_bytes(&self) -> [u8; 52] {
        let mut frame = [0u8; 52];
        frame[0..8].copy_from_slice(&PREAMBLE);
        frame[8..16].copy_from_slice(&USFD);
        frame[16..20].swap_with_slice(&mut self.mod_type.to_bytes());
        frame[20..36].swap_with_slice(&mut u16_to_bytes(self.length));
        let checksum = CRC.checksum(&frame[16..36]);
        frame[36..52].swap_with_slice(&mut u16_to_bytes(checksum));
        frame
    }

    pub fn from_bytes(bytes: &[u8; 36]) -> Result<Self> {
        let mod_type = ModulationType::from(&bytes[0..4]);
        let length = u16_from_bytes(&bytes[4..20]);
        let crc = u16_from_bytes(&bytes[20..36]);
        let detect_checksum = CRC.checksum(&bytes[0..20]) == crc;
        Ok(Self {
            mod_type,
            length,
            detect_checksum,
        })
    }
}

impl Display for Physical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Modulation type: {}", self.mod_type)?;
        writeln!(f, "Physical Header Length: {}", self.length)?;
        writeln!(f, "Cecksum detected: {}", self.detect_checksum)
    }
}

#[derive(Debug)]
pub enum ModulationType {
    BfskNoErrorCorrection,
}

impl From<&[u8]> for ModulationType {
    fn from(bytes: &[u8]) -> Self {
        match bytes {
            [0, 0, 0, 1] => ModulationType::BfskNoErrorCorrection,
            _ => panic!("Unknown modulation type"),
        }
    }
}

impl ModulationType {
    pub fn to_bytes(&self) -> [u8; 4] {
        match self {
            ModulationType::BfskNoErrorCorrection => [0, 0, 0, 1],
        }
    }
}

impl Display for ModulationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModulationType::BfskNoErrorCorrection => write!(f, "BFSK no error correction"),
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_checksum() {
        let checksum = CRC.checksum(b"123456789");
        println!("{:b}", checksum);
    }
    #[test]
    fn test_build_header() {
        let frame = Physical::new(100, ModulationType::BfskNoErrorCorrection).to_bytes();
        let mut frame_start = false;
        let mut data: Vec<u8> = vec![];

        for bit in frame.chunks(8) {
            if !frame_start && bit == USFD {
                println!("Found SFD");
                frame_start = true;
                continue;
            }
            if frame_start {
                data.extend(bit);
            }
        }
        println!("{:?}", data);
        Physical::from_bytes(&data[0..36].try_into().unwrap()).unwrap();
    }
}
