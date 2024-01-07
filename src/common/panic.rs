use core::{panic::PanicInfo, arch::asm};
use aarch64_cpu;

use crate::println;


#[panic_handler]
fn panic(info: &PanicInfo) -> !{

    let (file, line, column) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        _ => ("???", 0, 0)
    };

    println!(
        "Code panic!\n\n\
        Parnic location:\n      File '{}', line {}, column {}\n\n\
        {}", file, line, column, info.message().unwrap_or(&format_args!("")));

        // Call the ambulance!
        unsafe{ asm!("svc 0x999") };

    // Prevent Further Mayhem
    loop {
        aarch64_cpu::asm::wfe();
    }
}