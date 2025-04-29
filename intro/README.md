# Intro
This example demonstrates a minimal setup for getting started with Rust, Embassy and the Adafruit board.

## See
* `memory.x` - see section 2.6 of the [RP2040 Datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)
* `Cargo.toml` - minimal necessary 3rd party packages, metadata and build profile configs
* `.cargo/config.toml` - building and flashing config
* `build.rs` - linker config and args
* `main.rs` - app entry point

## Do
1. Build the firmware: `$ cargo build --release`
2. Plug the board (target) in to your machine (host)
3. Enter boot mode (see [here](https://learn.adafruit.com/feather-rp2040-rfm95/pinouts#buttons-and-rst-pin-3142971))
4. Flash the firmware: `$ cargo run --release`

## Resources
* [Adafruit Board](https://www.adafruit.com/product/5714)
* [RP2040 Datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)