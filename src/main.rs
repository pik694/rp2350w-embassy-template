#![no_std]
#![no_main]

use embassy_executor::Spawner;

use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::InterruptHandler;

use {defmt_rtt as _, panic_probe as _};

use defmt::info;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World");
}
