use anyhow::{bail, Result};
use log::trace;

#[derive(Debug, Clone)]
pub struct QPIGS2 {
    pub pv2_input_current: f32,
    pub pv2_input_voltage: f32,
    pub pv2_charging_power: usize,
}

impl QPIGS2 {
    pub fn new_from_string(input_data_string: &str) -> Result<Self> {
        let data_string = &input_data_string[1..];
        let parts: Vec<&str> = data_string.split(' ').collect();

        trace!("QPIGS2 Parting parts: {:?}", parts);

        if parts.len() < 3 {
            bail!("Insufficient data in QPIGS2 response")
        }

        Ok(QPIGS2 {
            pv2_input_current: parts[0].parse::<f32>()?,
            pv2_input_voltage: parts[1].parse::<f32>()?,
            pv2_charging_power: parts[2].parse::<usize>()?,
        })
    }
}
