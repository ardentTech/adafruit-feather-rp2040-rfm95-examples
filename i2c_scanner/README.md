# I2C Scanner
This example demonstrates how to scan the I2C bus for connected peripherals.

## Purpose
When introducing a new peripheral, checking that it's reachable at the documented address(es) is a great first step.
Say you bought an [AQ Sensor](https://www.adafruit.com/product/4632) and failed to register that when the datasheet says:
> 100K sps

it means the device **ONLY** works with an I2C speed of `100 kbit/s`. Also say, for some reason, you configured I2C to run
at 400 kbit/s, and then spent who knows how many hours [trying to figure out](https://forums.adafruit.com/viewtopic.php?p=1049635#p1049635)
why you couldn't communicate with your shiny new peripheral.

You're new. It happens. You move on and eventually discover how simple and common I2C scanners are, and now you sleep
better at night. Neat!

## Resources
* [Adafruit Board](https://www.adafruit.com/product/5714)
* [RP2040 Datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)
* [AQ Sensor](https://www.adafruit.com/product/4632)
* [AQ Sensor datasheet](https://cdn-shop.adafruit.com/product-files/4632/4505_PMSA003I_series_data_manual_English_V2.6.pdf)