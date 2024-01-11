use crate::{
    aliases::{AirPressure, BatteryVoltage, Humidity, Rssi, Temperature},
    mac::Mac,
    setting::SettingName,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Request {
    Ping,
    Hello {
        mac: Mac,
    },
    PostResults {
        temperature: Temperature,
        humidity: Humidity,
        air_pressure: Option<AirPressure>,
    },
    PostStats {
        battery: BatteryVoltage,
        wifi_ssid: Box<str>,
        wifi_rssi: Rssi,
    },
    SendNotification(Box<str>),
    Bye,
    GetSetting(SettingName),
    GetSettings(Box<[SettingName]>),
}
