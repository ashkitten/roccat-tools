#[cfg_attr(rustfmt, rustfmt_skip)]
const SDK_INDEX_TO_LIGHT_INDEX: [u8; 120] = [
    /*       0     1     2     3     4     5     6     7     8     9   */
    /*  0 */ 0x01, 0x02, 0x0e, 0x03, 0x0f, 0x04, 0x10, 0x05, 0x11, 0x06,
    /*  1 */ 0x07, 0x08, 0x14, 0x09, 0x15, 0x21, 0x00, 0x0c, 0x0d, 0x19,
    /*  2 */ 0x1a, 0x1b, 0x27, 0x1c, 0x28, 0x1d, 0x29, 0x12, 0x1e, 0x13,
    /*  3 */ 0x2c, 0x2d, 0x39, 0x45, 0x0a, 0x16, 0x0b, 0x17, 0x18, 0x24,
    /*  4 */ 0x25, 0x31, 0x26, 0x32, 0x33, 0x34, 0x40, 0x35, 0x2a, 0x36,
    /*  5 */ 0x1f, 0x2b, 0x38, 0x51, 0x5d, 0x69, 0x22, 0x2e, 0x23, 0x2f,
    /*  6 */ 0x30, 0x3c, 0x3d, 0x49, 0x3e, 0x4a, 0x3f, 0x4b, 0x4c, 0x41,
    /*  7 */ 0x42, 0x4e, 0x37, 0x44, 0x3a, 0x46, 0x3b, 0x48, 0x54, 0x55,
    /*  8 */ 0x61, 0x56, 0x62, 0x57, 0x63, 0x58, 0x4d, 0x59, 0x5a, 0x4f,
    /*  9 */ 0x50, 0x75, 0x52, 0x5e, 0x53, 0x5f, 0x60, 0x6c, 0x6d, 0x6e,
    /* 10 */ 0x70, 0x72, 0x73, 0x67, 0x5c, 0x68, 0x74, 0x76, 0x6a, 0x6b,
    /* 11 */ 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

pub fn sdk_index_to_light_index(sdk_index: u8) -> u8 {
    SDK_INDEX_TO_LIGHT_INDEX[sdk_index as usize]
}
