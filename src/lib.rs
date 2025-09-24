#![no_std]

pub fn add(lhs: u8, rhs: u8) -> u8 {
    lhs.overflowing_add(rhs).0
}
