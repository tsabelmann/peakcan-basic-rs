use peakcan_basic::{bus::usb::UsbBus, data_flow::allow_rtr_frames::{AllowRtrFrames, SetAllowRtrFrames}, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.allow_rtr_frames();
    println!("Allow-RTR-Frames-Status: {:?}", result);

    let result = socket.set_allow_rtr_frames(false);
    println!("Set-Allow-RTR-Frames-Status: {:?}", result);

    let result = socket.allow_rtr_frames();
    println!("Allow-RTR-Frames-Status: {:?}", result);

    let result = socket.set_allow_rtr_frames(true);
    println!("Set-Allow-RTR-Frames-Status: {:?}", result);

    let result = socket.allow_rtr_frames();
    println!("Allow-RTR-Frames-Status: {:?}", result);
}