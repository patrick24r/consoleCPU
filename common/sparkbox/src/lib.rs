#![no_std]
pub mod leds;


pub struct Sparkbox<'a, LedDrv> {
    led_driver: &'a mut LedDrv,

}


impl<'a, LedDrv> Sparkbox<'a, LedDrv> where 
LedDrv: leds::Driver,
{
    
}