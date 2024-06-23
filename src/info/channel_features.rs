use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;

pub(crate) trait HasChannelFeatures {}

pub trait ChannelFeatures {
    fn is_fd_capable(&self) -> Result<bool, PcanError>;
    fn is_delay_capable(&self) -> Result<bool, PcanError>;
    fn is_io_capable(&self) -> Result<bool, PcanError>;
}

impl<T: HasChannelFeatures + Channel> ChannelFeatures for T {    
    fn is_fd_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                let flag = value & pcan::FEATURE_FD_CAPABLE == pcan::FEATURE_FD_CAPABLE;
                Ok(flag)
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
    
    fn is_delay_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                let flag = value & pcan::FEATURE_DELAY_CAPABLE == pcan::FEATURE_DELAY_CAPABLE;
                Ok(flag)
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
    
    fn is_io_capable(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_CHANNEL_FEATURES as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                let flag = value & pcan::FEATURE_IO_CAPABLE == pcan::FEATURE_IO_CAPABLE;
                Ok(flag)
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}
