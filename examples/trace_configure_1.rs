use peakcan_basic::{bus::usb::UsbBus, socket::usb::UsbSocket, trace::{trace_configure::{SetTraceConfigure, TraceConfigure, TraceFileConfiguration, TraceFileOptions}, trace_location::{SetTraceLocation, TraceLocation}}};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, peakcan_basic::baudrate::Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.trace_configuration();
    println!("Trace-Configuration: {:?}", result);

    let result: Result<(), peakcan_basic::error::PcanError> = socket.set_trace_configuration(TraceFileOptions::Segmented);
    println!("Set-Trace-Configuration: {:?}", result);

    let result = socket.trace_configuration();
    println!("Trace-Configuration: {:?}", result);

    let result: Result<(), peakcan_basic::error::PcanError> = socket.set_trace_configuration(TraceFileOptions::Date);
    println!("Set-Trace-Configuration: {:?}", result);

    let result = socket.trace_configuration();
    println!("Trace-Configuration: {:?}", result);

    let result: Result<(), peakcan_basic::error::PcanError> = socket.set_trace_configuration(TraceFileOptions::Time);
    println!("Set-Trace-Configuration: {:?}", result);

    let result = socket.trace_configuration();
    println!("Trace-Configuration: {:?}", result);

    let result: Result<(), peakcan_basic::error::PcanError> = socket.set_trace_configuration(TraceFileOptions::Overwrite);
    println!("Set-Trace-Configuration: {:?}", result);

    let result = socket.trace_configuration();
    println!("Trace-Configuration: {:?}", result);

    let result: Result<(), peakcan_basic::error::PcanError> = socket.set_trace_configuration(TraceFileOptions::DataLength);
    println!("Set-Trace-Configuration: {:?}", result);

    let result = socket.trace_configuration();
    println!("Trace-Configuration: {:?}", result);

    let result: Result<(), peakcan_basic::error::PcanError> = socket.set_trace_configuration(TraceFileConfiguration::from_trace_file_options(&[TraceFileOptions::Segmented, TraceFileOptions::Date]));
    println!("Set-Trace-Configuration: {:?}", result);

    let result = socket.trace_configuration();
    println!("Trace-Configuration: {:?}", result);
}
