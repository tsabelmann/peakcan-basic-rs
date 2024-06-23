use crate::{error::PcanError, frame::CanFrame};
use crate::timestamp::Timestamp;

pub mod fd;
pub mod usb;

pub trait Socket {
    fn send(&self, frame: CanFrame) -> Result<(), PcanError>;
    fn recv(&self) -> Result<(CanFrame, Timestamp), PcanError>; 
    fn recv_frame(&self) -> Result<CanFrame, PcanError> {
        let (frame, _) = self.recv()?;
        Ok(frame)
    }
}