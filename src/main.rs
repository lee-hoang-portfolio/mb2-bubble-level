#![no_main]
#![no_std]

// Use statements
// =================================================

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::board::Board; // https://docs.rs/microbit-v2/0.16.0/microbit/

// Docs: https://docs.rs/lsm303agr/1.1.0/lsm303agr/

// =================================================

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let _board = Board::take().unwrap();
    let mut counter = 0u64;
    loop {
        rprintln!("{}", counter);
        counter += 1;
    }
}
