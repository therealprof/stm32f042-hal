#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt;

use cortex_m_rt::ExceptionFrame;

extern crate cortex_m;
extern crate panic_abort;
extern crate stm32f042_hal as hal;

use hal::prelude::*;
use hal::spi::Spi;
use hal::spi::{Mode, Phase, Polarity};
use hal::stm32f042;

exception!(*, default_handler);

fn default_handler(_irqn: i16) {}

exception!(HardFault, hard_fault);

fn hard_fault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

entry!(main);

fn main() -> ! {
    pub const MODE: Mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };

    if let Some(p) = stm32f042::Peripherals::take() {
        let mut rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.freeze();
        let mut gpioa = p.GPIOA.split();

        // Configure pins for SPI
        let sck = gpioa.pa5.into_alternate_af0();
        let miso = gpioa.pa6.into_alternate_af0();
        let mosi = gpioa.pa7.into_alternate_af0();

        // Configure SPI with 100kHz rate
        let mut spi = Spi::spi1(p.SPI1, (sck, miso, mosi), MODE, 100_000.hz(), clocks);

        // Cycle through colors on 16 chained APA102C LEDs
        loop {
            for r in 0..255 {
                let _ = spi.write(&[0, 0, 0, 0]);
                for _i in 0..16 {
                    let _ = spi.write(&[0b1110_0001, 0, 0, r]);
                }
                let _ = spi.write(&[0xFF, 0xFF, 0xFF, 0xFF]);
            }
            for b in 0..255 {
                let _ = spi.write(&[0, 0, 0, 0]);
                for _i in 0..16 {
                    let _ = spi.write(&[0b1110_0001, b, 0, 0]);
                }
                let _ = spi.write(&[0xFF, 0xFF, 0xFF, 0xFF]);
            }
            for g in 0..255 {
                let _ = spi.write(&[0, 0, 0, 0]);
                for _i in 0..16 {
                    let _ = spi.write(&[0b1110_0001, 0, g, 0]);
                }
                let _ = spi.write(&[0xFF, 0xFF, 0xFF, 0xFF]);
            }
        }
    }

    loop {}
}
