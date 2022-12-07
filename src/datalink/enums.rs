#[derive(Debug, Clone)]
pub enum FrameType {
    Data,
    Acknowledgement,
    Beacon,
    Command,
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
            _ => panic!("Unknown frame type"),
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
