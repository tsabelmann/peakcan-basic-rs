use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};

use std::ffi::c_void;

#[derive(Debug, PartialEq, Clone)]
pub enum LanServiceStatus {
    Stopped,
    Running
}

pub fn lan_service_status() -> Result<LanServiceStatus, PcanError> {
    let mut data = [0u8; 4];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_LAN_SERVICE_STATUS as u8,
            data.as_mut_ptr() as *mut c_void,
            data.len() as u32
        )
    };

    let value = u32::from_le_bytes(data);
    match PcanOkError::try_from(PcanErrorCode::from(code)) {
        Ok(PcanOkError::Ok) => {
            match value {
                pcan::SERVICE_STATUS_STOPPED => Ok(LanServiceStatus::Stopped),
                pcan::SERVICE_STATUS_RUNNING => Ok(LanServiceStatus::Running),
                _ => Err(PcanError::Unknown)
            }
        },
        Ok(PcanOkError::Err(err)) => Err(err),
        Err(_) => Err(PcanError::Unknown),
    }
}