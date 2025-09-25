use cyw43::{Control, JoinOptions, NetDriver, PowerManagementMode};
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
use defmt_or_log::{debug, info, warn};
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::Pio;
use static_cell::StaticCell;

use crate::measure_time;
use crate::resources::{Irqs, WifiResources};

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

pub struct WifiDriver;

impl WifiDriver {
    pub async fn init(spawner: &Spawner, resources: WifiResources, seed: u64) -> Stack<'static> {
        info!("Initializing WIFI stack");
        let (net_device, control) = init_cyw43_driver(spawner, resources).await;
        debug!("Initialized cyw43");

        let init_stack = init_stack(spawner, net_device, seed);
        let connect_wifi = connect_wifi(control);

        debug!("Connecting to WIFI and getting IP address");
        let (_, stack) =
            measure_time!("WifiDriver stack", { join(connect_wifi, init_stack).await });

        info!("Net stack is up!");

        stack
    }
}

async fn init_cyw43_driver(
    spawner: &Spawner,
    resources: WifiResources,
) -> (NetDriver<'static>, Control<'static>) {
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
    spawner.spawn(tasks::cyw43_task(runner)).unwrap();

    control.init(clm).await;
    control
        .set_power_management(PowerManagementMode::PowerSave)
        .await;

    (net_device, control)
}

async fn connect_wifi(mut control: Control<'static>) {
    measure_time!("Connect WIFI", {
        let mut led_state = false;
        while let Err(e) = control
            .join(WIFI_SSID, JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
        {
            warn!("Failed to connect to WiFi. Status={}", e.status);
            led_state = !led_state;
            control.gpio_set(0, led_state).await;
        }
        control.gpio_set(0, true).await;
    });
}

async fn init_stack(
    spawner: &Spawner,
    net_device: NetDriver<'static>,
    seed: u64,
) -> Stack<'static> {
    static RESOURCES: StaticCell<StackResources<5>> = StaticCell::new();

    let stack = measure_time!("DHCP", {
        let config = Config::dhcpv4(Default::default());
        let (stack, runner) = embassy_net::new(
            net_device,
            config,
            RESOURCES.init(StackResources::new()),
            seed,
        );

        spawner.spawn(tasks::net_task(runner)).unwrap();

        stack.wait_config_up().await;
        stack.wait_link_up().await;

        stack
    });
    debug!("Stack is up");

    stack
}

mod tasks {
    use super::*;
    #[embassy_executor::task]
    pub async fn cyw43_task(
        runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
    ) -> ! {
        runner.run().await
    }

    #[embassy_executor::task]
    pub async fn net_task(
        mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>,
    ) -> ! {
        runner.run().await
    }
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
