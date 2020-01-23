#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate feather_m0 as hal;
extern crate panic_halt;
extern crate rtfm;

use hal::{clock::GenericClockController, pac::Peripherals, prelude::*};
use palantir::{Bus, Palantir};

const DEVICE_ADDRESS: u8 = 0x1;

type UARTPeripheral = hal::sercom::UART0<
    hal::sercom::Sercom0Pad3<hal::gpio::Pa11<hal::gpio::PfC>>,
    hal::sercom::Sercom0Pad2<hal::gpio::Pa10<hal::gpio::PfC>>,
    (),
    (),
>;

#[rtfm::app(device = hal::pac)]
const APP: () = {
    struct Resources {
        palantir: Palantir,
        uart: UARTPeripheral,
        sercom0: hal::pac::SERCOM0,
    }
    #[init]
    fn init(_: init::Context) -> init::LateResources {
        let mut peripherals = Peripherals::take().unwrap();
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let mut pins = hal::Pins::new(peripherals.PORT);

        // Enable sercom0 receive complete interrupt and error interrupt
        peripherals.SERCOM0.usart_mut().intenset.write(|w| {
            w.rxc().set_bit();
            w.error().set_bit()
        });

        let uart = hal::uart(
            &mut clocks,
            1.mhz(),
            peripherals.SERCOM0,
            &mut peripherals.PM,
            pins.d0,
            pins.d1,
            &mut pins.port,
        );

        init::LateResources {
            palantir: Palantir::new(DEVICE_ADDRESS),
            uart: uart,
            sercom0: unsafe { Peripherals::steal().SERCOM0 },
        }
    }

    #[idle(spawn = [testing])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            cx.spawn.testing().unwrap();
        }
    }

    #[task(resources = [palantir])]
    fn testing(cx: testing::Context) {
        match cx.resources.palantir.poll() {
            Some(msg) => (),
            _ => (),
        };
    }

    #[task(binds = SERCOM0, resources = [palantir, uart, sercom0])]
    fn sercom0(cx: sercom0::Context) {
        let intflag = cx.resources.sercom0.usart_mut().intflag.read();
        if intflag.rxc().bit_is_set() {
            cx.resources.palantir.ingest();
        } else if intflag.error().bit_is_set() {
            // Collision error detected, wait for NAK and resend
            cx.resources
                .sercom0
                .usart_mut()
                .intflag
                .write(|w| w.error().set_bit());
        }
    }

    extern "C" {
        fn SERCOM5();
    }
};
