#![feature(used)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt;

use cortex_m_rt::ExceptionFrame;

extern crate panic_abort;
extern crate stm32f042_hal as hal;

use hal::prelude::*;
use hal::stm32f042;

exception!(*, default_handler);

fn default_handler(_irqn: i16) {}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

entry!(main);

fn main() -> ! {
    if let Some(p) = stm32f042::Peripherals::take() {
        let gpioa = p.GPIOA.split();

        /* (Re-)configure PA1 as output */
        let mut led = gpioa.pa1.into_push_pull_output();

        loop {
            /* Turn PA1 on a million times in a row */
            for _ in 0..1_000_000 {
                led.set_high();
            }
            /* Then turn PA1 off a million times in a row */
            for _ in 0..1_000_000 {
                led.set_low();
            }
        }
    }

    loop {}
}
