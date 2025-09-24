#![cfg_attr(not(test), no_std)]

pub mod logging;
pub mod networking;

#[cfg(feature = "bin")]
pub mod resources;

use logging::*;

pub fn add(lhs: u8, rhs: u8) -> u8 {
    info!("Hi");
    lhs.overflowing_add(rhs).0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hello() {
        init_logger();
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn overflows() {
        assert_eq!(add(128, 128), 0);
    }
}
