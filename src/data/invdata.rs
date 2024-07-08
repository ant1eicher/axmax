use core::str;
use std::fmt::Debug;

use super::{
    qpgs::QPGS,
    qpigs::QPIGS,
    usb::{fetch_command_data_usb, RAW_HID0},
};
use anyhow::{bail, Result};
use log::debug;

pub(crate) enum Command {
    QPGS1,
    QPGS2,
    QPIGS,
}
impl Command {
    fn as_str(&self) -> &str {
        match self {
            Command::QPGS1 => "QPGS1",
            Command::QPGS2 => "QPGS2",
            Command::QPIGS => "QPIGS",
        }
    }
}

pub(crate) trait InvData: Sized {
    async fn retrieve(command: Command) -> Result<Self>;
}

impl InvData for QPGS {
    async fn retrieve(command: Command) -> Result<Self> {
        Ok(fetch_and_parse::<QPGS>(RAW_HID0, command.as_str(), QPGS::new_from_string).await?)
    }
}

impl InvData for QPIGS {
    async fn retrieve(command: Command) -> Result<Self> {
        Ok(fetch_and_parse::<QPIGS>(RAW_HID0, command.as_str(), QPIGS::new_from_string).await?)
    }
}

async fn fetch_and_parse<T: Debug>(
    device: &str,
    command: &str,
    constructor: fn(&str) -> Result<T>,
) -> Result<T> {
    match fetch_command_data_usb(device, command).await {
        Ok(data) => {
            let input = String::from_utf8_lossy(&data);

            match constructor(&input) {
                Ok(q) => {
                    debug!("{}: {:?}", command, &q);
                    Ok(q)
                }
                Err(e) => {
                    bail!(
                        "Error marshalling {} response to structure: {}. Input: {}",
                        command,
                        e,
                        input
                    );
                }
            }
        }
        Err(e) => {
            bail!("Error fetching data from the inverter: {}", e);
        }
    }
}
