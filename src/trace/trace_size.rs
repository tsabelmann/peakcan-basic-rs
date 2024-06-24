use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;

pub(crate) trait HasTraceSize {}
pub(crate) trait HasSetTraceSize {}

pub trait TraceSize {
    fn trace_size(&self) -> Result<u8, PcanError>;
}

pub trait SetTraceSize {
    fn set_trace_size(&self, value: u8) -> Result<(), PcanError>;
}

impl<T: HasTraceSize + Channel> TraceSize for T {
    fn trace_size(&self) -> Result<u8, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_TRACE_SIZE as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                let value = value as u8;
                Ok(value)
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl<T: HasSetTraceSize + Channel> SetTraceSize for T {
    fn set_trace_size(&self, value: u8) -> Result<(), PcanError> {
        let data =  if value > 100 {
            100 as u32
        } else {
            value as u32
        }.to_le_bytes();

        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel().into(),
                pcan::PCAN_TRACE_SIZE as u8,
                data.as_ptr() as *mut c_void,
                data.len() as u32,
            )
        };
    
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}