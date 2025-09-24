#[cfg(feature = "defmt")]
pub use defmt_or_log_defmt::{debug, error, info, warn};

#[cfg(feature = "log")]
pub use defmt_or_log_log::{debug, error, info, warn};

#[cfg(test)]
pub fn init_logger() {
    env_logger::builder().is_test(true).init();
}
