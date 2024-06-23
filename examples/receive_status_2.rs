use peakcan_basic::{bus::usb::UsbBus, data_flow::receive_status::{ReceiveStatus, SetReceiveStatus}};

fn main() {
    let result = UsbBus::Usb1.receive_status();
    println!("Receive-Status: {:?}", result);

    let result = UsbBus::Usb1.set_receive_status(false);
    println!("Set-Receive-Status: {:?}", result);

    let result = UsbBus::Usb1.receive_status();
    println!("Receive-Status: {:?}", result);

    let result = UsbBus::Usb1.set_receive_status(true);
    println!("Set-Receive-Status: {:?}", result);

    let result = UsbBus::Usb1.receive_status();
    println!("Receive-Status: {:?}", result);
}