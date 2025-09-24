#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{clocks::RoscRng, watchdog::Watchdog};
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};

use rp2350w_embassy_template::resources::{AssignedResources, WatchdogResources, WifiResources};
use rp2350w_embassy_template::{logging::*, split_resources};

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
