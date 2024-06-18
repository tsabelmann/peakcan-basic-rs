#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ChannelCode(pub u16);

pub trait Channel {
    fn channel(&self) -> ChannelCode;
}

impl From<ChannelCode> for u16 {
    fn from(value: ChannelCode) -> Self {
        value.0
    }
}

impl From<u16> for ChannelCode {
    fn from(value: u16) -> Self {
        ChannelCode(value)
    }
}
