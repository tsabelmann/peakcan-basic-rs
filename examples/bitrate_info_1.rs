use peakcan_basic::{bus::usb::UsbBus, info::bitrate_info::BitrateInfo};

fn main() {
    let result = UsbBus::Usb1.bitrate_info();
    println!("Bitrate-Info: {:?}", result);
}