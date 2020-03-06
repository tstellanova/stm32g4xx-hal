#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

// extern crate cortex_m;
use cortex_m_rt as rt;

extern crate stm32g4xx_hal as hal;

#[cfg(debug_assertions)]
use panic_semihosting as _;
#[cfg(not(debug_assertions))]
use panic_halt as _;

use hal::prelude::*;
// use hal::rcc::Config;
use hal::stm32;
use rt::entry;

use hal::rng::RngExt;
use rand_core::RngCore;
use hal::rcc::{SysClockSrc};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");
    let clock_cfg = hal::rcc::Config::new(SysClockSrc::HSE(24.mhz()));
    
    let mut rcc = dp.RCC.freeze(clock_cfg);
    let clocks = rcc.clocks;

    let mut rand_source = dp.RNG.constrain(clocks);

    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob.pb8.into_push_pull_output();

    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        let rand_val = rand_source.next_u32() % 500;
        led.toggle().unwrap();
        delay.delay(rand_val.ms());
    }
}
