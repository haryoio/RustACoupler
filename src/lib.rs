pub mod binary;
pub mod config;
pub mod hamming;
pub mod receiver;
pub mod transmitter;
pub mod utils;

pub const DIAL_TONE: f64 = 800f64;
pub const ANSWER_TONE: f64 = 2200f64;
pub const PREAMBLE: [u8; 8] = [1, 0, 1, 0, 1, 0, 1, 0];
pub const SFD: [u8; 8] = [1, 0, 1, 0, 1, 0, 1, 1];
