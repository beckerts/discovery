#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    board.display_pins.col1.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();

    // infinite loop; just so we don't leave this stack frame
    loop {
        timer.delay_ms(1000u16);
        rprintln!("1000 ms passed");
    }
}
