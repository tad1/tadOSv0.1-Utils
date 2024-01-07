#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(format_args_nl)]

use core::mem::transmute;

use common::{kernel_call, kernel_call_r, kernel_call_a};
use embedded_sdmmc::{VolumeManager, VolumeIdx};

use crate::common::{KERNEL_GATE, get_kernel_gate, SDCARD, TestClock};
mod common;



#[no_mangle]
pub fn main(){
    // Currently a boilerplate that I'm willing to integrate automate with linker script, and base assembly
    KERNEL_GATE.set(unsafe {get_kernel_gate()});

    let mut volume_controller = VolumeManager::new(&SDCARD, TestClock{});
    let volume0 = volume_controller.open_volume(VolumeIdx(0)).unwrap();
    let dir = volume_controller.open_root_dir(volume0).unwrap();
    let _ = volume_controller.iterate_dir(dir, |entry| {
        println!("\t{}",entry.name);
    });
}