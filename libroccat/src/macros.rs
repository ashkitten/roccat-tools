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
        #[derive(Clone, Default)]
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

            pub fn read(interface: &::std::fs::File) -> Result<Self> {
                use std::os::unix::io::AsRawFd;
                use nix::{Error, Errno};

                let mut data = Self {
                    $($const_field_name: $const_field_val,)*
                    .. Default::default()
                };
                unsafe {
                    let mut errors = 0;
                    while errors < 10 {
                        match Self::hidraw_read(interface.as_raw_fd(), &mut data as *mut Self) {
                            Ok(_) => break,
                            Err(Error::Sys(Errno::EINTR)) => errors += 1,
                            Err(Error::Sys(Errno::EAGAIN)) => errors += 1,
                            Err(Error::Sys(Errno::ETIMEDOUT)) => errors += 1,
                            Err(other) => return Err(other.into()),
                        }
                    }
                }
                Ok(data)
            }

            pub fn write(interface: &::std::fs::File, data: &Self) -> Result<()> {
                use std::os::unix::io::AsRawFd;
                use nix::{Error, Errno};

                let mut data = data.clone();
                unsafe {
                    let mut errors = 0;
                    while errors < 10 {
                        match Self::hidraw_write(interface.as_raw_fd(), &mut data as *mut Self) {
                            Ok(_) => break,
                            Err(Error::Sys(Errno::EINTR)) => errors += 1,
                            Err(Error::Sys(Errno::EAGAIN)) => errors += 1,
                            Err(Error::Sys(Errno::ETIMEDOUT)) => errors += 1,
                            Err(other) => return Err(other.into()),
                        }
                    }
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
        #[derive(Clone, Default)]
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

            pub fn read(interface: &::std::fs::File) -> Result<Self> {
                use std::os::unix::io::AsRawFd;
                use nix::{Error, Errno};

                let mut data = Self {
                    $($const_field_name: $const_field_val,)*
                    .. Default::default()
                };
                unsafe {
                    let mut errors = 0;
                    while errors < 10 {
                        match Self::hidraw_read(interface.as_raw_fd(), &mut data as *mut Self) {
                            Ok(_) => break,
                            Err(Error::Sys(Errno::EINTR)) => errors += 1,
                            Err(Error::Sys(Errno::EAGAIN)) => errors += 1,
                            Err(Error::Sys(Errno::ETIMEDOUT)) => errors += 1,
                            Err(other) => return Err(other.into()),
                        }
                    }
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
        #[derive(Clone, Default)]
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

            pub fn write(interface: &::std::fs::File, data: &Self) -> Result<()> {
                use std::os::unix::io::AsRawFd;
                use nix::{Error, Errno};

                let mut data = data.clone();
                unsafe {
                    let mut errors = 0;
                    while errors < 10 {
                        match Self::hidraw_write(interface.as_raw_fd(), &mut data as *mut Self) {
                            Ok(_) => break,
                            Err(Error::Sys(Errno::EINTR)) => errors += 1,
                            Err(Error::Sys(Errno::EAGAIN)) => errors += 1,
                            Err(Error::Sys(Errno::ETIMEDOUT)) => errors += 1,
                            Err(other) => return Err(other.into()),
                        }
                    }
                }
                Ok(())
            }
        }
    );
}
