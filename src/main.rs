#![no_main]
#![no_std]

// Use statements
// =================================================

// for panic and printing functions
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

// cortex_m_rt
// used for defining the entry point
use cortex_m_rt::entry;

// Microbit functions
// https://docs.rs/microbit-v2/0.16.0/microbit/
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{pac::twim0::frequency::FREQUENCY_A, timer::Timer, twim::Twim},
};

// Used for manipulating the buttons
// Docs: https://docs.rs/embedded-hal/1.0.0/embedded_hal/index.html
use embedded_hal::digital::InputPin;

// Docs: https://docs.rs/lsm303agr/1.1.0/lsm303agr/
// used to talk to the IMU and get acceleration measurements
use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

// =================================================

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // set up the board, timer, and display
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // set up the i2c - it contains a TWIM object.
    // Based on https://docs.rust-embedded.org/discovery-mb2/12-i2c/using-a-driver.html
    let i2c = {
        Twim::new(
            board.TWIM0,               // board twim
            board.i2c_internal.into(), // board's internal i2c pins
            FREQUENCY_A::K100,          // frequency is 100 kbps
        )
    };

    // set up the sensor using the i2c
    // Based on
    // https://docs.rs/lsm303agr/1.1.0/lsm303agr/
    // https://docs.rs/lsm303agr/1.1.0/lsm303agr/enum.AccelMode.html
    // https://docs.rs/lsm303agr/1.1.0/lsm303agr/enum.AccelOutputDataRate.html
    // https://docs.rs/lsm303agr/1.1.0/lsm303agr/struct.Acceleration.html
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap(); // initialize the sensor
    sensor
        .set_accel_mode_and_odr(
            &mut timer,                // use the board timer
            AccelMode::Normal,         // use normal acceleration mode
            AccelOutputDataRate::Hz50, // output data rate is 50Hz
        )
        .unwrap();

    // blank display - use when the board is upside down
    // this is also the default configuration
    let blank_display = [
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
        [0u8, 0u8, 0u8, 0u8, 0u8],
    ];

    // set the current display - this will change
    let mut current_display = blank_display;

    // set the indices of the LED to light up
    let mut led_x = 2; // middle row
    let mut led_y = 2; // middle column

    // Define the buttons
    // if the B button is pressed, switch to fine mode - more sensitive measurements
    // if the A button is pressed, return to coarse mode
    let mut left_button = board.buttons.button_a;
    let mut right_button = board.buttons.button_b;

    // define the thresholds for coarse and fine mode
    // the following values are for coarse mode
    let mut t_left = -500.0;
    let mut t_left_n = -300.0;
    let mut t_center_1 = -100.0;
    let mut t_center_2 = 100.0;
    let mut t_right_n = 300.0;
    let mut t_right = 500.0;

    // loop
    loop {
        // show the display
        display.show(&mut timer, current_display, 200); // refresh every 200 ms

        if sensor.accel_status().unwrap().xyz_new_data() {
            // get the x, y, and z accel in mG
            // mG values are unscaled values x 4.
            // https://doc.rust-lang.org/nightly/core/primitive.f32.html#impl-From%3Ci16%3E-for-f32
            // https://docs.rs/lsm303agr/1.1.0/lsm303agr/struct.Acceleration.html#method.x_mg
            let data = sensor.acceleration().unwrap();
            let x_mg: f32 = (data.x_unscaled() * 4).into(); // unscaled data is i16 and can be converted to f32
            let y_mg: f32 = (data.y_unscaled() * 4).into();
            let z_mg: f32 = (data.z_unscaled() * 4).into();

            // Toggle how sensitive the "level" is to movement
            if left_button.is_low().unwrap() {
                // coarse mode
                // adjust thresholds
                t_left = -500.0;
                t_left_n = -300.0;
                t_center_1 = -100.0;
                t_center_2 = 100.0;
                t_right_n = 300.0;
                t_right = 500.0;
            } else if right_button.is_low().unwrap() {
                // fine mode
                // adjust thresholds by dividing by 10
                t_left = -50.0;
                t_left_n = -30.0;
                t_center_1 = -10.0;
                t_center_2 = 10.0;
                t_right_n = 30.0;
                t_right = 50.0;
            }

            // DEBUG: check mode and acceleration measurements
            rprintln!(
                "Acceleration values: x_mg: {}, y_mg: {}, z_mg: {}",
                x_mg,
                y_mg,
                z_mg
            );

            // light up the display when the board is not upside down
            // blank the display otherwise
            if z_mg > 0.0 {
                // z is positive
                rprintln!("Upside down board");
                current_display = blank_display;
            } else {
                // adjust LED based on x and y

                // determine the index of the LED to light up based on ranges

                // ** SET THE X INDEX **
                // board is tilted to the left
                if (x_mg >= t_left && x_mg < t_left_n) || (x_mg < t_left) {
                    led_x = 4;
                // board is tilted slightly to the left
                } else if x_mg >= t_left_n && x_mg < t_center_1 {
                    led_x = 3;
                // board is flat (neither tilted left or right)
                } else if x_mg >= t_center_1 && x_mg < t_center_2 {
                    led_x = 2;
                // board is tilted slightly to the right
                } else if x_mg >= t_center_2 && x_mg < t_right_n {
                    led_x = 1;
                // board is tilted to the right
                } else if (x_mg >= t_right_n && x_mg <= t_right) || (x_mg > t_right) {
                    led_x = 0;
                }

                // ** SET THE Y INDEX **
                // board is tilted downward
                if (y_mg >= t_left && y_mg < t_left_n) || (y_mg < t_left) {
                    led_y = 0;
                // board is tilted slightly downward
                } else if y_mg >= t_left_n && y_mg < t_center_1 {
                    led_y = 1;
                // board is flat (neither tilted up or down)
                } else if y_mg >= t_center_1 && y_mg < t_center_2 {
                    led_y = 2;
                // board is tilted slightly upward
                } else if y_mg >= t_center_2 && y_mg < t_right_n {
                    led_y = 3;
                // board is tilted upward
                } else if (y_mg >= t_right_n && y_mg <= t_right) || (y_mg > t_right) {
                    led_y = 4;
                }

                // clear the display
                current_display = blank_display;

                // light up the LED based on the above checks.
                current_display[led_y][led_x] = 1;
            }
        } // end sensor if statement
    } // end loop
} // end main
