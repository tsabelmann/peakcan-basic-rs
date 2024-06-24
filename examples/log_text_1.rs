use peakcan_basic::logging::set_log_text;

fn main() {
    let result = set_log_text("Hello World my Friend!");
    println!("Set-Log-Text: {:?}", result);
}
