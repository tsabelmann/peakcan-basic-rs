use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;

pub(crate) trait HasChannelIdentifying {}

pub trait ChannelIdentifying {
    fn is_identifying(&self) -> Result<bool, PcanError>;
    fn set_identifying(&self, value: bool) -> Result<(), PcanError>; 
}

impl<T: HasChannelIdentifying + Channel> ChannelIdentifying for T {
    fn is_identifying(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                data.as_mut_ptr() as *mut c_void,
                4
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                match value {
                    pcan::PCAN_PARAMETER_OFF => Ok(false),
                    pcan::PCAN_PARAMETER_ON => Ok(true),
                    _ => Err(PcanError::Unknown)
                }
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn set_identifying(&self, value: bool) -> Result<(), PcanError> {
        let mut data = match value {
            false => u32::to_le_bytes(pcan::PCAN_PARAMETER_OFF),
            true => u32::to_le_bytes(pcan::PCAN_PARAMETER_ON)
        };

        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel().into(),
                pcan::PCAN_CHANNEL_IDENTIFYING as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}