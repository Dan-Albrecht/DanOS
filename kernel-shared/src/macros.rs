// Compile time guaranteed size of a type is at most u32::MAX
#[macro_export]
macro_rules! guaranteed_size_of_u32 {
    ($t:ty) => {{
        const SIZE: u32 = {
            let s = core::mem::size_of::<$t>();
            assert!(s <= u32::MAX as usize);
            s as u32
        };
        SIZE
    }};
}

#[macro_export]
macro_rules! u64_to_usize {
    ($val:expr) => {{
        const _: () = assert!(u64::MAX as u128 <= usize::MAX as u128);
        $val as usize
    }};
}
