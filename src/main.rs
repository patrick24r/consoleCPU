#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32h7xx_hal::{pac, prelude::*};

#[allow(unused_imports)]
use panic_itm;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain power
    let power = dp.PWR.constrain();
    let power_config = power.freeze();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(power_config, &dp.SYSCFG);

    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    let mut led_1 = gpiob.pb0.into_push_pull_output();
    let mut led_2 = gpioe.pe1.into_push_pull_output();
    let mut led_3 = gpiob.pb14.into_push_pull_output();

    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        led_1.set_high();
        delay.delay_ms(300_u16);
        led_2.set_high();
        delay.delay_ms(300_u16);
        led_3.set_high();
        delay.delay_ms(300_u16);
        led_1.set_low();
        delay.delay_ms(300_u16);
        led_2.set_low();
        delay.delay_ms(300_u16);
        led_3.set_low();
        delay.delay_ms(300_u16);
    }
}
