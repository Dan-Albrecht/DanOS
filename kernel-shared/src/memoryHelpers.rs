use core::mem::size_of;

pub unsafe fn zeroMemory(address: usize, ammount: usize) {
    assert!(ammount <= isize::MAX as usize);
    let pointer = address as *mut u8;
    for index in 0..ammount as isize {
        *pointer.offset(index) = 0;
    }
}

pub unsafe fn zeroMemory2<T>(address: *const T) {
    let address = address as usize;
    let ammount = size_of::<T>();
    zeroMemory(address, ammount);
}
