use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, hw::device_id::{DeviceId, SetDeviceId}, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.device_id();
    println!("Device-Id: {:?}", result);

    let result = socket.set_device_id(0x43);
    println!("Set-Device-Id: {:?}", result);

    let result = socket.device_id();
    println!("Device-Id: {:?}", result);

    let result = socket.set_device_id(0x42);
    println!("Set-Device-Id: {:?}", result);

    let result = socket.device_id();
    println!("Device-Id: {:?}", result);
}