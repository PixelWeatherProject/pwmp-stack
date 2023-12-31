/* CREATE DATABASE "pixelweather"; */
CREATE TABLE devices (
    id SMALLSERIAL PRIMARY KEY,
    mac_address VARCHAR(17) UNIQUE NOT NULL CHECK (mac_address ~ E'^([0-9A-F]{2}:){5}[0-9A-F]{2}$'),
    location POINT DEFAULT NULL,
    note VARCHAR(16) DEFAULT NULL
);
CREATE TABLE measurements (
    id SMALLSERIAL PRIMARY KEY,
    node INT2 NOT NULL REFERENCES devices(id),
    "when" TIMESTAMP UNIQUE NOT NULL DEFAULT NOW(),
    temperature DECIMAL(4, 2) NOT NULL,
    humidity SMALLINT NOT NULL CHECK (
        humidity >= 0
        AND humidity <= 100
    ),
    air_pressure SMALLINT DEFAULT NULL
);
CREATE TABLE statistics (
    id SMALLSERIAL PRIMARY KEY,
    measurement INT2 NOT NULL REFERENCES measurements(id),
    battery DECIMAL(3, 2) NOT NULL CHECK (battery > 0),
    wifi_ssid VARCHAR(32) NOT NULL,
    wifi_rssi INT2 NOT NULL
);
CREATE TABLE settings (
    id SMALLSERIAL PRIMARY KEY,
    node INT2 UNIQUE NOT NULL REFERENCES devices(id),
    battery_ignore BOOLEAN NOT NULL DEFAULT FALSE,
    ota BOOLEAN NOT NULL DEFAULT FALSE,
    sleep_time INT2 NOT NULL DEFAULT 60 CHECK (sleep_time > 0),
    sbop BOOLEAN NOT NULL DEFAULT TRUE,
    mute_notifications BOOLEAN NOT NULL DEFAULT FALSE,
    device_specific JSON NOT NULL DEFAULT '{}'::json
);

/* Web */
CREATE TABLE web_users (
    id SMALLSERIAL PRIMARY KEY,
    username VARCHAR(16) NOT NULL CHECK (LENGTH(username) > 0),
    password VARCHAR(16) NOT NULL CHECK (LENGTH(password) > 0),
    is_admin BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE TABLE web_sessions(
    id SMALLSERIAL PRIMARY KEY,
    "user" INT2 NOT NULL REFERENCES web_users(id),
    token VARCHAR(16) NOT NULL CHECK (LENGTH(token) > 0),
    expiry TIMESTAMP UNIQUE NOT NULL DEFAULT NOW() + INTERVAL '12 hours'
);
