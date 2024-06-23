use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};
use crate::channel::Channel;

use std::ffi::c_void;
use std::str::{self, FromStr};

const FIRMWARE_VERSION_LENGTH: usize = 18usize;

#[derive(Debug, PartialEq, Clone)]
pub struct FwVerison {
    major: u16,
    minor: u16,
    release: u16,
}

impl FwVerison {
    pub fn major(&self) -> u16 {
        self.major
    }

    pub fn minor(&self) -> u16 {
        self.minor
    }

    pub fn release(&self) -> u16 {
        self.release
    }
}

impl FromStr for FwVerison {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string = s.trim().trim().trim_matches(char::from(0));
        let splits = string.split('.');

        let mut version = [0u16; 3];
        for (idx, split) in splits.enumerate() {
            if idx >= 3 {
                return Err(())
            }
            
            if let Ok(value) = split.parse() {
                version[idx] = value;
            } else {
                return Err(())
            }
        }
        let fwversion = FwVerison { major: version[0], minor: version[1], release: version[2] };
        Ok(fwversion)
    }
}

pub(crate) trait HasFirmwareVersion {}

pub trait FirmwareVersion {
    fn firmware_version(&self) -> Result<FwVerison, PcanError>;
}

impl<T: HasFirmwareVersion + Channel> FirmwareVersion for T {
    fn firmware_version(&self) -> Result<FwVerison, PcanError> {
        let mut data = [0u8; FIRMWARE_VERSION_LENGTH];
        let code = unsafe {
            pcan::CAN_GetValue(
                self.channel().into(),
                pcan::PCAN_FIRMWARE_VERSION as u8,
                data.as_mut_ptr() as *mut c_void,
                data.len() as u32
            )
        };

        match PcanOkError::try_from(PcanErrorCode::from(code)) {
            Ok(PcanOkError::Ok) => {
                if let Ok(string) = str::from_utf8(&data) {
                    match string.parse() {
                        Ok(api) => Ok(api),
                        Err(_) => Err(PcanError::Unknown)
                    }
                } else {
                    Err(PcanError::Unknown)
                }
            },
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}