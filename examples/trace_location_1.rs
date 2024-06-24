use std::path::PathBuf;

use peakcan_basic::{bus::usb::UsbBus, socket::usb::UsbSocket, trace::trace_location::{SetTraceLocation, TraceLocation}};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.trace_location();
    println!("Trace-Location: {:?}", result);

    let result: Result<(), peakcan_basic::error::PcanError> = socket.set_trace_location(&PathBuf::from("C:\\Users\\"));
    println!("Set-Trace-Location: {:?}", result);

    let result = socket.trace_location();
    println!("Trace-Location: {:?}", result);

    let result = socket.set_trace_location("");
    println!("Set-Trace-Location: {:?}", result);

    let result = socket.trace_location();
    println!("Trace-Location: {:?}", result);
}
