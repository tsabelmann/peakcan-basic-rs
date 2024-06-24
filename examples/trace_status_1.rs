use peakcan_basic::{bus::usb::UsbBus, socket::usb::UsbSocket, trace::trace_status::{SetTraceStatus, TraceStatus}};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.trace_status();
    println!("Trace-Status: {:?}", result);

    let result: Result<(), peakcan_basic::error::PcanError> = socket.set_trace_status(true);
    println!("Set-Trace-Status: {:?}", result);

    let result = socket.trace_status();
    println!("Trace-Status: {:?}", result);

    let result: Result<(), peakcan_basic::error::PcanError> = socket.set_trace_status(false);
    println!("Set-Trace-Status: {:?}", result);

    let result = socket.trace_status();
    println!("Trace-Status: {:?}", result);
}
