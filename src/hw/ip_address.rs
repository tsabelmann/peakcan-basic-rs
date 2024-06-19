
use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;
use std::net::Ipv4Addr;

pub(crate) trait HasIpAddress {}

pub trait IpAddress {
    fn ip_address(&self) -> Result<Ipv4Addr, PcanError>;
}

impl<T: HasIpAddress + Channel> IpAddress for T {
    fn ip_address(&self) -> Result<Ipv4Addr, PcanError> {
        let mut data = [0u8; 20];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_IP_ADDRESS as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                let ipv4 = match std::str::from_utf8(&data) {
                    Ok(string) => match string.parse::<Ipv4Addr>() {
                        Ok(addr) => addr,
                        Err(_) => return Err(PcanError::Unknown),
                    },
                    Err(_) => return Err(PcanError::Unknown),
                };
                Ok(ipv4)
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}