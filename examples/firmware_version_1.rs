use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, info::firmware_version::FirmwareVersion, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.firmware_version();
    println!("Firmware-Version: {:?}", result);

    match result {
        Ok(result) => println!("Firmware-Version: {}.{}.{}", result.major(), result.minor(), result.release()),
        Err(_) => println!("Something went wrong")
    }
}