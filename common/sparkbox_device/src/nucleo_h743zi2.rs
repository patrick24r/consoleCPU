use stm32h7::{stm32h743};
use stm32h7xx_hal::gpio::{ErasedPin, PinState};
use stm32h7xx_hal::{prelude::*, gpio::Output};


pub enum NucleoH743Error {
    // LED errors
    LedIndexOutOfRange,
}


// Led Driver
const LED_COUNT: usize = 3;
pub struct NucleoH743LedsDriver {
    pub leds: [ErasedPin<Output>; LED_COUNT],
}

impl sparkbox::leds::Driver for NucleoH743LedsDriver
{
    type Peripherals = stm32h743::Peripherals;
    type Error = NucleoH743Error;

    fn new(device_handle: Self::Peripherals) -> Self {
        // Power and Clocks init
        let power_config = device_handle.PWR.constrain().freeze();
        let rcc = device_handle.RCC.constrain();

        let ccdr = rcc
            .use_hse(8.MHz())
            .sys_ck(240.MHz())
            .hclk(120.MHz())
            .freeze(power_config, &device_handle.SYSCFG);

        // LEDs init
        let gpiob = device_handle.GPIOB.split(ccdr.peripheral.GPIOB);
        let gpioe = device_handle.GPIOE.split(ccdr.peripheral.GPIOE);

        Self {
            leds: [ 
                gpiob.pb0.into_push_pull_output().erase(),
                gpioe.pe1.into_push_pull_output().erase(),
                gpiob.pb14.into_push_pull_output().erase(),
            ],
        }
    }


    // LED function implementation
    fn count(&mut self) -> usize { self.leds.len() }

    fn set_high(&mut self, index: usize) -> Result<&Self, Self::Error> {
        if index >= self.leds.len() {
            return Err(NucleoH743Error::LedIndexOutOfRange)
        }

        self.leds[index].set_high();
        Ok(self)
    }

    fn set_low(&mut self, index: usize) -> Result<&Self, Self::Error> {
        if index >= self.leds.len() {
            return Err(NucleoH743Error::LedIndexOutOfRange)
        }

        self.leds[index].set_low();
        Ok(self)
    }

    fn toggle(&mut self, index: usize) -> Result<&Self, Self::Error> {
        if index >= self.leds.len() {
            return Err(NucleoH743Error::LedIndexOutOfRange)
        }

        self.leds[index].toggle();
        Ok(self)
    }

    fn state(&mut self, index: usize) -> bool {
        if index < self.leds.len() {
            self.leds[index].get_state() == PinState::High
        } else {
            false
        }
    }



}