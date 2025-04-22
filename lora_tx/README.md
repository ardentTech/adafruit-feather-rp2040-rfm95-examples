# LoRa
This example demonstrates LoRa P2P transmission.

## Now
The on-board LoRa 1276 module was the primary motivation behind selecting the Adafruit board. LoRa's low power
requirement and range made it an ideal candidate for transmitting sensor data from end devices to a gateway.

## Next
1. Integrate a [psuedo-RNG](https://github.com/raspberrypi/pico-sdk/blob/master/src/rp2_common/pico_rand/include/pico/rand.h) so packets can be encrypted
2. The board doesn't include a HRNG/TRNG, so find a suitable peripheral so packets can be encrypted.

## Resources
* [Adafruit Board](https://www.adafruit.com/product/5714)
* [LoRa SX1276/77/78 Datasheet](https://cdn-shop.adafruit.com/product-files/5714/SX1276-7-8.pdf)