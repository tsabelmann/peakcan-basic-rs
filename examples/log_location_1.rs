use std::path::PathBuf;

use peakcan_basic::logging::{log_location, set_log_location};


fn main() {
    let result = log_location();
    println!("Log-Location: {:?}", result);

    let result = set_log_location(&PathBuf::from("C:\\Users\\test"));
    println!("Set-Log-Location: {:?}", result);

    let result = log_location();
    println!("Log-Location: {:?}", result);

    let result = set_log_location("");
    println!("Set-Log-Location: {:?}", result);

    let result = log_location();
    println!("Log-Location: {:?}", result);
}
