# I2S Stereo In
This project demonstrates how to read data from a pair of SPH0645LM4H-B I2S mems microphones via DMA and a PIO program
implementing the I2S protocol.

## Why
The Embassy RP I2S PIO [program](https://github.com/embassy-rs/embassy/blob/main/embassy-rp/src/pio_programs/i2s.rs) only demonstrates how to use an I2S peripheral as output. Furthermore, the
SPH0645 mic sends data on the trailing edge of the clock signal (SCK), which differs from the Embassy example program
(which transmits data on the leading edge).

## Notes
- The Adafruit breakout board ties `SEL` to `GND`
- See [here](https://learn.adafruit.com/adafruit-i2s-mems-microphone-breakout/raspberry-pi-wiring-test#wiring-for-stereo-mic-3061608) for breadboard setup (board pins will differ)

## Resources
* [Adafruit Board](https://www.adafruit.com/product/5714)
* [RP2040 Datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)
* [Mic](https://www.adafruit.com/product/3421)
* [Mic Datasheet](https://cdn-shop.adafruit.com/product-files/3421/i2S+Datasheet.PDF)
* [PIO Tutorial in Chapter 3](https://datasheets.raspberrypi.com/pico/raspberry-pi-pico-c-sdk.pdf)
* [C++ Implementation](https://github.com/vijaymarupudi/sph0645-pico-troubleshooting)
