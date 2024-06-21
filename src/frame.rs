use crate::pcan;


#[derive(Debug, PartialEq, Clone)]
pub enum IdType {
    Standard,
    Extended
}

#[derive(Debug, PartialEq, Clone)]
pub enum FrameType {
    Normal,
    Rtr,
    Echo,
    Error,
    PcanStatus
}

#[derive(Debug, Clone)]
pub struct CanFrame {
    frame: pcan::TPCANMsg
}

pub const STANDARD_CAN_FRAME_MASK: u32 = 0x07_FF;
pub const EXTENDED_CAN_FRAME_MASK: u32 = 0x1F_FF_FF_FF;

impl CanFrame {
    pub fn new(can_id: u32, id_type: IdType) -> CanFrameBuilder {
        let can_id = can_id & match id_type {
            IdType::Standard => STANDARD_CAN_FRAME_MASK,
            IdType::Extended => EXTENDED_CAN_FRAME_MASK,
        };
        CanFrameBuilder { can_id: can_id, id_type: id_type, frame_type: FrameType::Normal, data: [0u8; 8] }
    }

    pub fn can_id(&self) -> u32 {
        if self.is_standard_frame() {
            self.frame.ID & STANDARD_CAN_FRAME_MASK
        } else if self.is_extended_frame() {
            self.frame.ID & EXTENDED_CAN_FRAME_MASK
        } else {
            self.frame.ID & EXTENDED_CAN_FRAME_MASK
        }
    }

    pub fn is_standard_frame(&self) -> bool {
        let mask = pcan::PCAN_MESSAGE_STANDARD as u8;
        if self.frame.MSGTYPE & mask == mask {
            true
        } else {
            false
        }
    }

    pub fn is_extended_frame(&self) -> bool {
        let mask = pcan::PCAN_MESSAGE_EXTENDED as u8;
        if self.frame.MSGTYPE & mask == mask {
            true
        } else {
            false
        }
    }

    pub fn is_remote_frame(&self) -> bool {
        let mask = pcan::PCAN_MESSAGE_RTR as u8;
        if self.frame.MSGTYPE & mask == mask {
            true
        } else {
            false
        }
    }

    pub fn is_echo_frame(&self) -> bool {
        let mask = pcan::PCAN_MESSAGE_ECHO as u8;
        if self.frame.MSGTYPE & mask == mask {
            true
        } else {
            false
        }
    }

    pub fn is_error_frame(&self) -> bool {
        let mask = pcan::PCAN_MESSAGE_ERRFRAME as u8;
        if self.frame.MSGTYPE & mask == mask {
            true
        } else {
            false
        }
    }

    pub fn is_pcan_status_frame(&self) -> bool {
        let mask = pcan::PCAN_MESSAGE_STATUS as u8;
        if self.frame.MSGTYPE & mask == mask {
            true
        } else {
            false
        }
    }

    pub fn dlc(&self) -> u8 {
        self.frame.LEN
    }

    pub fn data(&self) -> &[u8] {
        let dlc = if self.is_remote_frame() {
            0
        } else {
            self.dlc()
        };
        &self.frame.DATA[0..dlc as usize]
    }

    pub fn mut_data(&mut self) -> &mut [u8] {
        let dlc = if self.is_remote_frame() {
            0
        } else {
            self.dlc()
        };
        &mut self.frame.DATA[0..dlc as usize]
    }
}


pub struct CanFrameBuilder {
    can_id: u32,
    id_type: IdType,
    frame_type: FrameType,
    data: [u8; 8]
}


#[derive(Debug, Clone)]
pub struct CanFdFrame {
    frame: pcan::TPCANMsgFD
}

