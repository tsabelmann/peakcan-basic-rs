use peakcan_basic::{bus::usb::UsbBus, special_behaviors::listen_only::{ListenOnly, SetListenOnly}};

fn main() {
    let result = UsbBus::Usb1.listen_only();
    println!("Listen-Only: {:?}", result);

    let result = UsbBus::Usb1.set_listen_only(true);
    println!("Set-Listen-Only: {:?}", result);

    let result = UsbBus::Usb1.listen_only();
    println!("Listen-Only: {:?}", result);

    let result = UsbBus::Usb1.set_listen_only(false);
    println!("set-Listen-Only: {:?}", result);

    let result = UsbBus::Usb1.listen_only();
    println!("Listen-Only: {:?}", result);
}