use crate::{
    aliases::{AirPressure, BatteryVoltage, Humidity, Temperature},
    mac::Mac,
    setting::SettingName,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Request {
    Ping,
    Hello {
        mac: Mac,
    },
    PostResults {
        temperature: Temperature,
        humidity: Humidity,
        air_pressure: Option<AirPressure>,
        battery: BatteryVoltage,
    },
    DateTime,
    Bye,
    GetSetting(SettingName),
    GetSettings(Vec<SettingName>),
}
