use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;

pub(crate) trait HasAcceptanceFilter11Bit {}
pub(crate) trait HasSetAcceptanceFilter11Bit {}

pub trait AcceptanceFilter11Bit {
    fn acceptance_filter_11bit(&self) -> Result<(u32, u32), PcanError>;
}

pub trait SetAcceptanceFilter11Bit {
    fn set_acceptance_filter_11bit(&self, code: u32, mask: u32) -> Result<(), PcanError>; 
}

impl<T: HasAcceptanceFilter11Bit + Channel> AcceptanceFilter11Bit for T {
    fn acceptance_filter_11bit(&self) -> Result<(u32, u32), PcanError> {
        let mut data = [0u8; 8];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_ACCEPTANCE_FILTER_11BIT as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        let value = u64::from_le_bytes(data);
        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                let code = ((value >> 32) & 0xFF_FF_FF_FF) as u32;
                let mask = ((value >> 0) & 0xFF_FF_FF_FF) as u32;
                Ok((code, mask))
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

impl<T: HasSetAcceptanceFilter11Bit + Channel> SetAcceptanceFilter11Bit for T {
    fn set_acceptance_filter_11bit(&self, code: u32, mask: u32) -> Result<(), PcanError> {
        let value = ((code as u64) << 32) | ((mask as u64) << 0);
        let mut data = value.to_le_bytes();
        println!("{:?}", data);

        let code = unsafe {
            pcan::CAN_SetValue(
                self.channel().into(),
                pcan::PCAN_ACCEPTANCE_FILTER_11BIT as u8,
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