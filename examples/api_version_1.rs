use peakcan_basic::info::api_version::api_version;

fn main() {
    let result = api_version();
    println!("API-Version: {:?}", result);

    match result {
        Ok(result) => println!("{}.{}.{}.{}", result.major(), result.minor(), result.patch(), result.build()),
        Err(_) => println!("Something went wrong")
    }
}