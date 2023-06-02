#![no_main]

// libfuzzer-sys crate provides the fuzzing entry point
use libfuzzer_sys::fuzz_target;

// Import the necessary types and functions from the code being fuzzed
use kmon::kernel::log::KernelLogs;

// libFuzzer entry point
fuzz_target!(|data: &[u8]| {
    // Create a mutable instance of KernelLogs
    let mut kernel_logs = KernelLogs::update(); // Replace `new` with your constructor if available

    // Use the input data to set the area_height and area_sub variables
    let area_height = u16::from_le_bytes([data[0], data[1]]);
    let area_sub = u16::from_le_bytes([data[2], data[3]]);

    // Call the select function with the constructed variables
    let _ = kernel_logs.select(area_height, area_sub);
});
