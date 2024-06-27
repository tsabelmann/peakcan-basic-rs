use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, socket::usb::UsbSocket, special_behaviors::listen_only::{ListenOnly, SetListenOnly}};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.listen_only();
    println!("Listen-Only: {:?}", result);

    let result = socket.set_listen_only(true);
    println!("Set-Listen-Only: {:?}", result);

    let result = socket.listen_only();
    println!("Listen-Only: {:?}", result);

    let result = socket.set_listen_only(false);
    println!("set-Listen-Only: {:?}", result);

    let result = socket.listen_only();
    println!("Listen-Only: {:?}", result);
}