use crate::info::lan_channel_direction::HasLanChannelDirection;
use crate::pcan;

use crate::channel::{Channel, ChannelCode};


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

/* Channel trait implementation */

impl From<LanBus> for ChannelCode {
    fn from(value: LanBus) -> Self {
        let ret = match value {
            LanBus::Lan1 => pcan::PCAN_LANBUS1,
            LanBus::Lan2 => pcan::PCAN_LANBUS2,
            LanBus::Lan3 => pcan::PCAN_LANBUS3,
            LanBus::Lan4 => pcan::PCAN_LANBUS4,
            LanBus::Lan5 => pcan::PCAN_LANBUS5,
            LanBus::Lan6 => pcan::PCAN_LANBUS6,
            LanBus::Lan7 => pcan::PCAN_LANBUS7,
            LanBus::Lan8 => pcan::PCAN_LANBUS8,
            LanBus::Lan9 => pcan::PCAN_LANBUS9,
            LanBus::Lan10 => pcan::PCAN_LANBUS10,
            LanBus::Lan11 => pcan::PCAN_LANBUS11,
            LanBus::Lan12 => pcan::PCAN_LANBUS12,
            LanBus::Lan13 => pcan::PCAN_LANBUS13,
            LanBus::Lan14 => pcan::PCAN_LANBUS14,
            LanBus::Lan15 => pcan::PCAN_LANBUS15,
            LanBus::Lan16 => pcan::PCAN_LANBUS16,
        } as u16;
        ret.into()
    }
}

impl TryFrom<ChannelCode> for LanBus {
    type Error = ();
    fn try_from(value: ChannelCode) -> Result<Self, Self::Error> {
        let val = value.0 as u32;
        match val {
            pcan::PCAN_LANBUS1 => Ok(LanBus::Lan1),
            pcan::PCAN_LANBUS2 => Ok(LanBus::Lan2),
            pcan::PCAN_LANBUS3 => Ok(LanBus::Lan3),
            pcan::PCAN_LANBUS4 => Ok(LanBus::Lan4),
            pcan::PCAN_LANBUS5 => Ok(LanBus::Lan5),
            pcan::PCAN_LANBUS6 => Ok(LanBus::Lan6),
            pcan::PCAN_LANBUS7 => Ok(LanBus::Lan7),
            pcan::PCAN_LANBUS8 => Ok(LanBus::Lan8),
            pcan::PCAN_LANBUS9 => Ok(LanBus::Lan9),
            pcan::PCAN_LANBUS10 => Ok(LanBus::Lan10),
            pcan::PCAN_LANBUS11 => Ok(LanBus::Lan11),
            pcan::PCAN_LANBUS12 => Ok(LanBus::Lan12),
            pcan::PCAN_LANBUS13 => Ok(LanBus::Lan13),
            pcan::PCAN_LANBUS14 => Ok(LanBus::Lan14),
            pcan::PCAN_LANBUS15 => Ok(LanBus::Lan15),
            pcan::PCAN_LANBUS16 => Ok(LanBus::Lan16),
            _ => Err(())
        }
    }
}

impl Channel for LanBus {
    fn channel(&self) -> ChannelCode {
        self.clone().into()
    }
}

/* Marker trait implementation */

impl HasLanChannelDirection for LanBus {}
