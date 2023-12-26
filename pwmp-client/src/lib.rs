#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
use error::Error;
pub use pwmp_types;
use pwmp_types::{
    aliases::{AirPressure, BatteryVoltage, Humidity, Rssi, Temperature},
    datetime::DateTime,
    mac::Mac,
    multitype::SettingValue,
    request::Request,
    response::Response,
    setting::SettingName,
    Message,
};
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

const RCV_BUFFER_SIZE: usize = 128;
type Result<T> = ::std::result::Result<T, Error>;

/// Contains the [`Error`] type.
pub mod error;

/// PixelWeather Messaging Protocol Client.
pub struct PwmpClient(TcpStream);

impl PwmpClient {
    /// Create a new client by connecting to a PWMP server.
    /// # Errors
    /// If the server rejects the client (for eg. if it's unathorized)
    /// an `Err(Error::Reject)` is returned. An error is also returned
    /// if a generic I/O error occurred.
    pub fn new<A: ToSocketAddrs>(addr: A, mac: Mac) -> Result<Self> {
        let socket = TcpStream::connect(addr)?;
        let mut client = Self(socket);

        client.send_greeting(mac)?;

        Ok(client)
    }

    /// Try to ping the server. Returns whether the server responded correctly.
    /// On an I/O error, `false` is returned.
    pub fn ping(&mut self) -> bool {
        if self.send_request(Request::Ping).is_err() {
            return false;
        }

        let Ok(response) = self.await_response() else {
            return false;
        };

        response == Response::Pong
    }

    /// Get a setting value.
    /// # Errors
    /// Generic I/O.
    pub fn get_setting(&mut self, setting: SettingName) -> Result<SettingValue> {
        self.send_request(Request::GetSetting(setting))?;
        let response = self.await_response()?;

        let Response::Setting(value) = response else {
            return Err(Error::UnexpectedVariant);
        };

        Ok(value)
    }

    /// Get values of multiple settings.
    ///
    /// **Note:** This does not call [`get_setting()`](Self::get_setting) in a loop.
    ///
    /// # Errors
    /// Generic I/O.
    pub fn get_settings(&mut self, settings: &[SettingName]) -> Result<Vec<SettingValue>> {
        self.send_request(Request::GetSettings(settings.to_vec()))?;
        let response = self.await_response()?;

        let Response::Settings(values) = response else {
            return Err(Error::UnexpectedVariant);
        };

        Ok(values)
    }

    /// Post node measurements.
    /// # Errors
    /// Generic I/O.
    pub fn post_measurements(
        &mut self,
        temperature: Temperature,
        humidity: Humidity,
        air_pressure: Option<AirPressure>,
    ) -> Result<()> {
        self.send_request(Request::PostResults {
            temperature,
            humidity,
            air_pressure,
        })?;
        self.await_ok()?;

        Ok(())
    }

    /// Post node stats.
    /// # Errors
    /// Generic I/O.
    pub fn post_stats(
        &mut self,
        battery: BatteryVoltage,
        wifi_ssid: &str,
        wifi_rssi: Rssi,
    ) -> Result<()> {
        self.send_request(Request::PostStats {
            battery,
            wifi_ssid: wifi_ssid.to_string(),
            wifi_rssi,
        })?;
        self.await_ok()?;

        Ok(())
    }

    pub fn get_datetime(&mut self) -> Result<DateTime> {
        self.send_request(Request::DateTime)?;
        let response = self.await_response()?;

        let Response::DateTime(dt) = response else {
            return Err(Error::UnexpectedVariant);
        };

        Ok(dt)
    }

    fn send_greeting(&mut self, mac: Mac) -> Result<()> {
        self.send_request(Request::Hello { mac })?;
        self.await_ok()
    }

    fn send_request(&mut self, req: Request) -> Result<()> {
        self.0.write_all(&Message::Request(req).to_raw())?;
        self.0.flush()?;

        Ok(())
    }

    fn await_response(&mut self) -> Result<Response> {
        let mut buf = [0; RCV_BUFFER_SIZE];
        let read = self.0.read(&mut buf)?;

        let message = Message::from_raw(&buf[..read]).ok_or(Error::MessageParse)?;
        message.to_response().ok_or(Error::NotResponse)
    }

    fn await_ok(&mut self) -> Result<()> {
        let response = self.await_response()?;

        match response {
            Response::Ok => Ok(()),
            Response::Reject => Err(Error::Rejected),
            _ => Err(Error::NotResponse),
        }
    }

    fn connected(&mut self) -> bool {
        self.0.peek(&mut []).is_ok()
    }
}

impl Drop for PwmpClient {
    fn drop(&mut self) {
        let _ = self.send_request(Request::Bye);
        while self.connected() {}
    }
}
