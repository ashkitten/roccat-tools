#[macro_export]
macro_rules! impl_hidraw {
    (
        readwrite;
        $(#[$meta:meta])*
        $name:ident {
            $(@constant $const_field_name:ident: $const_field_type:ty = $const_field_val:expr,)*
            $($field_name:ident: $field_type:ty,)+
        }
    ) => (
        $(#[$meta])*
        #[derive(Clone, Default, Debug)]
        #[repr(packed)]
        pub struct $name {
            $($const_field_name: $const_field_type,)*
            $(pub $field_name: $field_type,)+
        }

        impl $name {
            ioctl!(readwrite hidraw_read with b'H', 0x07; Self);
            ioctl!(readwrite hidraw_write with b'H', 0x06; Self);

            pub fn new($($field_name: $field_type),+) -> Self {
                Self {
                    $($const_field_name: $const_field_val,)*
                    $($field_name: $field_name,)+
                }
            }

            pub fn read(path: &::std::path::Path) -> Result<Self> {
                let file = ::std::fs::OpenOptions::new().read(true).write(true).open(path)?;
                let mut data = Self {
                    $($const_field_name: $const_field_val,)*
                    .. Default::default()
                };
                unsafe {
                    use std::os::unix::io::AsRawFd;
                    Self::hidraw_read(file.as_raw_fd(), &mut data as *mut Self)?;
                }
                Ok(data)
            }

            pub fn write(path: &::std::path::Path, data: &Self) -> Result<()> {
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
        read;
        $(#[$meta:meta])*
        $name:ident {
            $(@constant $const_field_name:ident: $const_field_type:ty = $const_field_val:expr,)*
            $($field_name:ident: $field_type:ty,)+
        }
    ) => (
        $(#[$meta])*
        #[derive(Clone, Default, Debug)]
        #[repr(packed)]
        pub struct $name {
            $($const_field_name: $const_field_type,)*
            $(pub $field_name: $field_type,)+
        }

        impl $name {
            ioctl!(readwrite hidraw_read with b'H', 0x07; Self);
            ioctl!(readwrite hidraw_write with b'H', 0x06; Self);

            pub fn new($($field_name: $field_type),+) -> Self {
                Self {
                    $($const_field_name: $const_field_val,)*
                    $($field_name: $field_name,)+
                }
            }

            pub fn read(path: &::std::path::Path) -> Result<Self> {
                let file = ::std::fs::OpenOptions::new().read(true).write(true).open(path)?;
                let mut data = Self {
                    $($const_field_name: $const_field_val,)*
                    .. Default::default()
                };
                unsafe {
                    use std::os::unix::io::AsRawFd;
                    Self::hidraw_read(file.as_raw_fd(), &mut data as *mut Self)?;
                }
                Ok(data)
            }
        }
    );

    (
        write;
        $(#[$meta:meta])*
        $name:ident {
            $(@constant $const_field_name:ident: $const_field_type:ty = $const_field_val:expr,)*
            $($field_name:ident: $field_type:ty,)+
        }
    ) => (
        $(#[$meta])*
        #[derive(Clone, Default, Debug)]
        #[repr(packed)]
        pub struct $name {
            $($const_field_name: $const_field_type,)*
            $(pub $field_name: $field_type,)+
        }

        impl $name {
            ioctl!(readwrite hidraw_read with b'H', 0x07; Self);
            ioctl!(readwrite hidraw_write with b'H', 0x06; Self);

            pub fn new($($field_name: $field_type),+) -> Self {
                Self {
                    $($const_field_name: $const_field_val,)*
                    $($field_name: $field_name,)+
                }
            }

            pub fn write(path: &::std::path::Path, data: &Self) -> Result<()> {
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
