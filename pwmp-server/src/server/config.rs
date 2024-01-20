use serde::{Deserialize, Serialize};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: Ipv4Addr,
    pub port: u16,
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub max_devices: u32,
    pub kick_unauthorized_devices: bool,
    pub max_settings: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: Ipv4Addr::new(0, 0, 0, 0),
            port: 55300,
            db_host: "192.168.0.12".to_string(),
            db_port: 5432,
            db_user: "root".to_string(),
            db_password: "root".to_string(),
            db_name: "pixelweather".to_string(),
            max_devices: 10,
            kick_unauthorized_devices: false,
            max_settings: 10,
        }
    }
}

impl Config {
    pub fn default_path() -> PathBuf {
        homedir::get_my_home()
            .unwrap()
            .unwrap()
            .join(".pwmp-server/config.yml")
    }

    pub const fn server_bind_addr(&self) -> SocketAddrV4 {
        SocketAddrV4::new(self.host, self.port)
    }
}
