#![no_main]

// libfuzzer-sys crate provides the fuzzing entry point
use libfuzzer_sys::fuzz_target;

// Import the necessary types and functions from the code being fuzzed
use kmon::kernel::cmd::{Command, ModuleCommand};

// libFuzzer entry point
fuzz_target!(|data: &[u8]| {
    // Create a mutable instance of ModuleCommand
    let module_command = ModuleCommand::None; // Replace with your desired initial value

    // Use the input data to construct the module_name variable
    let module_name = std::str::from_utf8(data).unwrap_or("");

    // Call the get method with the constructed variables
    let _ = module_command.get(module_name);
});
