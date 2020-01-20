#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate panic_halt;
extern crate rtfm;

use atsamd21g18a::Interrupt;

#[rtfm::app(device = atsamd21g18a)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {
        rtfm::pend(Interrupt::SERCOM0);
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task(binds = SERCOM4)]
    fn sercom4(_: sercom4::Context) {}

    #[task(binds = EIC)]
    fn eic(_: eic::Context) {}
};
