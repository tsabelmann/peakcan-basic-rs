use peakcan_basic::{bus::usb::UsbBus, data_flow::acceptance_filter_11bit::{AcceptanceFilter11bit, SetAcceptanceFilter11bit}, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.acceptance_filter_11bit();
    println!("Acceptance-Filter-11bit: {:?}", result);

    let result = socket.set_acceptance_filter_11bit(0x123, 0x07_FF);
    println!("Set-Acceptance-Filter-11bit: {:?}", result);

    let result = socket.acceptance_filter_11bit();
    println!("Acceptance-Filter-11bit: {:?}", result);
}
