use crate::pcan;

#[derive(Debug, PartialEq, Clone)]
pub enum PcanError {
    /// Transmit buffer in CAN controller is full
    XmtFull,
    /// CAN controller was read too late
    Overrun,
    /// Bus error: an error counter reached the 'light' limit
    BusLight,
    /// Bus error: an error counter reached the 'heavy' limit
    BusHeavy,
    /// Bus error: the CAN controller is error passive
    BusPassive,
    /// Bus error: the CAN controller is in bus-off state
    BusOff,
    /// Receive queue is empty
    QrcvEmpty,
    /// Receive queue was read too late
    QOverrun,
    /// Transmit queue is full
    QXmtFull,
    /// Test of the CAN controller hardware registers failed (no hardware found)
    RegTest,
    /// Driver not loaded
    NoDriver,
    /// Hardware already in use by a Net
    HwInUse,
    /// A Client is already connected to the Net
    NetInUse,
    /// Hardware handle is invalid
    IllHw,
    /// Net handle is invalid
    IllNet,
    /// Client handle is invalid
    IllClient,
    /// Resource (FIFO, Client, timeout) cannot be created
    Resouce,
    /// Invalid parameter
    IllParamType,
    /// Invalid parameter value
    IllParamVal,
    /// Unknown error
    Unknown,
    /// Invalid data, function, or action
    IllData,
    /// Driver object state is wrong for the attempted operation
    IllMode,
    /// An operation was successfully carried out, however, irregularities were registered
    Caution,
    /// Channel is not initialized [Value was changed from 0x40000 to 0x4000000]
    Initialize,
    /// Invalid operation [Value was changed from 0x80000 to 0x8000000]
    IllOperation
}

