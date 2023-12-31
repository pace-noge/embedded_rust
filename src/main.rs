#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;
use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};


#[entry]
fn main() -> ! {
    // get access to core peripherals from cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();

    // Get access to specific device peripherals
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    loop {
        block!(timer.wait()).unwrap();
        led.toggle();
    }
}