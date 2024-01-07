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

    println!("loaded!");
    
    let mut volume_controller = VolumeManager::new(&SDCARD, TestClock{});
    let volume0 = volume_controller.open_volume(VolumeIdx(0)).unwrap();
    let dir = volume_controller.open_root_dir(volume0).unwrap();
    let file_res = volume_controller.open_file_in_dir(dir, "appl", embedded_sdmmc::Mode::ReadOnly);
    print!("\x1b[2J");

    if let Ok(file) = file_res {
        

        const HEIGHT: usize = 30;
        const WIDTH: usize = 100;
        let mut buffer: [u8; WIDTH * HEIGHT] = [b' '; WIDTH * HEIGHT];

        let _ = volume_controller.file_seek_from_start(file, 0);
        
        for i in 0..65430/HEIGHT{
            kernel_call_a(common::KernelFunction::Spin, 50);
            print!("\x1b[H");
            
            let n_bytes = volume_controller.read(file, &mut buffer).unwrap();
            let mut strs: [&str; HEIGHT] = [""; HEIGHT];
            for ii in 0..HEIGHT{
                strs[ii] = unsafe {transmute(&buffer[ii*WIDTH..(ii+1)*WIDTH] as &[u8])};
                println!("{}", strs[ii]);
            }

        }
        
        let _ = volume_controller.close_file(file);
    } else {
        println!("File appl not found!");
    }
    
}