use crate::config::Config;
use log::error;
use pwmp_types::{
    aliases::{AirPressure, BatteryVoltage, Humidity, MeasurementId, Rssi, Temperature},
    mac::Mac,
    multitype::SettingValue,
    setting::SettingName,
    NodeId,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use tokio::runtime::Runtime;

pub struct DatabaseClient(Runtime, Pool<Postgres>);

impl DatabaseClient {
    pub fn new(config: &Config) -> sqlx::Result<Self> {
        let rt = Runtime::new().unwrap();

        let pool = rt.block_on(async {
            PgPoolOptions::new()
                .max_connections(3)
                .connect(&format!(
                    "postgres://{}:{}@{}:{}/{}",
                    config.db_user,
                    config.db_password,
                    config.db_host,
                    config.db_port,
                    config.db_name
                ))
                .await
        })?;

        Ok(Self(rt, pool))
    }

    pub fn authorize_device(&self, mac: &Mac) -> Option<NodeId> {
        let mac = mac.to_string();

        let result = self.rt().block_on(async {
            sqlx::query_file!("queries/get_device_by_mac.sql", mac)
                .fetch_one(self.pool())
                .await
        });

        match result {
            Ok(res) => Some(res.id),
            Err(sqlx::Error::RowNotFound) => None,
            Err(why) => panic!("Database error: {why}"),
        }
    }

    pub fn get_setting(&self, id: NodeId, setting: SettingName) -> Option<SettingValue> {
        let name = setting.name();
        let query = format!("SELECT {name} FROM settings WHERE node = $1");

        let result = self
            .rt()
            .block_on(async { sqlx::query(&query).bind(id).fetch_one(self.pool()).await });

        if let Err(why) = result {
            match why {
                sqlx::Error::RowNotFound => {
                    return None;
                }
                why => {
                    error!("Failed to fetch setting {setting:?} for node {id}: {why}");
                    return None;
                }
            }
        }

        let row = result.unwrap();
        let value = match setting {
            SettingName::BatteryIgnore
            | SettingName::Ota
            | SettingName::Sbop
            | SettingName::MuteNotifications => row.get::<bool, _>(0).into(),
            SettingName::SleepTime => (row.get::<i16, _>(0) as u16).into(),
            SettingName::DeviceSpecific => unimplemented!(),
        };

        Some(value)
    }

    pub fn get_settings(&self, id: NodeId, settings: &[SettingName]) -> Vec<Option<SettingValue>> {
        let columns = settings
            .iter()
            .map(|setting| setting.name())
            .collect::<Vec<&str>>()
            .join(", ");
        let query = format!("SELECT {columns} FROM settings WHERE node = $1");
        let mut results = vec![None; settings.len()];

        let result = self
            .rt()
            .block_on(async { sqlx::query(&query).bind(id).fetch_one(self.pool()).await });

        if let Err(why) = result {
            error!("DB error: {why}");
        } else {
            let row = result.unwrap();

            for (i, setting) in settings.iter().enumerate() {
                let value: SettingValue = match setting {
                    SettingName::BatteryIgnore
                    | SettingName::Ota
                    | SettingName::Sbop
                    | SettingName::MuteNotifications => row.get::<bool, _>(i).into(),
                    SettingName::SleepTime => (row.get::<i16, _>(i) as u16).into(),
                    SettingName::DeviceSpecific => unimplemented!(),
                };

                results[i] = Some(value);
            }
        }

        results
    }

    #[allow(
        clippy::needless_pass_by_value,
        clippy::cast_possible_wrap,
        clippy::cast_lossless
    )]
    pub fn post_results(
        &self,
        node: NodeId,
        temp: Temperature,
        hum: Humidity,
        air_p: Option<AirPressure>,
    ) -> MeasurementId {
        self.rt()
            .block_on(async {
                sqlx::query_file!(
                    "queries/post_results.sql",
                    node,
                    temp,
                    hum as i16,
                    air_p.map(|value| value as i16)
                )
                .fetch_one(self.pool())
                .await
                .unwrap()
            })
            .id as u16
    }

    pub fn post_stats(
        &self,
        measurement: MeasurementId,
        battery: &BatteryVoltage,
        wifi_ssid: &str,
        wifi_rssi: Rssi,
    ) {
        self.rt().block_on(async {
            sqlx::query_file!(
                "queries/post_stats.sql",
                measurement as i16,
                battery,
                wifi_ssid,
                wifi_rssi as i16
            )
            .execute(self.pool())
            .await
            .unwrap()
        });
    }

    const fn rt(&self) -> &Runtime {
        &self.0
    }

    const fn pool(&self) -> &Pool<Postgres> {
        &self.1
    }
}
