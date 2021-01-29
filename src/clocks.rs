use crate::hal;
use hal::prelude::*;
use hal::pac;
use hal::pwr;
use hal::rcc;
use hal::time::Hertz;
use hal::time::MegaHertz;


// - constants ----------------------------------------------------------------

// SAI clock uses pll3
const PLL3_P: Hertz = Hertz(48_000 * 256);


// - types --------------------------------------------------------------------

pub trait SeedCrystal {
    const CRYSTAL_FREQ: MegaHertz = MegaHertz(16);

    fn use_seed_crystal(self) -> Self;
}

impl SeedCrystal for rcc::Rcc {
    fn use_seed_crystal(self) -> Self {
        self.use_hse(Self::CRYSTAL_FREQ)
    }
}


// - configure ----------------------------------------------------------------

/// Configures the 16 MHz crystal, a 480 MHz system clock and PLL3 for
/// SAI audio
///
/// The Daisy Seed has a 16 MHz crystal wired to the MCU's high-speed
/// external oscillator pins. We enable that, and use it to drive the
/// full 480 MHz system clock.
///
/// Usage:
///
/// ```
/// let dp = pac::Peripherals::take().unwrap();
/// let ccdr = configure(dp.PWR.constrain(), dp.RCC.constrain(), &dp.SYSCFG);
/// let clocks = configure(rcc);
/// ```
pub fn configure(pwr: pwr::Pwr, rcc: rcc::Rcc, syscfg: &pac::SYSCFG) -> rcc::Ccdr {
    let pwrcfg = pwr.vos0(syscfg).freeze();
    rcc.use_seed_crystal()
       .pll1_strategy(rcc::PllConfigStrategy::Iterative) // pll1 drives system clock
       .sys_ck(480.mhz())                                // system clock @ 480 MHz
       .pll3_p_ck(PLL3_P)
       .freeze(pwrcfg, syscfg)
}
