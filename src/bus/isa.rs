use crate::pcan;

#[derive(Debug, PartialEq, Clone)]
pub enum IsaBus {
    /// PCAN-ISA interface, channel 1
    Isa1,
    /// PCAN-ISA interface, channel 2
    Isa2,
    /// PCAN-ISA interface, channel 3
    Isa3,
    /// PCAN-ISA interface, channel 4
    Isa4,
    /// PCAN-ISA interface, channel 5
    Isa5,
    /// PCAN-ISA interface, channel 6
    Isa6,
    /// PCAN-ISA interface, channel 7
    Isa7,
    /// PCAN-ISA interface, channel 8
    Isa8
}