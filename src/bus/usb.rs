use crate::pcan;

#[derive(Debug, PartialEq, Clone)]
pub enum UsbBus {
    /// PCAN-USB interface, channel 1
    Usb1,
    /// PCAN-USB interface, channel 2
    Usb2,
    /// PCAN-USB interface, channel 3
    Usb3,
    /// PCAN-USB interface, channel 4
    Usb4,
    /// PCAN-USB interface, channel 5
    Usb5,
    /// PCAN-USB interface, channel 6
    Usb6,
    /// PCAN-USB interface, channel 7
    Usb7,
    /// PCAN-USB interface, channel 8
    Usb8,
    /// PCAN-USB interface, channel 9
    Usb9,
    /// PCAN-USB interface, channel 10
    Usb10,
    /// PCAN-USB interface, channel 11
    Usb11,
    /// PCAN-USB interface, channel 12
    Usb12,
    /// PCAN-USB interface, channel 13
    Usb13,
    /// PCAN-USB interface, channel 14
    Usb14,
    /// PCAN-USB interface, channel 15
    Usb15,
    /// PCAN-USB interface, channel 16
    Usb16,
}

// impl From<UsbBus> for u16 {
//     fn from(value: UsbBus) -> Self {
//         let ret = match value {
//             UsbBus::Usb1 => pcan::PCAN_USBBUS1,
//             UsbBus::Usb2 => pcan::PCAN_USBBUS2,
//             UsbBus::Usb3 => pcan::PCAN_USBBUS3,
//             UsbBus::Usb4 => pcan::PCAN_USBBUS4,
//             UsbBus::Usb5 => pcan::PCAN_USBBUS5,
//             UsbBus::Usb6 => pcan::PCAN_USBBUS6,
//             UsbBus::Usb7 => pcan::PCAN_USBBUS7,
//             UsbBus::Usb8 => pcan::PCAN_USBBUS8,
//             UsbBus::Usb9 => pcan::PCAN_USBBUS9,
//             UsbBus::Usb10 => pcan::PCAN_USBBUS10,
//             UsbBus::Usb11 => pcan::PCAN_USBBUS11,
//             UsbBus::Usb12 => pcan::PCAN_USBBUS12,
//             UsbBus::Usb13 => pcan::PCAN_USBBUS13,
//             UsbBus::Usb14 => pcan::PCAN_USBBUS14,
//             UsbBus::Usb15 => pcan::PCAN_USBBUS15,
//             UsbBus::Usb16 => pcan::PCAN_USBBUS16,
//         } as u16;
//         ret
//     }
// }

// impl TryFrom<u16> for UsbBus {
//     type Error = ();

//     fn try_from(value: u16) -> Result<Self, Self::Error> {
//         match value as u32 {
//             pcan::PCAN_USBBUS1 => Ok(UsbBus::USB1),
//             pcan::PCAN_USBBUS2 => Ok(UsbBus::USB2),
//             pcan::PCAN_USBBUS3 => Ok(UsbBus::USB3),
//             pcan::PCAN_USBBUS4 => Ok(UsbBus::USB4),
//             pcan::PCAN_USBBUS5 => Ok(UsbBus::USB5),
//             pcan::PCAN_USBBUS6 => Ok(UsbBus::USB6),
//             pcan::PCAN_USBBUS7 => Ok(UsbBus::USB7),
//             pcan::PCAN_USBBUS8 => Ok(UsbBus::USB8),
//             pcan::PCAN_USBBUS9 => Ok(UsbBus::USB9),
//             pcan::PCAN_USBBUS10 => Ok(UsbBus::USB10),
//             pcan::PCAN_USBBUS11 => Ok(UsbBus::USB11),
//             pcan::PCAN_USBBUS12 => Ok(UsbBus::USB12),
//             pcan::PCAN_USBBUS13 => Ok(UsbBus::USB13),
//             pcan::PCAN_USBBUS14 => Ok(UsbBus::USB14),
//             pcan::PCAN_USBBUS15 => Ok(UsbBus::USB15),
//             pcan::PCAN_USBBUS16 => Ok(UsbBus::USB16),
//             _ => Err(()),
//         }
//     }
// }