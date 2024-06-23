use peakcan_basic::{bus::usb::UsbBus, socket::usb::UsbSocket, special_behaviors::hard_reset_status::{HardResetStatus, SetHardResetStatus}};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.hard_reset_status();
    println!("Hard-Reset-Status: {:?}", result);

    let result = socket.set_hard_reset_status(true);
    println!("Set-Hard-Reset-Status: {:?}", result);

    let result = socket.hard_reset_status();
    println!("Hard-Reset-Status: {:?}", result);

    let result = socket.set_hard_reset_status(false);
    println!("Set-Hard-Reset-Status: {:?}", result);

    let result = socket.hard_reset_status();
    println!("Hard-Reset-Status: {:?}", result);
}