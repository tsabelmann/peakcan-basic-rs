use crate::pcan;

#[derive(Debug, PartialEq, Clone)]
pub enum PccBus {
    /// PCAN-PC Card interface, channel 1
    Pcc1,
    /// PCAN-PC Card interface, channel 2
    Pcc2
}