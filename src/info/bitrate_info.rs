use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;
use crate::baudrate::{Baudrate, Btr0Btr1Code};

use std::ffi::c_void;

pub(crate) trait HasBitrateInfo {}

pub trait BitrateInfo {
    fn bitrate_info(&self) -> Result<Baudrate, PcanError>;
}

impl<T: HasBitrateInfo + Channel> BitrateInfo for T {
    fn bitrate_info(&self) -> Result<Baudrate, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_BITRATE_INFO as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                let baudrate = match Btr0Btr1Code::try_from(value) {
                    Ok(value) => value.into(),
                    Err(_) => return Err(PcanError::Unknown)
                };
                Ok(baudrate)
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}