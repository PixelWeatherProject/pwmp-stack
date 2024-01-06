use std::io::Result;

pub trait ServiceManager {
    fn detect(&self) -> bool;
    fn check(&self) -> bool;
    fn install(&self) -> Result<()>;
    fn uninstall(&self) -> Result<()>;
    fn enabled(&self) -> Result<bool>;
    fn enable(&self) -> Result<()>;
    fn disable(&self) -> Result<()>;
    fn start(&self) -> Result<()>;
    fn stop(&self) -> Result<()>;
}
