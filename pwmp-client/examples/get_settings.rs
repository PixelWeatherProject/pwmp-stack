use pwmp_client::PwmpClient;
use pwmp_types::{mac::Mac, setting::SettingName};

fn main() {
    let mut client = PwmpClient::new("127.0.0.1:55300", Mac::new(1, 2, 3, 4, 5, 6)).unwrap();
    let settings = [
        SettingName::BatteryIgnore,
        SettingName::Ota,
        SettingName::SleepTime,
        SettingName::Sbop,
        SettingName::MuteNotifications,
    ];
    let values = client.get_settings(settings).unwrap();

    for i in 0..settings.len() {
        println!("{:?} => {:?}", settings[i], values[i]);
    }
}
