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

#[derive(Debug, PartialEq, Clone)]
pub enum LogOptions {
    /// This value is always active.
    Default,
    /// Logs when a function is entered.
    Entry,
    /// Logs the parameters passed to a function.
    Parameters,
    /// Logs when a function is leaved and its return value.
    Leave,
    /// Logs the parameters and CAN data passed to the CAN_Write function.
    Write,
    /// Logs the parameters and CAN data received through the CAN_Read function.
    Read
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogFunction {
    inner: u32
}

impl LogFunction {
    pub fn new(value: LogOptions) -> LogFunction {
        LogFunction::from_options(&[value])
    }

    pub fn from_options(values: &[LogOptions]) -> LogFunction {
        let mut value = 0u32;
        for opt in values {
            let mask = match opt {
                LogOptions::Default => pcan::LOG_FUNCTION_DEFAULT,
                LogOptions::Entry => pcan::LOG_FUNCTION_ENTRY,
                LogOptions::Parameters => pcan::LOG_FUNCTION_PARAMETERS,
                LogOptions::Leave => pcan::LOG_FUNCTION_LEAVE,
                LogOptions::Write => pcan::LOG_FUNCTION_WRITE,
                LogOptions::Read => pcan::LOG_FUNCTION_READ,
            };
            value |= mask;
        }
        value &= pcan::LOG_FUNCTION_ALL;
        LogFunction { inner: value }
    }

    pub fn log_default(&self) -> bool {
        self.inner & pcan::LOG_FUNCTION_DEFAULT == pcan::LOG_FUNCTION_DEFAULT
    }
    pub fn log_entry(&self) -> bool {
        self.inner & pcan::LOG_FUNCTION_DEFAULT == pcan::LOG_FUNCTION_ENTRY
    }

    pub fn log_parameters(&self) -> bool {
        self.inner & pcan::LOG_FUNCTION_DEFAULT == pcan::LOG_FUNCTION_PARAMETERS
    }
    
    pub fn log_leave(&self) -> bool {
        self.inner & pcan::LOG_FUNCTION_DEFAULT == pcan::LOG_FUNCTION_LEAVE
    }
    
    pub fn log_write(&self) -> bool {
        self.inner & pcan::LOG_FUNCTION_DEFAULT == pcan::LOG_FUNCTION_WRITE
    }
    
    pub fn log_read(&self) -> bool {
        self.inner & pcan::LOG_FUNCTION_DEFAULT == pcan::LOG_FUNCTION_READ
    }
}

impl From<LogOptions> for LogFunction {
    fn from(value: LogOptions) -> Self {
        let value = match value {
            LogOptions::Default => pcan::LOG_FUNCTION_DEFAULT,
            LogOptions::Entry => pcan::LOG_FUNCTION_ENTRY,
            LogOptions::Parameters => pcan::LOG_FUNCTION_PARAMETERS,
            LogOptions::Leave => pcan::LOG_FUNCTION_LEAVE,
            LogOptions::Write => pcan::LOG_FUNCTION_WRITE,
            LogOptions::Read => pcan::LOG_FUNCTION_READ,
        };
        LogFunction { inner: value }
    }
}

pub fn log_configuration() -> Result<LogFunction, PcanError> {
    let mut data = [0u8; 4];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LOG_CONFIGURE as u8,
            data.as_mut_ptr() as *mut c_void,
            data.len() as u32
        )
    };

    let value = u32::from_le_bytes(data);
    match PcanOkError::try_from(PcanErrorCode::from(code)) {
        Ok(PcanOkError::Ok) => {
            let value = value & pcan::LOG_FUNCTION_ALL;
            Ok( LogFunction {inner: value} )
        },
        Ok(PcanOkError::Err(err)) => Err(err),
        Err(_) => Err(PcanError::Unknown),
    }
}

pub fn set_log_configuration<T: Into<LogFunction>>(value: T) -> Result<(), PcanError> {
    let value: LogFunction = value.into();
    let data = value.inner.to_le_bytes();

    let code = unsafe {
        pcan::CAN_SetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LOG_CONFIGURE as u8,
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