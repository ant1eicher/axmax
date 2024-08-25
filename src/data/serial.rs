use std::{io::Read, time::Duration};

use anyhow::{bail, Result};
use crc16::State as CRCState;
use log::debug;
pub const TTY_USB0: &str = "/dev/ttyUSB0";
pub const TTY_USB1: &str = "/dev/ttyUSB1";

pub async fn fetch_command_data_serial(serial_port: &str, command: &str) -> Result<String> {
    let command = build_command(command).await;

    let mut port = serialport::new(serial_port, 2400)
        .timeout(Duration::from_secs(1))
        .open()
        .expect("Failed to open port");

    let bytes_written = port.write(&command.as_slice())?;
    debug!("Wrote {} bytes to {}", bytes_written, serial_port);

    read_result(&mut port).await
}

async fn build_command(command: &str) -> Vec<u8> {
    let mut command: Vec<u8> = String::from(command).into_bytes();
    let crc_16 = CRCState::<crc16::XMODEM>::calculate(command.as_slice());

    let crc: [u8; 2] = unsafe { std::mem::transmute::<u16, [u8; 2]>(crc_16) };

    command.push(crc[1]);
    command.push(crc[0]);
    command.push(0x0d_u8);

    let hex_string = hex::encode(&command);

    debug!("Command Built: {:?} {}", command, hex_string);

    command
}

async fn read_result(file: &mut dyn Read) -> Result<String> {
    let mut buf: [u8; 1000] = [0; 1000];
    let mut buf_slice = &mut buf[0..];

    let mut counter: usize = 0;

    while !buf_slice.contains(&b'\r') {
        buf_slice = &mut buf[counter..];
        match file.read(buf_slice) {
            Ok(br) => {
                counter += br;
                br
            }
            Err(e) => {
                bail!("failed to read bytes from serial port: {}", e);
            }
        };
    }

    let mut out = vec![];
    for c in buf {
        if c == 13u8 {
            break;
        }
        out.push(c);
    }

    Ok(String::from_utf8_lossy(&out[..out.len() - 3]).to_string())
}