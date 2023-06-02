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
    if let Ok(cmd) = std::str::from_utf8(data) {
        let cmd_args: Vec<&str> = vec![];
        let _ = exec_cmd(&cmd, &cmd_args);
    }
});
