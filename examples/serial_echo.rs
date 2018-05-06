#![feature(used)]
#![no_std]

extern crate panic_abort;
extern crate stm32f042_hal as hal;

use hal::prelude::*;
use hal::stm32f042;

#[macro_use(block)]
extern crate nb;

use hal::serial::Serial;

fn main() {
    if let Some(p) = stm32f042::Peripherals::take() {
        let gpioa = p.GPIOA.split();
        let mut rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        let tx = gpioa.pa9.into_alternate_af1();
        let rx = gpioa.pa10.into_alternate_af1();

        let serial = Serial::usart1(
            p.USART1,
            (tx, rx),
            115_200.bps(),
            clocks,
        );

        let (mut tx, mut rx) = serial.split();

        loop {
            let received = block!(rx.read()).unwrap();
            block!(tx.write(received)).ok();
        }
    }
}
