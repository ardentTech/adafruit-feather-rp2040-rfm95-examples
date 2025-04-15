# Board Support Package
This example demonstrates how to configure a simple board support package (BSP).

## Purpose
Boards don't necessarily expose all onboard chip functionality, and board pinouts don't necessarily map 1:1 with chip
pinouts. As an example, the RP2040 has a SWD interface, but this project's Adafruit Feather [board](https://www.adafruit.com/product/5714) doesn't expose the SWD
interface. Other Adafruit Feather boards, such as [this](https://www.adafruit.com/product/4884) one, do expose the SWD
interface, but you need to purchase and solder-on a [connector](https://www.adafruit.com/product/752) to use it.

So to avoid hopping between the documented board [pinouts](https://cdn-learn.adafruit.com/assets/assets/000/120/283/original/adafruit_products_Adafruit_Feather_RP2040_RFM95_Pinout.png?1681763258) and your code anytime you need to utilize a bus or integrate a
new peripheral, creating a BSP enables you to lock in the mappings for supported functionality once and then move on to
more interesting things.

Furthermore, human-readable names (e.g. `lora.reset`) are much easier to work with than numeric pins (e.g. `PIN_17`).

## Resources
* [Adafruit Board](https://www.adafruit.com/product/5714)