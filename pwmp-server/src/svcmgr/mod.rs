use self::{openrc::OpenRcManager, systemd::SystemdManager, traits::ServiceManager};
use crate::cli::ServiceCommand;
use log::{error, info, warn};
use std::process::exit;

mod openrc;
mod systemd;
mod traits;

pub fn svcmgr_main(cmd: ServiceCommand) {
    let manager = detect_manager();

    match cmd {
        ServiceCommand::Check => match (manager.check(), manager.enabled()) {
            (true, Ok(false)) => info!("Service is installed but not enabled"),
            (true, Ok(true)) => info!("Service is installed and enabled"),
            (true, Err(why)) => info!("Service is installed but may be corrupt ({why})"),
            (false, _) => info!("Service is not installed"),
        },
        ServiceCommand::Install => {
            if let Err(why) = manager.install() {
                error!("Failed to install service: {why}");
                exit(1);
            }

            info!("Service installed successfully");
            warn!("The service must be enabled manually");
        }
        ServiceCommand::Uninstall => {
            if let Err(why) = manager.uninstall() {
                error!("Failed to uninstall service: {why}");
                exit(1);
            }

            info!("Service uninstalled successfully");
        }
        ServiceCommand::Enable => {
            if let Err(why) = manager.enable() {
                error!("Failed to enable service: {why}");
                exit(1);
            }

            info!("Service enabled successfully");
        }
        ServiceCommand::Disable => {
            if let Err(why) = manager.disable() {
                error!("Failed to disable service: {why}");
                exit(1);
            }

            info!("Service disabled successfully");
        }
        ServiceCommand::Start => {
            if let Err(why) = manager.start() {
                error!("Failed to start service: {why}");
                exit(1);
            }

            info!("Service started successfully");
        }
        ServiceCommand::Stop => {
            if let Err(why) = manager.start() {
                error!("Failed to stop service: {why}");
                exit(1);
            }

            info!("Service stopped successfully");
        }
        ServiceCommand::Reinstall => {
            svcmgr_main(ServiceCommand::Reinstall);
        }
    }
}

fn detect_manager() -> Box<dyn ServiceManager> {
    if SystemdManager.detect() {
        return Box::new(SystemdManager);
    } else if OpenRcManager.detect() {
        return Box::new(OpenRcManager);
    }

    error!("Could not find a service manager on this system");
    exit(1);
}
