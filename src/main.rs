#![no_main]
#![no_std]

// Use statements
// =================================================

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;

// Microbit functions
// https://docs.rs/microbit-v2/0.16.0/microbit/
use microbit::{
    board::Board, 
    display::blocking::Display, 
    hal::{
        pac::twim0::frequency::FREQUENCY_A, 
        timer::Timer, 
        twim::Twim
    }
}; 

// Docs: https://docs.rs/lsm303agr/1.1.0/lsm303agr/
use lsm303agr::{
    Acceleration, 
    Lsm303agr
};

// =================================================

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // set up the board, timer, and display
    let _board = Board::take().unwrap();
    let mut timer = Timer::new(_board.TIMER0);
    let mut display = Display::new(_board.display_pins);

    // set up the i2c - it contains a TWIM object.
    // Based on https://docs.rust-embedded.org/discovery-mb2/12-i2c/using-a-driver.html
    let i2c = {
        Twim::new(
            _board.TWIM0, // board twim
            _board.i2c_internal.into(), // board's internal i2c pins
            FREQUENCY_A::K100 // frequency is 100 kbps
        )

    };

    // set up the sensor using the i2c
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();

    // TBD

    // default display - shows a dot in the middle
    let level = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 1u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    // loop
    loop {
        // placeholder: show the item
        display.show(&mut timer, level, 1000);
        rprintln!("Level");
    
        // TBD
    }
}
