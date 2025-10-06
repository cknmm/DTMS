# Distributed Tracing for MicroServices (DTMS)

## Summary
This project illustrates the use of OpenTelemetry for the generation of traces that help in distributed tracing when the microservices architecture is employed for cloud applications.

## Requirements
You need the following installed for running this project:
* `TypeScript` - `node`, `pnpm`, `tsc` (optional: `nvm` for switching node versions)
* `Python` - `python3`, `pip`
* `Rust` - `rustup`, `rustc`, `cargo`
* `Jaeger` - Download the all-in-one binary from [here](https://www.jaegertracing.io/download/#binaries).

## Setup
Run the following command from the root directory of the project for the setup:
```
pnpm run setup
```

## Starting the Program
To execute this program first run the `jaeger` binary and then run the following command:

For Windows:
```
pnpm run start:windows
```
For Linux and MacOs:
```
pnpm run start
```

Hit the APIs in browser at "http://localhost:\[SERVICE_PORT\]/api" and then open the jaeger UI at "http://localhost:16686" to find the services and traces emitted through OpenTelemetry based instrumentation.

The `SERVICE_PORT` value for the different microservices are:
* Express-ts: 3000
* FastApi-py: 8000
* Axum-rs: 4000
