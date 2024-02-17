# PixelWeather Messaging Protocol (PWMP) Stack
This repo contains:
- [PWMP Server & CLI](pwmp-server/)
- [Shared types](pwmp-types/)
- [PWMP Client library](pwmp-client/)

# Protocol description
PWMP allows nodes to communicate with the database in a fast and type-safe way. It uses *messages* to exchange information. The server is responsible for handling these messages and communicating with the database. The client library is used to create and send messages to the server.

A more in-depth description of the protocol can be found [here](pwmp-types/README.md).

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
The current version of the server does not enforce security measures beyond checking if the node is authorized to communicate with the server.

It should be noted that no sensitive data is exchanged between the server and the nodes. The server does not store any sensitive data.

Since PWOS only supports encrypted WiFi networks, all communication is encrypted to anyone outside the network.