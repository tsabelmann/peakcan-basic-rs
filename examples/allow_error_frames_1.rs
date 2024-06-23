use peakcan_basic::{bus::usb::UsbBus, data_flow::allow_error_frames::{AllowErrorFrames, SetAllowErrorFrames}, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.allow_error_frames();
    println!("Allow-Error-Frames-Status: {:?}", result);

    let result = socket.set_allow_error_frames(true);
    println!("Set-Allow-Error-Frames-Status: {:?}", result);

    let result = socket.allow_error_frames();
    println!("Allow-Error-Frames-Status: {:?}", result);

    let result = socket.set_allow_error_frames(false);
    println!("Set-Allow-Error-Frames-Status: {:?}", result);

    let result = socket.allow_error_frames();
    println!("Allow-Error-Frames-Status: {:?}", result);
}