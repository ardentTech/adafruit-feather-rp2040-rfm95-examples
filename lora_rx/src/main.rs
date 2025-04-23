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
use crate::radio::Radio;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logging(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
pub async fn radio_rx(
    spi_bus: &'static Spi1Bus,
    nss: Output<'static>,
    reset: Output<'static>,
    dio0: Input<'static>
) {
    let mut radio = Radio::new(spi_bus, nss, reset, dio0).await;
    let mut buffer = [00u8; 255];
    let packet_params = radio.init_rx(buffer.len() as u8).await.unwrap();

    loop {
        buffer = [00u8; 255];
        match radio.rx(&packet_params, &mut buffer).await {
            Ok((received_len, _rx_pkt_status)) => {
                if (received_len == 6)
                    && (buffer[0] == 0x00u8)
                    && (buffer[1] == 0x01u8)
                    && (buffer[2] == 0x00u8)
                    && (buffer[3] == 0x01u8)
                    && (buffer[4] == 0x00u8)
                    && (buffer[5] == 0x01u8)
                {
                    log::info!("RX successful");
                } else {
                    log::info!("RX unexpected packet");
                }
            }
            Err(err) => log::info!("RX unsuccessful = {:?}", err),
        }
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

    spawner.must_spawn(radio_rx(spi_bus, nss, reset, dio0));
}
