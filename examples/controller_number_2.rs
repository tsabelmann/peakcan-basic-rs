use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, hw::controller_number::{ControllerNumber, SetControllerNumber}, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.controller_number();
    println!("Controller-Number: {:?}", result);

    let result = socket.set_controller_number(42);
    println!("Set-Controller-Number: {:?}", result);

    let result = socket.controller_number();
    println!("Controller-Number: {:?}", result);

    let result = socket.set_controller_number(0);
    println!("Set-Controller-Number: {:?}", result);

    let result = socket.controller_number();
    println!("Controller-Number: {:?}", result);
}