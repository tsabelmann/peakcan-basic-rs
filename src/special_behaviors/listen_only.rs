use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;

pub(crate) trait HasListenOnly {}
pub(crate) trait HasSetListenOnly {}

pub trait ListenOnly {
    fn listen_only(&self) -> Result<bool, PcanError>;
}

pub trait SetListenOnly {
    fn set_listen_only(&self, value: bool) -> Result<(), PcanError>; 
}

impl<T: HasListenOnly + Channel> ListenOnly for T {
    fn listen_only(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_LISTEN_ONLY as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
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
}

impl<T: HasSetListenOnly + Channel> SetListenOnly for T {
    fn set_listen_only(&self, value: bool) -> Result<(), PcanError> {
        let mut data = match value {
            true => pcan::PCAN_PARAMETER_ON,
            false => pcan::PCAN_PARAMETER_OFF,
        }.to_le_bytes();
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel().into(),
                pcan::PCAN_LISTEN_ONLY as u8,
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