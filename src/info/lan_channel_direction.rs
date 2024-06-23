use crate::channel::Channel;
use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};

use std::ffi::c_void;

#[derive(Debug, PartialEq, Clone)]
pub enum LanDirection {
    Read,
    Write,
    ReadWrite
}

pub(crate) trait HasLanChannelDirection {}

pub trait LanChannelDirection {
    fn lan_channel_direction(&self) -> Result<LanDirection, PcanError>;
}

impl<T: HasLanChannelDirection + Channel> LanChannelDirection for T {
    fn lan_channel_direction(&self) -> Result<LanDirection, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_LAN_CHANNEL_DIRECTION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                let read_flag = value & pcan::LAN_DIRECTION_READ == pcan::LAN_DIRECTION_READ;
                let write_flag = value & pcan::LAN_DIRECTION_WRITE == pcan::LAN_DIRECTION_WRITE;
                match (read_flag, write_flag) {
                    (true, true) => Ok(LanDirection::ReadWrite),
                    (true, false) => Ok(LanDirection::Read),
                    (false, true) => Ok(LanDirection::Write),
                    (false, false) => Err(PcanError::Unknown),
                }
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}