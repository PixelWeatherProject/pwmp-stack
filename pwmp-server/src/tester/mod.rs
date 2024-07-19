use log::{error, info};
use pwmp_client::PwmpClient;
use pwmp_types::mac::Mac;
use std::str::FromStr;

pub fn test(host: String, port: Option<u16>, raw_mac: String) {
    let Ok(mac) = Mac::from_str(&raw_mac) else {
        error!("Invalid MAC address format");
        return;
    };

    let full_addr = format!("{}:{}", host, port.unwrap_or(5432));

    match PwmpClient::new(full_addr, mac) {
        Ok(_) => info!("Client connected successfully!"),
        Err(why) => error!("Failed to test connection: {why}"),
    };
}
