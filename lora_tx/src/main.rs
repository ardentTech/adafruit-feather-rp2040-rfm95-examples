#![no_std]
#![no_main]

mod radio;
mod common;

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::spi::Spi;
use embassy_rp::usb::Driver;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use embassy_time::Timer;
use panic_halt as _;
use static_cell::StaticCell;
use crate::common::Spi1Bus;
use crate::radio::{radio_tx, Radio};

enum Event {
    SensorReading([u8; 6])
}

static EVENT_CHANNEL: Channel<CriticalSectionRawMutex, Event, 8> = Channel::new();

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logging(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn orchestration(radio: &'static Mutex<NoopRawMutex, Radio>) {
    let receiver = EVENT_CHANNEL.receiver();
    loop {
        match receiver.receive().await {
            Event::SensorReading(data) => radio_tx(radio, &data).await
        }
    }
}

#[embassy_executor::task]
async fn some_sensor() {
    loop {
        EVENT_CHANNEL.send(Event::SensorReading([0, 1, 0, 1, 0, 1])).await;
        Timer::after_secs(3).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let usb_driver = Driver::new(p.USB, Irqs);
    spawner.must_spawn(logging(usb_driver));

    let spi = Spi::new(
        p.SPI1,
        p.PIN_14,
        p.PIN_15,
        p.PIN_8,
        p.DMA_CH0,
        p.DMA_CH1,
        embassy_rp::spi::Config::default()
    );
    static SPI_BUS: StaticCell<Spi1Bus> = StaticCell::new();
    let spi_bus = SPI_BUS.init(Mutex::new(spi));

    let nss = Output::new(p.PIN_16, Level::High);
    let reset = Output::new(p.PIN_17, Level::High);
    let dio0 = Input::new(p.PIN_21, Pull::None);

    static RADIO: StaticCell<Mutex<NoopRawMutex, Radio>> = StaticCell::new();
    let radio = RADIO.init(Mutex::new(Radio::new(spi_bus, nss, reset, dio0).await));

    spawner.must_spawn(orchestration(radio));
    spawner.must_spawn(some_sensor());
}
