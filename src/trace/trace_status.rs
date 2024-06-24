use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;

pub(crate) trait HasTraceStatus {}
pub(crate) trait HasSetTraceStatus {}

pub trait TraceStatus {
    fn trace_status(&self) -> Result<bool, PcanError>;
}

pub trait SetTraceStatus {
    fn set_trace_status(&self, value: bool) -> Result<(), PcanError>;
}

impl<T: HasTraceStatus + Channel> TraceStatus for T {
    fn trace_status(&self) -> Result<bool, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_TRACE_STATUS as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => match value {
                pcan::PCAN_PARAMETER_OFF => Ok(false),
                pcan::PCAN_PARAMETER_ON => Ok(true),
                _ => Err(PcanError::Unknown)
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl<T: HasSetTraceStatus + Channel> SetTraceStatus for T {
    fn set_trace_status(&self, value: bool) -> Result<(), PcanError> {
        let data = match value {
            false => pcan::PCAN_PARAMETER_OFF,
            true => pcan::PCAN_PARAMETER_ON
        }.to_le_bytes();

        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel().into(),
                pcan::PCAN_TRACE_STATUS as u8,
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