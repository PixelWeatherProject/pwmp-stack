use crate::{client::Client, db::DatabaseClient, error::Error, CONFIG};
use chrono::{Datelike, Local, Timelike};
use log::{debug, warn};
use pwmp_types::{
    datetime::{Date, DateTime, Time},
    request::Request,
    response::Response,
    Message,
};
use std::net::TcpStream;

pub fn handle_client(client: TcpStream, db: &DatabaseClient) -> Result<(), Error> {
    let mut client = Client::new(client)?;

    if let Some(id) = db.authorize_device(client.mac()) {
        debug!("Device {} authorized as node #{id}", client.mac());
        client.set_id(id);
        client.send_response(Response::Ok)?;
    } else {
        warn!("Device {} is not authorized", client.mac());
        if !CONFIG.kick_unauthorized_devices {
            client.send_response(Response::Reject)?;
        }
        return Ok(());
    }

    loop {
        let request = client.await_request()?;

        if request == Request::Bye {
            debug!("{}: Bye", client.id());
            break;
        }

        let response = handle_request(request, &client, db).ok_or(Error::BadRequest)?;

        client.send_response(response)?;
    }

    Ok(())
}

fn handle_request(req: Request, client: &Client, db: &DatabaseClient) -> Option<Response> {
    debug!(
        "Handling {req:#?} ({} bytes)",
        Message::Request(req.clone()).size()
    );

    match req {
        Request::Ping => Some(Response::Pong),
        Request::Hello { .. } => {
            warn!("Received double `Hello` messages");
            None
        }
        Request::PostResults {
            temperature,
            humidity,
            air_pressure,
            battery,
        } => {
            debug!(
                "{}: {temperature}C, {humidity}%, {air_pressure:?}hPa",
                client.id()
            );
            db.post_results(client.id(), temperature, humidity, air_pressure, battery);
            Some(Response::Ok)
        }
        Request::GetSetting(setting) => db.get_setting(client.id(), setting).map_or_else(
            || {
                let default = setting.default_value();
                warn!(
                    "{}: {setting:?} is not set, returning default {default:?}",
                    client.id()
                );
                Some(Response::Setting(default))
            },
            |value| Some(Response::Setting(value)),
        ),
        Request::GetSettings(settings) => {
            let values = db.get_settings(client.id(), &settings);
            let mut results = Vec::with_capacity(values.len());

            for (i, value) in values.into_iter().enumerate() {
                let setting = settings[i];

                let result = value.map_or_else(
                    || {
                        let default = settings[i].default_value();
                        warn!(
                            "{}: {setting:?} is not set, returning default {default:?}",
                            client.id()
                        );
                        settings[i].default_value()
                    },
                    |value| value,
                );

                results.push(result);
            }

            Some(Response::Settings(results))
        }
        Request::DateTime => {
            let now = Local::now();

            Some(Response::DateTime(DateTime(
                Date {
                    year: now.year() as _,
                    month: now.month() as _,
                    day: now.day() as _,
                },
                Time(now.hour() as _, now.minute() as _, now.second() as _),
            )))
        }
        Request::Bye => unreachable!(),
    }
}
