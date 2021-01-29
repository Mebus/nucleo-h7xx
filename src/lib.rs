//! Board support crate for nucleo-h745zi hardware
//!
//! # Usage - see examples/
//! ```

//#![deny(warnings)]
#![allow(unused_variables)]

#![no_std]

pub use stm32h7xx_hal as hal;
pub use hal::hal as embedded_hal;
pub use hal::pac;

use hal::prelude::*;
use hal::gpio;


// - modules ------------------------------------------------------------------

pub mod clocks;
pub use clocks::configure as configure_clocks;
pub mod led;
// TODO pub mod pin;
pub mod usart;

// TODO proper logging with compile-time feature selection of: semihosting/itm/rtt
//pub mod itm;
//#[macro_use]
//pub mod itm_macros;
use panic_semihosting as _;
//use cortex_m_rt::entry;


// - Board --------------------------------------------------------------------

#[allow(non_snake_case)]
pub struct Board<'a> {
    pub clocks: hal::rcc::CoreClocks,
    pub pins: pin::Pins,
    pub leds: led::Leds,

    pub USART1: usart::Interface<'a>,

    _marker: core::marker::PhantomData<&'a *const ()>,
}


// NOTE `no_mangle` is used here to prevent linking different minor versions of this crate as that
// would let you `take` the core peripherals more than once (one per minor version)
#[no_mangle]
static NUCLEO_BOARD: () = ();

/// Set to `true` when `take` or `steal` was called to make `Board` a singleton.
static mut TAKEN: bool = false;


impl<'a> Board<'a> {
    /// Returns the nucleo board *once*
    #[inline]
    pub fn take()  -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { TAKEN } {
                None
            } else {
                unsafe { TAKEN = true; }
                Some(Self::new(pac::CorePeripherals::take()?,
                               pac::Peripherals::take()?))
            }
        })
    }

    /// Unchecked version of `Board::take`
    #[inline]
    pub unsafe fn steal() -> Self {
        Self::new(pac::CorePeripherals::steal(),
                  pac::Peripherals::steal())
    }

    fn new(_cp: pac::CorePeripherals, dp: pac::Peripherals) -> Board<'a> {
        let ccdr: hal::rcc::Ccdr = clocks::configure(dp.PWR.constrain(),
                                                     dp.RCC.constrain(),
                                                     &dp.SYSCFG);

        let gpioa: gpio::gpioa::Parts = dp.GPIOA.split(ccdr.peripheral.GPIOA);
        let gpiob: gpio::gpiob::Parts = dp.GPIOB.split(ccdr.peripheral.GPIOB);
        let gpioc: gpio::gpioc::Parts = dp.GPIOC.split(ccdr.peripheral.GPIOC);
        let gpiod: gpio::gpiod::Parts = dp.GPIOD.split(ccdr.peripheral.GPIOD);
        let gpioe: gpio::gpioe::Parts = dp.GPIOE.split(ccdr.peripheral.GPIOE);
        let gpiog: gpio::gpiog::Parts = dp.GPIOG.split(ccdr.peripheral.GPIOG);

        let usart1_pins = (
            gpiob.pb14.into_alternate_af4(),    // USART1 TX - GPIO29 - Pin 36 <= GPIOB 14
            gpiob.pb15.into_alternate_af4(),    // USART1 RX - GPIO30 - Pin 37 => GPIOB 15
        );
        let usart1_interface = usart::Interface::init(&ccdr.clocks,
                                                      ccdr.peripheral.USART1,
                                                      usart1_pins).unwrap();

        Self {
            clocks: ccdr.clocks,
            leds: led::Leds {
                USER: led::LedUser::new(gpioc.pc7)
            },
            pins: pin::Pins {
                SEED_00: gpiob.pb12,
                SEED_01: gpioc.pc11,
                SEED_02: gpioc.pc10,
                SEED_03: gpioc.pc9,
                SEED_04: gpioc.pc8,
                SEED_05: gpiod.pd2,
                SEED_06: gpioc.pc12,
                SEED_07: gpiog.pg10,
                SEED_08: gpiog.pg11,
                SEED_09: gpiob.pb4,
                SEED_10: gpiob.pb5,
                SEED_11: gpiob.pb8,
                SEED_12: gpiob.pb9,
                SEED_13: gpiob.pb6,
                SEED_14: gpiob.pb7,
                SEED_15: gpioc.pc0,
                SEED_16: gpioa.pa3,
                SEED_17: gpiob.pb1,
                SEED_18: gpioa.pa7,
                SEED_19: gpioa.pa6,
                SEED_20: gpioc.pc1,
                SEED_21: gpioc.pc4,
                SEED_22: gpioa.pa5,
                SEED_23: gpioa.pa4,
                SEED_24: gpioa.pa1,
                SEED_25: gpioa.pa0,
                SEED_26: gpiod.pd11,
                SEED_27: gpiog.pg9,
                SEED_28: gpioa.pa2,
            },
            USART1: usart1_interface,

            _marker: core::marker::PhantomData
        }
    }
}
