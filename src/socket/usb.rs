use crate::baudrate::{Baudrate, Btr0Btr1Code};
use crate::frame::CanFrame;
use crate::info::bitrate_info::HasBitrateInfo;
use crate::info::busspeed_nominal::HasBusSpeedNominal;
use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::bus::usb::UsbBus;
use crate::channel::{Channel, ChannelCode};
use crate::timestamp::Timestamp;

use super::Socket;


#[derive(Debug, PartialEq)]
pub struct UsbSocket {
    channel: u16,
}

impl UsbSocket {
    pub fn open(bus: UsbBus, baud: Baudrate) -> Result<UsbSocket, PcanError> {
        let channel = bus.channel().into();
        let code = unsafe { pcan::CAN_Initialize(channel, Btr0Btr1Code::from(baud).into(), 0, 0, 0) };

        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => Ok(UsbSocket { channel }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Drop trait implementation */

impl Drop for UsbSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.channel) };
    }
}

/* Channel trait implemenation */

impl Channel for UsbSocket {
    fn channel(&self) -> crate::channel::ChannelCode {
        ChannelCode {
            0: self.channel
        }
    }
}

/* Marker traits */

impl HasBitrateInfo for UsbSocket {}
impl HasBusSpeedNominal for UsbSocket {}

/* Socket trait implementation */

impl Socket for UsbSocket {
    fn send(&self, frame: CanFrame) -> Result<(), PcanError> {
        let mut frame = frame;
        let code = unsafe {
            pcan::CAN_Write(
                self.channel, 
                &mut frame.frame as *mut pcan::TPCANMsg
            )    
        };
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn recv(&self) -> Result<(CanFrame, Timestamp), PcanError> {
        let mut frame = pcan::TPCANMsg {
            ID: 0,
            MSGTYPE: 0,
            LEN: 0,
            DATA: [0u8; 8]
        };

        let mut timestamp = pcan::TPCANTimestamp {
            micros: 0,
            millis: 0,
            millis_overflow: 0
        };

        let code = unsafe {
            pcan::CAN_Read(
                self.channel, 
                &mut frame as *mut pcan::TPCANMsg, 
                &mut timestamp as *mut pcan::TPCANTimestamp
            )    
        };
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                let fr = CanFrame {
                    frame: frame
                };
                let ts = Timestamp {
                    timestamp: timestamp
                };
                Ok((fr, ts))
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}