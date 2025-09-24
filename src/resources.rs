use assign_resources::assign_resources;
use embassy_rp::{Peri, peripherals};

use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::InterruptHandler;

bind_interrupts!(pub(crate) struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

assign_resources! {
    wifi: WifiResources{
        pwr: PIN_23,
        pio: PIO0,
        dma_ch: DMA_CH0,
        spi_dio: PIN_24,
        spi_cs: PIN_25,
        spi_clk: PIN_29
    }
}
