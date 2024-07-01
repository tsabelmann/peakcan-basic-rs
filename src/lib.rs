use peakcan_basic_sys::v4_9_0_942 as pcan;

pub mod baudrate;
pub mod bus;
pub mod channel;
pub mod error;
pub mod hw;
pub mod info;
pub mod frame;
pub mod socket;
pub mod timestamp;
pub mod special_behaviors;
pub mod data_flow;
pub mod logging;
pub mod trace;
pub mod constants;

/// Re-export of frame module
pub use frame::{CanFrame, CanFdFrame, CanFrameBuilder};