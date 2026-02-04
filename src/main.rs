#![no_main]
#![no_std]

// Use statements
// =================================================

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::timer::Timer}; // https://docs.rs/microbit-v2/0.16.0/microbit/

// Docs: https://docs.rs/lsm303agr/1.1.0/lsm303agr/
use lsm303agr::Acceleration;

// =================================================

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let _board = Board::take().unwrap();
    let mut display = Display::new(_board.display_pins);
    let mut timer = Timer::new(_board.TIMER0);

    let level = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 1u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    loop {
        display.show(&mut timer, level, 1000);
        rprintln!("Level");
    }
}
