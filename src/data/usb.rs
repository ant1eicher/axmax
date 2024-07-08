use anyhow::{bail, Result};
use crc16::State as CRCState;
use log::{debug, info, trace};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

pub const RAW_HID0: &str = "/dev/hidraw0";

pub(crate) async fn fetch_command_data_usb(
    usb_file_name: &str,
    command: &str,
) -> Result<[u8; 1000]> {
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
    info!("Bytes writen to usb device.");
    Ok(command.len())
}

async fn read_result_usb(usb_file: &mut File) -> Result<[u8; 1000]> {
    let mut buf: [u8; 1000] = [0; 1000];
    let mut buf_slice = &mut buf[0..];

    let mut counter: usize = 0;

    while !buf_slice.contains(&b'\r') {
        buf_slice = &mut buf[counter..];
        let bytes_read = match usb_file.read(buf_slice).await {
            Ok(br) => {
                counter += br;
                br
            }
            Err(e) => {
                bail!("failed to read bytes from serial port: {}", e);
            }
        };
        trace!("Bytes read: {}", bytes_read);
        trace!("Byte read: {}", buf_slice[0] as char);
    }
    Ok(buf)
}

async fn build_command(command: &str) -> Vec<u8> {
    let mut command: Vec<u8> = String::from(command).into_bytes();
    let crc_16 = CRCState::<crc16::XMODEM>::calculate(command.as_slice());

    let crc = unsafe { std::mem::transmute::<u16, [u8; 2]>(crc_16) };

    command.push(crc[1]);
    command.push(crc[0]);
    command.push(0x0d_u8);

    debug!("Command Built: {:?}", command);

    command
}
