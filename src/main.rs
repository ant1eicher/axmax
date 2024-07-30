use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_cloudwatch::{
    types::{MetricDatum, StandardUnit},
    Client,
};
use data::{
    qpgs::QPGS,
    qpigs::QPIGS,
    usb::{fetch_command_data_usb, RAW_HID0, RAW_HID1},
};
use log::info;

pub mod data;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Load the AWS configuration
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region("af-south-1")
        .load()
        .await;

    // Create the CloudWatch client
    let client = Client::new(&config);

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

    let i1qpigs = QPIGS::new_from_string(&result).expect("Failed to parse");
    info!("nverter 1 QPIGS: {:#?}", i1qpigs);

    let result = fetch_command_data_usb(RAW_HID1, "QPGS1")
        .await
        .expect("Failed to read response");

    let i1qpgs = QPGS::new_from_string(&result).expect("Failed to parse");
    info!("inverter 1 QPGS: {:#?}", i1qpgs);

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

    let i2qpigs = QPIGS::new_from_string(&result).expect("Failed to parse");
    info!("inverter 2 QPIGS: {:#?}", i2qpigs);

    let result = fetch_command_data_usb(RAW_HID1, "QPGS2")
        .await
        .expect("Failed to read response");

    let i2qpgs = QPGS::new_from_string(&result).expect("Failed to parse");
    info!("inverter 2 QPGS: {:#?}", i2qpgs);

    // Define the metric data
    let i1pv1watts = MetricDatum::builder()
        .metric_name("Inverter1Pv1Watts")
        .unit(StandardUnit::Count)
        .value(i1qpigs.pv1_charging_power as f64)
        .build();

    let i1pv2watts = MetricDatum::builder()
        .metric_name("Inverter1Pv2Watts")
        .unit(StandardUnit::Count)
        .value(i1qpgs.pv2_charging_power as f64)
        .build();

    let i2pv1watts = MetricDatum::builder()
        .metric_name("Inverter2Pv1Watts")
        .unit(StandardUnit::Count)
        .value(i2qpigs.pv1_charging_power as f64)
        .build();

    let i2pv2watts = MetricDatum::builder()
        .metric_name("Inverter2Pv2Watts")
        .unit(StandardUnit::Count)
        .value(i2qpgs.pv2_charging_power as f64)
        .build();

    let battery_percentage = MetricDatum::builder()
        .metric_name("BatteryPercentage")
        .unit(StandardUnit::Percent)
        .value(i1qpigs.battery_capacity_percent as f64)
        .build();

    let load_percentage = MetricDatum::builder()
        .metric_name("LoadPercentage")
        .unit(StandardUnit::Percent)
        .value(i1qpigs.load_percent as f64)
        .build();

    let load_watts = MetricDatum::builder()
        .metric_name("LoadWatts")
        .unit(StandardUnit::Count)
        .value((i1qpigs.ac_output_watts + i2qpigs.ac_output_watts) as f64)
        .build();

    // Send the metric data to CloudWatch
    let response = client
        .put_metric_data()
        .namespace("Home/Solar")
        .set_metric_data(Some(vec![
            i1pv1watts,
            i1pv2watts,
            i2pv1watts,
            i2pv2watts,
            battery_percentage,
            load_percentage,
            load_watts,
        ]))
        .send()
        .await?;

    info!("PutMetricData response: {:?}", response);

    Ok(())
}
