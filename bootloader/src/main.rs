#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};
use systick_monotonic::{fugit::Duration, Systick};
use sparkbox::{*};
use sparkbox_device::nucleo_h743zi2::*;

#[app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [TIM2])]
mod app {
    use sparkbox::leds::Driver;

    use super::*;

    #[shared]
    struct Shared {
        led_driver: NucleoH743LedsDriver,
    }

    #[local]
    struct Local {
        led_num: usize,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1000>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {

        // Initialize device
        let led_driver = NucleoH743LedsDriver::new(cx.device);

        // Initialize logger
        rtt_init_print!();
        rprintln!("init");

        let mono = Systick::new(cx.core.SYST, 240_000_000);
        // Schedule the blinking task
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
        (
            Shared {
                led_driver,
            },
            Local {
                led_num: 0,
            },
            init::Monotonics(mono),
        )
    }

    #[task(local = [led_num], shared = [led_driver])]
    fn blink(mut cx: blink::Context) {
        let led_num = cx.local.led_num;
        cx.shared.led_driver.lock(|led_driver| 
            { 
                let _ = led_driver.toggle(*led_num);
                *led_num += 1;
                *led_num %= led_driver.count();
            }
        );

        


        // Blink again later
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(250)).unwrap();
    }
}
