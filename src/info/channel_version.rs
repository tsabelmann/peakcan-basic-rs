use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;
use std::str;


#[derive(Debug, PartialEq, Clone)]
pub struct ChVersion {
    data: [u8; pcan::MAX_LENGTH_VERSION_STRING as usize]
}

impl AsRef<str> for ChVersion {
    fn as_ref(&self) -> &str {
        str::from_utf8(&self.data).unwrap_or("").trim().trim_matches(char::from(0))
    }
}

pub(crate) trait HasChannelVersion {}

pub trait ChannelVersion {
    fn channel_version(&self) -> Result<ChVersion, PcanError>; 
}

impl<T: HasChannelVersion + Channel> ChannelVersion for T {
    fn channel_version(&self) -> Result<ChVersion, PcanError> {
        let mut data = [b'\0'; pcan::MAX_LENGTH_VERSION_STRING as usize];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_CHANNEL_VERSION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                match data.is_ascii() {
                    false => Err(PcanError::Unknown),
                    true => Ok(ChVersion { data })
                }
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}