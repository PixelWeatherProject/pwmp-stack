#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
use error::Error;
pub use pwmp_types;
use pwmp_types::{
    aliases::{AirPressure, BatteryVoltage, Humidity, Rssi, Temperature},
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
    time::Duration,
};

const RCV_BUFFER_SIZE: usize = 96;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
const READ_TIMEOUT: Duration = Duration::from_secs(4);
const WRITE_TIMEOUT: Duration = Duration::from_secs(4);
type Result<T> = ::std::result::Result<T, Error>;

/// Contains the [`Error`] type.
pub mod error;

#[allow(clippy::doc_markdown)]
/// PixelWeather Messaging Protocol Client.
pub struct PwmpClient(TcpStream);

impl PwmpClient {
    /// Create a new client by connecting to a PWMP server.
    ///
    /// # Errors
    /// If the server rejects the client (for eg. if it's unathorized)
    /// an `Err(Error::Reject)` is returned. An error is also returned
    /// if a generic I/O error occurred.
    pub fn new<A: ToSocketAddrs>(addr: A, mac: Mac) -> Result<Self> {
        let addr = addr.to_socket_addrs()?.next().unwrap();
        let socket = TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT)?;

        socket.set_read_timeout(Some(READ_TIMEOUT))?;
        socket.set_write_timeout(Some(WRITE_TIMEOUT))?;

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

    /// Get values of multiple settings.
    ///
    /// # Errors
    /// Generic I/O.
    #[allow(clippy::items_after_statements)]
    pub fn get_settings<const N: usize>(
        &mut self,
        settings: [SettingName; N],
    ) -> Result<[SettingValue; N]> {
        self.send_request(Request::GetSettings(settings.into()))?;
        let response = self.await_response()?;

        let Response::Settings(values) = response else {
            return Err(Error::UnexpectedVariant);
        };

        if values.len() != N {
            return Err(Error::MalformedResponse);
        }

        const ARRAY_REPEAT_VALUE: SettingValue = SettingValue::Number(0);
        let mut array = [ARRAY_REPEAT_VALUE; N];

        for (i, value) in values.iter().cloned().enumerate() {
            array[i] = value;
        }

        Ok(array)
    }

    /// Post node measurements.
    ///
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
    ///
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
            wifi_ssid: wifi_ssid.into(),
            wifi_rssi,
        })?;
        self.await_ok()?;

        Ok(())
    }

    /// Send a text notification with the specified content.
    ///
    /// # Errors
    /// Generic I/O.
    pub fn send_notification(&mut self, content: &str) -> Result<()> {
        assert!(content.len() <= 64, "Message content too large");
        self.send_request(Request::SendNotification(content.into()))?;
        self.await_ok()?;

        Ok(())
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

    fn connected(&self) -> bool {
        if let Ok(amount) = self.0.peek(&mut []) {
            return amount > 0;
        }

        false
    }
}

impl Drop for PwmpClient {
    fn drop(&mut self) {
        let _ = self.send_request(Request::Bye);
        while self.connected() {}
    }
}
