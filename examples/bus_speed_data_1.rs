use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, info::busspeed_data::BusSpeedData, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.bus_speed_data();
    println!("Bus-Speed-Data: {:?}", result);
}