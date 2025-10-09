use aos_uefi::{
    fs::{file::File, fileinfo::FileInfo, sfs::SimpleFileSystem},
    guid::Guid,
    status::Status,
};

pub struct FileSystem(&'static SimpleFileSystem);

impl FileSystem {
    pub const GUID: Guid = Guid::new(
        0x0964e5b22,
        0x6459,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}

impl FileSystem {
    pub fn root(&self) -> Result<DirObject, Status> {
        todo!()
    }
}

pub struct FileObject(&'static File);

pub struct DirObject(&'static File);

impl From<&'static File> for DirObject {
    fn from(value: &'static File) -> Self {
        Self(value)
    }
}

impl From<&'static SimpleFileSystem> for DirObject {
    /// Gets root directory of the filesystem.
    ///
    /// This function can panic.
    fn from(value: &'static SimpleFileSystem) -> Self {
        let mut buf = [0u8; size_of::<File>()];
        let status = (value.open_volume)(value, &mut (buf.as_mut_ptr() as *mut _ as *mut File));
        match status {
            Status::SUCCESS => Self(unsafe { &*(buf.as_ptr() as *const _ as *const File) }),
            _s => panic!("Could not get root!"),
        }
    }
}

impl DirObject {
    pub fn next_entry(&self) -> Result<Option<FileInfo>, Status> {
        let mut len = size_of::<FileInfo>();
        let mut bytes = [0u8; size_of::<FileInfo>()];
        let status = (self.0.read)(&self.0, &mut len, bytes.as_mut_ptr());

        match status {
            Status::SUCCESS => match len {
                0 => Ok(None),
                _ => Ok(Some(unsafe {
                    *(bytes.as_mut_ptr() as *mut _ as *mut FileInfo)
                })),
            },
            _s => Err(_s),
        }
    }
}
