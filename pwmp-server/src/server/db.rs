use super::config::Config;
use log::error;
use pwmp_types::{
    aliases::{AirPressure, BatteryVoltage, Humidity, MeasurementId, Rssi, Temperature},
    mac::Mac,
    multitype::SettingValue,
    setting::SettingName,
    NodeId,
};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    Pool, Postgres, Row,
};
use tokio::runtime::Runtime;

pub struct DatabaseClient(Runtime, Pool<Postgres>);

impl DatabaseClient {
    pub fn new(config: &Config) -> sqlx::Result<Self> {
        let rt = Runtime::new().unwrap();
        let mut opts = PgConnectOptions::new()
            .host(&config.database.host)
            .port(config.database.port)
            .username(&config.database.user)
            .password(&config.database.password)
            .database(&config.database.name);

        if config.database.ssl {
            opts = opts.ssl_mode(PgSslMode::Require);
        }

        let pool = rt.block_on(async {
            PgPoolOptions::new()
                .max_connections(3)
                .connect_with(opts)
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

    pub fn create_notification(&self, id: NodeId, content: &str) {
        let result = self.rt().block_on(async {
            sqlx::query_file!("queries/create_notification.sql", id, content)
                .execute(self.pool())
                .await
        });

        if let Err(why) = result {
            error!("Failed to create notification: {why}");
        }
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
