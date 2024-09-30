pub trait MemoryStuff {
    fn allocate<T>(&mut self) -> *mut T;
    fn free(&mut self, address: usize);
}
