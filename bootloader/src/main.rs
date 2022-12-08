#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};
use stm32h7xx_hal::gpio::{gpiob::PB0, gpiob::PB14, gpioe::PE1, Output, PushPull};
use stm32h7xx_hal::prelude::*;
use systick_monotonic::{fugit::Duration, Systick};

#[app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [SDMMC])]
mod app {
    use super::*;

    #[shared]
    struct Shared {

    }

    #[local]
    struct Local {
        led_1: PB0<Output<PushPull>>,
        led_2: PE1<Output<PushPull>>,
        led_3: PB14<Output<PushPull>>,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1000>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Setup clocks
        let power_config = cx.device.PWR.constrain().freeze();
        let rcc = cx.device.RCC.constrain();

        let mono = Systick::new(cx.core.SYST, 120_000_000);

        rtt_init_print!();
        rprintln!("init");

        let _clocks = rcc
            .use_hse(8.MHz())
            .sys_ck(120.MHz())
            .pclk1(120.MHz())
            .hclk(120.MHz())
            .freeze(power_config, &cx.device.SYSCFG);

        // Setup LEDs
        let gpiob = cx.device.GPIOB.split(_clocks.peripheral.GPIOB);
        let gpioe = cx.device.GPIOE.split(_clocks.peripheral.GPIOE);

        let led_1 = gpiob.pb0.into_push_pull_output();
        let led_2 = gpioe.pe1.into_push_pull_output();
        let led_3 = gpiob.pb14.into_push_pull_output();

        // Schedule the blinking tasks
        blink_1::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
        blink_2::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
        blink_3::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();

        (
            Shared {},
            Local {
                led_1,
                led_2,
                led_3,
            },
            init::Monotonics(mono),
        )
    }

    #[task(local = [led_1])]
    fn blink_1(cx: blink_1::Context) {
        cx.local.led_1.toggle();

        // Blink again later
        blink_1::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
    }

    #[task(local = [led_2])]
    fn blink_2(cx: blink_2::Context) {
        cx.local.led_2.toggle();

        // Blink again later
        blink_2::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
    }

    #[task(local = [led_3])]
    fn blink_3(cx: blink_3::Context) {
        cx.local.led_3.toggle();

        // Blink again later
        blink_3::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
    }
}
