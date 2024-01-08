# PixelWeather Messaging Protocol (PWMP) Stack
This repo contains:
- [PWMP Server & CLI](pwmp-server/)
- [Shared types](pwmp-types/)
- [PWMP Client library](pwmp-client/)

# Protocol description
PWMP allows nodes to communicate with the database in a fast and type-safe way. It works in a similar way as MQTT, by utilizing "messages" between the client and the server, but doesn't use a publish-subscribe model, instead just a simple client-server send/receive.

READMEs can be found in each crate separately.

# Database
The PWMP server uses a PostgreSQL database. It's recommended to use the latest version (16.x+). You must prepare your database by running the `create_schema.sql` script.

The [examples](pwmp-client/examples/) use a dummy MAC address which you'll need to add to the database to run them:
```sql
INSERT INTO devices("id", "mac_address", "note")
VALUES ('1', '01:02:03:04:05:06', 'testing device');
INSERT INTO settings("node")
VALUES (1);
```

# Security
The server is currently not designed to force secure communication between the database and itself, nor between the nodes and itself. There are ways to add additional protection for the database, for eg. by using both the server and the database in Docker container(s). The database does (and should) **not** be exposed. It should be only accessible by the server itself.

PWMP does not exchange any sensitive data, and the firmware for the nodes requires using encrypted WiFi networks, so all communication is encrypted. Due to this, no additional security is planned as of now.