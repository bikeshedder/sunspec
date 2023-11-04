# Model 103 example

This code connects to a device that supports sunspec via modbus TCP and
outputs the contents of model 1 and then proceeds reading model 103
(three phase inverter) outputting the value "W / W_SF" and "WH / WH_SF"
every second.

Usage example:

```
$ cargo run 192.168.178.38:1502 1
```

Example output:

```
Manufacturer: SolarEdge 
Model: SE25K-RW00IBNM4
Version: 0004.0018.0518
Serial Number: -redacted-
Supported models: 1, 103
     157.185 kWh     2.292 kW
     157.185 kWh     2.269 kW
     157.186 kWh     2.270 kW
...
```
