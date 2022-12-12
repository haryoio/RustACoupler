use cpal::traits::{DeviceTrait, HostTrait};

use crate::error::Result;

pub fn list_input_devices() -> Result<()> {
    let host = cpal::default_host();
    let input_device = host.input_devices().expect("no input device available");
    for (i, device) in input_device.enumerate() {
        let name = device.name().expect("no device name");
        println!("{}: {}", i, name);

        let default_config = device.default_input_config().unwrap();
        println!("  Channels: {}", default_config.channels());
        println!("  Sample Rate: {}", default_config.sample_rate().0);
        println!("  Buffer Size: {:?}", default_config.buffer_size());
        println!("  Sample Format: {:?}", default_config.sample_format());
        println!();
    }
    Ok(())
}

pub fn list_output_devices() -> Result<()> {
    let host = cpal::default_host();
    let output_device = host.output_devices().expect("no output device available");
    for (i, device) in output_device.enumerate() {
        let name = device.name().expect("no device name");
        println!("{}: {}", i, name);

        let default_config = device.default_output_config().unwrap();
        println!("  Channels: {}", default_config.channels());
        println!("  Sample Rate: {}", default_config.sample_rate().0);
        println!("  Buffer Size: {:?}", default_config.buffer_size());
        println!("  Sample Format: {:?}", default_config.sample_format());
        println!();
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list_input_devices() {
        list_input_devices().unwrap();
    }
    #[test]
    fn test_list_output_devices() {
        list_output_devices().unwrap();
    }
}
