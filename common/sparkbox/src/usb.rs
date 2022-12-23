use super::device::Driver;
use embedded_sdmmc::{*, BlockDevice};

// Sparkbox layer implementation of the USB FAT Host
#[derive(Debug)]
struct SparkboxBlockDevice<Device> {

}

impl<Device> BlockDevice<Device> for LinuxBlockDevice {


    fn read() {

    }

    fn write() {

    }

    fn num_blocks() {
        
    }
}