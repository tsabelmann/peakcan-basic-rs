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


pub fn add(left: usize, right: usize) -> usize {
    let val = unsafe {
        pcan::CAN_Initialize((pcan::PCAN_USBBUS1 as usize).try_into().unwrap(), pcan::PCAN_BAUD_500K.try_into().unwrap(), 0, 0, 0)
    } as usize;
    val
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
