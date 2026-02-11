# Level
# Lee Hoang

# What I did

I created a Microbit V2 program to simulate a [bubble level](https://en.wikipedia.org/wiki/Spirit_level). A single LED is lit up to represent the level point.

## Features

- Hold and move the board to see the level point change.
- Two modes: Coarse mode and fine mode. These options determine how sensitive the level point is to movement.
- Coarse mode is the default.
- Press the B button to enter fine mode. This makes the level point more sensitive to movement.
- While in fine mode, press the A button to return to coarse mode.

## How to use

Clone the project into a directory of your choice:

`git clone https://github.com/lee-hoang-portfolio/mb2-bubble-level.git`

Attach the Microbit v2 to your computer and run the program with the following command:

`cargo embed --release`

To run the program in debug mode, use the following command:

`cargo embed`

If you are running the program on WSL2, you will need to follow the steps in this [Microsoft article](https://learn.microsoft.com/en-us/windows/wsl/connect-usb) to set up `usbipd` and the steps in Section 3 of the [MB2 Discovery Book](https://docs.rust-embedded.org/discovery-mb2/03-setup/index.html) to set up `probe-rs` rules.

# How it went

The project was not too difficult. The main difficulties lay in the following: 

1) Ensuring that calculations used floating point numbers instead of integers. To ensure that calculations used floating point numbers, I converted the acceleration measurements from signed integers to floating point numbers.
2) Having clean and easy to understand code for the coarse and fine modes.
3) Avoiding duplicate code.

## Highlights

It was fun watching the level point move around as I shifted the board's orientation. 

# Observations

As I played with the level and observed the level point moving around, I observed the following behaviors:

## Fine mode is very sensitive

When the level is in coarse mode, it is not very sensitive to movement and it is easy to get the level point to remain in the center. However, in fine mode, it is more difficult to keep the level point centered even when the board is held flat. 

The level point may move over the board repeatedly even when the board is idle.

## Program does not run after plugging in the Microbit

I observed that when I plugged my Microbit into the computer, the level program would not run unless I pressed the Reset button on the back of the Microbit. At this time, I do not know what is causing the issue.

## Multiple button presses required to change modes

Sometimes, it takes more than one button press to change modes. 

# Acknowledgements

- Documentation for the `microbit-v2`, `lsm303agr`, and other crates. Links can be found in the source code.
- [Rust Core Library docs](https://doc.rust-lang.org/nightly/core/index.html) - Provided tips and tricks on converting integers to floating point numbers.
- [MB2 Discovery Book](https://docs.rust-embedded.org/discovery-mb2/index.html) - Especially Chapter 11. Provided starting points for code.
- [pdx-cs-rust-embedded](https://github.com/pdx-cs-rust-embedded) - Provided starting points for setting up the project.