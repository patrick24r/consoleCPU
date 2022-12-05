#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32h7xx_hal::{prelude::*, stm32};

#[allow(unused_imports)]
use panic_itm;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Constrain power
    let power = dp.PWR.constrain();
    let power_config = power.freeze();
    let rcc = dp.RCC.constrain();

    let ccdr = rcc
        .use_hse(8.MHz())
        .sys_ck(120.MHz())
        .sysclk(120.MHz())
        .pclk1(24.MHz())
        .pclk2(24.MHz())
        .freeze(power_config, &dp.SYSCFG);

    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    let mut led_1 = gpiob.pb0.into_push_pull_output();
    let mut led_2 = gpioe.pe1.into_push_pull_output();
    let mut led_3 = gpiob.pb14.into_push_pull_output();

    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        led_1.toggle();
        delay.delay_ms(300_u16);
        led_2.toggle();
        delay.delay_ms(300_u16);
        led_3.toggle();
        delay.delay_ms(300_u16);
    }
}
