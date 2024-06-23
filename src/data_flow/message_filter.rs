use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;

#[derive(Debug, PartialEq, Clone)]
pub enum MessageFilterMode {
    Open,
    Closed
}

#[derive(Debug, PartialEq, Clone)]
pub enum MessageFilterModeStatus {
    Open,
    Closed,
    Custom
}

pub(crate) trait HasMessageFilter {}
pub(crate) trait HasSetMessageFilter {}

pub trait MessageFilter {
    fn message_filter(&self) -> Result<MessageFilterModeStatus, PcanError>;
}

pub trait SetMessageFilter {
    fn set_message_filter(&self, value: MessageFilterMode) -> Result<(), PcanError>; 
}

impl<T: HasMessageFilter + Channel> MessageFilter for T {
    fn message_filter(&self) -> Result<MessageFilterModeStatus, PcanError> {
        let mut data = [0u8; 4];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_MESSAGE_FILTER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        let value = u32::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => match value {
                pcan::PCAN_FILTER_OPEN => Ok(MessageFilterModeStatus::Open),
                pcan::PCAN_FILTER_CLOSE => Ok(MessageFilterModeStatus::Closed),
                pcan::PCAN_FILTER_CUSTOM => Ok(MessageFilterModeStatus::Custom),
                _ => Err(PcanError::Unknown)
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl<T: HasSetMessageFilter + Channel> SetMessageFilter for T {
    fn set_message_filter(&self, value: MessageFilterMode) -> Result<(), PcanError> {
        let mut data = match value {
            MessageFilterMode::Open => pcan::PCAN_FILTER_OPEN,
            MessageFilterMode::Closed => pcan::PCAN_FILTER_CLOSE,
        }.to_le_bytes();
        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel().into(),
                pcan::PCAN_MESSAGE_FILTER as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}