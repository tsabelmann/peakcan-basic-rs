use peakcan_basic::info::attached_channels_count::attached_channels_count;

fn main() {
    let result = attached_channels_count();
    println!("Attached-Channels-Count: {:?}", result);
}