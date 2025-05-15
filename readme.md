# Rust MQTT clients evaluation

This repo is a subjective evaluation of the following MQTT clients for use in an embedded Linux environment:

- [rumqtt](https://docs.rs/rumqttc/latest/rumqttc/)
- [paho-mqtt](https://docs.rs/paho-mqtt/latest/paho_mqtt/)

## Conclusion

*paho-mqtt* offers two significant advantages:

* use of system libraries
  `paho-mqtt` can leverage system-provided libraries when built with the `bundled` feature **disabled** and the `ssl` feature **enabled**. This setup requires both *paho-mqtt* and *openssl* to be available on the system.

* Transport status
  Compared to *rumqtt*, *paho-mqtt* provides more direct feedback on the underlying connection. Its `publish` function offers immediate feedback on the connection status. In contrast, *rumqtt*'s `publish()` method simply enqueues the message to be sent asynchronously in another thread. As a result, any connection or send errors must be handled during the `poll()` call. Unfortunately, this means the `publish()` function in *rumqtt* does not provide concrete or immediate information about the current transport (e.g. TCP/TLS) state.

Note on manual *ack* feature available in *rumqtt* but not in *paho-mqtt*:
Allowing the user to manually manage acks has huge implications because the application must
handle the ack in a manner that respects the MQTT protocol.
This is discussed here: <https://github.com/eclipse-paho/paho.mqtt.rust/issues/141#issuecomment-2305487832>

## Configuration

- Disabling `paho-mqtt/bundled` feature will requires to install the `libpaho-mqtt` library 
  (`sudo apt install libpaho-mqtt-dev` on Debian/Ubuntu).
- Configure a password with `mosquitto_passwd passwd testuser` command.

## Run the broker:

1. Generate client and broker certificates: with `./certs.sh` script.
2. Build or retrieve `mosquitto` binary or docker image.
3. Run the broker:
    - Unsecure configuration (port 1883):

    ```bash
    mosquitto -c mosquitto.conf
    ```
    - Secure configuration (port 8883 with TLS + authentication + authorization):
    ```
    mosquitto -c mosquitto-secure.conf
    ```

## Run the clients:

For each generated output:
1. The sample is started first
2. The broker is started afterwards (after ~5s)
3. The broker is then stopped after (~5s)
4. The sample is stopped afterwards

### Synchronous `rumqtt` client (no TLS)

Sample: [src/sync-rumqttc.rs](src/sync-rumqttc.rs)

- Run the synchronous `rumqtt` client with `mosquitto.conf` configuration:
```bash
cargo run --release --bin sync-rumqttc
```

Output:

- We notice that the `publish()` function always returns `Ok(())` even if the broker is not available.

```
lucas@zgw:~/projects/rust-mqtt-eval$ cargo run --release --bin sync-rumqttc
   Compiling rust-mqtt-eval v0.1.0 (/home/lucas/projects/rust-mqtt-eval)
    Finished `release` profile [optimized] target(s) in 0.42s
     Running `target/release/sync-rumqttc`
Publish result = Ok(())
0. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
1. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
2. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Publish result = Ok(())
3. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
4. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
5. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Publish result = Ok(())
6. Notification = Incoming(ConnAck(ConnAck { session_present: false, code: Success }))
2025-05-15T19:59:36.492Z DEBUG [rumqttc::state] Publish. Topic = hello/0/world, Pkid = 0, Payload Size = 0
7. Notification = Outgoing(Publish(0))
2025-05-15T19:59:36.492Z DEBUG [rumqttc::state] Publish. Topic = hello/1/world, Pkid = 0, Payload Size = 1
8. Notification = Outgoing(Publish(0))
2025-05-15T19:59:36.493Z DEBUG [rumqttc::state] Publish. Topic = hello/2/world, Pkid = 0, Payload Size = 2
9. Notification = Outgoing(Publish(0))
Publish result = Ok(())
2025-05-15T19:59:39.488Z DEBUG [rumqttc::state] Publish. Topic = hello/3/world, Pkid = 0, Payload Size = 3
10. Notification = Outgoing(Publish(0))
2025-05-15T19:59:41.494Z DEBUG [rumqttc::state] Pingreq,
            last incoming packet before 11007 millisecs,
            last outgoing request before 2006 millisecs
11. Notification = Outgoing(PingReq)
12. Notification = Incoming(PingResp)
Publish result = Ok(())
2025-05-15T19:59:42.488Z DEBUG [rumqttc::state] Publish. Topic = hello/4/world, Pkid = 0, Payload Size = 4
13. Notification = Outgoing(Publish(0))
14. Notification = MqttState(Io(Custom { kind: ConnectionAborted, error: "connection closed by peer" }))
15. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Publish result = Ok(())
16. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
17. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
18. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Publish result = Ok(())
19. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
20. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
21. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Publish result = Ok(())
22. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
23. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
24. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Publish result = Ok(())
25. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
26. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
27. Notification = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Publish result = Ok(())
```

### Synchronous `rumqtt` client (no TLS)

Sample: [src/sync-rumqttc.rs](src/sync-rumqttc.rs)

- Run the asynchronous `rumqtt` client with `mosquitto.conf` configuration:
```bash
cargo run --release --bin async-rumqttc
```

Output:

- We notice that the `publish()` function always returns `Ok(())` even if the broker is not available.

```
lucas@zgw:~/projects/rust-mqtt-eval$ cargo run --release --bin async-rumqttc
   Compiling rust-mqtt-eval v0.1.0 (/home/lucas/projects/rust-mqtt-eval)
    Finished `release` profile [optimized] target(s) in 0.87s
     Running `target/release/async-rumqttc`
Publish result = Ok(())
Error = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Error = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Error = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Publish result = Ok(())
Error = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Event = Incoming(ConnAck(ConnAck { session_present: false, code: Success }))
2025-05-15T20:01:10.180Z DEBUG [rumqttc::state] Publish. Topic = hello/0/world, Pkid = 0, Payload Size = 0
Event = Outgoing(Publish(0))
2025-05-15T20:01:10.180Z DEBUG [rumqttc::state] Publish. Topic = hello/1/world, Pkid = 0, Payload Size = 1
Event = Outgoing(Publish(0))
Publish result = Ok(())
2025-05-15T20:01:12.174Z DEBUG [rumqttc::state] Publish. Topic = hello/2/world, Pkid = 0, Payload Size = 2
Event = Outgoing(Publish(0))
Publish result = Ok(())
2025-05-15T20:01:15.176Z DEBUG [rumqttc::state] Publish. Topic = hello/3/world, Pkid = 0, Payload Size = 3
Event = Outgoing(Publish(0))
2025-05-15T20:01:15.181Z DEBUG [rumqttc::state] Pingreq,
            last incoming packet before 9009 millisecs,
            last outgoing request before 5 millisecs
Event = Outgoing(PingReq)
Event = Incoming(PingResp)
Publish result = Ok(())
2025-05-15T20:01:18.177Z DEBUG [rumqttc::state] Publish. Topic = hello/4/world, Pkid = 0, Payload Size = 4
Event = Outgoing(Publish(0))
2025-05-15T20:01:20.182Z DEBUG [rumqttc::state] Pingreq,
            last incoming packet before 5001 millisecs,
            last outgoing request before 2005 millisecs
Event = Outgoing(PingReq)
Event = Incoming(PingResp)
Publish result = Ok(())
2025-05-15T20:01:21.177Z DEBUG [rumqttc::state] Publish. Topic = hello/5/world, Pkid = 0, Payload Size = 5
Event = Outgoing(Publish(0))
Error = MqttState(Io(Custom { kind: ConnectionAborted, error: "connection closed by peer" }))
Error = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Publish result = Ok(())
Error = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Error = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Error = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
Publish result = Ok(())
Error = Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })
```

### Asynchronous `paho` client (no TLS)

Sample: [src/async-paho.rs](src/async-paho.rs)

- Run the asynchronous `paho` client with `mosquitto.conf` configuration:
```bash
cargo run --release --bin async-paho
```

Output:

- We notice that the `publish()` function returns the actial status of the connection.

```
lucas@zgw:~/projects/rust-mqtt-eval$ cargo run --release --bin async-paho
   Compiling rust-mqtt-eval v0.1.0 (/home/lucas/projects/rust-mqtt-eval)
    Finished `release` profile [optimized] target(s) in 0.42s
     Running `target/release/async-paho`
Connecting to the MQTT server at 'mqtt://localhost:1883'
Connecting to the broker: Err(TcpTlsConnectFailure)
Connecting to the broker: Err(TcpTlsConnectFailure)
Connecting to the broker: Err(TcpTlsConnectFailure)
Connecting to the broker: Ok(ServerResponse { rsp: Connect(ConnectResponse { server_uri: "localhost:1883", mqtt_version: 4, session_present: false }), props: Properties { cprops: MQTTProperties { count: 0, max_count: 0, length: 0, array: 0x0 } }, reason_code: Success })
Publishing the message: Ok(())
Publishing the message: Ok(())
Publishing the message: Ok(())
Publishing the message: Ok(())
Publishing the message: Err(Disconnected)
Connecting to the broker: Err(TcpTlsConnectFailure)
Connecting to the broker: Err(TcpTlsConnectFailure)
Connecting to the broker: Err(TcpTlsConnectFailure)
Connecting to the broker: Err(TcpTlsConnectFailure)
```

### Asynchronous `paho` client (with TLS)

Sample: [src/async-tls-paho.rs](src/async-tls-paho.rs)

- Run the asynchronous `async-mqtt` client + TLS with `mosquitto-secure.conf` configuration:
```bash
cargo run --release --bin async-tls-paho
```

Output:

- We notice that the `publish()` function returns the actial status of the connection.

```
lucas@zgw:~/projects/rust-mqtt-eval$ cargo run --release --bin async-tls-paho
   Compiling rust-mqtt-eval v0.1.0 (/home/lucas/projects/rust-mqtt-eval)
    Finished `release` profile [optimized] target(s) in 0.44s
     Running `target/release/async-tls-paho`
Connecting to the MQTT server at 'mqtts://localhost:8883'
Connecting to the broker: Err(TcpTlsConnectFailure)
Connecting to the broker: Err(TcpTlsConnectFailure)
Connecting to the broker: Err(TcpTlsConnectFailure)
Connecting to the broker: Err(TcpTlsConnectFailure)
Connecting to the broker: Ok(ServerResponse { rsp: Connect(ConnectResponse { server_uri: "localhost:8883", mqtt_version: 4, session_present: false }), props: Properties { cprops: MQTTProperties { count: 0, max_count: 0, length: 0, array: 0x0 } }, reason_code: Success })
Publishing the message: Ok(())
Publishing the message: Ok(())
Publishing the message: Ok(())
Publishing the message: Ok(())
Publishing the message: Ok(())
Publishing the message: Ok(())
Publishing the message: Ok(())
Publishing the message: Err(Disconnected)
Connecting to the broker: Err(TcpTlsConnectFailure)
```