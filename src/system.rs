use aos_uefi::{boot::BootServices, fs::sfs::SimpleFileSystem, guid::Guid, status::Status};

use crate::{println, system_table, wrappers::fs::FileSystem};

pub struct System;

impl System {
    pub const fn bootsrv() -> &'static BootServices {
        unsafe { system_table().boot_srv }
    }

    pub fn get_protocol(id: &Guid) -> Result<usize, Status> {
        let mut ptr = 0usize;
        let status = (Self::bootsrv().locate_protocol)(id, 0, &mut ptr);
        match status {
            Status::SUCCESS => Ok(ptr),
            _s => {
                println!("Status: {}", _s);
                panic!()
            }
        }
    }

    pub fn get_fs() -> Result<FileSystem, Status> {
        match Self::get_protocol(&FileSystem::GUID) {
            Ok(u) => unsafe { Ok(FileSystem::from(&*(u as *mut SimpleFileSystem))) },
            Err(s) => Err(s),
        }
    }
}
