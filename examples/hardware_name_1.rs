use peakcan_basic::{bus::usb::UsbBus, hw::hardware_name::HardwareName};

fn main() {
    let result = UsbBus::Usb1.hardware_name();
    println!("Hardware-Name: {:?}", result);

    match result {
        Ok(result) => println!("Hardware-Name: {}", result.as_ref()),
        Err(_) => println!("Something went wrong")
    }
}