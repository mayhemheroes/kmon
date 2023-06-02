#![no_main]
// libfuzzer-sys crate provides the fuzzing entry point
use libfuzzer_sys::fuzz_target;

// Import the necessary types and functions from the code being fuzzed
use kmon::util::exec_cmd;

// std::process::Command is used in the function being fuzzed
use std::process::Command;

// libFuzzer entry point
fuzz_target!(|data: &[u8]| {
    // Use the input data to construct a command and command arguments
    let cmd = String::from_utf8_lossy(data).to_string();
    let cmd_args: Vec<&str> = vec![];

    // Call the exec_cmd function with the constructed command and command arguments
    let _ = exec_cmd(&cmd, &cmd_args);
});
