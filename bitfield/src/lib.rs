pub trait BitField {
    fn bit_length() -> usize;
    fn get_bit(&self, index: usize) -> bool;
    fn set_bit(&mut self, index: usize, value: bool) -> &mut Self;
}

pub trait BitArray<T: BitField> {
    fn bit_length(&self) -> usize;
    fn get_bit(&self, index: usize) -> bool;
    fn set_bit(&mut self, index: usize, value: bool);
}

macro_rules! bitfield_impl {
    ($($t:ty)*) => ($(
        impl BitField for $t {
            fn bit_length() -> usize {
                ::std::mem::size_of::<Self>() * 8
            }

            fn get_bit(&self, index: usize) -> bool {
                assert!(index < Self::bit_length());

                (*self & (1 << index)) != 0
            }

            fn set_bit(&mut self, index: usize, value: bool) -> &mut Self {
                assert!(index < Self::bit_length());

                if value {
                    *self |= 1 << index;
                } else {
                    *self &= !(1 << index);
                }

                self
            }
        }
    )*)
}

bitfield_impl! { u8 u16 u32 u64 usize }

impl<T: BitField> BitArray<T> for [T] {
    fn bit_length(&self) -> usize {
        self.len() * (T::bit_length() as usize)
    }

    fn get_bit(&self, bit: usize) -> bool {
        let slice_index = bit / T::bit_length();
        let bit_index = bit % T::bit_length();
        self[slice_index].get_bit(bit_index)
    }

    fn set_bit(&mut self, bit: usize, value: bool) {
        let slice_index = bit / T::bit_length();
        let bit_index = bit % T::bit_length();
        self[slice_index].set_bit(bit_index, value);
    }
}

pub trait NibbleField {
    fn nibble_length() -> usize;
    fn get_nibble(&self, index: usize) -> u8;
    fn set_nibble(&mut self, index: usize, value: u8) -> &mut Self;
}

pub trait NibbleArray<T: NibbleField> {
    fn nibble_length(&self) -> usize;
    fn get_nibble(&self, bit: usize) -> u8;
    fn set_nibble(&mut self, bit: usize, value: u8);
}

macro_rules! nibblefield_impl {
    ($($t:ty)*) => ($(
        impl NibbleField for $t {
            fn nibble_length() -> usize {
                ::std::mem::size_of::<Self>() * 2
            }

            fn get_nibble(&self, index: usize) -> u8 {
                assert!(index < Self::nibble_length());

                (*self >> (index * 4)) as u8 & 0x0f
            }

            fn set_nibble(&mut self, index: usize, value: u8) -> &mut Self {
                assert!(index < Self::nibble_length());
                assert!(value <= 0xf, "Value is more than a nibble");

                let index = index as u32 * 4;

                *self &= (<$t>::max_value()) ^ 0xf << index;
                *self |= (<$t>::from(value)) << index;

                self
            }
        }
    )*)
}

#[cfg(test)]
#[test]
fn nibble() {
    assert_eq!(0b0110, 0b10010110u8.get_nibble(0));
    assert_eq!(&mut 0b00001111u8, 0b00000001u8.set_nibble(0, 0b1111));
    assert_eq!(&mut 0b01101001u8, 0b10011001u8.set_nibble(1, 0b0110));
}

nibblefield_impl! { u8 u16 u32 u64 usize }

impl<T: NibbleField> NibbleArray<T> for [T] {
    fn nibble_length(&self) -> usize {
        self.len() * (T::nibble_length() as usize)
    }

    fn get_nibble(&self, index: usize) -> u8 {
        let slice_index = index / T::nibble_length();
        let nibble_index = index % T::nibble_length();
        self[slice_index].get_nibble(nibble_index)
    }

    fn set_nibble(&mut self, index: usize, value: u8) {
        let slice_index = index / T::nibble_length();
        let nibble_index = index % T::nibble_length();
        self[slice_index].set_nibble(nibble_index, value);
    }
}
