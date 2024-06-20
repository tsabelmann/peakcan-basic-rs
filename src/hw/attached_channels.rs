use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};

use std::ffi::c_void;

pub fn attached_channel_count() -> Result<u32, PcanError> {
    let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                pcan::PCAN_NONEBUS as u16,
                pcan::PCAN_ATTACHED_CHANNELS_COUNT as u8,
                data.as_mut_ptr() as *mut c_void,
                4
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => Ok(value),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
}