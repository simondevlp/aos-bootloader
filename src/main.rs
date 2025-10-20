#![no_std]
#![no_main]

extern crate aos_uefi_wrappers as std;

mod panic;
use std::{status::Status, stdio::Stdout, system::System, *};

#[unsafe(no_mangle)]
extern "C" fn amain() -> Status {
    match main() {
        Ok(()) => Status::SUCCESS,
        Err(_s) => panic!("main failed with status {}", _s),
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
