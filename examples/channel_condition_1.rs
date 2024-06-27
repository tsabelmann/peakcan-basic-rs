use peakcan_basic::{bus::usb::UsbBus, hw::channel_condition::ChannelCondition};

fn main() {
    let result = UsbBus::Usb1.channel_condition();
    println!("Channel-Condition: {:?}", result);
}