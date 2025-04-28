#![no_std]
#![no_main]

use cortex_m::asm::nop;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_time::Timer;
use panic_halt as _;

// map IRQs to handlers here
bind_interrupts!(struct Irqs {});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // bootstrap the HAL so peripherals can be accessed
    let p = embassy_rp::init(Default::default());

    // since we're not using the Spawner instance to launch embassy tasks, nop then delay before
    // continuing the loop
    loop {
        nop();
        Timer::after_secs(3).await;
    }
}
