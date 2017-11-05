#[derive(Default)]
pub struct HardwareColor {
    pub brightness: u8,
    /// Pulse Width Modulation
    pub pwm: u8,
}

impl HardwareColor {
    fn from_u16(hardware: u16) -> Self {
        Self {
            brightness: (hardware >> 8) as u8,
            pwm: (hardware & 0xff) as u8,
        }
    }

    pub fn from_color(color: u8) -> Self {
        Self::from_u16(COLOR_TO_HARDWARE[color as usize])
    }

    pub fn to_color(&self) -> u8 {
        let level = self.pwm as usize * (self.brightness as usize + 1);
        LEVEL_TO_COLOR[level]
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const COLOR_TO_HARDWARE: [u16; 256] = [
    /*       0       1       2       3       4       5       6       7       8       9     */
    /*  0 */ 0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0005, 0x0006, 0x0007, 0x0008, 0x0009,
    /*  1 */ 0x000a, 0x000b, 0x000c, 0x000d, 0x000e, 0x000f, 0x0010, 0x0011, 0x0012, 0x0013,
    /*  2 */ 0x0014, 0x0015, 0x0016, 0x0017, 0x0018, 0x0019, 0x001a, 0x001b, 0x001c, 0x001d,
    /*  3 */ 0x001e, 0x001f, 0x0020, 0x0021, 0x0022, 0x0023, 0x0024, 0x0025, 0x0026, 0x0027,
    /*  4 */ 0x0028, 0x0029, 0x002a, 0x002b, 0x002c, 0x002d, 0x002e, 0x002f, 0x0030, 0x0031,
    /*  5 */ 0x0032, 0x0033, 0x0034, 0x0035, 0x0036, 0x0037, 0x0038, 0x0039, 0x003a, 0x003b,
    /*  6 */ 0x003c, 0x003d, 0x003e, 0x003f, 0x0120, 0x040d, 0x050b, 0x0311, 0x0217, 0x040e,
    /*  7 */ 0x050c, 0x0125, 0x040f, 0x0313, 0x060b, 0x050d, 0x070a, 0x0809, 0x0b07, 0x0a08,
    /*  8 */ 0x0909, 0x0c07, 0x0b08, 0x0c08, 0x0e07, 0x0b09, 0x0a0a, 0x0d08, 0x0c09, 0x0b0a,
    /*  9 */ 0x0d09, 0x090d, 0x0a0c, 0x0f08, 0x090e, 0x0a0d, 0x0810, 0x090f, 0x0713, 0x0811,
    /* 10 */ 0x0a0e, 0x0b0d, 0x0910, 0x0812, 0x0a0f, 0x0b0e, 0x0c0d, 0x0911, 0x0813, 0x0a10,
    /* 11 */ 0x0912, 0x061a, 0x0717, 0x0815, 0x0913, 0x0b10, 0x0c0f, 0x0d0e, 0x0a12, 0x0914,
    /* 12 */ 0x0b11, 0x0c10, 0x0a13, 0x0915, 0x0818, 0x061f, 0x0916, 0x0c11, 0x0d10, 0x0b13,
    /* 13 */ 0x0917, 0x0a15, 0x081a, 0x0622, 0x071e, 0x0a16, 0x081b, 0x0623, 0x071f, 0x0919,
    /* 14 */ 0x081c, 0x0720, 0x091a, 0x0a18, 0x091b, 0x0a19, 0x0b17, 0x091c, 0x0a1a, 0x0b18,
    /* 15 */ 0x091d, 0x0d15, 0x0a1b, 0x0b19, 0x0822, 0x0a1c, 0x0b1a, 0x0a1d, 0x0b1b, 0x0a1e,
    /* 16 */ 0x0b1c, 0x0a1f, 0x072b, 0x0b1d, 0x0923, 0x0a20, 0x0924, 0x0d1a, 0x0829, 0x0b1f,
    /* 17 */ 0x0e19, 0x0d1b, 0x0c1e, 0x0b21, 0x0c1f, 0x0b22, 0x0c20, 0x0b23, 0x082f, 0x0a27,
    /* 18 */ 0x0b24, 0x0a28, 0x0b25, 0x092d, 0x0c23, 0x0739, 0x0a2a, 0x0b27, 0x0d22, 0x0b28,
    /* 19 */ 0x0a2c, 0x0b29, 0x0a2d, 0x0932, 0x0b2a, 0x0c27, 0x0b2b, 0x0c28, 0x0b2c, 0x0c29,
    /* 20 */ 0x0b2d, 0x0c2a, 0x0b2e, 0x0c2b, 0x0b2f, 0x0e26, 0x0c2c, 0x0b30, 0x0c2d, 0x0a36,
    /* 21 */ 0x0c2e, 0x0b32, 0x0c2f, 0x0d2c, 0x0c30, 0x0d2d, 0x0b35, 0x0d2e, 0x0c32, 0x0d2f,
    /* 22 */ 0x0c33, 0x0d30, 0x0c34, 0x0d31, 0x0e2e, 0x0d32, 0x0d33, 0x0d34, 0x0e31, 0x0d35,
    /* 23 */ 0x0d36, 0x0c3b, 0x0d37, 0x0d38, 0x0c3d, 0x0d39, 0x0e36, 0x0c3f, 0x0d3b, 0x0d3c,
    /* 24 */ 0x0d3d, 0x0e3a, 0x0d3f, 0x0e3b, 0x0f38, 0x0e3c, 0x0f39, 0x0e3d, 0x0f3a, 0x0e3e,
    /* 25 */ 0x0f3b, 0x0e3f, 0x0f3c, 0x0f3d, 0x0f3e, 0x0f3f,
];

#[cfg_attr(rustfmt, rustfmt_skip)]
const LEVEL_TO_COLOR: [u8; 1009] = [
    /*        0     1     2     3     4     5     6     7     8     9   */
    /*   0 */ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
    /*   1 */ 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13,
    /*   2 */ 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
    /*   3 */ 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
    /*   4 */ 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31,
    /*   5 */ 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
    /*   6 */ 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x42, 0x43, 0x44,
    /*   7 */ 0x45, 0x45, 0x46, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4b,
    /*   8 */ 0x4c, 0x4d, 0x4d, 0x4e, 0x4e, 0x4e, 0x4e, 0x4f, 0x4f, 0x4f,
    /*   9 */ 0x50, 0x51, 0x51, 0x51, 0x52, 0x52, 0x52, 0x52, 0x52, 0x52,
    /*  10 */ 0x52, 0x53, 0x53, 0x53, 0x53, 0x54, 0x54, 0x55, 0x55, 0x55,
    /*  11 */ 0x56, 0x56, 0x57, 0x57, 0x57, 0x58, 0x58, 0x58, 0x58, 0x59,
    /*  12 */ 0x59, 0x59, 0x59, 0x59, 0x5a, 0x5a, 0x5a, 0x5a, 0x5d, 0x5b,
    /*  13 */ 0x5b, 0x5b, 0x5c, 0x5c, 0x5d, 0x5d, 0x5d, 0x5d, 0x5e, 0x5e,
    /*  14 */ 0x5e, 0x5e, 0x5f, 0x5f, 0x60, 0x60, 0x60, 0x60, 0x61, 0x61,
    /*  15 */ 0x61, 0x61, 0x62, 0x63, 0x64, 0x64, 0x65, 0x65, 0x65, 0x66,
    /*  16 */ 0x66, 0x66, 0x67, 0x67, 0x68, 0x68, 0x68, 0x69, 0x69, 0x6a,
    /*  17 */ 0x6b, 0x6c, 0x6c, 0x6c, 0x6d, 0x6d, 0x6d, 0x6d, 0x6d, 0x6e,
    /*  18 */ 0x6e, 0x6e, 0x6f, 0x6f, 0x70, 0x70, 0x70, 0x71, 0x71, 0x71,
    /*  19 */ 0x72, 0x72, 0x73, 0x73, 0x74, 0x74, 0x75, 0x75, 0x76, 0x76,
    /*  20 */ 0x77, 0x77, 0x77, 0x78, 0x78, 0x78, 0x78, 0x79, 0x79, 0x7a,
    /*  21 */ 0x7b, 0x7b, 0x7b, 0x7b, 0x7c, 0x7c, 0x7c, 0x7d, 0x7d, 0x7e,
    /*  22 */ 0x7e, 0x7f, 0x7f, 0x80, 0x80, 0x80, 0x80, 0x81, 0x81, 0x81,
    /*  23 */ 0x82, 0x83, 0x83, 0x84, 0x84, 0x84, 0x84, 0x85, 0x85, 0x85,
    /*  24 */ 0x86, 0x86, 0x87, 0x88, 0x88, 0x89, 0x89, 0x8a, 0x8a, 0x8a,
    /*  25 */ 0x8b, 0x8b, 0x8c, 0x8c, 0x8c, 0x8d, 0x8d, 0x8d, 0x8d, 0x8e,
    /*  26 */ 0x8e, 0x8e, 0x8e, 0x8f, 0x8f, 0x8f, 0x8f, 0x8f, 0x90, 0x90,
    /*  27 */ 0x90, 0x90, 0x90, 0x91, 0x91, 0x91, 0x92, 0x92, 0x92, 0x93,
    /*  28 */ 0x93, 0x93, 0x93, 0x93, 0x94, 0x94, 0x94, 0x94, 0x95, 0x95,
    /*  29 */ 0x96, 0x96, 0x96, 0x97, 0x97, 0x97, 0x98, 0x98, 0x98, 0x99,
    /*  30 */ 0x99, 0x99, 0x99, 0x99, 0x9a, 0x9a, 0x9a, 0x9a, 0x9b, 0x9b,
    /*  31 */ 0x9b, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9d, 0x9d, 0x9d, 0x9d,
    /*  32 */ 0x9d, 0x9d, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9f, 0x9f,
    /*  33 */ 0x9f, 0x9f, 0x9f, 0x9f, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa1,
    /*  34 */ 0xa1, 0xa1, 0xa1, 0xa2, 0xa2, 0xa2, 0xa2, 0xa3, 0xa3, 0xa3,
    /*  35 */ 0xa4, 0xa4, 0xa5, 0xa5, 0xa5, 0xa5, 0xa5, 0xa6, 0xa6, 0xa6,
    /*  36 */ 0xa6, 0xa6, 0xa6, 0xa7, 0xa7, 0xa7, 0xa7, 0xa8, 0xa8, 0xa8,
    /*  37 */ 0xa8, 0xa9, 0xa9, 0xa9, 0xaa, 0xaa, 0xaa, 0xab, 0xab, 0xab,
    /*  38 */ 0xab, 0xab, 0xab, 0xab, 0xab, 0xac, 0xac, 0xac, 0xac, 0xac,
    /*  39 */ 0xac, 0xac, 0xac, 0xac, 0xad, 0xad, 0xad, 0xad, 0xad, 0xad,
    /*  40 */ 0xae, 0xae, 0xae, 0xae, 0xae, 0xae, 0xaf, 0xaf, 0xaf, 0xaf,
    /*  41 */ 0xaf, 0xaf, 0xaf, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb1,
    /*  42 */ 0xb1, 0xb1, 0xb2, 0xb2, 0xb2, 0xb2, 0xb2, 0xb3, 0xb3, 0xb3,
    /*  43 */ 0xb3, 0xb4, 0xb4, 0xb4, 0xb4, 0xb4, 0xb4, 0xb5, 0xb5, 0xb5,
    /*  44 */ 0xb5, 0xb5, 0xb5, 0xb6, 0xb6, 0xb6, 0xb6, 0xb6, 0xb7, 0xb7,
    /*  45 */ 0xb7, 0xb7, 0xb7, 0xb8, 0xb8, 0xb8, 0xb9, 0xb9, 0xb9, 0xb9,
    /*  46 */ 0xba, 0xba, 0xba, 0xba, 0xba, 0xba, 0xbb, 0xbb, 0xbb, 0xbb,
    /*  47 */ 0xbb, 0xbb, 0xbb, 0xbc, 0xbc, 0xbc, 0xbc, 0xbc, 0xbc, 0xbd,
    /*  48 */ 0xbd, 0xbd, 0xbd, 0xbe, 0xbe, 0xbe, 0xbe, 0xbe, 0xbe, 0xbf,
    /*  49 */ 0xbf, 0xbf, 0xbf, 0xbf, 0xc0, 0xc0, 0xc0, 0xc0, 0xc1, 0xc1,
    /*  50 */ 0xc1, 0xc1, 0xc1, 0xc2, 0xc2, 0xc2, 0xc3, 0xc3, 0xc3, 0xc3,
    /*  51 */ 0xc3, 0xc3, 0xc4, 0xc4, 0xc4, 0xc4, 0xc4, 0xc4, 0xc4, 0xc5,
    /*  52 */ 0xc5, 0xc5, 0xc5, 0xc5, 0xc5, 0xc6, 0xc6, 0xc6, 0xc6, 0xc6,
    /*  53 */ 0xc6, 0xc7, 0xc7, 0xc7, 0xc7, 0xc7, 0xc7, 0xc8, 0xc8, 0xc8,
    /*  54 */ 0xc8, 0xc8, 0xc8, 0xc8, 0xc9, 0xc9, 0xc9, 0xc9, 0xc9, 0xc9,
    /*  55 */ 0xca, 0xca, 0xca, 0xca, 0xca, 0xca, 0xcb, 0xcb, 0xcb, 0xcb,
    /*  56 */ 0xcb, 0xcb, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcd, 0xcd,
    /*  57 */ 0xcd, 0xcd, 0xce, 0xce, 0xce, 0xcf, 0xcf, 0xcf, 0xcf, 0xcf,
    /*  58 */ 0xcf, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0,
    /*  59 */ 0xd1, 0xd1, 0xd1, 0xd1, 0xd1, 0xd1, 0xd1, 0xd2, 0xd2, 0xd2,
    /*  60 */ 0xd3, 0xd3, 0xd3, 0xd3, 0xd3, 0xd3, 0xd4, 0xd4, 0xd4, 0xd4,
    /*  61 */ 0xd4, 0xd4, 0xd4, 0xd4, 0xd5, 0xd5, 0xd5, 0xd5, 0xd5, 0xd5,
    /*  62 */ 0xd5, 0xd6, 0xd6, 0xd6, 0xd6, 0xd6, 0xd6, 0xd6, 0xd7, 0xd7,
    /*  63 */ 0xd7, 0xd7, 0xd7, 0xd7, 0xd8, 0xd8, 0xd8, 0xd8, 0xd8, 0xd8,
    /*  64 */ 0xd8, 0xd9, 0xd9, 0xd9, 0xd9, 0xd9, 0xd9, 0xd9, 0xda, 0xda,
    /*  65 */ 0xda, 0xda, 0xda, 0xda, 0xda, 0xdb, 0xdb, 0xdb, 0xdb, 0xdb,
    /*  66 */ 0xdb, 0xdc, 0xdc, 0xdc, 0xdc, 0xdc, 0xdc, 0xdc, 0xdd, 0xdd,
    /*  67 */ 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xde, 0xde, 0xde, 0xde, 0xde,
    /*  68 */ 0xde, 0xde, 0xdf, 0xdf, 0xdf, 0xdf, 0xdf, 0xdf, 0xdf, 0xe0,
    /*  69 */ 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe1, 0xe1, 0xe1, 0xe1,
    /*  70 */ 0xe1, 0xe1, 0xe1, 0xe1, 0xe1, 0xe1, 0xe1, 0xe1, 0xe2, 0xe2,
    /*  71 */ 0xe2, 0xe2, 0xe2, 0xe2, 0xe2, 0xe2, 0xe2, 0xe2, 0xe2, 0xe2,
    /*  72 */ 0xe2, 0xe2, 0xe3, 0xe3, 0xe3, 0xe3, 0xe3, 0xe3, 0xe3, 0xe3,
    /*  73 */ 0xe3, 0xe3, 0xe4, 0xe4, 0xe4, 0xe4, 0xe4, 0xe4, 0xe4, 0xe5,
    /*  74 */ 0xe5, 0xe5, 0xe5, 0xe5, 0xe5, 0xe5, 0xe5, 0xe5, 0xe5, 0xe5,
    /*  75 */ 0xe6, 0xe6, 0xe6, 0xe6, 0xe6, 0xe6, 0xe6, 0xe6, 0xe6, 0xe6,
    /*  76 */ 0xe6, 0xe6, 0xe7, 0xe7, 0xe7, 0xe7, 0xe7, 0xe7, 0xe7, 0xe8,
    /*  77 */ 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe9, 0xe9,
    /*  78 */ 0xe9, 0xe9, 0xe9, 0xe9, 0xe9, 0xe9, 0xe9, 0xe9, 0xe9, 0xea,
    /*  79 */ 0xea, 0xea, 0xea, 0xea, 0xea, 0xea, 0xeb, 0xeb, 0xeb, 0xeb,
    /*  80 */ 0xeb, 0xeb, 0xeb, 0xeb, 0xeb, 0xec, 0xec, 0xec, 0xec, 0xec,
    /*  81 */ 0xec, 0xec, 0xec, 0xec, 0xec, 0xed, 0xed, 0xed, 0xed, 0xed,
    /*  82 */ 0xed, 0xed, 0xed, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee,
    /*  83 */ 0xee, 0xee, 0xee, 0xee, 0xef, 0xef, 0xef, 0xef, 0xef, 0xef,
    /*  84 */ 0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 0xf0, 0xf0,
    /*  85 */ 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0,
    /*  86 */ 0xf0, 0xf0, 0xf0, 0xf1, 0xf1, 0xf1, 0xf1, 0xf1, 0xf1, 0xf1,
    /*  87 */ 0xf1, 0xf1, 0xf1, 0xf1, 0xf1, 0xf1, 0xf1, 0xf2, 0xf2, 0xf2,
    /*  88 */ 0xf2, 0xf2, 0xf2, 0xf2, 0xf3, 0xf3, 0xf3, 0xf3, 0xf3, 0xf3,
    /*  89 */ 0xf3, 0xf4, 0xf4, 0xf4, 0xf4, 0xf4, 0xf4, 0xf4, 0xf4, 0xf5,
    /*  90 */ 0xf5, 0xf5, 0xf5, 0xf5, 0xf5, 0xf5, 0xf5, 0xf6, 0xf6, 0xf6,
    /*  91 */ 0xf6, 0xf6, 0xf6, 0xf6, 0xf7, 0xf7, 0xf7, 0xf7, 0xf7, 0xf7,
    /*  92 */ 0xf7, 0xf7, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8, 0xf8,
    /*  93 */ 0xf9, 0xf9, 0xf9, 0xf9, 0xf9, 0xf9, 0xf9, 0xf9, 0xfa, 0xfa,
    /*  94 */ 0xfa, 0xfa, 0xfa, 0xfa, 0xfa, 0xfb, 0xfb, 0xfb, 0xfb, 0xfb,
    /*  95 */ 0xfb, 0xfb, 0xfb, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc,
    /*  96 */ 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfc, 0xfd,
    /*  97 */ 0xfd, 0xfd, 0xfd, 0xfd, 0xfd, 0xfd, 0xfd, 0xfd, 0xfd, 0xfd,
    /*  98 */ 0xfd, 0xfd, 0xfd, 0xfd, 0xfd, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
    /*  99 */ 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
    /* 100 */ 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];