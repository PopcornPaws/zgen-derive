pub trait MaxSerializedLen {
    const MAX_SERIALIZED_LEN: usize;
}

macro_rules! max_serialized_len {
    ($this:ty, $len:expr) => {
        impl MaxSerializedLen for $this {
            const MAX_SERIALIZED_LEN: usize = $len;
        }
    };
}

max_serialized_len!(bool, 1);
max_serialized_len!(u8, 1);
max_serialized_len!(u16, 2);
max_serialized_len!(u32, 4);
max_serialized_len!(u64, 8);
max_serialized_len!(u128, 16);
max_serialized_len!(i8, 1);
max_serialized_len!(i16, 2);
max_serialized_len!(i32, 4);
max_serialized_len!(i64, 8);
max_serialized_len!(i128, 16);
// NOTE for faster compilation this is commented out
//use solana_program::pubkey::Pubkey;
//max_serialized_len!(Pubkey, 32);

impl<T> MaxSerializedLen for Option<T>
where
    T: MaxSerializedLen,
{
    const MAX_SERIALIZED_LEN: usize = 1 + T::MAX_SERIALIZED_LEN;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn primitive_types() {
        assert_eq!(bool::MAX_SERIALIZED_LEN, 1);
        assert_eq!(u8::MAX_SERIALIZED_LEN, 1);
        assert_eq!(u16::MAX_SERIALIZED_LEN, 2);
        assert_eq!(u32::MAX_SERIALIZED_LEN, 4);
        assert_eq!(u64::MAX_SERIALIZED_LEN, 8);
        assert_eq!(u128::MAX_SERIALIZED_LEN, 16);
        assert_eq!(Option::<u8>::MAX_SERIALIZED_LEN, 2);
        assert_eq!(Option::<[u8; 32]>::MAX_SERIALIZED_LEN, 33);
    }
}
