#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f0xx_hal as hal;

use crate::hal::prelude::*;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (
        hal::stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let rcc = p.RCC.constrain();

        // (Re-)configure PB3 as output
        let mut led = p.GPIOB.split().pb3.into_push_pull_output();

        // Disable the watchdog when the cpu is stopped under debug
        p.DBGMCU.apb1_fz.modify(|_, w| w.dbg_iwdg_stop().set_bit());

        let mut watchdog = hal::watchdog::Watchdog::new(p.IWDG);
        let clocks = rcc.cfgr.sysclk(8.mhz()).freeze();

        // Get delay provider
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        // LED on for 2s to indicate start of example
        led.set_high();
        delay.delay_ms(2000_u16);
        led.set_low();

        // Arm watchdog with 1s timeout
        watchdog.start(hal::time::Hertz(1));

        // Toggle LED a few times a tad slower within the timeout
        for _ in 0..=3 {
            led.toggle();
            delay.delay_ms(200_u16);
        }

        // Feed the watchdog once to reset the timer
        watchdog.feed();

        // Now keep on toggling the LED quickly until the watchdog triggers a reset
        loop {
            led.toggle();
            delay.delay_ms(100_u16);
        }
    }

    loop {
        continue;
    }
}
