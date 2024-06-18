use crate::pcan;

#[derive(Debug, PartialEq, Clone)]
pub enum DngBus {
    /// PCAN-Dongle/LPT interface, channel 1
    Dng1
}