use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, socket::usb::UsbSocket, special_behaviors::five_volts_power::{FiveVoltsPower, SetFiveVoltsPower}};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.five_volts_power();
    println!("Five-Volts-Power: {:?}", result);

    let result = socket.set_five_volts_power(true);
    println!("Set-Five-Volts-Power: {:?}", result);

    let result = socket.five_volts_power();
    println!("Five-Volts-Power: {:?}", result);

    let result = socket.set_five_volts_power(false);
    println!("Set-Five-Volts-Power: {:?}", result);

    let result = socket.five_volts_power();
    println!("Five-Volts-Power: {:?}", result);
}