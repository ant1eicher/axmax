use data::{invdata::InvData, qpgs::QPGS, qpigs::QPIGS};
use log::{error, info};

pub mod data;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Retrieving QPIGS");
    match QPIGS::retrieve(data::invdata::Command::QPIGS).await {
        Ok(data) => {
            info!("Received: {:?}", data);
        }
        Err(e) => {
            error!("Unable to retrieve: {}", e);
        }
    }
    info!("Retrieving QPGS1");
    match QPGS::retrieve(data::invdata::Command::QPGS1).await {
        Ok(data) => {
            info!("Received: {:?}", data);
        }
        Err(e) => {
            error!("Unable to retrieve: {}", e);
        }
    }
    info!("Retrieving QPGS2");
    match QPGS::retrieve(data::invdata::Command::QPGS2).await {
        Ok(data) => {
            info!("Received: {:?}", data);
        }
        Err(e) => {
            error!("Unable to retrieve: {}", e);
        }
    }
}
