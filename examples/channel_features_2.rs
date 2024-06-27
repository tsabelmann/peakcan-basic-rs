use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, info::channel_features::ChannelFeatures, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.is_fd_capable();
    println!("Is-Fd-capble: {:?}", result);

    let result = socket.is_fd_capable();
    println!("Is-Delay-capble: {:?}", result);

    let result = socket.is_fd_capable();
    println!("Is-I/O-capble: {:?}", result);
}