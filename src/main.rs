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
    AccelMode,
    AccelOutputDataRate,
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
    let mut fine_mode = false; // default mode is coarse mode - if true, adjust the limits
    

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
    // Based on 
    // https://docs.rs/lsm303agr/1.1.0/lsm303agr/
    // https://docs.rs/lsm303agr/1.1.0/lsm303agr/struct.Acceleration.html
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap(); // initialize the sensor
    sensor.set_accel_mode_and_odr(
        &mut timer, // use the board timer
        AccelMode::Normal, // use normal acceleration mode
        AccelOutputDataRate::Hz50, // output data rate is 50Hz
    ).unwrap();

    // default display - shows a dot in the middle
    // the dot will move depending on how the board is held.
    let level_default = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 1u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    // board is turned to the left
    let board_left_1 = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 1u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    let board_left_2 = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 1u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    // board is turned to the right
    let board_right_1 = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [1u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    let board_right_2 = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 1u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    // board is tilted up
    let board_up_1 = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 1u8, 0u8, 0u8],
    ];

    let board_up_2 = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 1u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    // board is tilted down
    let board_down_1 = [
        [0u8, 0u8, 1u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    let board_down_2 = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 1u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    // blank display - use when the board is upside down
    let blank_display = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    // set the current display - this will change
    let mut current_display = level_default;

    // loop
    loop {
        // placeholder: show the item
        display.show(&mut timer, current_display, 200); // refresh every 200 ms
    
        // TBD
        // if the B button is pressed, switch to fine mode
        // if the A button is pressed, return to coarse mode
        if sensor.accel_status().unwrap().xyz_new_data() {
            // get the x, y, and z accel in mG
            let data = sensor.acceleration().unwrap();
            let x_mg = data.x_mg();
            let y_mg = data.y_mg();
            let z_mg = data.z_mg();

            // light up the display when the board is not upside down
            // blank the display otherwise
            if z_mg > 0 { // z is positive
                rprintln!("Upside down board");
                current_display = blank_display;
            } else { // adjust LED based on x and y

                // board is in the center
                if (x_mg >= -100 && x_mg < 100) || (y_mg >= -100 && y_mg < 100) {
                    current_display = level_default;
                }
                
                // tilting board to the left - light is on the right
                if x_mg >= -300 && x_mg < -100 {
                    current_display = board_left_2
                }

                if x_mg >= -500 && x_mg < -300 {
                    current_display = board_left_1
                }

                // tilting board to the right
                if x_mg >= 100 && x_mg < 300 {
                    current_display = board_right_2
                }

                if x_mg >= 300 && x_mg <= 500 {
                    current_display = board_right_1
                }

                // tilting the board up
                if y_mg >= 100 && y_mg < 300 {
                    current_display = board_up_2
                }

                if y_mg >= 300 && y_mg <= 500 {
                    current_display = board_up_1
                }

                // tilting the board down
                if y_mg >= -300 && y_mg < -100 {
                    current_display = board_down_2
                }

                if y_mg >= -500 && y_mg < -300 {
                    current_display = board_down_1
                }

            }
        } // end sensor if statement
    } // end loop
} // end main
