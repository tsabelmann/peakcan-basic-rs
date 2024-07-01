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

/// Re-export of baudrate module.
pub use baudrate::{Baudrate, Btr0Btr1, Btr0Btr1Code};

/// Re-export of bus module.
pub use bus::{dng::DngBus, isa::IsaBus, lan::LanBus, pcc::PccBus, pci::PciBus, usb::UsbBus};

/// Re-export of channel module.
pub use channel::{Channel, ChannelCode};

/// Re-export of error module.
pub use error::{PcanError, PcanOkError, PcanErrorCode};

/// Re-export of frame module.
pub use frame::{CanFrame, CanFdFrame, CanFrameBuilder};