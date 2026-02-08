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
// used to talk to the IMU and get acceleration measurements
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
    let mut fine_mode = false; // default mode is coarse mode

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
    // the dot will move depending on how the board is held.
    let level_default = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 1u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    // loop
    loop {
        // placeholder: show the item
        display.show(&mut timer, level_default, 200); // refresh every 200 ms
        rprintln!("Level");
    
        // TBD
    }
}
