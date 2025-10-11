#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate aos_uefi;

mod system;
mod wrappers;

use aos_uefi::{Handle, status::Status, system::SystemTable};
use core::panic::PanicInfo;

use crate::{system::System, wrappers::stdio::Stdout};

static mut SYSTEM_TABLE: *mut SystemTable = 0 as *mut SystemTable;

pub const unsafe fn system_table() -> &'static SystemTable {
    unsafe { &*SYSTEM_TABLE }
}

pub unsafe fn system_table_mut() -> &'static mut SystemTable {
    unsafe { &mut *SYSTEM_TABLE }
}

#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(
    _image_handle: Handle,
    system_table: &'static mut SystemTable,
) -> Status {
    unsafe {
        SYSTEM_TABLE = &mut *system_table;
        match main() {
            Ok(()) => loop {},
            Err(_s) => {
                panic!("main failed returning status {}", _s)
            }
        }
    }
}

fn main() -> Result<(), Status> {
    Stdout.clear_screen()?;
    let fs = System::get_fs()?;
    let root = fs.root()?;
    loop {
        let entry = match root.next_entry()? {
            Some(_e) => _e,
            None => break,
        };
        println!("Found entry\n{}", entry);
    }
    println!("I made it here!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(&l) = _info.location() {
        println!(
            "At file: {}, line {}, column {}",
            l.file(),
            l.line(),
            l.column()
        );
    }
    println!(
        "Panicked: {}",
        _info.message().as_str().unwrap_or("No message...")
    );
    loop {}
}
