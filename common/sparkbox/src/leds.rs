pub trait Driver : Sized {
    type Peripherals;
    type Error;

    fn new(peripherals: Self::Peripherals) -> Self;

    // Returns the total number of LEDs
    fn count(&mut self) -> usize;
    // Sets the LED at index high
    fn set_high(&mut self, index: usize) -> Result<&Self, Self::Error>;
    // Sets the LED at index low
    fn set_low(&mut self, index: usize) -> Result<&Self, Self::Error>;
    // Toggles the LED at index
    fn toggle(&mut self, index: usize) -> Result<&Self, Self::Error>;
    // Gets the state of the LED at index: true = high, false = low
    fn state(&mut self, index: usize) -> bool;
}