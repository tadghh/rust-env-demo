fn main() {
    // Set a custom environment variable
    let compile_test_status = "embedded compile value"; // Example value
    println!("cargo:rustc-env=COMPILE_TIME_ENV={}", compile_test_status);

    // Set a compile-time configuration
    // println!("cargo:rustc-cfg=COMPILE_TIME_ENV");
}
