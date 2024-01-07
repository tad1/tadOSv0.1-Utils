#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(format_args_nl)]

use common::{kernel_call, kernel_call_r, kernel_call_a};

use crate::common::{KERNEL_GATE, get_kernel_gate};
mod common;



#[no_mangle]
pub fn main(){
    // Currently a boilerplate that I'm willing to integrate automate with linker script, and base assembly
    KERNEL_GATE.set(unsafe {get_kernel_gate()});

    print!("\x1b[2J");
    
}