#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(alloc_error_handler)]

use cortex_m::asm;
use cortex_m_rt::exception;
use cortex_m_rt::{entry, ExceptionFrame};
use freertos_rust::*;
use core::alloc::Layout;

use cortex_m;
use sparkbox_device::nucleo_h743zi2 as device;
use sparkbox::device::Driver;


extern crate panic_halt; // panic handler

#[global_allocator]
static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;

#[entry]
fn main() -> ! {
    // Initialize device
    _ = device::NucleoH743ZI2Driver::init();

    Task::new().name("blinky").stack_size(128).priority(TaskPriority(2)).start(|| {
        let mut i = 0;
        loop {
            CurrentTask::delay(Duration::ms(250));
            _ = device::NucleoH743ZI2Driver::leds_toggle(i);
            i = (i + 1) % 3;
        }
    }).unwrap();

    FreeRtosUtils::start_scheduler();
}


#[exception]
unsafe fn DefaultHandler(_irqn: i16) {
// custom default handler
// irqn is negative for Cortex-M exceptions
// irqn is positive for device specific (line IRQ)
// set_led(true);(true);
// panic!("Exception: {}", irqn);
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop {}
}

#[no_mangle]
fn vApplicationStackOverflowHook(_px_task: FreeRtosTaskHandle, _pc_task_name: FreeRtosCharPtr) {
    asm::bkpt();
}