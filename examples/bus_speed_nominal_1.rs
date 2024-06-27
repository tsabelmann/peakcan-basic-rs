use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, info::busspeed_nominal::BusSpeedNominal, socket::usb::UsbSocket};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.bus_speed_nominal();
    println!("Bus-Speed-Nominal: {:?}", result);
}