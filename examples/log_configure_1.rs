use peakcan_basic::logging::{log_configuration, set_log_configuration, LogFunction, LogOptions};

fn main() {
    let result = log_configuration();
    println!("Log-Configuration: {:?}", result);

    let result = set_log_configuration(LogOptions::Default);
    println!("Set-Log-Configuration: {:?}", result);

    let result = log_configuration();
    println!("Log-Configuration: {:?}", result);

    let result = set_log_configuration(LogOptions::Entry);
    println!("Set-Log-Configuration: {:?}", result);

    let result = log_configuration();
    println!("Log-Configuration: {:?}", result);

    let result = set_log_configuration(LogOptions::Parameters);
    println!("Set-Log-Configuration: {:?}", result);

    let result = log_configuration();
    println!("Log-Configuration: {:?}", result);

    let result = set_log_configuration(LogOptions::Leave);
    println!("Set-Log-Configuration: {:?}", result);

    let result = log_configuration();
    println!("Log-Configuration: {:?}", result);

    let result = set_log_configuration(LogOptions::Write);
    println!("Set-Log-Configuration: {:?}", result);

    let result = log_configuration();
    println!("Log-Configuration: {:?}", result);

    let result = set_log_configuration(LogOptions::Read);
    println!("Set-Log-Configuration: {:?}", result);

    let result = log_configuration();
    println!("Log-Configuration: {:?}", result);

    let result = set_log_configuration(LogFunction::from_options(&[LogOptions::Write, LogOptions::Read]));
    println!("Set-Log-Configuration: {:?}", result);

    let result = log_configuration();
    println!("Log-Configuration: {:?}", result);
}
