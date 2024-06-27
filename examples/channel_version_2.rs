use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, info::channel_version::ChannelVersion, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.channel_version();
    println!("Channel-Version: {:?}", result);

    match result {
        Ok(result) => println!("Channel-Version: {}", result.as_ref()),
        Err(_) => println!("Something went wrong")
    }
}