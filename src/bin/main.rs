#![no_std]
#![no_main]

use defmt_or_log::info;
use embassy_executor::Spawner;
use embassy_rp::{clocks::RoscRng, watchdog::Watchdog};
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};

// Program metadata for `picotool info`.
// This isn't needed, but it's recommended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"RP2350W Embassy Template"),
    embassy_rp::binary_info::rp_program_description!(
        c"Embassy-based template for RP2350W microcontroller"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

use rp2350w_embassy_template::resources::{AssignedResources, WatchdogResources, WifiResources};
use rp2350w_embassy_template::split_resources;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut rng = RoscRng;

    let r = split_resources!(p);
    let mut wd = Watchdog::new(r.watchdog.w);
    wd.start(Duration::from_secs(10));

    #[cfg(feature = "wifi")]
    {
        use rp2350w_embassy_template::networking::WifiDriver;

        WifiDriver::init(&spawner, r.wifi, rng.next_u64()).await;
    }

    info!("Hello World");

    loop {
        info!("Loop");
        Timer::after_secs(1).await;
        wd.feed();
    }
}
