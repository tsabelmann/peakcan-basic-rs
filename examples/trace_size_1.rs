use peakcan_basic::{bus::usb::UsbBus, socket::usb::UsbSocket, trace::trace_size::{SetTraceSize, TraceSize}};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.trace_size();
    println!("Trace-Size: {:?}", result);

    let result: Result<(), peakcan_basic::error::PcanError> = socket.set_trace_size(42);
    println!("Set-Trace-Size: {:?}", result);

    let result = socket.trace_size();
    println!("Trace-Size: {:?}", result);

    let result: Result<(), peakcan_basic::error::PcanError> = socket.set_trace_size(0);
    println!("Set-Trace-Size: {:?}", result);

    let result = socket.trace_size();
    println!("Trace-Size: {:?}", result);
}
