# LoRa RX
This example demonstrates receiving a sensor reading using LoRa. Assuming you have two Adafruit boards, when this
example is used in conjunction with the [TX example](https://github.com/ardentTech/adafruit-feather-rp2040-rfm95-examples/tree/main/lora_tx), you'll have a simple P2P network. 

## Now
The on-board LoRa 1276 module was the primary motivation behind selecting the Adafruit board. LoRa's low power
requirement and range make it an ideal candidate for transmitting sensor data from end devices to a gateway. Since this
example does not use encryption (yet), if two or more LoRa devices are in range and use the same modulation parameters,
they can communicate with each other.

## Next
1. Decrypt packets once encryption is configured for transmission

## Resources
* [Adafruit Board](https://www.adafruit.com/product/5714)
* [LoRa SX1276/77/78 Datasheet](https://cdn-shop.adafruit.com/product-files/5714/SX1276-7-8.pdf)