use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, hw::hardware_name::HardwareName, socket::usb::UsbSocket};


fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.hardware_name();
    println!("Hardware-Name: {:?}", result);

    match result {
        Ok(result) => println!("Hardware-Name: {}", result.as_ref()),
        Err(_) => println!("Something went wrong")
    }
}