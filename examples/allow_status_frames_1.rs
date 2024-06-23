use peakcan_basic::{bus::usb::UsbBus, data_flow::allow_status_frames::{AllowStatusFrames, SetAllowStatusFrames}, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.allow_status_frames();
    println!("Allow-Status-Frames-Status: {:?}", result);

    let result = socket.set_allow_status_frames(false);
    println!("Set-Allow-Status-Frames-Status: {:?}", result);

    let result = socket.allow_status_frames();
    println!("Allow-Status-Frames-Status: {:?}", result);

    let result = socket.set_allow_status_frames(true);
    println!("Set-Allow-Status-Frames-Status: {:?}", result);

    let result = socket.allow_status_frames();
    println!("Allow-Status-Frames-Status: {:?}", result);
}