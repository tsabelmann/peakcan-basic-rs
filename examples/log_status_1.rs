use peakcan_basic::logging::{log_status, set_log_status};

fn main() {
    let result = log_status();
    println!("Log-Status: {:?}", result);

    let result = set_log_status(true);
    println!("Set-Log-Status: {:?}", result);

    let result = log_status();
    println!("Log-Status: {:?}", result);

    let result = set_log_status(false);
    println!("Set-Log-Status: {:?}", result);

    let result = log_status();
    println!("Log-Status: {:?}", result);
}
