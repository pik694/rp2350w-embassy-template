use cyw43::{NetDriver, PowerManagementMode};
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::Pio;
use static_cell::StaticCell;

use crate::resources::{Irqs, WifiResources};

pub struct WifiDriver;

impl WifiDriver {
    pub async fn init(spawner: &Spawner, resources: WifiResources) -> NetDriver<'static> {
        static STATE: StaticCell<cyw43::State> = StaticCell::new();

        let pwr = Output::new(resources.pwr, Level::Low);
        let mut pio = Pio::new(resources.pio, Irqs);
        let spi = PioSpi::new(
            &mut pio.common,
            pio.sm0,
            DEFAULT_CLOCK_DIVIDER,
            pio.irq0,
            Output::new(resources.spi_cs, Level::High),
            resources.spi_dio,
            resources.spi_clk,
            resources.dma_ch,
        );
        let state = STATE.init(cyw43::State::new());
        let (fw, clm) = cyw_firmware();

        let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;

        spawner
            .spawn(cyw43_task(runner))
            .expect("Failed to spawn cyw43 task");

        control.init(clm).await;
        control
            .set_power_management(PowerManagementMode::PowerSave)
            .await;

        control.gpio_set(0, true).await;

        net_device
    }
}

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

fn cyw_firmware() -> (&'static [u8], &'static [u8]) {
    #[cfg(not(feature = "baked-cyw43"))]
    return (
        include_bytes!("../../assets/cyw43-firmware/43439A0.bin"),
        include_bytes!("../../assets/cyw43-firmware/43439A0_clm.bin"),
    );

    #[cfg(feature = "baked-cyw43")]
    return unsafe {
        (
            core::slice::from_raw_parts(0x10100000 as *const u8, 230321),
            core::slice::from_raw_parts(0x10140000 as *const u8, 4752),
        )
    };
}
