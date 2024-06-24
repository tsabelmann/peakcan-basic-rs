use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use std::path::{Path, PathBuf};
use std::ffi::c_void;

const LOGGING_MAX_PATH: usize = 4096usize;

pub fn log_location() -> Result<PathBuf, PcanError> {
    let mut data = [b'\0'; LOGGING_MAX_PATH];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LOG_LOCATION as u8,
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

pub fn set_log_location<P: AsRef<Path>>(path: P) -> Result<(), PcanError> {
    let data = match path.as_ref().to_str() {
        Some(s) => s,
        None => return Err(PcanError::Unknown)
    };

    let code = unsafe {
        pcan::CAN_SetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LOG_LOCATION as u8,
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

pub fn log_status() -> Result<bool, PcanError> {
    let mut data = [0u8; 4];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LOG_STATUS as u8,
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

pub fn set_log_status(value: bool) -> Result<(), PcanError> {
    let data = match value {
        false => pcan::PCAN_PARAMETER_OFF,
        true => pcan::PCAN_PARAMETER_ON
    }.to_le_bytes();

    let code = unsafe {
        pcan::CAN_SetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LOG_STATUS as u8,
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