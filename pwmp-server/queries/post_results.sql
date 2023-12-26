INSERT INTO measurements(
        "node",
        "temperature",
        "humidity",
        "air_pressure",
        "battery"
    )
VALUES ($1, $2, $3, $4, $5);