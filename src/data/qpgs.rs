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
    pub ac_output_va: usize,
    pub ac_output_watts: usize,
    pub load_percent: usize,
    pub battery_voltage: f32,
    pub battery_charging_current: usize,
    pub battery_capacity_percent: usize,
    pub pv1_input_voltage: f32,
    pub total_charging_current: usize,
    pub total_ac_output_va: usize,
    pub total_output_active_power: usize,
    pub total_ac_output_percent: usize,
    pub inverter_status: String,
    pub output_mode: usize,
    pub charger_source_priority: usize,
    pub max_charger_current: usize,
    pub max_charger_range: usize,
    pub max_ac_charger_current: usize,
    pub pv1_input_current: f32,
    pub battery_discharge_current: usize,
    pub pv2_input_voltage: f32,
    pub pv2_input_current: f32,
}

impl QPGS {
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
            ac_output_va: parts[8].parse::<usize>()?,
            ac_output_watts: parts[9].parse::<usize>()?,
            load_percent: parts[10].parse::<usize>()?,
            battery_voltage: parts[11].parse::<f32>()?,
            battery_charging_current: parts[12].parse::<usize>()?,
            battery_capacity_percent: parts[13].parse::<usize>()?,
            pv1_input_voltage: parts[14].parse::<f32>()?,
            total_charging_current: parts[15].parse::<usize>()?,
            total_ac_output_va: parts[16].parse::<usize>()?,
            total_output_active_power: parts[17].parse::<usize>()?,
            total_ac_output_percent: parts[18].parse::<usize>()?,
            inverter_status: parts[19].to_string(),
            output_mode: parts[20].parse::<usize>()?,
            charger_source_priority: parts[21].parse::<usize>()?,
            max_charger_current: parts[22].parse::<usize>()?,
            max_charger_range: parts[23].parse::<usize>()?,
            max_ac_charger_current: parts[24].parse::<usize>()?,
            pv1_input_current: parts[25].parse::<f32>()?,
            battery_discharge_current: parts[26].parse::<usize>()?,
            pv2_input_voltage: parts[27].parse::<f32>()?,
            pv2_input_current: parts[28].parse::<f32>()?,
        })
    }
}
