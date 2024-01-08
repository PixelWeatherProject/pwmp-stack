# PWMP - Shared types
This crate contains the actual message definitions, de/serialization and other methods to interact with messages.

# Message structure
A "message" is a simple `enum` that can be one of two variants:
- `Request`
    - Server requested a resource from the client (node) or vice-versa.
- `Response`
    - Contains the resource requested by the other party.

Requests and responses contain deeper level variants.

```mermaid
graph TD;
    Message-->Request
    Message-->Response

    Request-.->Ping
    Request-.->Hello
    Request-.->PostResults
    Request-.->PostStats
    Request-.->Bye
    Request-.->GetSetting
    Request-.->GetSettings

    Response-.->Pong
    Response-.->Ok
    Response-.->Reject
    Response-.->Setting
    Response-.->Settings

    Ping-.-Pong
```

# Example communication sequence
```mermaid
sequenceDiagram
    Node->>Server: Hello (incl. MAC address)
    Server->>Node: Ok
    Node->>Server: GetSettings [...]
    Server->>Node: Settings [...]
    Node->>Server: PostResults [temperature, humidity, ...]
    Server->>Node: Ok
    Node->>Server: PostStats (node statistics)
    Server->>Node: Ok
    Node->>Server: Bye
```

If the node's MAC address is not in the database, it's not authorized to communicate with the server.
```mermaid
sequenceDiagram
    Node->>Server: Hello (incl. MAC address)
    Server->>Node: Reject
```

It's also possible to configure the server to abruptly close the socket if the device is unauthorized, instead of sending a `Reject` response.

# Message rules
The node shall only send **one** `PostResults` message, duplicates will be rejected and the socket will be abruptly closed. The communication between nodes and the server should be exactly as specified in the diagram above. No more messages should be exchanged.

When the client (node) is done communicating with the server, it shall **always**:
1. Send a `Bye` request to the server.
2. **Wait** until the server closes the connection.

The [client library](../pwmp-client/) will guarantee the last two two requirements, but not the first one.