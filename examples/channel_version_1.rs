use peakcan_basic::{bus::usb::UsbBus, info::channel_version::ChannelVersion};

fn main() {
    let result = UsbBus::Usb1.channel_version();
    println!("Channel-Version: {:?}", result);

    match result {
        Ok(result) => println!("Channel-Version: {}", result.as_ref()),
        Err(_) => println!("Something went wrong")
    }
}