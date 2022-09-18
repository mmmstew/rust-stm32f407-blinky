// main.rs

// Alternates LEDs LD3 and LD4 on the STM32F407VGT6 discovery board

// Usage:
// cargo build --release
// cargo flash --chip STM32F407VGTx --connect-under-reset --release

#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry; // The runtime
use cortex_m;
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Set up system clock
    let rcc = dp.RCC.constrain();
    //let clocks = rcc.cfgr.freeze();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    // Acquire the GPIOC peripheral
    let gpiod = dp.GPIOD.split();

    // Configure GPIO for discovery board LD3 = orange, LD4 = green
    let mut green_led = gpiod.pd12.into_push_pull_output();
    let mut orange_led = gpiod.pd13.into_push_pull_output();

    // Configure the syst timer for a blocking delay
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        orange_led.set_high();
        green_led.set_low();
        delay.delay_ms(1_000_u16);

        orange_led.set_low();
        green_led.set_high();
        delay.delay(1.secs());
    }
}
