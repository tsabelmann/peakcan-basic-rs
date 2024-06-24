use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;

pub(crate) trait HasTraceConfigure {}
pub(crate) trait HasSetTraceConfigure {}


pub enum TraceFileOptions {
    Segmented,
    Date,
    Time,
    Overwrite,
    DataLength
}


pub struct TraceFileConfiguration {
    inner: u32
}

impl TraceFileConfiguration {
    pub fn new(value: TraceFileOptions) -> TraceFileConfiguration {
        TraceFileConfiguration::from_trace_file_options(&[value])
    }

    pub fn from_trace_file_options(values: &[TraceFileOptions]) -> TraceFileConfiguration {
        let mut value = 0u32;
        for opt in values {
            let mask = match opt {
                TraceFileOptions::Segmented => pcan::TRACE_FILE_SEGMENTED,
                TraceFileOptions::Date => pcan::TRACE_FILE_DATE,
                TraceFileOptions::Time => pcan::TRACE_FILE_TIME,
                TraceFileOptions::Overwrite => pcan::TRACE_FILE_OVERWRITE,
                TraceFileOptions::DataLength => pcan::TRACE_FILE_DATA_LENGTH,
            };
            value |= mask;
        }
        TraceFileConfiguration { inner: value }
    }

    pub fn single_file(&self) -> bool {
        !self.segmented_file()
    }

    pub fn segmented_file(&self) -> bool {
        self.inner & pcan::TRACE_FILE_SEGMENTED == pcan::TRACE_FILE_SEGMENTED
    }

    pub fn file_date(&self) -> bool {
        self.inner & pcan::TRACE_FILE_DATE == pcan::TRACE_FILE_DATE
    }

    pub fn file_time(&self) -> bool {
        self.inner & pcan::TRACE_FILE_TIME == pcan::TRACE_FILE_TIME
    }

    pub fn overwrite_file(&self) -> bool {
        self.inner & pcan::TRACE_FILE_OVERWRITE == pcan::TRACE_FILE_OVERWRITE
    }

    pub fn file_data_length(&self) -> bool {
        self.inner & pcan::TRACE_FILE_DATA_LENGTH == pcan::TRACE_FILE_DATA_LENGTH
    }
}

pub trait TraceConfigure {
    fn trace_configuration(&self) -> Result<TraceFileConfiguration, PcanError>;
}

pub trait SetTraceConfigure {
    fn set_trace_configuration<T: Into<TraceFileConfiguration>>(&self, value: T) -> Result<(), PcanError>;
}

impl<T: HasTraceConfigure + Channel> TraceConfigure for T {
    fn trace_configuration(&self) -> Result<TraceFileConfiguration, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_TRACE_CONFIGURE as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => Ok(TraceFileConfiguration { inner: value}),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl<T: HasSetTraceConfigure + Channel> SetTraceConfigure for T {
    fn set_trace_configuration<S: Into<TraceFileConfiguration>>(&self, value: S) -> Result<(), PcanError> {
        let data = Into::<TraceFileConfiguration>::into(value).inner.to_le_bytes();
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel().into(),
                pcan::PCAN_TRACE_CONFIGURE as u8,
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