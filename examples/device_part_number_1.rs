use peakcan_basic::{bus::usb::UsbBus, hw::device_part_number::DevicePartNumber};

fn main() {
    let result = UsbBus::Usb1.device_part_number();
    println!("Device-Part-Number: {:?}", result);

    match result {
        Ok(result) => println!("Device-Part-Number: {}", result.as_ref()),
        Err(_) => println!("Something went wrong")
    }
}