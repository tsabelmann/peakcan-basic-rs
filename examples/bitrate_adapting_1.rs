use peakcan_basic::{bus::usb::UsbBus, special_behaviors::bitrate_adapting::{BitrateAdapting, SetBitrateAdapting}};

fn main() {
    let result = UsbBus::Usb1.bitrate_adapting();
    println!("Bitrate-Adapting: {:?}", result);

    let result = UsbBus::Usb1.set_bitrate_adapting(true);
    println!("Set-Bitrate-Adapting: {:?}", result);

    let result = UsbBus::Usb1.bitrate_adapting();
    println!("Bitrate-Adapting: {:?}", result);

    let result = UsbBus::Usb1.set_bitrate_adapting(false);
    println!("Set-Bitrate-Adapting: {:?}", result);

    let result = UsbBus::Usb1.bitrate_adapting();
    println!("Bitrate-Adapting: {:?}", result);
}