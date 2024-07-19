#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::module_name_repetitions
)]
use crate::{
    cli::Command,
    server::{config::Config, server_main},
    svcmgr::svcmgr_main,
};
use always_cell::AlwaysCell;
use clap::Parser;
use log::{debug, error, info};
use simple_logger::SimpleLogger;
use std::process::exit;
use time::macros::format_description;

mod cli;
mod error;
mod server;
mod svcmgr;
mod tester;

static CONFIG: AlwaysCell<Config> = AlwaysCell::new();

fn main() {
    let args = cli::Cli::parse();

    let logger = SimpleLogger::new().with_timestamp_format(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ));

    #[cfg(not(debug_assertions))]
    let logger = logger.with_level(log::LevelFilter::Info);
    #[cfg(debug_assertions)]
    let logger = logger.with_level(log::LevelFilter::Debug);

    logger.init().unwrap();

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

    match args.command {
        Some(Command::Service { command }) => svcmgr_main(command),
        Some(Command::Test { host, mac, port }) => tester::test(host, port, mac),
        None => server_main(),
    }
}