#[derive(Debug, PartialEq, Clone)]
pub enum PcanOkError {
    /// No error 
    Ok,
    /// Some error. See [PcanError].
    Err(PcanError)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PcanErrorCode(u32);

impl From<PcanError> for PcanErrorCode {
    fn from(value: PcanError) -> Self {
        match value {
            PcanError::XmtFull => PcanErrorCode(pcan::PCAN_ERROR_XMTFULL),
            PcanError::Overrun =>PcanErrorCode(pcan::PCAN_ERROR_OVERRUN),
            PcanError::BusLight => PcanErrorCode(pcan::PCAN_ERROR_BUSLIGHT),
            PcanError::BusHeavy => PcanErrorCode(pcan::PCAN_ERROR_BUSHEAVY),
            PcanError::BusPassive => PcanErrorCode(pcan::PCAN_ERROR_BUSPASSIVE),
            PcanError::BusOff => PcanErrorCode(pcan::PCAN_ERROR_BUSOFF),
            PcanError::QrcvEmpty => PcanErrorCode(pcan::PCAN_ERROR_QRCVEMPTY),
            PcanError::QOverrun => PcanErrorCode(pcan::PCAN_ERROR_QOVERRUN),
            PcanError::QXmtFull => PcanErrorCode(pcan::PCAN_ERROR_QXMTFULL),
            PcanError::RegTest => PcanErrorCode(pcan::PCAN_ERROR_REGTEST),
            PcanError::NoDriver => PcanErrorCode(pcan::PCAN_ERROR_NODRIVER),
            PcanError::HwInUse => PcanErrorCode(pcan::PCAN_ERROR_HWINUSE),
            PcanError::NetInUse => PcanErrorCode(pcan::PCAN_ERROR_NETINUSE),
            PcanError::IllHw => PcanErrorCode(pcan::PCAN_ERROR_ILLHW),
            PcanError::IllNet => PcanErrorCode(pcan::PCAN_ERROR_ILLNET),
            PcanError::IllClient => PcanErrorCode(pcan::PCAN_ERROR_ILLCLIENT),
            PcanError::Resouce => PcanErrorCode(pcan::PCAN_ERROR_RESOURCE),
            PcanError::IllParamType => PcanErrorCode(pcan::PCAN_ERROR_ILLPARAMTYPE),
            PcanError::IllParamVal => PcanErrorCode(pcan::PCAN_ERROR_ILLPARAMVAL),
            PcanError::Unknown => PcanErrorCode(pcan::PCAN_ERROR_UNKNOWN),
            PcanError::IllData => PcanErrorCode(pcan::PCAN_ERROR_ILLDATA),
            PcanError::IllMode => PcanErrorCode(pcan::PCAN_ERROR_ILLMODE),
            PcanError::Caution => PcanErrorCode(pcan::PCAN_ERROR_CAUTION),
            PcanError::Initialize => PcanErrorCode(pcan::PCAN_ERROR_INITIALIZE),
            PcanError::IllOperation => PcanErrorCode(pcan::PCAN_ERROR_ILLOPERATION),
        }
    }
}

impl From<PcanOkError> for PcanErrorCode {
    fn from(value: PcanOkError) -> Self {
        match value {
            PcanOkError::Ok => PcanErrorCode(pcan::PCAN_ERROR_OK),
            PcanOkError::Err(err) => err.into(),
        }
    }
}

impl TryFrom<PcanErrorCode> for PcanError {    
    type Error = ();
    fn try_from(value: PcanErrorCode) -> Result<Self, Self::Error> {
        match value.0 {
            pcan::PCAN_ERROR_XMTFULL => Ok(PcanError::XmtFull),
            pcan::PCAN_ERROR_OVERRUN => Ok(PcanError::Overrun),
            pcan::PCAN_ERROR_BUSLIGHT => Ok(PcanError::BusLight),
            pcan::PCAN_ERROR_BUSHEAVY => Ok(PcanError::BusHeavy),
            pcan::PCAN_ERROR_BUSPASSIVE => Ok(PcanError::BusPassive),
            pcan::PCAN_ERROR_BUSOFF => Ok(PcanError::BusOff),
            pcan::PCAN_ERROR_QRCVEMPTY => Ok(PcanError::QrcvEmpty),
            pcan::PCAN_ERROR_QOVERRUN => Ok(PcanError::QOverrun),
            pcan::PCAN_ERROR_QXMTFULL => Ok(PcanError::QXmtFull),
            pcan::PCAN_ERROR_REGTEST => Ok(PcanError::RegTest),
            pcan::PCAN_ERROR_NODRIVER => Ok(PcanError::NoDriver),
            pcan::PCAN_ERROR_HWINUSE => Ok(PcanError::HwInUse),
            pcan::PCAN_ERROR_NETINUSE => Ok(PcanError::NetInUse),
            pcan::PCAN_ERROR_ILLHW => Ok(PcanError::IllHw),
            pcan::PCAN_ERROR_ILLNET => Ok(PcanError::IllNet),
            pcan::PCAN_ERROR_ILLCLIENT => Ok(PcanError::IllClient),
            pcan::PCAN_ERROR_RESOURCE => Ok(PcanError::Resouce),
            pcan::PCAN_ERROR_ILLPARAMTYPE => Ok(PcanError::IllParamType),
            pcan::PCAN_ERROR_ILLPARAMVAL => Ok(PcanError::IllParamVal),
            pcan::PCAN_ERROR_UNKNOWN => Ok(PcanError::Unknown),
            pcan::PCAN_ERROR_ILLDATA => Ok(PcanError::IllData),
            pcan::PCAN_ERROR_ILLMODE => Ok(PcanError::IllMode),
            pcan::PCAN_ERROR_CAUTION => Ok(PcanError::Caution),
            pcan::PCAN_ERROR_INITIALIZE => Ok(PcanError::Initialize),
            pcan::PCAN_ERROR_ILLOPERATION => Ok(PcanError::IllOperation),
            _ => Err(())
        }
    }
}

impl TryFrom<PcanErrorCode> for PcanOkError {
    type Error = ();
    fn try_from(value: PcanErrorCode) -> Result<Self, Self::Error> {
        match value.0 {
            pcan::PCAN_ERROR_OK => Ok(PcanOkError::Ok),
            _ => {
                let error = value.try_into()?;
                Ok(PcanOkError::Err(error))
            }
        }
    }
}

impl From<PcanErrorCode> for u32 {
    fn from(value: PcanErrorCode) -> Self {
        value.0
    }
}

impl From<u32> for PcanErrorCode {
    fn from(value: u32) -> Self {
        PcanErrorCode(value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_ok() {
        let err1: PcanErrorCode = PcanOkError::Ok.into();
        let err2: PcanErrorCode = PcanError::XmtFull.into();
        assert_ne!(err1, err2);
    }

    #[test]
    fn error_xmt_full() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::XmtFull).into();
        let err2: PcanErrorCode = PcanError::XmtFull.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_overrun() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::Overrun).into();
        let err2: PcanErrorCode = PcanError::Overrun.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_bus_light() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::BusLight).into();
        let err2: PcanErrorCode = PcanError::BusLight.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_bus_heavy() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::BusHeavy).into();
        let err2: PcanErrorCode = PcanError::BusHeavy.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_bus_passive() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::BusPassive).into();
        let err2: PcanErrorCode = PcanError::BusPassive.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_bus_off() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::BusOff).into();
        let err2: PcanErrorCode = PcanError::BusOff.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_qrcv_empty() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::QrcvEmpty).into();
        let err2: PcanErrorCode = PcanError::QrcvEmpty.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_qxmt_full() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::QXmtFull).into();
        let err2: PcanErrorCode = PcanError::QXmtFull.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_reg_test() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::RegTest).into();
        let err2: PcanErrorCode = PcanError::RegTest.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_no_driver() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::NoDriver).into();
        let err2: PcanErrorCode = PcanError::NoDriver.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_hw_in_use() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::HwInUse).into();
        let err2: PcanErrorCode = PcanError::HwInUse.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_net_in_use() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::NetInUse).into();
        let err2: PcanErrorCode = PcanError::NetInUse.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_ill_hw() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::IllHw).into();
        let err2: PcanErrorCode = PcanError::IllHw.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_ill_net() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::IllNet).into();
        let err2: PcanErrorCode = PcanError::IllNet.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_ill_client() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::IllClient).into();
        let err2: PcanErrorCode = PcanError::IllClient.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_resource() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::Resouce).into();
        let err2: PcanErrorCode = PcanError::Resouce.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_ill_param_type() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::IllParamType).into();
        let err2: PcanErrorCode = PcanError::IllParamType.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_ill_param_val() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::IllParamVal).into();
        let err2: PcanErrorCode = PcanError::IllParamVal.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_unknown() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::Unknown).into();
        let err2: PcanErrorCode = PcanError::Unknown.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_ill_data() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::IllData).into();
        let err2: PcanErrorCode = PcanError::IllData.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_ill_mode() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::IllMode).into();
        let err2: PcanErrorCode = PcanError::IllMode.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_caution() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::Caution).into();
        let err2: PcanErrorCode = PcanError::Caution.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_initialize() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::Initialize).into();
        let err2: PcanErrorCode = PcanError::Initialize.into();
        assert_eq!(err1, err2);
    }

    #[test]
    fn error_ill_operation() {
        let err1: PcanErrorCode = PcanOkError::Err(PcanError::IllOperation).into();
        let err2: PcanErrorCode = PcanError::IllOperation.into();
        assert_eq!(err1, err2);
    }
}