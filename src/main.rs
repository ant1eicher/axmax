use data::{
    qpgs::QPGS,
    qpigs::QPIGS,
    usb::{fetch_command_data_usb, RAW_HID0, RAW_HID1},
};
use log::info;

pub mod data;

#[tokio::main]
async fn main() {
    env_logger::init();

    let result = fetch_command_data_usb(RAW_HID0, "QID")
        .await
        .expect("Failed to read response");
    info!(
        "inverter 1 serial: {}",
        result
            .split(" ")
            .last()
            .expect("could not parse")
            .strip_prefix("(")
            .expect("could not strip prefix")
    );

    let result = fetch_command_data_usb(RAW_HID0, "QPIGS")
        .await
        .expect("Failed to read response");

    let qpigs = QPIGS::new_from_string(&result);
    info!("nverter 1 QPIGS: {:#?}", qpigs);

    println!("");

    let result = fetch_command_data_usb(RAW_HID1, "QPGS1")
        .await
        .expect("Failed to read response");

    let qpgs = QPGS::new_from_string(&result);
    info!("inverter 1 QPGS: {:#?}", qpgs);

    println!("");

    let result = fetch_command_data_usb(RAW_HID1, "QID")
        .await
        .expect("Failed to read response");
    info!(
        "inverter 2 serial: {}",
        result
            .split(" ")
            .last()
            .expect("could not parse")
            .strip_prefix("(")
            .expect("could not strip prefix")
    );

    let result = fetch_command_data_usb(RAW_HID1, "QPIGS")
        .await
        .expect("Failed to read response");

    let qpigs = QPIGS::new_from_string(&result);
    info!("inverter 2 QPIGS: {:#?}", qpigs);

    println!("");

    let result = fetch_command_data_usb(RAW_HID1, "QPGS2")
        .await
        .expect("Failed to read response");

    let qpgs = QPGS::new_from_string(&result);
    info!("inverter 2 QPGS: {:#?}", qpgs);

    println!("");
}
