#![no_std]
#![no_main]

mod i2s;

use core::mem;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::Pio;
use crate::i2s::{PioI2sIn, PioI2sInProgram};
use panic_halt as _;
use static_cell::StaticCell;

const BUFFER_SIZE: usize = 960;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // TODO would PIO I2S be more accurate with the XOSC instead of default ROSC?
    let p = embassy_rp::init(Default::default());
    let Pio { mut common, sm0, .. } = Pio::new(p.PIO0, Irqs);

    // serial clock
    let sck_pin = p.PIN_26;
    // word select: low == left, high == right
    let ws_pin = p.PIN_27;
    // serial data
    let sd_pin = p.PIN_28;

    let program = PioI2sInProgram::new(&mut common);
    let mut i2s = PioI2sIn::new(
        &mut common,
        sm0,
        p.DMA_CH0,
        sd_pin,
        sck_pin,
        ws_pin,
        &program
    );

    static DMA_BUFFER: StaticCell<[u32; BUFFER_SIZE * 2]> = StaticCell::new();
    let dma_buffer = DMA_BUFFER.init_with(|| [0u32; BUFFER_SIZE * 2]);
    let (mut back_buffer, mut front_buffer) = dma_buffer.split_at_mut(BUFFER_SIZE);
    loop {
        i2s.read(&mut front_buffer).await;
        mem::swap(&mut back_buffer, &mut front_buffer);
    }
}
