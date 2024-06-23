use crate::pcan;
// use crate::error::{PcanError, PcanErrorCode, PcanOkError};
// use crate::channel::Channel;

// use std::ffi::c_void;
// use std::str;

#[derive(Debug, Clone)]
pub struct Timestamp {
    pub(crate) timestamp: pcan::TPCANTimestamp
}


#[derive(Debug, PartialEq, Clone)]
pub struct FdTimestamp(u64);

