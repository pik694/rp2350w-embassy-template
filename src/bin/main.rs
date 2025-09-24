#![no_std]
#![no_main]

use embassy_executor::Spawner;

use {defmt_rtt as _, panic_probe as _};

use rp2350w_embassy_template::resources::{AssignedResources, WifiResources};
use rp2350w_embassy_template::{logging::*, split_resources};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let r = split_resources!(p);

    #[cfg(feature = "wifi")]
    {
        use rp2350w_embassy_template::networking::WifiDriver;
        let _net_driver = WifiDriver::init(&spawner, r.wifi);
    }

    info!("Hello World");
}
