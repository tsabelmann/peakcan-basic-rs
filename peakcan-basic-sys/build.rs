use bindgen;
use std::env;
use std::path::PathBuf;


#[cfg(feature = "v4_9_0_942")]
fn bindgen_v4_9_0_942() {
    // Use the following directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut library_path = PathBuf::new();
    library_path.push(&manifest_dir);
    library_path.push("ext");
    library_path.push("PCAN-Basic-v4_9_0_942");

    // Set up correct linker paths for the according architecture
    if cfg!(target_arch = "x86") {
        library_path.push("x86");
        println!("cargo:rustc-link-search=native={}", library_path.to_str().expect("Invalid library path"));
    } else if cfg!(target_arch = "x86_64") {
        library_path.push("x64");
        println!("cargo:rustc-link-search=native={}", library_path.to_str().expect("Invalid library path"));
    } else if cfg!(target_arch = "aarch64") {
        library_path.push("ARM64");
        println!("cargo:rustc-link-search=native={}", library_path.to_str().expect("Invalid library path"));
    } else {
        panic!("Unsupported architecture");
    }

    // Link against the correct target library
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=PCANBasic");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=pcanbasic");
    }

    let mut header_path = PathBuf::new();
    header_path.push(&manifest_dir);
    header_path.push("ext");
    header_path.push("PCAN-Basic-v4_9_0_942");
    header_path.push("Include");
    header_path.push("wrapper.h");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", header_path.to_str().expect("Invalid header path"));

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header_path.to_str().expect("Invalid header path"))
        // Allow function that begin with CAN
        .allowlist_function("CAN_.*")
        // Allow variables that begin with PCAN
        .allowlist_var("PCAN_.*")
        // Allow variables that begin with LAN
        .allowlist_var("LAN_.*")
        // Allow variables that begin with LOG
        .allowlist_var("LOG_.*")
        // Allow variables that begin with TRACE
        .allowlist_var("TRACE_.*")
        // Allow variables that begin with FEATURE
        .allowlist_var("FEATURE_.*")
        // Allow variables that begin with SERVICE
        .allowlist_var("SERVICE_.*")
        // Allow variables that begin with MAX
        .allowlist_var("MAX_.*")
        // Allow variables that begin with LOOKUP
        .allowlist_var("LOOKUP_.*")
        // Allow variables that begin with TPC
        .allowlist_var("TPC_.*")
        // Allow types that begin with PCAN
        .allowlist_type("TPC_.*")
        // Allow types that begin with tag
        .allowlist_type("tag.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings-v4_9_0_942.rs"))
        .expect("Couldn't write bindings!");
}


fn main() {
    #[cfg(feature = "v4_9_0_942")]
    bindgen_v4_9_0_942();
}