#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate feather_m0 as hal;
extern crate panic_halt;

#[macro_use]
extern crate nb;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use hal::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut pins = hal::Pins::new(peripherals.PORT);

    let mut d6 = pins.d6.into_push_pull_output(&mut pins.port);
    d6.set_low().unwrap();

    let mut uart = hal::uart(
        &mut clocks,
        9600.hz(),
        peripherals.SERCOM0,
        &mut peripherals.PM,
        pins.d0,
        pins.d1,
        &mut pins.port,
    );

    loop {
        for i in 0..8 {
            block!(uart.write(1 << i));
            block!(uart.flush());
            let val: u8 = match block!(uart.read()) {
                Ok(val) => val,
                Err(_) => 0xff,
            };
            if count_bits(val) != 1 {
                d6.set_high().unwrap();
                delay.delay_ms(500u32);
                d6.set_low().unwrap();
                delay.delay_ms(500u32);
            }
        }
    }
}

fn count_bits(n: u8) -> u8 {
    let mut placeholder = n;
    let mut count: u8 = 0;
    while placeholder != 0 {
        count += placeholder & 1;
        placeholder >>= 1;
    }
    count
}
