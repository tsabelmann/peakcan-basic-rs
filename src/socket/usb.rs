use crate::baudrate::{Baudrate, Btr0Btr1Code};
use crate::data_flow::acceptance_filter_11bit::{HasAcceptanceFilter11Bit, HasSetAcceptanceFilter11Bit};
use crate::data_flow::acceptance_filter_29bit::{HasAcceptanceFilter29Bit, HasSetAcceptanceFilter29Bit};
use crate::data_flow::allow_echo_frames::{HasAllowEchoFrames, HasSetAllowEchoFrames};
use crate::data_flow::allow_error_frames::{HasAllowErrorFrames, HasSetAllowErrorFrames};
use crate::data_flow::allow_rtr_frames::{HasAllowRtrFrames, HasSetAllowRtrFrames};
use crate::data_flow::allow_status_frames::{HasAllowStatusFrames, HasSetAllowStatusFrames};
use crate::data_flow::message_filter::{HasMessageFilter, HasSetMessageFilter};
use crate::data_flow::receive_status::{HasReceiveStatus, HasSetReceiveStatus};
use crate::frame::CanFrame;
use crate::info::bitrate_info::HasBitrateInfo;
use crate::info::busspeed_data::HasBusSpeedData;
use crate::info::busspeed_nominal::HasBusSpeedNominal;
use crate::info::firmware_version::HasFirmwareVersion;
use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::bus::usb::UsbBus;
use crate::channel::{Channel, ChannelCode};
use crate::special_behaviors::busoff_autoreset::{HasBusOffAutoreset, HasSetBusOffAutoreset};
use crate::special_behaviors::five_volts_power::{HasFiveVoltsPower, HasSetFiveVoltsPower};
use crate::special_behaviors::hard_reset_status::{HasHardResetStatus, HasSetHardResetStatus};
use crate::special_behaviors::interframe_delay::{HasInterframeDelay, HasSetInterframeDelay};
use crate::special_behaviors::listen_only::{HasListenOnly, HasSetListenOnly};
use crate::timestamp::Timestamp;
use crate::trace::trace_location::{HasSetTraceLocation, HasTraceLocation};
use crate::trace::trace_size::{HasSetTraceSize, HasTraceSize};
use crate::trace::trace_status::{HasSetTraceStatus, HasTraceStatus};

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
impl HasBusSpeedData for UsbSocket {}
impl HasFirmwareVersion for UsbSocket {}

/* Marker trait implementation - special behavior */

impl HasFiveVoltsPower for UsbSocket {}
impl HasSetFiveVoltsPower for UsbSocket {}

impl HasBusOffAutoreset for UsbSocket {}
impl HasSetBusOffAutoreset for UsbSocket {}

impl HasListenOnly for UsbSocket {}
impl HasSetListenOnly for UsbSocket {}

impl HasInterframeDelay for UsbSocket {}
impl HasSetInterframeDelay for UsbSocket {}

impl HasHardResetStatus for UsbSocket {}
impl HasSetHardResetStatus for UsbSocket {}

/* Marker trait implementation - data flow */

impl HasMessageFilter for UsbSocket {}
impl HasSetMessageFilter for UsbSocket {}

impl HasReceiveStatus for UsbSocket {}
impl HasSetReceiveStatus for UsbSocket {}

impl HasAllowStatusFrames for UsbSocket {}
impl HasSetAllowStatusFrames for UsbSocket {}

impl HasAllowRtrFrames for UsbSocket {}
impl HasSetAllowRtrFrames for UsbSocket {}

impl HasAllowErrorFrames for UsbSocket {}
impl HasSetAllowErrorFrames for UsbSocket {}

impl HasAllowEchoFrames for UsbSocket {}
impl HasSetAllowEchoFrames for UsbSocket {}

impl HasAcceptanceFilter11Bit for UsbSocket {}
impl HasSetAcceptanceFilter11Bit for UsbSocket {}

impl HasAcceptanceFilter29Bit for UsbSocket {}
impl HasSetAcceptanceFilter29Bit for UsbSocket {}

/* Marker trait implementation - trace */

impl HasTraceLocation for UsbSocket {}
impl HasSetTraceLocation for UsbSocket {}

impl HasTraceStatus for UsbSocket {}
impl HasSetTraceStatus for UsbSocket {}

impl HasTraceSize for UsbSocket {}
impl HasSetTraceSize for UsbSocket {}

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