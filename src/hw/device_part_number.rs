use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;
use std::str;

const DEVICE_PART_NUMBER_LENGTH: usize = 100usize;

#[derive(Debug, PartialEq, Clone)]
pub struct DevPartNumber {
    data: [u8; DEVICE_PART_NUMBER_LENGTH]
}

impl AsRef<str> for DevPartNumber {
    fn as_ref(&self) -> &str {
        str::from_utf8(&self.data).unwrap_or("").trim().trim_matches(char::from(0))
    }
}

pub(crate) trait HasDevicePartNumber {}

pub trait DevicePartNumber {
    fn device_part_number(&self) -> Result<DevPartNumber, PcanError>; 
}

impl<T: HasDevicePartNumber + Channel> DevicePartNumber for T {
    fn device_part_number(&self) -> Result<DevPartNumber, PcanError> {
        let mut data: [u8; DEVICE_PART_NUMBER_LENGTH] = [b'\0'; DEVICE_PART_NUMBER_LENGTH];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_DEVICE_PART_NUMBER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                match data.is_ascii() {
                    false => Err(PcanError::Unknown),
                    true => Ok(DevPartNumber {data})
                }
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}