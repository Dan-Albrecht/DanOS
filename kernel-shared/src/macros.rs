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
