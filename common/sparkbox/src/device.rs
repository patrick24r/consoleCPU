// use embedded_sdmmc::*;
pub trait Driver : Sized {
    type Error;
    // Get the refernce to the driver singleton
    fn get_driver_ref() -> &'static mut Self;

    // Initialize the device
    fn init() -> Result<&'static Self, Self::Error>;

    // LEDs functions
    // Returns the total number of LEDs
    fn leds_count() -> usize;
    // Sets the LED at index high
    fn leds_set_high(index: usize) -> Result<&'static Self, Self::Error>;
    // Sets the LED at index low
    fn leds_set_low(index: usize) -> Result<&'static Self, Self::Error>;
    // Toggles the LED at index
    fn leds_toggle(index: usize) -> Result<&'static Self, Self::Error>;
    // Gets the state of the LED at index: true = high, false = low
    fn leds_state(index: usize) -> bool;


    // USB functions
    // fn usb_get_timestamp();
    // fn usb_read(blocks: &mut [Block], start_block_idk: BlockIdx, reason: &str);
    // fn usb_write();
    // fn usb_num_blocks();

}