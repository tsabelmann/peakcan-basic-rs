use peakcan_basic::{bus::usb::UsbBus, data_flow::receive_status::{ReceiveStatus, SetReceiveStatus}, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.receive_status();
    println!("Receive-Status: {:?}", result);

    let result = socket.set_receive_status(false);
    println!("Set-Receive-Status: {:?}", result);

    let result = socket.receive_status();
    println!("Receive-Status: {:?}", result);

    let result = socket.set_receive_status(true);
    println!("Set-Receive-Status: {:?}", result);

    let result = socket.receive_status();
    println!("Receive-Status: {:?}", result);
}