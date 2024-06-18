use crate::pcan;

#[derive(Debug, PartialEq, Clone)]
pub enum LanBus {
    /// PCAN-LAN interface, channel 1
    Lan1,
    /// PCAN-LAN interface, channel 2
    Lan2,
    /// PCAN-LAN interface, channel 3
    Lan3,
    /// PCAN-LAN interface, channel 4
    Lan4,
    /// PCAN-LAN interface, channel 5
    Lan5,
    /// PCAN-LAN interface, channel 6
    Lan6,
    /// PCAN-LAN interface, channel 7
    Lan7,
    /// PCAN-LAN interface, channel 8
    Lan8,
    /// PCAN-LAN interface, channel 9
    Lan9,
    /// PCAN-LAN interface, channel 10
    Lan10,
    /// PCAN-LAN interface, channel 11
    Lan11,
    /// PCAN-LAN interface, channel 12
    Lan12,
    /// PCAN-LAN interface, channel 13
    Lan13,
    /// PCAN-LAN interface, channel 14
    Lan14,
    /// PCAN-LAN interface, channel 15
    Lan15,
    /// PCAN-LAN interface, channel 16
    Lan16,
}