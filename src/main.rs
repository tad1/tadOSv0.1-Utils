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

    print!("Enter x:");
    let x = get_number();
    print!("\nEnter y:");
    let y = get_number();
    println!("\n{}+{}={}",x,y, x+y);

    
}

fn get_char() -> char{
    let mut arg = '\0';
    unsafe{
        let ptr = &mut arg as *mut char;
        kernel_call_a(common::KernelFunction::ReadChar, ptr as u64);
    }
    arg
}

fn get_number() -> u64{
    let mut char = get_char();
    let mut res: u64 = 0;
    
    while char <= '9' && char >= '0' {
        res *= 10 as u64;
        res += char as u64 - '0' as u64;
        char = get_char();
    }
    res
}