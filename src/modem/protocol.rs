use crate::{
    bytes::encode_u8,
    datalink::frame::{Datalink, FrameType},
    error::{Error, Result},
    physical::frame::{ModulationType, Physical},
};
#[derive(Debug)]
pub struct Protocol {
    datalink: Datalink,
    physical: Physical,
}

impl Protocol {
    pub fn new(data: &str, src: u8, dst: u8, seq: u16, f_type: FrameType) -> Self {
        let datalink = Datalink::new(f_type, dst, src, seq, encode_u8(data));
        let physical = Physical::new(
            datalink.to_bytes().len() as u16,
            ModulationType::BfskNoErrorCorrection,
        );

        Self { datalink, physical }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.physical.to_bytes());
        bytes.extend(self.datalink.to_bytes());

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 68 {
            return Err(Error::InvalidFrameLength);
        }
        let mut physical_bytes = [0u8; 36];
        physical_bytes.copy_from_slice(&bytes[0..36]);
        let physical = Physical::from_bytes(&physical_bytes)?;
        let datalink = Datalink::from_bytes(&bytes[physical.to_bytes().len()..])?;
        Ok(Self { datalink, physical })
    }
}

impl From<Vec<u8>> for Protocol {
    fn from(bytes: Vec<u8>) -> Self {
        if bytes.len() < 68 {
            panic!(
                "Invalid frame length. Expected at least 68 bytes, got {}",
                bytes.len()
            );
        }
        let mut physical_bytes = [0u8; 36];
        physical_bytes.copy_from_slice(&bytes[0..36]);
        let physical = Physical::from_bytes(&physical_bytes).expect("Invalid physical frame");
        let datalink = Datalink::from_bytes(&bytes[physical.to_bytes().len()..])
            .expect("Invalid datalink frame");
        Self { datalink, physical }
    }
}

impl From<Vec<i8>> for Protocol {
    fn from(bytes: Vec<i8>) -> Self {
        let mut bytes_u8 = vec![];
        for byte in bytes {
            bytes_u8.push(byte as u8);
        }
        Self::from(bytes_u8)
    }
}
