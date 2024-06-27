use peakcan_basic::{baudrate::Baudrate, bus::usb::UsbBus, socket::usb::UsbSocket, special_behaviors::interframe_delay::{InterframeDelay, SetInterframeDelay}};

fn main() {
    let socket = UsbSocket::open(UsbBus::Usb1, Baudrate::Baud500K).expect("Could not open USB1");

    let result = socket.interframe_delay();
    println!("Inteframe-Delay: {:?}", result);

    let result = socket.set_interframe_delay(1000);
    println!("Set-Inteframe-Delay: {:?}", result);
    
    let result = socket.interframe_delay();
    println!("Inteframe-Delay: {:?}", result);

    let result = socket.set_interframe_delay(0);
    println!("Set-Inteframe-Delay: {:?}", result);

    let result = socket.interframe_delay();
    println!("Inteframe-Delay: {:?}", result);
}