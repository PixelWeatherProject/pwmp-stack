# PWMP Server
This crate contains the PixelWeather Messaging Protocol Server and CLI.

# Server configuration
The server will read the configuration file from `~/.config/pwmp-server/config.yml`. If it doesn't exist, it'll be created. Optionally, you can provide a `--config` flag, with an alternative path.

## Defaults
```yml
# Server binding
host: "0.0.0.0"
port: 55300

# Database connection settings.
# PostgreSQL is the only supported database.
db_host: "123.456.789.012"
db_port: 5432
db_user: "root"
db_password: "root"
db_name: "pixelweather"

# Maximum number of devices that can be connected at the same time.
# This limit cannot be disabled.
max_devices: 10

# Sets how the server will handle unauthorized devices when anonymous_devices is set to false.
# If set to true, the server will not respond to the device and will just close the socket.
# If set to false, the server will respond with a `Reject` message and then close the socket.
kick_unauthorized_devices: false

# Sets how many settings can be requested using the `Message::GetSettings` message.
# This limit cannot be disabled.
max_settings: 10
```

# Using as a service
The server can be configured as a background service. The CLI has a `service` subcommand, which allows managing said service.

Service management is only supported on **Linux** systems using **Systemd**. There is a boilerplate implementation for **OpenRC**, but it's not supported yet.

The service command can be used to:
- Install and uninstall the service
- Enable/disable the service on boot
- Check the status of the service
- Start/stop the service