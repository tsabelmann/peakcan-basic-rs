use crate::hw::channel_identifying::HasChannelIdentifying;
use crate::hw::controller_number::HasControllerNumber;
use crate::hw::device_id::HasDeviceId;
use crate::hw::hardware_name::HasHardwareName;
use crate::pcan;

use crate::hw::channel_condition::HasChannelCondition;
use crate::channel::{Channel, ChannelCode};


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

impl From<UsbBus> for ChannelCode {
    fn from(value: UsbBus) -> Self {
        let ret = match value {
            UsbBus::Usb1 => pcan::PCAN_USBBUS1,
            UsbBus::Usb2 => pcan::PCAN_USBBUS2,
            UsbBus::Usb3 => pcan::PCAN_USBBUS3,
            UsbBus::Usb4 => pcan::PCAN_USBBUS4,
            UsbBus::Usb5 => pcan::PCAN_USBBUS5,
            UsbBus::Usb6 => pcan::PCAN_USBBUS6,
            UsbBus::Usb7 => pcan::PCAN_USBBUS7,
            UsbBus::Usb8 => pcan::PCAN_USBBUS8,
            UsbBus::Usb9 => pcan::PCAN_USBBUS9,
            UsbBus::Usb10 => pcan::PCAN_USBBUS10,
            UsbBus::Usb11 => pcan::PCAN_USBBUS11,
            UsbBus::Usb12 => pcan::PCAN_USBBUS12,
            UsbBus::Usb13 => pcan::PCAN_USBBUS13,
            UsbBus::Usb14 => pcan::PCAN_USBBUS14,
            UsbBus::Usb15 => pcan::PCAN_USBBUS15,
            UsbBus::Usb16 => pcan::PCAN_USBBUS16,
        } as u16;
        ret.into()
    }
}

impl TryFrom<ChannelCode> for UsbBus {
    type Error = ();
    fn try_from(value: ChannelCode) -> Result<Self, Self::Error> {
        let val = value.0 as u32;
        match val {
            pcan::PCAN_USBBUS1 => Ok(UsbBus::Usb1),
            pcan::PCAN_USBBUS2 => Ok(UsbBus::Usb2),
            pcan::PCAN_USBBUS3 => Ok(UsbBus::Usb3),
            pcan::PCAN_USBBUS4 => Ok(UsbBus::Usb4),
            pcan::PCAN_USBBUS5 => Ok(UsbBus::Usb5),
            pcan::PCAN_USBBUS6 => Ok(UsbBus::Usb6),
            pcan::PCAN_USBBUS7 => Ok(UsbBus::Usb7),
            pcan::PCAN_USBBUS8 => Ok(UsbBus::Usb8),
            pcan::PCAN_USBBUS9 => Ok(UsbBus::Usb9),
            pcan::PCAN_USBBUS10 => Ok(UsbBus::Usb10),
            pcan::PCAN_USBBUS11 => Ok(UsbBus::Usb11),
            pcan::PCAN_USBBUS12 => Ok(UsbBus::Usb12),
            pcan::PCAN_USBBUS13 => Ok(UsbBus::Usb13),
            pcan::PCAN_USBBUS14 => Ok(UsbBus::Usb14),
            pcan::PCAN_USBBUS15 => Ok(UsbBus::Usb15),
            pcan::PCAN_USBBUS16 => Ok(UsbBus::Usb16),
            _ => Err(())
        }
    }
}

impl Channel for UsbBus {
    fn channel(&self) -> ChannelCode {
        self.clone().into()
    }
}

impl HasChannelCondition for UsbBus {}
impl HasChannelIdentifying for UsbBus {}
impl HasDeviceId for UsbBus {}
impl HasHardwareName for UsbBus {}
impl HasControllerNumber for UsbBus {}