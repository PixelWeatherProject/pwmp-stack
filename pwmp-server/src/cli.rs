use clap::{command, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Service management
    Service {
        #[command(subcommand)]
        command: ServiceCommand,
    },
}

#[derive(Debug, Subcommand, Clone, Copy)]
pub enum ServiceCommand {
    /// Start the service
    Start,
    /// Stop the service
    Stop,
    /// Enable
    Enable,
    /// Disable
    Disable,
    /// Install as service
    Install,
    /// Uninstall service
    Uninstall,
    /// Check if service is installed
    Check,
    /// Reinstall service
    Reinstall,
}
