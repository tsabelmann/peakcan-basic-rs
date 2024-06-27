use crate::pcan;
use crate::error::{PcanError, PcanErrorCode, PcanOkError};

use std::ffi::c_void;
use std::str::{self, FromStr};

const API_VERSION_LENGTH: usize = 24usize;

#[derive(Debug, PartialEq, Clone)]
pub struct ApiVerison {
    major: u16,
    minor: u16,
    patch: u16,
    build: u16
}

impl ApiVerison {
    pub fn major(&self) -> u16 {
        self.major
    }

    pub fn minor(&self) -> u16 {
        self.minor
    }

    pub fn patch(&self) -> u16 {
        self.patch
    }

    pub fn build(&self) -> u16 {
        self.build
    }
}

impl FromStr for ApiVerison {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string = s.trim().trim().trim_matches(char::from(0));
        let splits = string.split('.');

        let mut version = [0u16; 4];
        for (idx, split) in splits.enumerate() {
            if idx >= 4 {
                return Err(())
            }
            
            if let Ok(value) = split.parse() {
                version[idx] = value;
            } else {
                return Err(())
            }
        }
        let api = ApiVerison {major: version[0], minor: version[1], patch: version[2], build: version[3]};
        Ok(api)
    }
}

pub fn api_version() -> Result<ApiVerison, PcanError> {
    let mut data = [0u8; API_VERSION_LENGTH];
    let code = unsafe {
        pcan::CAN_GetValue(
            pcan::PCAN_NONEBUS as u16,
            pcan::PCAN_API_VERSION as u8,
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