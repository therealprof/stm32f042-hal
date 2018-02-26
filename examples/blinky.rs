#![feature(used)]
#![no_std]

extern crate stm32f042_hal as hal;

use hal::prelude::*;
use hal::stm32f042;

fn main() {
    if let Some(p) = stm32f042::Peripherals::take() {
        let rcc = p.RCC;
        let gpioa = p.GPIOA.split();

        /* Enable clock for SYSCFG, else everything will behave funky! */
        rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());

        /* Enable clock for GPIO Port A */
        rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());

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
}
