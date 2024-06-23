use peakcan_basic::{bus::usb::UsbBus, data_flow::message_filter::{MessageFilter, MessageFilterMode, SetMessageFilter}, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.message_filter();
    println!("Message-Filter-Status: {:?}", result);

    let result = socket.set_message_filter(MessageFilterMode::Closed);
    println!("Set-Message-Filter-Status: {:?}", result);

    let result = socket.message_filter();
    println!("Message-Filter-Status: {:?}", result);

    let result = socket.set_message_filter(MessageFilterMode::Open);
    println!("Set-Message-Filter-Status: {:?}", result);

    let result = socket.message_filter();
    println!("Message-Filter-Status: {:?}", result);
}