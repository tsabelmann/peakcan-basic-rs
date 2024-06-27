use peakcan_basic::{bus::usb::UsbBus, hw::channel_identifying::ChannelIdentifying};

fn main() {
    let result = UsbBus::Usb1.is_identifying();
    println!("Is-Identifying: {:?}", result);

    let result = UsbBus::Usb1.set_identifying(true);
    println!("Set-Identifying: {:?}", result);

    let result = UsbBus::Usb1.is_identifying();
    println!("Is-Identifying: {:?}", result);

    let result = UsbBus::Usb1.set_identifying(false);
    println!("Set-Identifying: {:?}", result);

    let result = UsbBus::Usb1.is_identifying();
    println!("Is-Identifying: {:?}", result);
}