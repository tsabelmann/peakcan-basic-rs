use peakcan_basic::{bus::usb::UsbBus, special_behaviors::five_volts_power::FiveVoltsPower};

fn main() {
    let result = UsbBus::Usb1.five_volts_power();
    println!("Five-Volts-Power: {:?}", result);
}