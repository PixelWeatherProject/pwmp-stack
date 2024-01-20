use crate::{
    server::{db::DatabaseClient, server_handle::server_loop},
    CONFIG,
};
use log::{error, info};
use std::{net::TcpListener, process::exit};

mod client;
mod client_handle;
pub mod config;
pub mod db;
pub mod rate_limit;
pub mod server_handle;

pub fn server_main() {
    info!("Connecting to database at {}", CONFIG.database.host);
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
