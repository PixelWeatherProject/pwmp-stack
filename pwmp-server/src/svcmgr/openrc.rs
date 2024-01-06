use super::traits::ServiceManager;
use std::io::Result;

pub struct OpenRcManager;

impl ServiceManager for OpenRcManager {
    fn detect(&self) -> bool {
        false
    }

    fn check(&self) -> bool {
        false
    }

    fn install(&self) -> Result<()> {
        unimplemented!()
    }

    fn uninstall(&self) -> Result<()> {
        unimplemented!()
    }

    fn disable(&self) -> Result<()> {
        unimplemented!()
    }

    fn enable(&self) -> Result<()> {
        unimplemented!()
    }

    fn enabled(&self) -> Result<bool> {
        unimplemented!()
    }

    fn start(&self) -> Result<()> {
        unimplemented!()
    }

    fn stop(&self) -> Result<()> {
        unimplemented!()
    }
}
