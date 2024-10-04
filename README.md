# ESP32-S3-DevKitC-1 - Blink

Getting started with the [ESP32-S3-DevKitC-1] by controlling the on board LED.

## Introduction

Implement a blink program for the ESP32-S3-DevKitC-1 powered by Rust and the [esp-idf-template].

## Getting Started

The best way to get started is using the provided [VSCode Dev Container]. Start the Dev Container by opepeing the `Command Palette` (Ctrl + Shift + P) and executing the `Dev Containers: Reopen in Dev Container` command. If your are having problems with the Dev Container consider building it new without cache using the `Dev Containers: Rebuild without Cache and Reopen in Dev Container` command.

Connect the [ESP32-S3-DevKitC-1] to your host PC using a UBS-C cable. Use the `COM` USB-C port on your [ESP32-S3-DevKitC-1]. The Dev Container should have access to the serial connection at the port `/dev/ttyACM0` on your host (see [troubleshooting](#troubleshooting)).

You can build the program using `cargo build`. Executing `cargo run` will build and upload the program to the [ESP32-S3-DevKitC-1].

## Simulation using [Wokwi]

After building the program you can do a simulation using [Wokwi]. The Dev Container comes with the [Wokwi VSCode extension] by default. Refer to the [Wokwi VSCode extension] for a setup and usage guide.

## Troubleshooting

### Dev Container has no Access to the connected ESP32-S3-DevKitC-1

Ensure that you are using a high quality USB-C cable that can transfer data. Securely connect the [ESP32-S3-DevKitC-1] using `COM` USB-C port. A serial connection port should open on `/dev/ttyACM0` - this assumes that your are using a linux machine.

In the serial connection shows up on a different port change the config in the [espflash config](espflash.toml) and the [Dev Container Docker Compose File](.devcontainer/docker-compose.yml).

## Sources

- [ESP32-S3-DevKitC-1]
- [esp-idf-template]
- [VSCode Dev Container]
- [Wokwi]
- [Wokwi VSCode extension]
- [The Rust on ESP Book]
- [Crate esp_idf_hal]
- [Safe Rust wrappers for the drivers in the ESP IDF SDK]
- [esp-hal-smartled]

[ESP32-S3-DevKitC-1]: https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/hw-reference/esp32s3/user-guide-devkitc-1.html
[esp-idf-template]: https://github.com/esp-rs/esp-idf-template
[VSCode Dev Container]: https://code.visualstudio.com/docs/devcontainers/create-dev-container
[Wokwi]: https://wokwi.com/
[Wokwi VSCode extension]: https://docs.wokwi.com/vscode/getting-started
[The Rust on ESP Book]: https://docs.esp-rs.org/book/introduction.html
[Crate esp_idf_hal]: https://docs.esp-rs.org/esp-idf-hal/esp_idf_hal/#
[Safe Rust wrappers for the drivers in the ESP IDF SDK]: https://github.com/esp-rs/esp-idf-hal[esp-hal-smartled]
: https://docs.rs/crate/esp-hal-smartled/latest