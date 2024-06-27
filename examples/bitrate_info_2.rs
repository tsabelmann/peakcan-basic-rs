use peakcan_basic::{info::bitrate_info::BitrateInfo, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(peakcan_basic::bus::usb::UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud100K).expect("Could not open USB1");

    let result = socket.bitrate_info();
    println!("Bitrate-Info: {:?}", result);
}