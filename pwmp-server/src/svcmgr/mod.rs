use self::{openrc::OpenRcManager, systemd::SystemdManager, traits::ServiceManager};
use crate::cli::ServiceCommand;
use log::{error, info, warn};
use std::{io, process::exit};

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
            perform_cmd(|| manager.install(), "install", "installed");
            warn!("The service must be enabled manually");
        }
        ServiceCommand::Uninstall => {
            perform_cmd(|| manager.uninstall(), "uninstall", "uninstalled");
        }
        ServiceCommand::Enable => {
            perform_cmd(|| manager.enable(), "enable", "enabled");
        }
        ServiceCommand::Disable => {
            perform_cmd(|| manager.disable(), "disable", "disabled");
        }
        ServiceCommand::Start => {
            perform_cmd(|| manager.start(), "start", "started");
        }
        ServiceCommand::Stop => {
            perform_cmd(|| manager.stop(), "stop", "stopped");
        }
        ServiceCommand::Reinstall => {
            svcmgr_main(ServiceCommand::Reinstall);
        }
    }
}

fn perform_cmd<F: FnOnce() -> io::Result<()>>(func: F, action_name: &str, action_past: &str) {
    if let Err(why) = func() {
        error!("Failed to {action_name} service: {why}");
        exit(1);
    }

    info!("Service {action_past} successfully");
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
