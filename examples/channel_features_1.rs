use peakcan_basic::{bus::usb::UsbBus, info::channel_features::ChannelFeatures};

fn main() {
    let result = UsbBus::Usb1.is_fd_capable();
    println!("Is-Fd-capble: {:?}", result);

    let result = UsbBus::Usb1.is_delay_capable();
    println!("Is-Delay-capble: {:?}", result);

    let result = UsbBus::Usb1.is_io_capable();
    println!("Is-I/O-capble: {:?}", result);
}