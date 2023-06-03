#![no_main]
use libfuzzer_sys::fuzz_target;
use kmon::kernel::cmd::ModuleCommand;

fuzz_target!(|data: &[u8]| {
    // Convert the fuzzed data to a string
    let module_name = String::from_utf8_lossy(data);

    // Call the is_module_filename function
    let _ = ModuleCommand::is_module_filename(&module_name);
});
