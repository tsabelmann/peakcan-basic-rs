use crate::pcan;
use crate::error::{PcanError, PcanOkError, PcanErrorCode};
use crate::channel::Channel;

use std::ffi::c_void;

#[derive(Debug, PartialEq, Clone)]
pub enum ChannelConditionStatus {
    /// The PCAN-Channel handle is illegal, or its associated hardware is not available
    Unavailable,
    /// The PCAN-Channel handle is available to be connected (PnP Hardware: it means furthermore that the hardware is plugged-in)
    Available,
    /// The PCAN-Channel handle is valid, and is already being used
    Occupied,
    /// The PCAN-Channel handle is already being used by a PCAN-View application, but is available to connect
    PcanView
}

pub struct ChannelConditionStatusCode(u32);

impl From<ChannelConditionStatusCode> for u32 {
    fn from(value: ChannelConditionStatusCode) -> Self {
        value.0
    }
}

impl From<u32> for ChannelConditionStatusCode {
    fn from(value: u32) -> Self {
        ChannelConditionStatusCode(value)
    }
}

impl From<ChannelConditionStatus> for ChannelConditionStatusCode {
    fn from(value: ChannelConditionStatus) -> Self {
        match value {
            ChannelConditionStatus::Unavailable => ChannelConditionStatusCode(pcan::PCAN_CHANNEL_UNAVAILABLE),
            ChannelConditionStatus::Available => ChannelConditionStatusCode(pcan::PCAN_CHANNEL_AVAILABLE),
            ChannelConditionStatus::Occupied => ChannelConditionStatusCode(pcan::PCAN_CHANNEL_OCCUPIED),
            ChannelConditionStatus::PcanView => ChannelConditionStatusCode(pcan::PCAN_CHANNEL_PCANVIEW),
        }
    }
}

impl TryFrom<ChannelConditionStatusCode> for ChannelConditionStatus {
    type Error = ();
    fn try_from(value: ChannelConditionStatusCode) -> Result<Self, Self::Error> {
        match value.0 {
            pcan::PCAN_CHANNEL_UNAVAILABLE => Ok(ChannelConditionStatus::Unavailable),
            pcan::PCAN_CHANNEL_AVAILABLE => Ok(ChannelConditionStatus::Available),
            pcan::PCAN_CHANNEL_OCCUPIED => Ok(ChannelConditionStatus::Occupied),
            pcan::PCAN_CHANNEL_PCANVIEW => Ok(ChannelConditionStatus::PcanView),
            _ => Err(())
        }
    }
}

pub(crate) trait HasChannelCondition {}

pub trait ChannelCondition {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError>;
}

impl<T: HasChannelCondition + Channel> ChannelCondition for T {
    fn channel_condition(&self) -> Result<ChannelConditionStatus, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_CHANNEL_CONDITION as u8,
                data.as_mut_ptr() as *mut c_void,
                4
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => match ChannelConditionStatus::try_from(ChannelConditionStatusCode::from(value)) {
                Ok(status) => Ok(status),
                Err(_) => Err(PcanError::Unknown),
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}