use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::path::{Path, PathBuf};
use std::ffi::c_void;

const TRACE_MAX_PATH: usize = 4096usize;

pub(crate) trait HasTraceLocation {}
pub(crate) trait HasSetTraceLocation {}

pub trait TraceLocation {
    fn trace_location(&self) -> Result<PathBuf, PcanError>;
}

pub trait SetTraceLocation {
    fn set_trace_location<T: AsRef<Path>>(&self, value: T) -> Result<(), PcanError>;
}

impl<T: HasTraceLocation + Channel> TraceLocation for T {
    fn trace_location(&self) -> Result<PathBuf, PcanError> {
        let mut data = [b'\0'; TRACE_MAX_PATH];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_TRACE_LOCATION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => match std::str::from_utf8(&data) {
                Ok(string) => {
                    let string = string.trim().trim_matches(char::from(0));
                    Ok(PathBuf::from(string))
                },
                _ => Err(PcanError::Unknown)
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl<T: HasSetTraceLocation + Channel> SetTraceLocation for T {
    fn set_trace_location<S: AsRef<Path>>(&self, value: S) -> Result<(), PcanError> {
        let data = match value.as_ref().to_str() {
            Some(s) => s,
            None => return Err(PcanError::Unknown)
        };

        // construct null terminated string for the C-library
        let data = if data == "" {
            "\0"
        } else {
            data
        };

        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel().into(),
                pcan::PCAN_TRACE_LOCATION as u8,
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