#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Invalid baudrate")]
    InvalidBaudrate,
    #[error("Invalid samplerate")]
    InvalidSamplerate,
    #[error("Invalid channels")]
    InvalidChannels,
    #[error("Invalid carrier frequency")]
    InvalidCarrierFreq,
    #[error("Invalid deviation frequency")]
    InvalidDeviationFreq,
    #[error("Invalid modulation method")]
    InvalidModulationMethod,
    #[error("Invalid frame length")]
    InvalidFrameLength,
    #[error("Frame check sequence validation failed")]
    FCSValidationFailed,
    #[error("Invalid physical layer frame")]
    InvalidPhysicalFrame,
}
pub type Result<T> = std::result::Result<T, Error>;
