use peakcan_basic_sys::v4_9_0_942;

pub fn add(left: usize, right: usize) -> usize {
    let val = unsafe {
        v4_9_0_942::CAN_Initialize((v4_9_0_942::PCAN_USBBUS1 as usize).try_into().unwrap(), v4_9_0_942::PCAN_BAUD_500K.try_into().unwrap(), 0, 0, 0)
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
