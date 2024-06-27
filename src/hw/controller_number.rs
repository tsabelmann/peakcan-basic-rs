use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;

pub(crate) trait HasControllerNumber {}

pub(crate) trait HasSetControllerNumber {}

pub trait ControllerNumber {
    fn controller_number(&self) -> Result<u32, PcanError>;
}

pub trait SetControllerNumber {
    fn set_controller_number(&self, value: u32) -> Result<(), PcanError>;
}

impl<T: HasControllerNumber + Channel> ControllerNumber for T {
    fn controller_number(&self) -> Result<u32, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };
        
        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => Ok(value),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl<T: HasSetControllerNumber + Channel> SetControllerNumber for T {
    fn set_controller_number(&self, value: u32) -> Result<(), PcanError> {
        let mut data = u32::to_le_bytes(value);
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel().into(),
                pcan::PCAN_CONTROLLER_NUMBER as u8,
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