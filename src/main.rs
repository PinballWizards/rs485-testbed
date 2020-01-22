#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate feather_m0 as hal;
extern crate panic_halt;
extern crate rtfm;

use hal::{
    clock::GenericClockController,
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};
use rs485_transport::Transport;

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
        transport: Transport,
        uart: UARTPeripheral,
        sercom0: hal::pac::SERCOM0,
    }
    #[init]
    fn init(_: init::Context) -> init::LateResources {
        let mut core = CorePeripherals::take().unwrap();
        let mut peripherals = Peripherals::take().unwrap();
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let mut pins = hal::Pins::new(peripherals.PORT);

        // Enable sercom0 receive complete interrupt
        peripherals
            .SERCOM0
            .usart_mut()
            .intenset
            .write(|w| w.rxc().set_bit());

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
            transport: Transport::new(DEVICE_ADDRESS),
            uart: uart,
            sercom0: unsafe { Peripherals::steal().SERCOM0 },
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task(binds = SERCOM0, resources = [transport, uart, sercom0])]
    fn sercom0(cx: sercom0::Context) {
        if cx
            .resources
            .sercom0
            .usart_mut()
            .intflag
            .read()
            .rxc()
            .bit_is_set()
        {
            let data = cx.resources.sercom0.usart().data.read().bits();
            match cx.resources.transport.ingest(data) {
                Some(resp) => cx.resources.uart.bwrite_all(&resp).unwrap(),
                _ => (),
            };
        }
    }
};
