use peakcan_basic::{bus::usb::UsbBus, hw::controller_number::ControllerNumber};

fn main() {
    let result = UsbBus::Usb1.controller_number();
    println!("Controller-Number: {:?}", result);
}