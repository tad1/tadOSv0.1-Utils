use crate::common::{NullLock, interface::Mutex};

// TODO: a auto run code to get gateway, or just a one time exectuable function
use super::definitions::*;


struct KernelGateInner{
    funciton: Option<KernelCall>
}


pub struct KernelGate{
    inner: NullLock<KernelGateInner>
}

impl KernelGate{
    pub const fn new() -> Self{
        KernelGate { inner: NullLock::new(KernelGateInner { 
            funciton: None
        }) }
    }

    pub fn set(&self, function: KernelCall){
        self.inner.lock(|inner| inner.funciton = Some(function))   
    }

    pub fn call(&self, function: KernelFunction, arg: u64, resp: u64){
        self.inner.lock(|inner| if let Some(kernel_gate) = inner.funciton{
            kernel_gate(function, arg, resp);
        } else {
            panic!("Kernel Call is not obtained");
        })
    }
}

pub static KERNEL_GATE: KernelGate = KernelGate::new();

pub fn kernel_call_r(funciton: KernelFunction, arg: u64, resp: u64){
    KERNEL_GATE.call(funciton, arg, resp);
}

pub fn kernel_call_a(funciton: KernelFunction, arg: u64){
    KERNEL_GATE.call(funciton, arg, 0);

}
pub fn kernel_call(funciton: KernelFunction){
    KERNEL_GATE.call(funciton, 0, 0);
}
