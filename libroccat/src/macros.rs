#[macro_export]
macro_rules! impl_hidraw {
    () => (
        ioctl!(readwrite hidraw_read with b'H', 0x07; Self);
        ioctl!(readwrite hidraw_write with b'H', 0x06; Self);
    )
}
