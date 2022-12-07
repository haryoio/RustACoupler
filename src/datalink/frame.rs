use crc::{Crc, CRC_32_MPEG_2};

use crate::{
    bytes::{
        u16_from_bytes,
        u16_to_bytes,
        u32_from_bytes,
        u32_to_bytes,
        u8_from_bytes,
        u8_to_bytes,
    },
    error::{Error, Result},
};

pub const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_MPEG_2);

///
/// Datalink Frame Structure 52 bits
/// *------------*---------------------*----------------*-----------------*------*---------*
/// | Frame Type | Destination Address | Source Address | Sequence Number | Data | FCS     |
/// *------------*---------------------*----------------*-----------------*------*---------*
/// | 4bit       | 1byte               | 1byte          | 2byte           |  ..  | 4byte   |
/// *------------*---------------------*----------------*-----------------*------*---------*
///
/// Frame Type:
/// 0x0: Data Frame
/// 0x1: Acknowledgement Frame
/// 0x2: Beacon Frame
/// 0x3: Command Frame
/// 0x4-0xF: Reserved
#[derive(Debug, Clone)]
pub struct Datalink {
    pub frame_type:          FrameType,
    pub destination_address: u8,
    pub source_address:      u8,
    pub sequence_number:     u16,
    pub data:                Vec<u8>,
    fcs:                     u32,
}

impl Datalink {
    pub fn new(
        frame_type: FrameType,
        destination_address: u8,
        source_address: u8,
        sequence_number: u16,
        data: Vec<u8>,
    ) -> Self {
        let mut frame = vec![];
        frame.extend(frame_type.to_bytes());
        frame.extend(u8_to_bytes(destination_address));
        frame.extend(u8_to_bytes(source_address));
        frame.extend(u16_to_bytes(sequence_number));
        frame.extend(data.clone());

        let fcs = CRC.checksum(frame.as_slice());

        Self {
            frame_type,
            destination_address,
            source_address,
            sequence_number,
            data,
            fcs,
        }
    }
    pub fn detect_checksum(&self) -> bool {
        let mut frame = vec![];
        frame.extend(self.frame_type.to_bytes());
        frame.extend(u8_to_bytes(self.destination_address));
        frame.extend(u8_to_bytes(self.source_address));
        frame.extend(u16_to_bytes(self.sequence_number));
        frame.extend(self.data.clone());

        let fcs = CRC.checksum(frame.as_slice());

        fcs == self.fcs
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 52 {
            return Err(Error::InvalidFrameLength);
        }

        let frame_type = FrameType::from(&bytes[0..4]);
        let destination_address = u8_from_bytes(&bytes[4..12]);
        let source_address = u8_from_bytes(&bytes[12..20]);
        let sequence_number = u16_from_bytes(&bytes[20..36]);
        let data = bytes[36..bytes.len() - 32].to_vec();
        let fcs = u32_from_bytes(&bytes[bytes.len() - 32..]);

        Ok(Self {
            frame_type,
            destination_address,
            source_address,
            sequence_number,
            data,
            fcs,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut frame = vec![];
        frame.extend(self.frame_type.to_bytes());
        frame.extend(u8_to_bytes(self.destination_address));
        frame.extend(u8_to_bytes(self.source_address));
        frame.extend(u16_to_bytes(self.sequence_number));
        frame.extend(self.data.clone());

        let fcs = CRC.checksum(frame.as_slice());
        frame.extend(u32_to_bytes(fcs));

        frame
    }
}

#[derive(Debug, Clone)]
pub enum FrameType {
    Data,
    Acknowledgement,
    Beacon,
    Command,
    Error,
}

impl FrameType {
    pub fn is_data(&self) -> bool {
        matches!(self, FrameType::Data)
    }
    pub fn is_acknowledgement(&self) -> bool {
        matches!(self, FrameType::Acknowledgement)
    }
    pub fn is_beacon(&self) -> bool {
        matches!(self, FrameType::Beacon)
    }
    pub fn is_command(&self) -> bool {
        matches!(self, FrameType::Command)
    }
    pub fn to_bytes(&self) -> [u8; 4] {
        match self {
            FrameType::Data => [0, 0, 0, 0],
            FrameType::Acknowledgement => [0, 0, 0, 1],
            FrameType::Beacon => [0, 0, 1, 0],
            FrameType::Command => [0, 0, 1, 1],
            FrameType::Error => [1, 1, 1, 1],
        }
    }
}

impl From<&[u8]> for FrameType {
    fn from(bytes: &[u8]) -> Self {
        match bytes {
            [0, 0, 0, 0] => FrameType::Data,
            [0, 0, 0, 1] => FrameType::Acknowledgement,
            [0, 0, 1, 0] => FrameType::Beacon,
            [0, 0, 1, 1] => FrameType::Command,
            _ => FrameType::Error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_type() {
        let frame_type = FrameType::from(vec![0, 0, 0, 0].as_slice());
        assert!(frame_type.is_data());
    }
}
