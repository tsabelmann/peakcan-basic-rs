use crate::pcan;

#[derive(Debug, PartialEq, Clone)]
pub enum PciBus {
    /// PCAN-PCI interface, channel 1
    Pci1,
    /// PCAN-PCI interface, channel 2
    Pci2,
    /// PCAN-PCI interface, channel 3
    Pci3,
    /// PCAN-PCI interface, channel 4
    Pci4,
    /// PCAN-PCI interface, channel 5
    Pci5,
    /// PCAN-PCI interface, channel 6
    Pci6,
    /// PCAN-PCI interface, channel 7
    Pci7,
    /// PCAN-PCI interface, channel 8
    Pci8,
    /// PCAN-PCI interface, channel 9
    Pci9,
    /// PCAN-PCI interface, channel 10
    Pci10,
    /// PCAN-PCI interface, channel 11
    Pci11,
    /// PCAN-PCI interface, channel 12
    Pci12,
    /// PCAN-PCI interface, channel 13
    Pci13,
    /// PCAN-PCI interface, channel 14
    Pci14,
    /// PCAN-PCI interface, channel 15
    Pci15,
    /// PCAN-PCI interface, channel 16
    Pci16
}