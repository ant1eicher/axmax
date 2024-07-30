use std::time::Duration;

use anyhow::{bail, Result};
use crc16::State as CRCState;
use log::debug;
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
    time::sleep,
};

pub const RAW_HID0: &str = "/dev/hidraw0";
pub const RAW_HID1: &str = "/dev/hidraw1";

pub(crate) async fn fetch_command_data_usb(usb_file_name: &str, command: &str) -> Result<String> {
    let command = build_command(command).await;
    let mut usb_file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(usb_file_name)
        .await?;
    write_command_usb(&mut usb_file, command).await?;
    read_result_usb(&mut usb_file).await
}

async fn write_command_usb(usb_file: &mut File, command: Vec<u8>) -> Result<usize> {
    usb_file.write_all(&command.as_slice()).await?;
    Ok(command.len())
}

async fn read_result_usb(usb_file: &mut File) -> Result<String> {
    let mut buf: [u8; 1000] = [0; 1000];
    let mut buf_slice = &mut buf[0..];

    let mut counter: usize = 0;

    while !buf_slice.contains(&b'\r') {
        buf_slice = &mut buf[counter..];
        match usb_file.read(buf_slice).await {
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
