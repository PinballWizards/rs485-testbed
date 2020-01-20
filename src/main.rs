#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate feather_m0 as hal;
extern crate panic_halt;
extern crate rtfm;

use hal::pac::{CorePeripherals, Interrupt, Peripherals};
use rs485_transport::Transport;

const DEVICE_ADDRESS: u8 = 0x1;

#[rtfm::app(device = hal::pac)]
const APP: () = {
    struct Resources {
        core: CorePeripherals,
        peripherals: Peripherals,
        transport: Transport,
    }
    #[init]
    fn init(_: init::Context) -> init::LateResources {
        init::LateResources {
            core: CorePeripherals::take().unwrap(),
            peripherals: Peripherals::take().unwrap(),
            transport: Transport::new(DEVICE_ADDRESS),
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task(binds = SERCOM0)]
    fn sercom0(cx: sercom0::Context) {}

    #[task(binds = EIC)]
    fn eic(_: eic::Context) {}
};
