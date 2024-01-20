use super::{client::Client, db::DatabaseClient, rate_limit::RateLimiter};
use crate::{error::Error, CONFIG};
use log::{debug, error, info, warn};
use pwmp_types::{aliases::MeasurementId, request::Request, response::Response, Message};
use std::{
    net::TcpStream,
    panic,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};

pub fn handle_client(
    client: TcpStream,
    db: &DatabaseClient,
    connection_count: Arc<AtomicU32>,
) -> Result<(), Error> {
    set_panic_hook(connection_count);
    let mut client = Client::new(client)?;
    let mut rate_limiter = RateLimiter::new(
        Duration::from_secs(CONFIG.rate_limits.time_frame),
        CONFIG.rate_limits.max_requests,
    );

    if let Some(id) = db.authorize_device(client.mac()) {
        info!("Device {} authorized as node #{id}", client.mac());
        client.set_id(id);
        client.send_response(Response::Ok)?;
    } else {
        warn!("Device {} is not authorized", client.mac());
        if !CONFIG.kick_unauthorized_devices {
            client.send_response(Response::Reject)?;
        }
        return Ok(());
    }

    let mut last_submit = None;

    loop {
        let request = client.await_request()?;

        if rate_limiter.hit() {
            error!("{}: Exceeded request limits", client.id());
            break;
        }

        if request == Request::Bye {
            info!("{}: Bye", client.id());
            break;
        }

        let response =
            handle_request(request, &client, db, &mut last_submit).ok_or(Error::BadRequest)?;

        client.send_response(response)?;
    }

    Ok(())
}

fn handle_request(
    req: Request,
    client: &Client,
    db: &DatabaseClient,
    last_submit: &mut Option<MeasurementId>,
) -> Option<Response> {
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
        } => {
            if last_submit.is_some() {
                error!(
                    "{}: Submitted multiple posts, which is not allowed",
                    client.id()
                );
                return None;
            }

            debug!(
                "{}: {temperature}C, {humidity}%, {air_pressure:?}hPa",
                client.id()
            );
            *last_submit = Some(db.post_results(client.id(), temperature, humidity, air_pressure));
            Some(Response::Ok)
        }
        Request::PostStats {
            ref battery,
            wifi_ssid,
            wifi_rssi,
        } => {
            let Some(last_measurement_id) = last_submit else {
                error!("{}: Missing measurement", client.id());
                return None;
            };

            db.post_stats(*last_measurement_id, battery, &wifi_ssid, wifi_rssi);
            Some(Response::Ok)
        }
        Request::SendNotification(message) => {
            db.create_notification(client.id(), &message);
            Some(Response::Ok)
        }
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

            Some(Response::Settings(results.into_boxed_slice()))
        }
        Request::Bye => unreachable!(),
    }
}

fn set_panic_hook(connection_count: Arc<AtomicU32>) {
    let default = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        warn!("A client thread has paniced");
        default(panic_info);
        connection_count.fetch_sub(1, Ordering::Relaxed);
    }));
}
