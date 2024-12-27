#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut lightarray = [[0; 5]; 5];
    let dur = 50;
    
    loop {
        // Show light_it_all for 1000ms
        // display.show(&mut timer, light_it_all, 1000);
        // // clear the display again
        // display.clear();
        // timer.delay_ms(1000_u32);
        

        for col in 0..5 {
            if col > 0 {
                lightarray[0][col - 1] = 0;
            }
            lightarray[0][col] = 1;
            display.show(&mut timer, lightarray, dur);
            //timer.delay_ms(500_u32);
        }

        for row in 1..5 {
            if row > 0 {
                lightarray[row - 1][4] = 0;
            }
            lightarray[row][4] = 1;
            display.show(&mut timer, lightarray, dur);
            //timer.delay_ms(500_u32);
        }

        for col in (0..4).rev() {
            lightarray[4][col + 1] = 0;
            lightarray[4][col] = 1;
            display.show(&mut timer, lightarray, dur);
            //timer.delay_ms(500_u32);
        }

        for row in (1..4).rev() {
            lightarray[row + 1][0] = 0;
            lightarray[row][0] = 1;
            display.show(&mut timer, lightarray, dur);
            //timer.delay_ms(500_u32);
        }

        lightarray[1][0] = 0;
        display.clear();
    }
}
