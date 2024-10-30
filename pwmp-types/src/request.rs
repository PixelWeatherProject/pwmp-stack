use crate::{
    aliases::{AirPressure, BatteryVoltage, Humidity, Rssi, Temperature},
    mac::Mac,
    setting::SettingName,
};
use serde::{Deserialize, Serialize};

/// A request message used by nodes to ask the PWMP server to perform an operation.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Request {
    /// Used to check if the server is alive.
    Ping,

    /// Tell the server that the session is over and the node will disconnect.
    Bye,

    /// Ask to server to authorize the node using it's MAC address.
    Hello {
        #[allow(clippy::doc_markdown)]
        /// The node's MAC address. This address should be that of the WiFi interface.
        mac: Mac,
    },

    /// Post measurement results to the database.
    PostResults {
        /// Temperature
        temperature: Temperature,
        /// Humidity
        humidity: Humidity,
        /// Air pressure *(if supported by the node)*
        air_pressure: Option<AirPressure>,
    },

    /// Post node statistics to the database.
    PostStats {
        /// Node's battery voltage
        battery: BatteryVoltage,
        /// ESSID of the wireless network
        wifi_ssid: Box<str>,
        /// RSSI *(signal quality)* of the connection to the wireless network in dBm *(decibel meters)*.
        /// Higher values (closer to 0) mean a better quality.
        wifi_rssi: Rssi,
    },

    /// Store a notification in the database. These can be read by other applications.
    ///
    /// Notifications are usually used to inform about low battery status.
    SendNotification(Box<str>),

    /// Retrieve the node's settings from the database.
    GetSettings(Box<[SettingName]>),
}
