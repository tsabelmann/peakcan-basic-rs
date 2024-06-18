use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;
use std::str;


#[derive(Debug, PartialEq, Clone)]
pub struct HwName {
    data: [u8; pcan::MAX_LENGTH_HARDWARE_NAME as usize]
}

impl AsRef<str> for HwName {
    fn as_ref(&self) -> &str {
        str::from_utf8(&self.data).unwrap_or("")
    }
}

pub(crate) trait HasHardwareName {}

pub trait HardwareName {
    fn hardware_name(&self) -> Result<HwName, PcanError>; 
}

impl<T: HasHardwareName + Channel> HardwareName for T {
    fn hardware_name(&self) -> Result<HwName, PcanError> {
        let mut data: [u8; 33] = [b'\0'; pcan::MAX_LENGTH_HARDWARE_NAME as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_HARDWARE_NAME as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                match data.is_ascii() {
                    false => Err(PcanError::Unknown),
                    true => Ok(HwName {data})
                }
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}