use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;

pub(crate) trait HasInteframeDelay {}
pub(crate) trait HasSetInterframeDelay{}

pub trait InterframeDelay {
    fn interframe_delay(&self) -> Result<u16, PcanError>;
}

pub trait SetInterframeDelay {
    fn set_interframe_delay(&self, value: u16) -> Result<(), PcanError>; 
}

impl<T: HasInteframeDelay + Channel> InterframeDelay for T {
    fn interframe_delay(&self) -> Result<u16, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_INTERFRAME_DELAY as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => Ok(value as u16),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl<T: HasSetInterframeDelay + Channel> SetInterframeDelay for T {
    fn set_interframe_delay(&self, value: u16) -> Result<(), PcanError> {
        let mut data = if value > 1023 {
            1023 as u32
        } else {
            value as u32
        }.to_le_bytes();
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel().into(),
                pcan::PCAN_INTERFRAME_DELAY as u8,
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