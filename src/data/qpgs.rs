use anyhow::{bail, Result};
use log::trace;

#[derive(Debug, Clone)]
pub struct QPGS {
    pub parallel_num: usize,
    pub serial_number: String,
    pub work_mode: char,
    pub fault_code: usize,
    pub grid_voltage: f32,
    pub grid_frequency: f32,
    pub ac_output_voltage: f32,
    pub ac_output_frequency: f32,
    pub battery_voltage: f32,

    pub pv2_charging_power: f32,
}

impl QPGS {
    /// Note: this only works on newer firmware
    pub fn new_from_string(input_data_string: &str) -> Result<Self> {
        let data_string = &input_data_string[1..].trim();
        let parts: Vec<&str> = data_string.split_whitespace().collect();

        trace!("QPGS Parsing parts: {:?}", parts);

        if parts.len() < 29 {
            bail!("Insufficient data in QPGS response")
        }

        Ok(QPGS {
            parallel_num: parts[0].parse::<usize>()?,
            serial_number: parts[1].to_string(),
            work_mode: parts[2].chars().next().unwrap(),
            fault_code: parts[3].parse::<usize>()?,
            grid_voltage: parts[4].parse::<f32>()?,
            grid_frequency: parts[5].parse::<f32>()?,
            ac_output_voltage: parts[6].parse::<f32>()?,
            ac_output_frequency: parts[7].parse::<f32>()?,
            battery_voltage: parts[11].parse::<f32>()?,

            pv2_charging_power: parts[28].parse::<f32>()?,
        })
    }
}
