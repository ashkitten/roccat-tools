#[macro_export]
macro_rules! impl_hidraw {
    (
        readwrite, report_id: $report_id:expr;
        $(#[$meta:meta])*
        pub struct $name:ident {
            $(pub $field_name:ident: $field_type:ty,)+
        }
    ) => (
        $(#[$meta])*
        #[derive(Clone)]
        pub struct $name {
            _report_id: u8,
            _size: u8,
            $(pub $field_name: $field_type,)+
        }

        impl $name {
            ioctl!(readwrite hidraw_read with b'H', 0x07; Self);
            ioctl!(readwrite hidraw_write with b'H', 0x06; Self);

            pub fn new($($field_name: $field_type),+) -> Self {
                Self {
                    _report_id: $report_id,
                    _size: ::std::mem::size_of::<Self>() as u8,
                    $($field_name: $field_name,)+
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new($(<$field_type>::default()),+)
            }
        }

        impl ::device::HidrawData for $name {
            fn read(path: &::std::path::Path) -> Result<Self> {
                let file = ::std::fs::OpenOptions::new().read(true).write(true).open(path)?;
                let mut data = Self::default();
                unsafe {
                    use std::os::unix::io::AsRawFd;
                    Self::hidraw_read(file.as_raw_fd(), &mut data as *mut Self)?;
                }
                Ok(data)
            }

            fn write(path: &::std::path::Path, data: &Self) -> Result<()> {
                let file = ::std::fs::OpenOptions::new().read(true).write(true).open(path)?;
                let mut data = data.clone();
                unsafe {
                    use std::os::unix::io::AsRawFd;
                    Self::hidraw_write(file.as_raw_fd(), &mut data as *mut Self)?;
                }
                Ok(())
            }
        }
    );

    (
        read, report_id: $report_id:expr;
        $(#[$meta:meta])*
        pub struct $name:ident {
            $(pub $field_name:ident: $field_type:ty,)+
        }
    ) => (
        $(#[$meta])*
        #[derive(Clone)]
        pub struct $name {
            _report_id: u8,
            _size: u8,
            $(pub $field_name: $field_type,)+
        }

        impl $name {
            ioctl!(readwrite hidraw_read with b'H', 0x07; Self);
            ioctl!(readwrite hidraw_write with b'H', 0x06; Self);

            pub fn new($($field_name: $field_type),+) -> Self {
                Self {
                    _report_id: $report_id,
                    _size: ::std::mem::size_of::<Self>() as u8,
                    $($field_name: $field_name,)+
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new($(<$field_type>::default()),+)
            }
        }

        impl ::device::HidrawData for $name {
            fn read(path: &::std::path::Path) -> Result<Self> {
                let file = ::std::fs::OpenOptions::new().read(true).write(true).open(path)?;
                let mut data = Self::default();
                unsafe {
                    use std::os::unix::io::AsRawFd;
                    Self::hidraw_read(file.as_raw_fd(), &mut data as *mut Self)?;
                }
                Ok(data)
            }

            fn write(_path: &::std::path::Path, _data: &Self) -> Result<()> {
                bail!(stringify!($name).to_owned() + " is read-only");
            }
        }
    );

    (
        write, report_id: $report_id:expr;
        $(#[$meta:meta])*
        pub struct $name:ident {
            $(pub $field_name:ident: $field_type:ty,)+
        }
    ) => (
        $(#[$meta])*
        #[derive(Clone)]
        pub struct $name {
            _report_id: u8,
            _size: u8,
            $(pub $field_name: $field_type,)+
        }

        impl $name {
            ioctl!(readwrite hidraw_read with b'H', 0x07; Self);
            ioctl!(readwrite hidraw_write with b'H', 0x06; Self);

            pub fn new($($field_name: $field_type),+) -> Self {
                Self {
                    _report_id: $report_id,
                    _size: ::std::mem::size_of::<Self>() as u8,
                    $($field_name: $field_name,)+
                }
            }
        }

        impl default for $name {
            fn default() -> self {
                Self::new($(<$field_type>::default()),+)
            }
        }

        impl ::device::HidrawData for $name {
            fn read(_path: &::std::path::Path) -> Result<Self> {
                bail!(stringify!($name).to_owned() + " is write-only");
            }

            fn write(path: &::std::path::Path, data: &Self) -> Result<()> {
                let file = ::std::fs::OpenOptions::new().read(true).write(true).open(path)?;
                let mut data = data.clone();
                unsafe {
                    use std::os::unix::io::AsRawFd;
                    Self::hidraw_write(file.as_raw_fd(), &mut data as *mut Self)?;
                }
                Ok(())
            }
        }
    );
}
