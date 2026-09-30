# TLS example

This code connects to a device that supports sunspec via Modbus/TCP Security
(Modbus over TLS) and outputs the contents of model 1 and the list of
supported models.

The TLS stream is created using `tokio-rustls` and passed to
`tokio_modbus::client::tcp::attach`. Other than that, the code is the same
as for plain Modbus TCP.

Usage example:

```
$ cargo run inverter.local 1 ca.pem client.pem client.key
```
