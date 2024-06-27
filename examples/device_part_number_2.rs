use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, hw::device_part_number::DevicePartNumber, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.device_part_number();
    println!("Device-Part-Number: {:?}", result);

    match result {
        Ok(result) => println!("{}", result.as_ref()),
        Err(_) => println!("Something went wrong")
    }
}