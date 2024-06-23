use peakcan_basic::{bus::usb::UsbBus, data_flow::allow_echo_frames::{AllowEchoFrames, SetAllowEchoFrames}, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.allow_echo_frames();
    println!("Allow-Echo-Frames-Status: {:?}", result);

    let result = socket.set_allow_echo_frames(true);
    println!("Set-Allow-Echo-Frames-Status: {:?}", result);

    let result = socket.allow_echo_frames();
    println!("Allow-Echo-Frames-Status: {:?}", result);

    let result = socket.set_allow_echo_frames(false);
    println!("Set-Allow-Echo-Frames-Status: {:?}", result);

    let result = socket.allow_echo_frames();
    println!("Allow-Echo-Frames-Status: {:?}", result);
}