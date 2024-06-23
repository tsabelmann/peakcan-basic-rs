use crate::pcan;

#[derive(Debug, PartialEq, Clone)]
pub enum Baudrate {
    ///   1 MBit/s
    Baud1M,
    /// 800 kBit/s
    Baud800K,
    /// 500 kBit/s
    Baud500K,
    /// 250 kBit/s
    Baud250K,
    /// 125 kBit/s
    Baud125K,
    /// 100 kBit/s
    Baud100K,
    ///  95,238 kBit/s
    Baud95K,
    ///  83,333 kBit/s
    Baud83K,
    ///  50 kBit/s
    Baud50K,
    ///  47,619 kBit/s
    Baud47K,
    ///  33,333 kBit/s
    Baud33K,
    ///  20 kBit/s
    Baud20K,
    ///  10 kBit/s
    Baud10K,
    ///   5 kBit/s
    Baud5K,
    ///   Variable configuration  
    Btr {
        btr0: u8,
        btr1: u8
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Btr0Btr1Code(u16);

impl Btr0Btr1Code {
    pub fn new(btr0btr1: u16) -> Btr0Btr1Code {
        Btr0Btr1Code { 0: btr0btr1 }
    }
}

impl From<Baudrate> for Btr0Btr1Code {
    fn from(value: Baudrate) -> Self {
        match value {
            Baudrate::Baud1M => Btr0Btr1Code(pcan::PCAN_BAUD_1M as u16),
            Baudrate::Baud800K => Btr0Btr1Code(pcan::PCAN_BAUD_800K as u16),
            Baudrate::Baud500K => Btr0Btr1Code(pcan::PCAN_BAUD_500K as u16),
            Baudrate::Baud250K => Btr0Btr1Code(pcan::PCAN_BAUD_250K as u16),
            Baudrate::Baud125K => Btr0Btr1Code(pcan::PCAN_BAUD_125K as u16),
            Baudrate::Baud100K => Btr0Btr1Code(pcan::PCAN_BAUD_100K as u16),
            Baudrate::Baud95K => Btr0Btr1Code(pcan::PCAN_BAUD_95K as u16),
            Baudrate::Baud83K => Btr0Btr1Code(pcan::PCAN_BAUD_83K as u16),
            Baudrate::Baud50K => Btr0Btr1Code(pcan::PCAN_BAUD_50K as u16),
            Baudrate::Baud47K => Btr0Btr1Code(pcan::PCAN_BAUD_47K as u16),
            Baudrate::Baud33K => Btr0Btr1Code(pcan::PCAN_BAUD_33K as u16),
            Baudrate::Baud20K => Btr0Btr1Code(pcan::PCAN_BAUD_20K as u16),
            Baudrate::Baud10K => Btr0Btr1Code(pcan::PCAN_BAUD_10K as u16),
            Baudrate::Baud5K => Btr0Btr1Code(pcan::PCAN_BAUD_5K as u16),
            Baudrate::Btr { btr0, btr1 } => {
                let val: u16 = (btr0 as u16) << 8 | (btr1 as u16);
                Btr0Btr1Code(val)
            },
        }
    }
}

impl From<Btr0Btr1Code> for Baudrate {
    fn from(value: Btr0Btr1Code) -> Self {
        match value {
            Btr0Btr1Code(value) => {
                match value as u32 {
                    pcan::PCAN_BAUD_1M => Baudrate::Baud1M,
                    pcan::PCAN_BAUD_800K => Baudrate::Baud800K,
                    pcan::PCAN_BAUD_500K => Baudrate::Baud500K,
                    pcan::PCAN_BAUD_250K => Baudrate::Baud250K,
                    pcan::PCAN_BAUD_125K => Baudrate::Baud125K,
                    pcan::PCAN_BAUD_100K => Baudrate::Baud100K,
                    pcan::PCAN_BAUD_95K => Baudrate::Baud95K,
                    pcan::PCAN_BAUD_83K => Baudrate::Baud83K,
                    pcan::PCAN_BAUD_50K => Baudrate::Baud50K,
                    pcan::PCAN_BAUD_47K => Baudrate::Baud47K,
                    pcan::PCAN_BAUD_33K => Baudrate::Baud33K,
                    pcan::PCAN_BAUD_20K => Baudrate::Baud20K,
                    pcan::PCAN_BAUD_10K => Baudrate::Baud10K,
                    pcan::PCAN_BAUD_5K => Baudrate::Baud5K,
                    _ => Baudrate::Btr { btr0: (value >> 8) as u8, btr1: (value & 0xFF) as u8 }
                }
            }
        }
    }
}

impl TryFrom<u32> for Btr0Btr1Code {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match u16::try_from(value) {
            Ok(value) => Ok(Btr0Btr1Code::new(value)),
            Err(_) => Err(()),
        }
    }
}

pub trait Btr0Btr1 {
    fn btr0btr1(&self) -> Btr0Btr1Code;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baudrate_1m() {
        let baud1: Baudrate = Baudrate::Baud1M;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_800k() {
        let baud1: Baudrate = Baudrate::Baud800K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_500k() {
        let baud1: Baudrate = Baudrate::Baud500K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_250k() {
        let baud1: Baudrate = Baudrate::Baud250K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_125k() {
        let baud1: Baudrate = Baudrate::Baud125K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_100k() {
        let baud1: Baudrate = Baudrate::Baud100K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_95k() {
        let baud1: Baudrate = Baudrate::Baud95K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_83k() {
        let baud1: Baudrate = Baudrate::Baud83K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_50k() {
        let baud1: Baudrate = Baudrate::Baud50K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_47k() {
        let baud1: Baudrate = Baudrate::Baud47K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_33k() {
        let baud1: Baudrate = Baudrate::Baud33K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_20k() {
        let baud1: Baudrate = Baudrate::Baud20K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_10k() {
        let baud1: Baudrate = Baudrate::Baud10K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_5k() {
        let baud1: Baudrate = Baudrate::Baud5K;
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }

    #[test]
    fn baudrate_btr() {
        let baud1: Baudrate = Baudrate::Btr { btr0: 0x12, btr1: 0x34 };
        let baud2: Baudrate = Btr0Btr1Code::from(baud1.clone()).into();
        assert_eq!(baud1, baud2);
    }


}


