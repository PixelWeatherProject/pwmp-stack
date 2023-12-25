#![allow(clippy::cast_sign_loss)]
use crate::{config::Config, db::DatabaseClient, server_handle::server_loop};
use always_cell::AlwaysCell;
use clap::Parser;
use log::{debug, error, info};
use simple_logger::SimpleLogger;
use std::{net::TcpListener, process::exit};
use time::macros::format_description;

mod cli;
mod client;
mod client_handle;
mod config;
mod db;
mod error;
mod server_handle;

static CONFIG: AlwaysCell<Config> = AlwaysCell::new();

fn main() {
    let args = cli::Cli::parse();
    SimpleLogger::new()
        .with_timestamp_format(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .init()
        .unwrap();

    info!("PixelWeather Server v{}", env!("CARGO_PKG_VERSION"));
    debug!("Arguments: {args:?}");

    let config_path = args.config.unwrap_or_else(Config::default_path);
    info!("Loading config from {}", config_path.display());

    let config: Config = match confy::load_path(config_path) {
        Ok(config) => config,
        Err(why) => {
            error!("Failed to load configuration: {why}");
            exit(1);
        }
    };
    AlwaysCell::<Config>::set(&CONFIG, config);

    info!("Connecting to database at {}", CONFIG.db_host);
    let Ok(db) = DatabaseClient::new(&CONFIG) else {
        error!("Failed to connect to database");
        exit(1);
    };

    let Ok(server) = TcpListener::bind(CONFIG.server_bind_addr()) else {
        eprintln!("Failed to bind to {}", CONFIG.server_bind_addr());
        exit(1);
    };

    info!("Server started on {}", CONFIG.server_bind_addr());

    server_loop(&server, db);
}
