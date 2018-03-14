#![feature(used)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate embedded_hal;
extern crate stm32f042_hal as hal;

extern crate numtoa;
use numtoa::NumToA;

use hal::prelude::*;
use hal::i2c::*;
use hal::serial::*;
use hal::stm32f042;

extern crate ina260;
use ina260::*;

use core::fmt::Write;

fn main() {
    if let Some(p) = stm32f042::Peripherals::take() {
        let gpiof = p.GPIOF.split();
        let gpioa = p.GPIOA.split();
        let mut clocks = p.RCC.constrain().cfgr.freeze();

        /* Initialise serial pins */
        let tx = gpioa.pa9.into_alternate_af1();
        let rx = gpioa.pa10.into_alternate_af1();

        /* Setup serial port */
        let serial = Serial::usart1(p.USART1, (tx, rx), 115_200.bps(), clocks);
        let (mut tx, mut _rx) = serial.split();

        /* Initialise I2C pins */
        let scl = gpiof
            .pf1
            .into_alternate_af1()
            .internal_pull_up(true)
            .set_open_drain();
        let sda = gpiof
            .pf0
            .into_alternate_af1()
            .internal_pull_up(true)
            .set_open_drain();

        /* Setup I2C1 */
        let mut i2c = I2c::i2c1(p.I2C1, (scl, sda), 1.khz());
        let mut ina260 = INA260::new(i2c, 0x40).unwrap();

        /* Endless loop */
        loop {
            let mut buffer = [0u8; 10];

            /* Read and print voltage */
            let voltage = ina260.voltage().unwrap();
            let count_start = voltage.numtoa(10, &mut buffer);
            let _ = tx.write_str(unsafe { core::str::from_utf8_unchecked(&buffer[count_start..]) });
            let _ = tx.write_str("uV\n\r");

            /* Read and print current */
            let current = ina260.current().unwrap();
            let count_start = current.numtoa(10, &mut buffer);
            let _ = tx.write_str(unsafe { core::str::from_utf8_unchecked(&buffer[count_start..]) });
            let _ = tx.write_str("uA\n\r");
        }
    }
}
