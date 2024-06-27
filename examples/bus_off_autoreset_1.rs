use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, socket::usb::UsbSocket, special_behaviors::busoff_autoreset::{BusOffAutoreset, SetBusOffAutoreset}};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");
    
    let result = socket.bus_off_autoreset();
    println!("Bus-Off-Autoreset: {:?}", result);

    let result = socket.set_bus_off_autoreset(true);
    println!("Set-Bus-Off-Autoreset: {:?}", result);

    let result = socket.bus_off_autoreset();
    println!("Bus-Off-Autoreset: {:?}", result);

    let result = socket.set_bus_off_autoreset(false);
    println!("Set-Bus-Off-Autoreset: {:?}", result);

    let result = socket.bus_off_autoreset();
    println!("Bus-Off-Autoreset: {:?}", result);
}