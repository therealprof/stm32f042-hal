stm32f042-hal
=============

_stm32f042-hal_ contains a hardware abstraction on top of the peripheral access
API for the STMicro stm32f042 series microcontroller.

This crate relies on my [stm32f042][] crate to provide appropriate register
definitions and implements a partial set of the [embedded-hal][] traits.

Since this chip is very easy to use and put into your own boards I've
developped quite a few boards with those chipped and often throw one in just
for good measure. Hence I don't have any ready-made eval board I can recommend
although there is a [nucleo-f042k6][] board on the market.

Some of the implementation was shamelessly adapted from the [stm32f103xx-hal][]
crate by Jorge Aparicio.

[stm32f042]: https://github.com/therealprof/stm32f042.git
[stm32f103xx-hal]: https://github.com/japaric/stm32f103xx-hal
[embedded-hal]: https://github.com/japaric/embedded-hal.git
[nucleo-f042k6]: https://os.mbed.com/platforms/ST-Nucleo-F042K6/

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
