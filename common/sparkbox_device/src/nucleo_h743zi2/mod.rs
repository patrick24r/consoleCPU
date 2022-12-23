use stm32h7::{stm32h743};
use stm32h7xx_hal::gpio::{ErasedPin, PinState, Output};
use stm32h7xx_hal::{prelude::*, pwr::*};
use error::*;

pub mod error;

static mut DRIVER: NucleoH743ZI2Driver = NucleoH743ZI2Driver {
    is_initialized: false,
    // LEDs
    leds: [ None, None, None ],

    // USB
};

// Led Driver

pub struct NucleoH743ZI2Driver {
    is_initialized: bool,
    pub leds: [Option<ErasedPin<Output>>; 3],
}

impl sparkbox::device::Driver for NucleoH743ZI2Driver {
    type Error = NucleoH743Error;

    fn get_driver_ref() -> &'static mut Self {
        unsafe {
            &mut DRIVER
        }
    }

    // Initialize all peripherals
    fn init() -> Result<&'static Self, Self::Error> {
        unsafe {
            // If already initialized, do nothing
            if DRIVER.is_initialized {
                return Ok(&DRIVER);
            }

            // Get the core driver to get peripherals
            let periph = stm32h743::Peripherals::take().unwrap();
            let power_config = periph.PWR.constrain().freeze();
            let rcc = periph.RCC.constrain();

            let ccdr = rcc
                .use_hse(8.MHz())
                .sys_ck(240.MHz())
                .hclk(120.MHz())
                .freeze(power_config, &periph.SYSCFG);
            
            // LEDs init
            let gpiob = periph.GPIOB.split(ccdr.peripheral.GPIOB);
            let gpioe = periph.GPIOE.split(ccdr.peripheral.GPIOE);

            // Save leds in the array
            DRIVER.leds[0] = Some(gpiob.pb0.into_push_pull_output().erase()); 
            DRIVER.leds[1] = Some(gpioe.pe1.into_push_pull_output().erase());
            DRIVER.leds[2] = Some(gpiob.pb14.into_push_pull_output().erase());
            DRIVER.is_initialized = true;

            Ok(&DRIVER)
        }
    }

    // LED function implementation
    fn leds_count() -> usize {
        unsafe {
            DRIVER.leds.len()
        }
    }

    // Set the specified LED high
    fn leds_set_high(index: usize) -> Result<&'static Self, Self::Error> {
        unsafe {
            if !DRIVER.is_initialized {
                return Err(NucleoH743Error::DriverUninitialized)
            } else if index >= DRIVER.leds.len() {
                return Err(NucleoH743Error::LedsIndexOutOfRange)
            }
    
            DRIVER.leds[index].as_mut().unwrap().set_high();
            Ok(&DRIVER)
        }
    }

    // Set the specified LED low
    fn leds_set_low(index: usize) -> Result<&'static Self, Self::Error> {
        unsafe {
            if !DRIVER.is_initialized {
                return Err(NucleoH743Error::DriverUninitialized)
            } else if index >= DRIVER.leds.len() {
                return Err(NucleoH743Error::LedsIndexOutOfRange)
            }
    
            DRIVER.leds[index].as_mut().unwrap().set_low();
            Ok(&DRIVER)
        }
    }

    // Toggle the LED at the provided index
    fn leds_toggle(index: usize) -> Result<&'static Self, Self::Error> {
        unsafe {
            if !DRIVER.is_initialized {
                return Err(NucleoH743Error::DriverUninitialized)
            } else if index >= DRIVER.leds.len() {
                return Err(NucleoH743Error::LedsIndexOutOfRange)
            }
    
            DRIVER.leds[index].as_mut().unwrap().toggle();
            Ok(&DRIVER)
        }
    }

    // Get the state of the LED at the provided index
    fn leds_state(index: usize) -> bool {
        unsafe {
            if !DRIVER.is_initialized || index >= DRIVER.leds.len() {
                false
            } else {
                DRIVER.leds[index].as_mut().unwrap().get_state() == PinState::High
            }
        }
        
    }
}