#![no_std]
#![no_main]

mod i2c_scanner;

use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_rp::i2c::I2c;
use embassy_rp::peripherals::I2C1;
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::channel::Channel;
use embassy_time::Timer;
use panic_halt as _;
use static_cell::StaticCell;
use crate::i2c_scanner::I2cScanner;

const AQ_SENSOR_ADDR: u8 = 0x12;

pub type I2c1Bus = Mutex<NoopRawMutex, I2c<'static, I2C1, embassy_rp::i2c::Async>>;

bind_interrupts!(struct Irqs {
    I2C1_IRQ => embassy_rp::i2c::InterruptHandler<I2C1>;
});

enum Event {
    AqSensorIdentified,
    AqSensorNotIdentified
}

// nice way to separate concerns
static EVENT_CHANNEL: Channel<CriticalSectionRawMutex, Event, 8> = Channel::new();

#[embassy_executor::task]
async fn check_aq_sensor_addr(i2c_bus: &'static I2c1Bus) {
    let i2c_dev = I2cDevice::new(i2c_bus);
    let mut scanner = I2cScanner::new(i2c_dev);
    Timer::after_secs(1).await;
    match scanner.check(AQ_SENSOR_ADDR).await {
        true => EVENT_CHANNEL.send(Event::AqSensorIdentified).await,
        false => EVENT_CHANNEL.send(Event::AqSensorNotIdentified).await
    };
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_13.degrade(), Level::Low);

    let mut i2c_config = embassy_rp::i2c::Config::default();
    // PMSA003I only works with 100 kbit/s (standard speed)
    // see: https://cdn-shop.adafruit.com/product-files/4632/4505_PMSA003I_series_data_manual_English_V2.6.pdf
    // other I2C speed options: 400 kbit/s (full), 1.0 Mbit/s (fast) and 3.2 Mbit/s (high)
    i2c_config.frequency = 100_000;
    // the on-board STEMMA QT connector is wired to I2C1 (https://learn.adafruit.com/feather-rp2040-rfm95/pinouts)
    let i2c = embassy_rp::i2c::I2c::new_async(p.I2C1, p.PIN_3, p.PIN_2, Irqs, i2c_config);
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(i2c));

    spawner.must_spawn(check_aq_sensor_addr(i2c_bus));

    loop {
        match EVENT_CHANNEL.receive().await {
            Event::AqSensorIdentified => {
                led.set_high();
            },
            _ => {}
        }
    }
}
