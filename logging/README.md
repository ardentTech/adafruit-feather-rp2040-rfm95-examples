# Logging
This example demonstrates USB logging.

## Purpose
Blinking morse code via the on-board LED is fun, for a while, but eventually you'll want some form of logging. The board
has a USB interface used for flashing new firmware and powering, and it can also be leveraged for serial communication.

Unfortunately, despite the RP2040 supporting SWD, the Adafruit board doesn't expose this interface and I haven't dug
into options for directly connecting to the necessary processor pins.

On the host side, there are a lot of options for serial communication, and FWIW I use [Minicom](https://github.com/Distrotech/minicom). 

## Resources
* [Adafruit Board](https://www.adafruit.com/product/5714)
* [Minicom](https://github.com/Distrotech/minicom)