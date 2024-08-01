//! The module containing the [`panic_handler`] function.

use crate::x86_instructions::{cli, hlt};
use alloc::{format, string::ToString};
use log::error;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo<'_>) -> ! {
    if let Some(location) = info.location() {
          
        };
       
    
    loop {
        // Stop execution of the current processor as much as possible.
        cli();
        hlt();
    }
}
